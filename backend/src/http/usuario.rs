use crate::{
    AppState,
    mail::{EmailType, send_email},
    repos::usuario::{
        self, UsuarioInfo, UsuarioJwt, actualizar_estado_usuario, contar_usuarios,
        get_usuario_actual, jwt, listar_usuarios, login_usuario, actualizar_rol_usuario,
    },
    tipos::{Respuesta, Ress},
};
use axum::{
    Extension, Json,
    extract::{Query, State},
    http::{StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Json as Js, Response},
};
use serde::Deserialize;
use serde_json::json;
use sqlx::types::Uuid;
use std::sync::Arc;
use tracing::{debug, error};

#[derive(Deserialize)]
pub struct UsuariosQuery {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub provincia_id: Option<i32>,
    pub municipio_id: Option<i32>,
    pub oficina_id: Option<i32>,
}

#[derive(serde::Serialize)]
struct PaginatedResponse<T> {
    usuarios: T,
    total: i64,
    page: usize,
    limit: usize,
}

#[derive(Debug, Deserialize)]
pub struct UsuarioDto {
    email: String,
    pass_word: String,
    persona_id: Option<String>,
}

#[derive(Deserialize)]
pub struct EnviarCorreoUsuarioDto {
    usuario_id: String,
    asunto: String,
    cuerpo: String,
}

#[derive(Deserialize)]
pub struct ActualizarEstadoUsuarioDto {
    usuario_id: String,
    activo: bool,
    motivo: Option<String>,
}

#[derive(Deserialize)]
pub struct ActualizarRolUsuarioDto {
    usuario_id: String,
    oficina_id: Option<i32>,
}

fn is_valid_email(email: &str) -> bool {
    if email.len() < 3 {
        return false;
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }

    let local = parts[0];
    let domain = parts[1];

    if local.is_empty() {
        return false;
    }

    if !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') {
        return false;
    }

    if domain.is_empty() || domain.contains("..") {
        return false;
    }

    let valid_local_chars = |c: char| c.is_alphanumeric() || "!#$%&'*+/=?^_`{|}~.-".contains(c);
    if !local.chars().all(valid_local_chars) {
        return false;
    }

    if local.starts_with('.') || local.ends_with('.') {
        return false;
    }

    let valid_domain_chars = |c: char| c.is_alphanumeric() || c == '-' || c == '.';
    if !domain.chars().all(valid_domain_chars) {
        return false;
    }

    let tld = domain.split('.').next_back().unwrap_or("");
    if tld.len() < 2 {
        return false;
    }
    true
}

async fn get_usuario_activo_por_email(
    db: &sqlx::Pool<sqlx::Postgres>,
    email: &str,
) -> Result<Option<bool>, sqlx::Error> {
    sqlx::query_scalar("select activo from usuario where email = $1;")
        .bind(email)
        .fetch_optional(db)
        .await
}

pub async fn crear_usuario_h(
    State(estado): State<Arc<AppState>>,
    Json(body): Json<UsuarioDto>,
) -> Response {
    if !is_valid_email(&body.email) {
        return (
            StatusCode::BAD_REQUEST,
            Js(json!({"message":"Error", "description": "Email inválido"})),
        )
            .into_response();
    }

    if body.pass_word.len() < 8 {
        return (
            StatusCode::BAD_REQUEST,
            Js(json!({"message":"Error", "description": "La contraseña debe tener 8 o mas caracteres"})),
        ).into_response();
    }

    if let Some(id) = body.persona_id
        && let Ok(persona_id) = Uuid::parse_str(&id)
    {
        let r = usuario::crear_usuario(&estado.db, &body.email, &body.pass_word, &persona_id).await;
        match r {
            Ok(sub) => {
                let rol = match usuario::get_rol_usuario(&estado.db, &sub).await {
                    Ok(r) => r,
                    Err(e) => {
                        error!("{}", e);
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Js(json!(Ress::<u8> {
                                message: Respuesta::Error.as_str(),
                                description: "Error obteniendo rol de usuario",
                                data: None
                            })),
                        )
                            .into_response();
                    }
                };
                if let Some(user_jwt) = UsuarioJwt::new(sub, body.email, rol) {
                    let token = jwt(&user_jwt, &estado.env_config.jwt_secret);
                    match token {
                        Ok(val) => (
                            StatusCode::CREATED,
                            [(
                                SET_COOKIE,
                                format!("jwt={}; HttpOnly; Path=/; SameSite=Strict", val),
                            )],
                            Js(json!(Ress::<()> {
                                message: Respuesta::Success.as_str(),
                                description: "Usuario creado",
                                data: None
                            })),
                        )
                            .into_response(),
                        Err(e) => {
                            error!("{}", e);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Js(json!(Ress::<u8> {
                                    message: Respuesta::Error.as_str(),
                                    description: "Error creando usuario",
                                    data: None
                                })),
                            )
                                .into_response()
                        }
                    }
                } else {
                    error!("Error instanciando usuario");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Js(json!(Ress::<u8> {
                            message: Respuesta::Error.as_str(),
                            description: "Error creando usuario",
                            data: None
                        })),
                    )
                        .into_response()
                }
            }
            Err(e) => {
                if let sqlx::Error::Database(db_err) = &e
                    && db_err.is_unique_violation()
                {
                    return (
                        StatusCode::CONFLICT,
                        Js(json!({"message":"Error", "description": "Ya existe un usuario con ese email"})),
                    ).into_response();
                }
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Js(json!({"message":"Error", "description": "Error creando usuario"})),
                )
                    .into_response()
            }
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Js(json!({"message":"Error", "description": "Persona_id debe ser un uuid valido"})),
        )
            .into_response()
    }
}

pub async fn login_usuario_h(
    State(state): State<Arc<AppState>>,
    Json(body): Json<UsuarioDto>,
) -> Response {
    if body.email.is_empty() || body.pass_word.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Js(json!(Ress::<u32> {
                message: Respuesta::Error.as_str(),
                description: "Complete los campos",
                data: None
            })),
        )
            .into_response();
    }
    let verificacion = login_usuario(
        &body.email,
        &body.pass_word,
        &state.db,
        &state.env_config.jwt_secret,
    )
    .await;

    match verificacion {
        Ok((token, v)) => {
            if v {
                (
                    StatusCode::OK,
                    [(
                        SET_COOKIE,
                        format!(
                            "jwt={}; HttpOnly; Path=/; SameSite=Strict; Max-Age={}",
                            token,
                            60 * 60 * 24
                        ),
                    )],
                    Js(json!(Ress::<()> {
                        message: Respuesta::Success.as_str(),
                        description: "Ha iniciado sesión correctamente",
                        data: None
                    })),
                )
                    .into_response()
            } else {
                if let Ok(Some(false)) = get_usuario_activo_por_email(&state.db, &body.email).await
                {
                    return (
                        StatusCode::FORBIDDEN,
                        Js(json!(Ress::<u8> {
                            message: Respuesta::Warn.as_str(),
                            description: "Tu cuenta ha sido desactivada, revisa tu correo para más información",
                            data: None
                        })),
                    )
                        .into_response();
                }
                (
                    StatusCode::UNAUTHORIZED,
                    Js(json!(Ress::<u8> {
                        message: Respuesta::Error.as_str(),
                        description: "Nombre de usuario o contraseña incorrectos",
                        data: None
                    })),
                )
                    .into_response()
            }
        }
        Err(sqlx::Error::RowNotFound) => {
            if let Ok(Some(false)) = get_usuario_activo_por_email(&state.db, &body.email).await {
                return (
                    StatusCode::FORBIDDEN,
                    Js(json!(Ress::<u8> {
                        message: Respuesta::Warn.as_str(),
                        description: "Tu cuenta ha sido desactivada, revisa tu correo para más información",
                        data: None
                    })),
                )
                    .into_response();
            }
            (
                StatusCode::UNAUTHORIZED,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Nombre de usuario o contraseña incorrectos",
                    data: None
                })),
            )
                .into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "Error comprobando credenciales",
                data: None
            })),
        )
            .into_response(),
    }
}

pub async fn me_h(
    State(state): State<Arc<AppState>>,
    Extension(usr): Extension<UsuarioJwt>,
) -> Response {
    match get_usuario_actual(&state.db, &usr).await {
        Ok(info) => (
            StatusCode::OK,
            Js(json!(Ress::<UsuarioInfo> {
                message: Respuesta::Success.as_str(),
                description: "Usuario obtenido",
                data: Some(info)
            })),
        )
            .into_response(),
        Err(e) => {
            error!("{}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error obteniendo usuario",
                    data: None
                })),
            )
                .into_response()
        }
    }
}

pub async fn logout_h() -> Response {
    (
        StatusCode::OK,
        [(
            SET_COOKIE,
            "jwt=; HttpOnly; Path=/; SameSite=Strict; Max-Age=0",
        )],
        Js(json!(Ress::<()> {
            message: Respuesta::Success.as_str(),
            description: "Sesión cerrada",
            data: None
        })),
    )
        .into_response()
}

pub async fn listar_usuarios_h(
    State(state): State<Arc<AppState>>,
    Extension(usr): Extension<UsuarioJwt>,
    Query(params): Query<UsuariosQuery>,
) -> Response {
    if usr.rol != "Administrador" {
        return (
            StatusCode::FORBIDDEN,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "No autorizado",
                data: None
            })),
        )
            .into_response();
    }

    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(10).max(1).min(100);
    let offset = (page - 1) * limit;

    let total = contar_usuarios(
        &state.db,
        params.provincia_id,
        params.municipio_id,
        params.oficina_id,
    )
    .await
    .unwrap_or(0);
    let result = listar_usuarios(
        &state.db,
        limit,
        offset,
        params.provincia_id,
        params.municipio_id,
        params.oficina_id,
    )
    .await;

    debug!("Este es el resultado: {:?}", &result);

    match result {
        Ok(usuarios) => (
            StatusCode::OK,
            Js(json!(Ress::<PaginatedResponse<_>> {
                message: Respuesta::Success.as_str(),
                description: "Usuarios obtenidos",
                data: Some(PaginatedResponse {
                    usuarios,
                    total,
                    page,
                    limit,
                })
            })),
        )
            .into_response(),
        Err(e) => {
            error!("{}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error obteniendo usuarios",
                    data: None
                })),
            )
                .into_response()
        }
    }
}

#[derive(sqlx::FromRow)]
struct EmailUser {
    nombre: String,
    apellido: String,
    email: String,
}

pub async fn enviar_correo_usuario_h(
    State(state): State<Arc<AppState>>,
    Extension(usr): Extension<UsuarioJwt>,
    Json(body): Json<EnviarCorreoUsuarioDto>,
) -> Response {
    if usr.rol != "Administrador" {
        return (
            StatusCode::FORBIDDEN,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "No autorizado",
                data: None
            })),
        )
            .into_response();
    }

    let asunto = body.asunto.trim();
    if asunto.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "El asunto es obligatorio",
                data: None
            })),
        )
            .into_response();
    }

    let cuerpo = body.cuerpo.trim();
    if cuerpo.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "El cuerpo es obligatorio",
                data: None
            })),
        )
            .into_response();
    }

    let usuario_id = match Uuid::parse_str(body.usuario_id.trim()) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Usuario inválido",
                    data: None
                })),
            )
                .into_response();
        }
    };

    let user_envia = sqlx::query_as::<_, EmailUser>(
        "select p.nombre, p.apellido, u.email
         from usuario u
         join persona p on u.persona_id = p.id
         where u.id = $1;",
    )
    .bind(usr.sub)
    .fetch_one(&state.db)
    .await;

    let user_recibe = sqlx::query_as::<_, EmailUser>(
        "select p.nombre, p.apellido, u.email
         from usuario u
         join persona p on u.persona_id = p.id
         where u.id = $1;",
    )
    .bind(usuario_id)
    .fetch_one(&state.db)
    .await;

    let (usr_env, usr_rc) = match (user_envia, user_recibe) {
        (Ok(env), Ok(rec)) => (env, rec),
        _ => {
            return (
                StatusCode::NOT_FOUND,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Usuario no encontrado",
                    data: None
                })),
            )
                .into_response();
        }
    };

    let result = send_email(
        format!("{} {}", usr_env.nombre, usr_env.apellido),
        usr_env.email,
        format!("{} {}", usr_rc.nombre, usr_rc.apellido),
        usr_rc.email,
        EmailType::MailPlain(asunto.to_string(), cuerpo.to_string()),
        &state.env_config,
    );

    match result {
        Ok(_) => (
            StatusCode::OK,
            Js(json!(Ress::<()> {
                message: Respuesta::Success.as_str(),
                description: "Correo enviado correctamente",
                data: None
            })),
        )
            .into_response(),
        Err(e) => {
            error!("Error enviando correo: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "No se pudo enviar el correo",
                    data: None
                })),
            )
                .into_response()
        }
    }
}

pub async fn actualizar_estado_usuario_h(
    State(state): State<Arc<AppState>>,
    Extension(usr): Extension<UsuarioJwt>,
    Json(body): Json<ActualizarEstadoUsuarioDto>,
) -> Response {
    if usr.rol != "Administrador" {
        return (
            StatusCode::FORBIDDEN,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "No autorizado",
                data: None
            })),
        )
            .into_response();
    }

    let usuario_id = match Uuid::parse_str(body.usuario_id.trim()) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Usuario inválido",
                    data: None
                })),
            )
                .into_response();
        }
    };

    if usuario_id == usr.sub {
        return (
            StatusCode::BAD_REQUEST,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "No puedes desactivar tu propio usuario",
                data: None
            })),
        )
            .into_response();
    }

    if !body.activo {
        let motivo = body.motivo.as_deref().unwrap_or("").trim();
        if motivo.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "La causa de desactivación es obligatoria",
                    data: None
                })),
            )
                .into_response();
        }
    }

    match actualizar_estado_usuario(&state.db, &usuario_id, body.activo).await {
        Ok(affected) if affected > 0 => {
            if !body.activo {
                let user_envia = sqlx::query_as::<_, EmailUser>(
                    "select p.nombre, p.apellido, u.email
                     from usuario u
                     join persona p on u.persona_id = p.id
                     where u.id = $1;",
                )
                .bind(usr.sub)
                .fetch_one(&state.db)
                .await;

                let user_recibe = sqlx::query_as::<_, EmailUser>(
                    "select p.nombre, p.apellido, u.email
                     from usuario u
                     join persona p on u.persona_id = p.id
                     where u.id = $1;",
                )
                .bind(usuario_id)
                .fetch_one(&state.db)
                .await;

                if let (Ok(usr_env), Ok(usr_rc)) = (user_envia, user_recibe) {
                    let motivo = body.motivo.as_deref().unwrap_or("").trim();
                    let _ = send_email(
                        format!("{} {}", usr_env.nombre, usr_env.apellido),
                        usr_env.email,
                        format!("{} {}", usr_rc.nombre, usr_rc.apellido),
                        usr_rc.email,
                        EmailType::MailWithBody(
                            "Cuenta desactivada".to_string(),
                            format!(
                                "<p>Tu cuenta ha sido desactivada.</p><p><strong>Motivo:</strong> {}</p>",
                                motivo.replace('\n', "<br />")
                            ),
                        ),
                        &state.env_config,
                    );
                }
            }

            (
                StatusCode::OK,
                Js(json!(Ress::<()> {
                    message: Respuesta::Success.as_str(),
                    description: "Estado del usuario actualizado",
                    data: None
                })),
            )
                .into_response()
        }
        Ok(_) => (
            StatusCode::NOT_FOUND,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "Usuario no encontrado",
                data: None
            })),
        )
            .into_response(),
        Err(e) => {
            error!("Error actualizando estado de usuario: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error actualizando estado del usuario",
                    data: None
                })),
            )
                .into_response()
        }
    }
}

pub async fn actualizar_rol_usuario_h(
    State(state): State<Arc<AppState>>,
    Extension(usr): Extension<UsuarioJwt>,
    Json(body): Json<ActualizarRolUsuarioDto>,
) -> Response {
    if usr.rol != "Administrador" {
        return (
            StatusCode::FORBIDDEN,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "No autorizado",
                data: None
            })),
        )
            .into_response();
    }

    let usuario_id = match Uuid::parse_str(body.usuario_id.trim()) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Usuario inválido",
                    data: None
                })),
            )
                .into_response();
        }
    };

    if usuario_id == usr.sub {
        return (
            StatusCode::BAD_REQUEST,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "No puedes cambiar tu propio rol",
                data: None
            })),
        )
            .into_response();
    }

    let rol_id = match body.oficina_id {
        Some(_) => 2,
        None => 1,
    };

    if rol_id == 2 {
        let tiene_activos: i64 = match sqlx::query_scalar(
            "select count(*)
             from tramite t
             join persona p on p.id = t.persona_id
             where p.id = (select persona_id from usuario where id = $1)
               and t.estado_id in (1, 2);",
        )
        .bind(usuario_id)
        .fetch_one(&state.db)
        .await
        {
            Ok(count) => count,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Js(json!(Ress::<u8> {
                        message: Respuesta::Error.as_str(),
                        description: "Error validando trámites activos",
                        data: None
                    })),
                )
                    .into_response();
            }
        };
        if tiene_activos > 0 {
            return (
                StatusCode::CONFLICT,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Warn.as_str(),
                    description: "El usuario tiene trámites pendientes o en proceso",
                    data: None
                })),
            )
                .into_response();
        }
    } else {
        let tiene_en_proceso: i64 = match sqlx::query_scalar(
            "select count(*) from tramite where registrador_id = $1 and estado_id = 2;",
        )
        .bind(usuario_id)
        .fetch_one(&state.db)
        .await
        {
            Ok(count) => count,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Js(json!(Ress::<u8> {
                        message: Respuesta::Error.as_str(),
                        description: "Error validando trámites en proceso",
                        data: None
                    })),
                )
                    .into_response();
            }
        };
        if tiene_en_proceso > 0 {
            return (
                StatusCode::CONFLICT,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Warn.as_str(),
                    description: "El registrador tiene trámites en proceso",
                    data: None
                })),
            )
                .into_response();
        }
    }

    match actualizar_rol_usuario(&state.db, &usuario_id, body.oficina_id, rol_id).await {
        Ok(affected) if affected > 0 => (
            StatusCode::OK,
            Js(json!(Ress::<()> {
                message: Respuesta::Success.as_str(),
                description: "Rol del usuario actualizado",
                data: None
            })),
        )
            .into_response(),
        Ok(_) => (
            StatusCode::NOT_FOUND,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "Usuario no encontrado",
                data: None
            })),
        )
            .into_response(),
        Err(e) => {
            error!("Error actualizando rol de usuario: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error actualizando rol del usuario",
                    data: None
                })),
            )
                .into_response()
        }
    }
}
