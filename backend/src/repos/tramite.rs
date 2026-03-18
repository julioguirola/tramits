use sqlx::{Error, Pool, Postgres};

use crate::repos::usuario::claims_from_cookie;

pub async fn crear_tramite(
    db: &Pool<Postgres>,
    secret: &str,
    cookie_header: &str,
    nucleo_id: i32,
    tipo_id: i32,
) -> Result<i32, Error> {
    let claims = claims_from_cookie(cookie_header, secret)
        .ok_or_else(|| Error::Protocol("Token inválido".into()))?;

    let persona_id: sqlx::types::Uuid =
        sqlx::query_scalar("select persona_id from usuario where email = $1;")
            .bind(&claims.email)
            .fetch_one(db)
            .await?;

    sqlx::query_scalar(
        "insert into tramite (persona_id, nucleo_id, tipo_id) values ($1, $2, $3) returning id;",
    )
    .bind(persona_id)
    .bind(nucleo_id)
    .bind(tipo_id)
    .fetch_one(db)
    .await
}
