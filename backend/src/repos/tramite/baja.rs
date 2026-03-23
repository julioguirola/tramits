use axum::http::StatusCode;
use axum::response::{IntoResponse, Json as Js, Response};
use serde_json::json;
use sqlx::{Error, Pool, Postgres, types::Uuid};

use crate::repos::usuario::UsuarioJwt;
use crate::tipos::{Respuesta, Ress};

/// Verifica si el usuario tiene una solicitud de baja pendiente
pub async fn tiene_baja_pendiente(db: &Pool<Postgres>, persona_id: &Uuid) -> Result<bool, Error> {
    let count: i64 = sqlx::query_scalar(
        "select count(*) from tramite 
         where persona_id = $1 
           and tipo_id = 2 
           and estado_id in (1, 2);",
    )
    .bind(persona_id)
    .fetch_one(db)
    .await?;

    Ok(count > 0)
}

pub async fn crear_baja(db: &Pool<Postgres>, usr: &UsuarioJwt, nucleo_id: i32) -> Response {
    let persona_id: Uuid =
        match sqlx::query_scalar("select persona_id from usuario where email = $1;")
            .bind(&usr.email)
            .fetch_one(db)
            .await
        {
            Ok(id) => id,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Js(json!(Ress::<u8> {
                        message: Respuesta::Error.as_str(),
                        description: "Error creando solicitud de baja",
                        data: None
                    })),
                )
                    .into_response();
            }
        };

    // Verificar si ya tiene una solicitud de baja pendiente
    match tiene_baja_pendiente(db, &persona_id).await {
        Ok(true) => {
            return (
                StatusCode::CONFLICT,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Warn.as_str(),
                    description: "Ya tienes una solicitud de baja pendiente",
                    data: None
                })),
            )
                .into_response();
        }
        Ok(false) => {}
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error creando solicitud de baja",
                    data: None
                })),
            )
                .into_response();
        }
    }

    // Crear solicitud de baja (tipo_id = 2)
    match sqlx::query_scalar(
        "insert into tramite (persona_id, nucleo_id, tipo_id) values ($1, $2, $3) returning id;",
    )
    .bind(persona_id)
    .bind(nucleo_id)
    .bind(2) // tipo_id = 2 para baja
    .fetch_one(db)
    .await
    {
        Ok(id) => (
            StatusCode::CREATED,
            Js(json!(Ress::<Uuid> {
                message: Respuesta::Success.as_str(),
                description: "Solicitud de baja creada",
                data: Some(id)
            })),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "Error creando solicitud de baja",
                data: None
            })),
        )
            .into_response(),
    }
}
