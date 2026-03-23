use sqlx::{Error, Pool, Postgres, types::Uuid};

use crate::repos::usuario::UsuarioJwt;

pub async fn crear_tramite_alta(
    db: &Pool<Postgres>,
    usr: &UsuarioJwt,
    nucleo_id: i32,
) -> Result<Uuid, Error> {
    // Obtener persona_id del usuario
    let persona_id: Uuid = sqlx::query_scalar("select persona_id from usuario where email = $1;")
        .bind(&usr.email)
        .fetch_one(db)
        .await?;

    // Verificar si la persona ya tiene un núcleo asignado
    let tiene_nucleo: Option<i32> =
        sqlx::query_scalar("select nucleo_id from persona where id = $1;")
            .bind(persona_id)
            .fetch_one(db)
            .await?;

    if tiene_nucleo.is_some() {
        return Err(Error::Protocol(String::from(
            "Ya tienes un núcleo asignado",
        )));
    }

    // Verificar si ya tiene una solicitud de alta pendiente o en proceso
    let tiene_solicitud: Option<Uuid> = sqlx::query_scalar(
        "select id from tramite 
         where persona_id = $1 
         and tipo_id = 1 
         and estado_id in (1, 2);",
    )
    .bind(persona_id)
    .fetch_optional(db)
    .await?;

    if tiene_solicitud.is_some() {
        return Err(Error::Protocol(String::from(
            "Ya tienes una solicitud de alta pendiente",
        )));
    }

    // Crear el trámite de alta
    let tramite_id: Uuid = sqlx::query_scalar(
        "insert into tramite (tipo_id, persona_id, nucleo_id, estado_id) 
         values (1, $1, $2, 1) 
         returning id;",
    )
    .bind(persona_id)
    .bind(nucleo_id)
    .fetch_one(db)
    .await?;

    Ok(tramite_id)
}
