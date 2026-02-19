use crate::{AppState, repos::usuario};
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
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CrearUsuarioDto {
    #[validate(email(message = "Email no válido"))]
    email: String,
    #[validate(length(min = 1, message = "Campo requerido"))]
    pass_word: String,
    #[validate(length(min = 1, message = "Campo requerido"))]
    persona_id: String,
}

pub async fn crear_usuario_h(
    State(estado): State<Arc<AppState>>,
    Json(body): Json<CrearUsuarioDto>,
) -> impl IntoResponse {
    let validado = body.validate();

    if let Err(e) = validado
        && let Some((_, value)) = e.field_errors().iter().next()
        && let Some(m) = value.iter().next()
    {
        return (
            StatusCode::BAD_REQUEST,
            Js(json!({"message":"Error", "description": m.message })),
        );
    }

    let persona_id = match Uuid::parse_str(&body.persona_id) {
        Ok(v) => v,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Js(json!({"message":"Error", "description": "persona_id debe ser un uuid valido"})),
            );
        }
    };

    let r = usuario::crear_usuario(&estado.db, &body.email, &body.pass_word, &persona_id).await;

    match r {
        Ok(_) => (
            StatusCode::CREATED,
            Js(json!({"message":"Éxito", "description": "Usuario creado"})),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Js(json!({"message":"Error", "description": "Error creando usuario"})),
        ),
    }
}
