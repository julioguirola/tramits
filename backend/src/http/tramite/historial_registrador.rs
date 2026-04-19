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
    repos::{
        tramite::historial_registrador::{
            HistorialRegistradorPaginado, get_historial_finalizado_registrador,
        },
        usuario::UsuarioJwt,
    },
    tipos::{Respuesta, Ress},
};

#[derive(Deserialize)]
pub struct HistorialRegistradorQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

pub async fn get_historial_registrador_h(
    State(state): State<Arc<AppState>>,
    Extension(usr): Extension<UsuarioJwt>,
    Query(params): Query<HistorialRegistradorQuery>,
) -> Response {
    if usr.rol != "Registrador" {
        return (
            StatusCode::FORBIDDEN,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "Solo los registradores pueden ver este historial",
                data: None
            })),
        )
            .into_response();
    }

    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(10).clamp(1, 50);

    match get_historial_finalizado_registrador(&state.db, &usr, page, limit).await {
        Ok(data) => (
            StatusCode::OK,
            Js(json!(Ress::<HistorialRegistradorPaginado> {
                message: Respuesta::Success.as_str(),
                description: "Historial finalizado del registrador obtenido",
                data: Some(data)
            })),
        )
            .into_response(),
        Err(e) => {
            error!("{}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error obteniendo historial finalizado",
                    data: None
                })),
            )
                .into_response()
        }
    }
}
