use std::sync::Arc;

use axum::{
    Extension,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json as Js, Response},
};
use serde_json::json;
use tracing::error;

use crate::{
    AppState,
    repos::{tramite::libreta::crear_tramite_libreta, usuario::UsuarioJwt},
    tipos::{Respuesta, Ress},
};

pub async fn crear_libreta_h(
    State(state): State<Arc<AppState>>,
    Extension(usr): Extension<UsuarioJwt>,
) -> Response {
    match crear_tramite_libreta(&state.db, &usr).await {
        Ok(_) => (
            StatusCode::CREATED,
            Js(json!(Ress::<()> {
                message: Respuesta::Success.as_str(),
                description: "Solicitud de libreta creada exitosamente",
                data: None
            })),
        )
            .into_response(),
        Err(e) => {
            let error_msg = e.to_string();
            error!("{}", error_msg);

            if error_msg.contains("Ya tienes una solicitud") {
                return (
                    StatusCode::BAD_REQUEST,
                    Js(json!(Ress::<u8> {
                        message: Respuesta::Warn.as_str(),
                        description: "Ya tienes una solicitud de libreta pendiente",
                        data: None
                    })),
                )
                    .into_response();
            }

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error creando solicitud de libreta",
                    data: None
                })),
            )
                .into_response()
        }
    }
}
