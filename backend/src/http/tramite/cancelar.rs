use std::sync::Arc;

use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json as Js, Response},
};
use serde_json::json;
use sqlx::types::Uuid;
use tracing::error;

use crate::{
    AppState,
    repos::{tramite::cancelar::{CancelarTramiteError, cancelar_tramite}, usuario::UsuarioJwt},
    tipos::{Respuesta, Ress},
};

pub async fn cancelar_tramite_h(
    State(state): State<Arc<AppState>>,
    Extension(usr): Extension<UsuarioJwt>,
    Path(tramite_id): Path<Uuid>,
) -> Response {
    match cancelar_tramite(&state.db, &usr, tramite_id).await {
        Ok(_) => (
            StatusCode::OK,
            Js(json!(Ress::<()> {
                message: Respuesta::Success.as_str(),
                description: "Solicitud cancelada correctamente",
                data: None
            })),
        )
            .into_response(),
        Err(CancelarTramiteError::NoEncontrado) => (
            StatusCode::NOT_FOUND,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "No existe el tramite solicitado",
                data: None
            })),
        )
            .into_response(),
        Err(CancelarTramiteError::NoAutorizado) => (
            StatusCode::FORBIDDEN,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "No tienes permiso para cancelar este tramite",
                data: None
            })),
        )
            .into_response(),
        Err(CancelarTramiteError::NoPendiente) => (
            StatusCode::CONFLICT,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "Solo se pueden cancelar solicitudes pendientes",
                data: None
            })),
        )
            .into_response(),
        Err(CancelarTramiteError::Db(e)) => {
            error!("{}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error cancelando solicitud",
                    data: None
                })),
            )
                .into_response()
        }
    }
}
