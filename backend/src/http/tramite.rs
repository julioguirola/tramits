use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode, header::COOKIE},
    response::{IntoResponse, Json as Js, Response},
};
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::{
    AppState,
    config::tipos::{Respuesta, Ress},
    repos::tramite::crear_tramite,
};

#[derive(Deserialize)]
pub struct TramiteDto {
    nucleo_id: i32,
    tipo_id: i32,
}

pub async fn crear_tramite_h(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<TramiteDto>,
) -> Response {
    let cookie_header = headers
        .get(COOKIE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    match crear_tramite(
        &state.db,
        &state.env_config.jwt_secret,
        cookie_header,
        body.nucleo_id,
        body.tipo_id,
    )
    .await
    {
        Ok(id) => (
            StatusCode::CREATED,
            Js(json!(Ress::<i32> {
                message: Respuesta::Success.as_str(),
                description: "Trámite creado",
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
                    description: "Error creando trámite",
                    data: None
                })),
            )
                .into_response()
        }
    }
}
