use sqlx::{Error, Pool, Postgres, types::Uuid};

use crate::repos::usuario::UsuarioJwt;

pub async fn crear_tramite_libreta(
    db: &Pool<Postgres>,
    usr: &UsuarioJwt,
) -> Result<Uuid, Error> {
    let persona_id: Uuid = sqlx::query_scalar("select persona_id from usuario where id = $1;")
        .bind(usr.sub)
        .fetch_one(db)
        .await?;

    let tiene_solicitud: Option<Uuid> = sqlx::query_scalar(
        "select id from tramite 
         where persona_id = $1 
         and tipo_id = 3 
         and estado_id in (1, 2);",
    )
    .bind(persona_id)
    .fetch_optional(db)
    .await?;

    if tiene_solicitud.is_some() {
        return Err(Error::Protocol(String::from(
            "Ya tienes una solicitud de libreta pendiente",
        )));
    }

    let nucleo_id: i32 = sqlx::query_scalar("select nucleo_id from persona where id = $1;")
        .bind(persona_id)
        .fetch_one(db)
        .await?;

    let tramite_id: Uuid = sqlx::query_scalar(
        "insert into tramite (tipo_id, persona_id, nucleo_id, estado_id) 
         values (3, $1, $2, 1) 
         returning id;",
    )
    .bind(persona_id)
    .bind(nucleo_id)
    .fetch_one(db)
    .await?;

    Ok(tramite_id)
}
