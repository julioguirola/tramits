use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json as Js, Response},
};
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::{
    AppState,
    repos::{tramite::baja::crear_baja, usuario::UsuarioJwt},
    tipos::{Respuesta, Ress},
};

#[derive(Deserialize)]
pub struct BajaDto {
    nucleo_id: i32,
}

pub async fn crear_baja_h(
    State(state): State<Arc<AppState>>,
    Extension(usr): Extension<UsuarioJwt>,
    Json(body): Json<BajaDto>,
) -> Response {
    match crear_baja(&state.db, &usr, body.nucleo_id).await {
        Ok(id) => (
            StatusCode::CREATED,
            Js(json!(Ress::<i32> {
                message: Respuesta::Success.as_str(),
                description: "Solicitud de baja creada",
                data: Some(id)
            })),
        )
            .into_response(),
        Err(e) => {
            error!("{}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error creando solicitud de baja",
                    data: None
                })),
            )
                .into_response()
        }
    }
}
