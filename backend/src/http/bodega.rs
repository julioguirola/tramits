use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Json as Js, Response},
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    AppState,
    config::tipos::{Respuesta, Ress},
    repos::bodega::{Bodega, get_bodegas},
};

#[derive(Deserialize)]
pub struct GetBodegasParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub oficina_id: Option<i32>,
}

pub async fn get_bodegas_h(
    State(estado): State<Arc<AppState>>,
    Query(params): Query<GetBodegasParams>,
) -> Response {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).clamp(1, 100);

    match get_bodegas(page, limit, params.oficina_id, &estado.db).await {
        Ok(bodegas) if bodegas.is_empty() => (
            StatusCode::NOT_FOUND,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "No se encontraron bodegas",
                data: None,
            })),
        )
            .into_response(),
        Ok(bodegas) => (
            StatusCode::OK,
            Js(json!(Ress::<Vec<Bodega>> {
                message: Respuesta::Success.as_str(),
                description: "Bodegas encontradas",
                data: Some(bodegas),
            })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("{}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error obteniendo bodegas",
                    data: None,
                })),
            )
                .into_response()
        }
    }
}
