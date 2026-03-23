use std::sync::Arc;

use axum::{
    Extension,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json as Js, Response},
};
use serde_json::json;
use tracing::error;

use crate::{
    AppState,
    repos::{tramite::historial::{get_historial_tramites, TramiteHistorial}, usuario::UsuarioJwt},
    tipos::{Respuesta, Ress},
};

pub async fn get_historial_h(
    State(state): State<Arc<AppState>>,
    Extension(usr): Extension<UsuarioJwt>,
) -> Response {
    match get_historial_tramites(&state.db, &usr).await {
        Ok(tramites) => (
            StatusCode::OK,
            Js(json!(Ress::<Vec<TramiteHistorial>> {
                message: Respuesta::Success.as_str(),
                description: "Historial de trámites obtenido",
                data: Some(tramites)
            })),
        )
            .into_response(),
        Err(e) => {
            error!("{}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error obteniendo historial de trámites",
                    data: None
                })),
            )
                .into_response()
        }
    }
}
