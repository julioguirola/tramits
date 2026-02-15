use crate::{AppState, repos::usuario};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json as Js},
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CrearUsuarioDto {
    email: String,
    pass_word: String,
    persona_id: String,
}

pub async fn crear_usuario_h(
    State(estado): State<Arc<AppState>>,
    Json(body): Json<CrearUsuarioDto>,
) -> impl IntoResponse {
    let r = usuario::crear_usuario(
        estado.db.clone(),
        body.email,
        body.pass_word,
        body.persona_id,
    )
    .await;

    match r {
        Ok(_) => (
            StatusCode::CREATED,
            Js(json!({"mensaje": "Usuario creado"})),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Js(json!({"mensaje": "Error creando usuario"})),
        ),
    }
}
