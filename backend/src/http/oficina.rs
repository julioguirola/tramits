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
    repos::oficina::{Oficina, get_oficinas},
    tipos::{Respuesta, Ress},
};

#[derive(Deserialize)]
pub struct GetOficinasParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub municipio_id: Option<i32>,
}

pub async fn get_oficinas_h(
    State(estado): State<Arc<AppState>>,
    Query(params): Query<GetOficinasParams>,
) -> Response {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).clamp(1, 100);

    match get_oficinas(page, limit, params.municipio_id, &estado.db).await {
        Ok(oficinas) if oficinas.is_empty() => (
            StatusCode::NOT_FOUND,
            Js(json!(Ress::<u8> {
                message: Respuesta::Warn.as_str(),
                description: "No se encontraron oficinas",
                data: None,
            })),
        )
            .into_response(),
        Ok(oficinas) => (
            StatusCode::OK,
            Js(json!(Ress::<Vec<Oficina>> {
                message: Respuesta::Success.as_str(),
                description: "Oficinas encontradas",
                data: Some(oficinas),
            })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("{}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error obteniendo oficinas",
                    data: None,
                })),
            )
                .into_response()
        }
    }
}
