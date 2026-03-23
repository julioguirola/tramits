use std::sync::Arc;

use axum::{Extension, Json, extract::State, response::Response};
use serde::Deserialize;

use crate::{
    AppState,
    repos::{tramite::baja::crear_baja, usuario::UsuarioJwt},
};

#[derive(Deserialize)]
pub struct BajaDto {
    nucleo_id: i32,
}

pub async fn crear_baja_h(
    State(state): State<Arc<AppState>>,
    Extension(usr): Extension<UsuarioJwt>,
    Json(body): Json<BajaDto>,
) -> Response {
    return crear_baja(&state.db, &usr, body.nucleo_id).await;
}
