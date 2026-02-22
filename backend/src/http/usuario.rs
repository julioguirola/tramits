use crate::{
    AppState,
    config::tipos::Ress,
    repos::usuario::{self, login_usuario},
};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json as Js},
};
use serde::Deserialize;
use serde_json::json;
use sqlx::types::Uuid;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct CrearUsuarioDto {
    email: String,
    pass_word: String,
    persona_id: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUsuarioDto {
    email: String,
    pass_word: String,
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
    Json(body): Json<CrearUsuarioDto>,
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

    let persona_id = match Uuid::parse_str(&body.persona_id) {
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
    Json(body): Json<LoginUsuarioDto>,
) -> impl IntoResponse {
    if body.email.is_empty() || body.pass_word.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Js(json!(Ress::<u32> {
                message: "Error",
                description: "Complete los campos",
                data: None
            })),
        );
    }
    let verificacion = login_usuario(&body.email, &body.pass_word, &state.db).await;

    match verificacion {
        Ok(v) => {
            if v {
                (
                    StatusCode::OK,
                    Js(json!(Ress::<u8> {
                        message: "Éxito",
                        description: "Ha iniciado sesión correctamente",
                        data: None
                    })),
                )
            } else {
                (
                    StatusCode::UNAUTHORIZED,
                    Js(json!(Ress::<u8> {
                        message: "Error",
                        description: "Nombre de usuario o contraseña incorrectos",
                        data: None
                    })),
                )
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Js(json!(Ress::<u8> {
                message: "Error",
                description: "Error comprobando credenciales",
                data: None
            })),
        ),
    }
}
