use std::sync::Arc;

use argon2::{Argon2, PasswordHasher, password_hash::SaltString, password_hash::rand_core::OsRng};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json as Js, Response},
};
use serde::Deserialize;
use serde_json::json;
use sqlx::types::Uuid;
use tracing::error;

use crate::{
    AppState,
    mail::{EmailType, send_email},
    tipos::{Respuesta, Ress},
};

#[derive(Deserialize)]
pub struct SolicitarDto {
    email: String,
}

#[derive(Deserialize)]
pub struct RestablecerDto {
    password: String,
}

pub async fn solicitar_h(
    State(state): State<Arc<AppState>>,
    Json(body): Json<SolicitarDto>,
) -> Response {
    let email = body.email.trim();
    if email.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "El email es obligatorio",
                data: None
            })),
        )
            .into_response();
    }

    let usuario = sqlx::query_as::<_, (Uuid, String, String)>(
        "select u.id, p.nombre, p.apellido
         from usuario u
         join persona p on p.id = u.persona_id
         where u.email = $1 and u.activo = true;",
    )
    .bind(email)
    .fetch_optional(&state.db)
    .await;

    let Ok(Some((user_id, nombre, apellido))) = usuario else {
        return (
            StatusCode::OK,
            Js(json!(Ress::<()> {
                message: Respuesta::Success.as_str(),
                description: "Si el correo existe, recibirás un enlace para restablecer tu contraseña",
                data: None
            })),
        )
            .into_response();
    };

    let recovery_id = match sqlx::query_scalar::<_, Uuid>(
        "insert into password_recovery (usuario_id) values ($1) returning id;",
    )
    .bind(user_id)
    .fetch_one(&state.db)
    .await
    {
        Ok(id) => id,
        Err(e) => {
            error!("Error creando recovery: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error al procesar la solicitud",
                    data: None
                })),
            )
                .into_response();
        }
    };

    let link = format!("{}/recovery/{}", state.env_config.spa_url, recovery_id);
    let body_html = format!(
        "<p>Hola <strong>{} {}</strong>,</p><p>Recibimos una solicitud para restablecer tu contraseña.</p><p>Haz clic en el siguiente enlace para continuar:</p><p style=\"text-align:center;margin:20px 0;\"><a href=\"{}\" style=\"display:inline-block;padding:12px 24px;background-color:#16a34a;color:#ffffff;text-decoration:none;border-radius:8px;font-weight:600;\">Restablecer contraseña</a></p><p>El enlace expira en 1 hora. Si no solicitaste este cambio, ignora este correo.</p>",
        nombre, apellido, link
    );

    let from_name = "Tramits".to_string();
    let from_email = state.env_config.admin_email.clone();

    if let Err(e) = send_email(
        from_name,
        from_email,
        format!("{} {}", nombre, apellido),
        email.to_string(),
        EmailType::MailWithBody("Restablece tu contraseña".to_string(), body_html),
        &state.env_config,
    ) {
        error!("Error enviando correo de recuperación: {}", e);
    }

    (
        StatusCode::OK,
        Js(json!(Ress::<()> {
            message: Respuesta::Success.as_str(),
            description: "Si el correo existe, recibirás un enlace para restablecer tu contraseña",
            data: None
        })),
    )
        .into_response()
}

pub async fn restablecer_h(
    State(state): State<Arc<AppState>>,
    Path(recovery_id): Path<Uuid>,
    Json(body): Json<RestablecerDto>,
) -> Response {
    let password = body.password.trim();
    if password.len() < 8 {
        return (
            StatusCode::BAD_REQUEST,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "La contraseña debe tener al menos 8 caracteres",
                data: None
            })),
        )
            .into_response();
    }

    let recovery = sqlx::query_as::<_, (Uuid,)>(
        "select usuario_id from password_recovery
         where id = $1 and usado = false and expira_en > now();",
    )
    .bind(recovery_id)
    .fetch_optional(&state.db)
    .await;

    let Some((usuario_id,)) = recovery.ok().flatten() else {
        return (
            StatusCode::BAD_REQUEST,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "El enlace no es válido o ha expirado",
                data: None
            })),
        )
            .into_response();
    };

    let salt = SaltString::generate(&mut OsRng);
    let hash = match Argon2::default().hash_password(password.as_bytes(), &salt) {
        Ok(h) => h.to_string(),
        Err(e) => {
            error!("Error hasheando contraseña: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error al procesar la solicitud",
                    data: None
                })),
            )
                .into_response();
        }
    };

    let mut tx = match state.db.begin().await {
        Ok(t) => t,
        Err(e) => {
            error!("Error iniciando transacción: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error al procesar la solicitud",
                    data: None
                })),
            )
                .into_response();
        }
    };

    if let Err(e) = sqlx::query("update usuario set pass_word = $1 where id = $2;")
        .bind(&hash)
        .bind(usuario_id)
        .execute(&mut *tx)
        .await
    {
        error!("Error actualizando password: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "Error al procesar la solicitud",
                data: None
            })),
        )
            .into_response();
    }

    if let Err(e) =
        sqlx::query("update password_recovery set usado = true where id = $1;")
            .bind(recovery_id)
            .execute(&mut *tx)
            .await
    {
        error!("Error marcando recovery como usado: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "Error al procesar la solicitud",
                data: None
            })),
        )
            .into_response();
    }

    if let Err(e) = tx.commit().await {
        error!("Error haciendo commit: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "Error al procesar la solicitud",
                data: None
            })),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Js(json!(Ress::<()> {
            message: Respuesta::Success.as_str(),
            description: "Contraseña restablecida correctamente",
            data: None
        })),
    )
        .into_response()
}
