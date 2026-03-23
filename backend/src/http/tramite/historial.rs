use std::sync::Arc;

use axum::{
    Extension,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Json as Js, Response},
};
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::{
    AppState,
    repos::{tramite::historial::{get_historial_tramites, get_todas_solicitudes, TramiteHistorial}, usuario::UsuarioJwt},
    tipos::{Respuesta, Ress},
};

#[derive(Deserialize)]
pub struct TramiteQuery {
    pub estado_id: Option<i32>,
}

pub async fn get_historial_h(
    State(state): State<Arc<AppState>>,
    Extension(usr): Extension<UsuarioJwt>,
    Query(params): Query<TramiteQuery>,
) -> Response {
    // Si el rol es Registrador o Administrador, obtener todas las solicitudes
    let result = if usr.rol == "Registrador" || usr.rol == "Administrador" {
        get_todas_solicitudes(&state.db, params.estado_id).await
    } else {
        get_historial_tramites(&state.db, &usr, params.estado_id).await
    };

    match result {
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
