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
    repos::municipio::{Municipio, get_municipios},
    tipos::{Respuesta, Ress},
};

#[derive(Deserialize)]
pub struct GetMunicipiosParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub provincia_id: Option<i32>,
}

pub async fn get_municipios_h(
    State(estado): State<Arc<AppState>>,
    Query(params): Query<GetMunicipiosParams>,
) -> Response {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).clamp(1, 100);

    match get_municipios(page, limit, params.provincia_id, &estado.db).await {
        Ok(municipios) if municipios.is_empty() => (
            StatusCode::NOT_FOUND,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "No se encontraron municipios",
                data: None,
            })),
        )
            .into_response(),
        Ok(municipios) => (
            StatusCode::OK,
            Js(json!(Ress::<Vec<Municipio>> {
                message: Respuesta::Success.as_str(),
                description: "Municipios encontrados",
                data: Some(municipios),
            })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("{}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error obteniendo municipios",
                    data: None,
                })),
            )
                .into_response()
        }
    }
}
