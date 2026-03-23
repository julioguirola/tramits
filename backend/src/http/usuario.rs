use crate::{
    AppState,
    repos::usuario::{self, UsuarioInfo, UsuarioJwt, get_usuario_actual, jwt, login_usuario},
    tipos::{Respuesta, Ress},
};
use axum::{
    Extension, Json,
    extract::State,
    http::{StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Json as Js, Response},
};
use serde::Deserialize;
use serde_json::json;
use sqlx::types::Uuid;
use std::sync::Arc;
use tracing::error;

#[derive(Debug, Deserialize)]
pub struct UsuarioDto {
    email: String,
    pass_word: String,
    persona_id: Option<String>,
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
        Err(sqlx::Error::RowNotFound) => (
            StatusCode::UNAUTHORIZED,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "Nombre de usuario o contraseña incorrectos",
                data: None
            })),
        )
            .into_response(),
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
