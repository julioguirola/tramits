use std::sync::Arc;

use axum::{
    Extension, Json,
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
    repos::{
        tramite::gestionar::{AccionGestion, GestionarTramiteError, gestionar_tramite},
        usuario::UsuarioJwt,
    },
    tipos::{Respuesta, Ress},
};

#[derive(Deserialize)]
pub struct GestionDto {
    accion: AccionGestion,
}

pub async fn gestionar_tramite_h(
    State(state): State<Arc<AppState>>,
    Extension(usr): Extension<UsuarioJwt>,
    Path(tramite_id): Path<Uuid>,
    Json(body): Json<GestionDto>,
) -> Response {
    if usr.rol != "Registrador" {
        return (
            StatusCode::FORBIDDEN,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "Solo un registrador puede gestionar solicitudes",
                data: None
            })),
        )
            .into_response();
    }

    match gestionar_tramite(state.clone(), &usr, tramite_id, body.accion).await {
        Ok(_) => (
            StatusCode::OK,
            Js(json!(Ress::<()> {
                message: Respuesta::Success.as_str(),
                description: "Solicitud gestionada correctamente",
                data: None
            })),
        )
            .into_response(),
        Err(GestionarTramiteError::NoEncontrado) => (
            StatusCode::NOT_FOUND,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "No existe el tramite solicitado",
                data: None
            })),
        )
            .into_response(),
        Err(GestionarTramiteError::NoAsignado) => (
            StatusCode::CONFLICT,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "La solicitud no tiene registrador asignado",
                data: None
            })),
        )
            .into_response(),
        Err(GestionarTramiteError::NoEnProceso) => (
            StatusCode::CONFLICT,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "Solo se pueden gestionar solicitudes en proceso",
                data: None
            })),
        )
            .into_response(),
        Err(GestionarTramiteError::YaFinalizado) => (
            StatusCode::CONFLICT,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "La solicitud ya fue finalizada",
                data: None
            })),
        )
            .into_response(),
        Err(GestionarTramiteError::NoEsTuSolicitud) => (
            StatusCode::FORBIDDEN,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "Solo puedes gestionar tus solicitudes asignadas",
                data: None
            })),
        )
            .into_response(),
        Err(GestionarTramiteError::NoActualizado) => (
            StatusCode::CONFLICT,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "No se pudo actualizar la solicitud",
                data: None
            })),
        )
            .into_response(),
        Err(GestionarTramiteError::Db(e)) => {
            error!("{}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error gestionando solicitud",
                    data: None
                })),
            )
                .into_response()
        }
    }
}
