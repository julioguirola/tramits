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
    repos::{
        tramite::procesar::{ProcesarBajaError, procesar_solicitud_baja},
        usuario::UsuarioJwt,
    },
    tipos::{Respuesta, Ress},
};

pub async fn procesar_solicitud_h(
    State(state): State<Arc<AppState>>,
    Extension(usr): Extension<UsuarioJwt>,
    Path(tramite_id): Path<Uuid>,
) -> Response {
    if usr.rol != "Registrador" {
        return (
            StatusCode::FORBIDDEN,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "Solo un registrador puede procesar solicitudes",
                data: None
            })),
        )
            .into_response();
    }

    match procesar_solicitud_baja(&state.db, &usr, tramite_id).await {
        Ok(_) => (
            StatusCode::OK,
            Js(json!(Ress::<()> {
                message: Respuesta::Success.as_str(),
                description: "Solicitud de baja en proceso",
                data: None
            })),
        )
            .into_response(),
        Err(ProcesarBajaError::NoEncontrado) => (
            StatusCode::NOT_FOUND,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "No existe el tramite solicitado",
                data: None
            })),
        )
            .into_response(),
        Err(ProcesarBajaError::NoEsBaja) => (
            StatusCode::BAD_REQUEST,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "Solo se pueden procesar solicitudes de baja",
                data: None
            })),
        )
            .into_response(),
        Err(ProcesarBajaError::NoPendiente) => (
            StatusCode::CONFLICT,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "La solicitud ya no esta pendiente",
                data: None
            })),
        )
            .into_response(),
        Err(ProcesarBajaError::YaAsignado) => (
            StatusCode::CONFLICT,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "La solicitud ya fue tomada por otro registrador",
                data: None
            })),
        )
            .into_response(),
        Err(ProcesarBajaError::SinOficina) => (
            StatusCode::FORBIDDEN,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "Tu usuario no tiene oficina asignada",
                data: None
            })),
        )
            .into_response(),
        Err(ProcesarBajaError::FueraDeOficina) => (
            StatusCode::FORBIDDEN,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "Solo puedes procesar solicitudes de tu oficina",
                data: None
            })),
        )
            .into_response(),
        Err(ProcesarBajaError::Db(e)) => {
            error!("{}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error procesando solicitud de baja",
                    data: None
                })),
            )
                .into_response()
        }
    }
}
