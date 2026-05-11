use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json as Js, Response},
};
use axum::extract::Path;
use serde::Deserialize;
use serde_json::json;
use sqlx::types::Uuid;
use tracing::error;

use crate::{
    AppState,
    mail::{EmailType, send_email},
    repos::usuario::UsuarioJwt,
    tipos::{Respuesta, Ress},
};

#[derive(Deserialize)]
pub struct EnviarCorreoTramiteDto {
    asunto: String,
    cuerpo: String,
}

#[derive(sqlx::FromRow)]
struct EmailUser {
    nombre: String,
    apellido: String,
    email: String,
}

#[derive(sqlx::FromRow)]
struct TramiteAsignado {
    registrador_id: Option<Uuid>,
    estado_id: i32,
}

#[derive(sqlx::FromRow)]
struct TramitePropio {
    persona_id: Uuid,
    registrador_id: Option<Uuid>,
    estado_id: i32,
}

pub async fn enviar_correo_tramite_h(
    State(state): State<Arc<AppState>>,
    Extension(usr): Extension<UsuarioJwt>,
    Path(tramite_id): Path<Uuid>,
    Json(body): Json<EnviarCorreoTramiteDto>,
) -> Response {
    if usr.rol != "Registrador" {
        return (
            StatusCode::FORBIDDEN,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "Solo un registrador puede enviar correos",
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

    let tramite = sqlx::query_as::<_, TramiteAsignado>(
        "select registrador_id, estado_id from tramite where id = $1;",
    )
    .bind(tramite_id)
    .fetch_optional(&state.db)
    .await;

    let Some(tramite) = tramite.ok().flatten() else {
        return (
            StatusCode::NOT_FOUND,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "Trámite no encontrado",
                data: None
            })),
        )
            .into_response();
    };

    if tramite.registrador_id != Some(usr.sub) {
        return (
            StatusCode::FORBIDDEN,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "Solo puedes enviar correos a trámites asignados",
                data: None
            })),
        )
            .into_response();
    }

    if tramite.estado_id != 2 {
        return (
            StatusCode::BAD_REQUEST,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "Solo puedes enviar correos a trámites en proceso",
                data: None
            })),
        )
            .into_response();
    }

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
         from tramite t
         join persona p on t.persona_id = p.id
         join usuario u on p.id = u.persona_id
         where t.id = $1 and u.rol_id = 1;",
    )
    .bind(tramite_id)
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

pub async fn enviar_correo_registrador_h(
    State(state): State<Arc<AppState>>,
    Extension(usr): Extension<UsuarioJwt>,
    Path(tramite_id): Path<Uuid>,
    Json(body): Json<EnviarCorreoTramiteDto>,
) -> Response {
    if usr.rol != "Consumidor" {
        return (
            StatusCode::FORBIDDEN,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "Solo un consumidor puede enviar este correo",
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

    let persona_id: Uuid = match sqlx::query_scalar("select persona_id from usuario where id = $1")
        .bind(usr.sub)
        .fetch_one(&state.db)
        .await
    {
        Ok(id) => id,
        Err(_) => {
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

    let tramite = sqlx::query_as::<_, TramitePropio>(
        "select persona_id, registrador_id, estado_id from tramite where id = $1;",
    )
    .bind(tramite_id)
    .fetch_optional(&state.db)
    .await;

    let Some(tramite) = tramite.ok().flatten() else {
        return (
            StatusCode::NOT_FOUND,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "Trámite no encontrado",
                data: None
            })),
        )
            .into_response();
    };

    if tramite.persona_id != persona_id {
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

    if tramite.estado_id != 1 && tramite.estado_id != 2 {
        return (
            StatusCode::BAD_REQUEST,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "Solo puedes enviar correos a trámites activos",
                data: None
            })),
        )
            .into_response();
    }

    let Some(registrador_id) = tramite.registrador_id else {
        return (
            StatusCode::CONFLICT,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "Aún no hay registrador asignado",
                data: None
            })),
        )
            .into_response();
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
    .bind(registrador_id)
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
