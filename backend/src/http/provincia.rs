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
    repos::provincia::{Provincia, get_provincias},
    tipos::{Respuesta, Ress},
};

#[derive(Deserialize)]
pub struct GetProvinciasParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

pub async fn get_provincias_h(
    State(estado): State<Arc<AppState>>,
    Query(params): Query<GetProvinciasParams>,
) -> Response {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).clamp(1, 100);

    match get_provincias(page, limit, &estado.db).await {
        Ok(provincias) if provincias.is_empty() => (
            StatusCode::NOT_FOUND,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "No se encontraron provincias",
                data: None,
            })),
        )
            .into_response(),
        Ok(provincias) => (
            StatusCode::OK,
            Js(json!(Ress::<Vec<Provincia>> {
                message: Respuesta::Success.as_str(),
                description: "Provincias encontradas",
                data: Some(provincias),
            })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("{}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error obteniendo provincias",
                    data: None,
                })),
            )
                .into_response()
        }
    }
}
