use crate::{
    AppState,
    config::tipos::Ress,
    repos::usuario::{self, login_usuario},
};
use axum::{
    Json,
    extract::State,
    http::{StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Json as Js, Response},
};
use serde::Deserialize;
use serde_json::json;
use sqlx::types::Uuid;
use std::sync::Arc;

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
) -> impl IntoResponse {
    if !is_valid_email(&body.email) {
        return (
            StatusCode::BAD_REQUEST,
            Js(json!({"message":"Error", "description": "Email inválido"})),
        );
    }

    if body.pass_word.len() < 8 {
        return (
            StatusCode::BAD_REQUEST,
            Js(
                json!({"message":"Error", "description": "La contraseña debe tener 8 o mas caracteres"}),
            ),
        );
    }

    if !&body.persona_id.is_some() {
        return (
            StatusCode::BAD_REQUEST,
            Js(json!({"message":"Error", "description": "Persona_id debe ser un uuid valido"})),
        );
    }

    let persona_id = match Uuid::parse_str(&body.persona_id.unwrap()) {
        Ok(v) => v,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Js(json!({"message":"Error", "description": "Persona_id debe ser un uuid valido"})),
            );
        }
    };

    let r = usuario::crear_usuario(&estado.db, &body.email, &body.pass_word, &persona_id).await;

    match r {
        Ok(_) => (
            StatusCode::CREATED,
            Js(json!({"message":"Éxito", "description": "Usuario creado"})),
        ),
        Err(e) => {
            if let sqlx::Error::Database(db_err) = &e
                && db_err.is_unique_violation()
            {
                return (
                    StatusCode::CONFLICT,
                    Js(
                        json!({"message":"Error", "description": "Ya existe un usuario con ese email"}),
                    ),
                );
            }
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!({"message":"Error", "description": "Error creando usuario"})),
            )
        }
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
                message: "Error",
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
                    [(SET_COOKIE, token)],
                    Js(json!(Ress::<u8> {
                        message: "Éxito",
                        description: "Ha iniciado sesión correctamente",
                        data: None
                    })),
                )
                    .into_response()
            } else {
                (
                    StatusCode::UNAUTHORIZED,
                    Js(json!(Ress::<u8> {
                        message: "Error",
                        description: "Nombre de usuario o contraseña incorrectos",
                        data: None
                    })),
                )
                    .into_response()
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Js(json!(Ress::<u8> {
                message: "Error",
                description: "Error comprobando credenciales",
                data: None
            })),
        )
            .into_response(),
    }
}
