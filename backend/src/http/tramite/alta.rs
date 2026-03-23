use std::sync::Arc;

use axum::{
    Extension,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json as Js, Response},
};
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::{
    AppState,
    repos::{tramite::alta::crear_tramite_alta, usuario::UsuarioJwt},
    tipos::{Respuesta, Ress},
};

#[derive(Deserialize)]
pub struct AltaDto {
    pub nucleo_id: i32,
}

pub async fn crear_alta_h(
    State(state): State<Arc<AppState>>,
    Extension(usr): Extension<UsuarioJwt>,
    Js(body): Js<AltaDto>,
) -> Response {
    match crear_tramite_alta(&state.db, &usr, body.nucleo_id).await {
        Ok(_) => (
            StatusCode::CREATED,
            Js(json!(Ress::<()> {
                message: Respuesta::Success.as_str(),
                description: "Solicitud de alta creada exitosamente",
                data: None
            })),
        )
            .into_response(),
        Err(e) => {
            let error_msg = e.to_string();
            error!("{}", error_msg);
            
            if error_msg.contains("Ya tienes un núcleo") {
                return (
                    StatusCode::BAD_REQUEST,
                    Js(json!(Ress::<u8> {
                        message: Respuesta::Warn.as_str(),
                        description: "Ya tienes un núcleo asignado",
                        data: None
                    })),
                )
                    .into_response();
            }
            
            if error_msg.contains("Ya tienes una solicitud") {
                return (
                    StatusCode::BAD_REQUEST,
                    Js(json!(Ress::<u8> {
                        message: Respuesta::Warn.as_str(),
                        description: "Ya tienes una solicitud de alta pendiente",
                        data: None
                    })),
                )
                    .into_response();
            }
            
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error creando solicitud de alta",
                    data: None
                })),
            )
                .into_response()
        }
    }
}
