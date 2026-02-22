use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    AppState,
    config::tipos::Ress,
    repos::persona::{Persona, get_personas},
};

#[derive(Deserialize)]
pub struct CrearPersonaDto {
    pub carnet: Option<String>,
    // pub page: Option<String>,
    // pub limit: Option<String>,
}

pub async fn get_personas_h(
    State(estado): State<Arc<AppState>>,
    Query(params): Query<CrearPersonaDto>,
) -> impl IntoResponse {
    if let Some(carnet) = &params.carnet {
        if carnet.len() != 11 {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!(Ress::<u8> {
                    message: "Error",
                    description: "Carnet no válido, debe tener 11 caracteres",
                    data: None,
                })),
            );
        }
        if carnet.parse::<u64>().is_err() {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!(Ress::<u8> {
                    message: "Error",
                    description: "Carnet no válido, solo debe tener caracteres numéricos",
                    data: None,
                })),
            );
        }
    }

    let personas = get_personas(&params, &estado.db).await;

    match personas {
        Ok(p) => {
            if p.is_empty() {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!(Ress::<u8> {
                        message: "Atención",
                        description: "Persona/s no encontrada/s",
                        data: None,
                    })),
                );
            }
            (
                StatusCode::OK,
                Json(json!(Ress::<Vec<Persona>> {
                    message: "Éxito",
                    description: "Persona/s encontrada/s",
                    data: Some(p),
                })),
            )
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(Ress::<u8> {
                message: "Error",
                description: "Error obteniendo persona/s",
                data: None
            })),
        ),
    }
}
