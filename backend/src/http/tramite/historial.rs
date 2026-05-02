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
    repos::{tramite::historial::{get_historial_tramites, get_todas_solicitudes, get_historial_tramites_count, get_todas_solicitudes_count, TramiteHistorial}, usuario::UsuarioJwt},
    tipos::{Respuesta, Ress},
};

#[derive(Deserialize)]
pub struct TramiteQuery {
    pub estado_id: Option<i32>,
    pub asignadas: Option<bool>,
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

use serde::Serialize;

#[derive(Serialize)]
struct PaginatedResponse<T> {
    tramites: T,
    total: i64,
    page: usize,
    limit: usize,
}

pub async fn get_historial_h(
    State(state): State<Arc<AppState>>,
    Extension(usr): Extension<UsuarioJwt>,
    Query(params): Query<TramiteQuery>,
) -> Response {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(10).max(1).min(100);
    let offset = (page - 1) * limit;

    // Si el rol es Registrador o Administrador, obtener todas las solicitudes
    // (Registrador verá solo de su oficina, Administrador verá todas)
    let result = if usr.rol == "Registrador" || usr.rol == "Administrador" {
        get_todas_solicitudes(&state.db, &usr, params.estado_id, params.asignadas, Some(limit), Some(offset)).await
    } else {
        get_historial_tramites(&state.db, &usr, params.estado_id, Some(limit), Some(offset)).await
    };

    // Obtener total sin paginación
    let total_result = if usr.rol == "Registrador" || usr.rol == "Administrador" {
        get_todas_solicitudes_count(&state.db, &usr, params.estado_id, params.asignadas).await
    } else {
        get_historial_tramites_count(&state.db, &usr, params.estado_id).await
    };

    let total = total_result.unwrap_or(0);

    match result {
        Ok(tramites) => (
            StatusCode::OK,
            Js(json!(Ress::<PaginatedResponse<Vec<TramiteHistorial>>> {
                message: Respuesta::Success.as_str(),
                description: "Historial de trámites obtenido",
                data: Some(PaginatedResponse {
                    tramites,
                    total,
                    page,
                    limit,
                })
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
