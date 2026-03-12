use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Json as Js, Response, IntoResponse},
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    AppState,
    config::tipos::{Respuesta, Ress},
    repos::nucleo::{Nucleo, get_nucleos},
};

#[derive(Deserialize)]
pub struct GetNucleosParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub bodega_id: Option<i32>,
}

pub async fn get_nucleos_h(
    State(estado): State<Arc<AppState>>,
    Query(params): Query<GetNucleosParams>,
) -> Response {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).clamp(1, 100);

    match get_nucleos(page, limit, params.bodega_id, &estado.db).await {
        Ok(nucleos) if nucleos.is_empty() => (
            StatusCode::NOT_FOUND,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "No se encontraron núcleos",
                data: None,
            })),
        )
            .into_response(),
        Ok(nucleos) => (
            StatusCode::OK,
            Js(json!(Ress::<Vec<Nucleo>> {
                message: Respuesta::Success.as_str(),
                description: "Núcleos encontrados",
                data: Some(nucleos),
            })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("{}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error obteniendo núcleos",
                    data: None,
                })),
            )
                .into_response()
        }
    }
}
