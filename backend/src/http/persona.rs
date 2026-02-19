use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::json;

use crate::{AppState, repos::persona::get_personas};

#[derive(Deserialize)]
pub struct CrearPersonaDto {
    pub carnet: Option<String>,
    pub page: Option<String>,
    pub limit: Option<String>,
}

pub async fn get_personas_h(
    State(estado): State<Arc<AppState>>,
    Query(params): Query<CrearPersonaDto>,
) -> impl IntoResponse {
    if let Some(carnet) = &params.carnet {
        if carnet.len() != 11 {
            return (
                StatusCode::BAD_REQUEST,
                Json(
                    json!({"message":"Error", "description": "Carnet no válido, debe tener 11 caracteres"}),
                ),
            );
        }
        if carnet.parse::<u64>().is_err() {
            return (
                StatusCode::BAD_REQUEST,
                Json(
                    json!({"message":"Error", "description": "Carnet no válido, solo debe tener caracteres numéricos"}),
                ),
            );
        }
    }

    let personas = get_personas(&params, &estado.db).await;

    match personas {
        Ok(p) => {
            if p.is_empty() {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({"message":"Atención", "description": "Persona/s no encontrada/s"})),
                );
            }
            (
                StatusCode::OK,
                Json(
                    json!({"message":"Éxito", "description": "Persona/s encontrada/s", "data": p}),
                ),
            )
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message":"Error", "description": "Error obteniendo persona/s"})),
        ),
    }
}
