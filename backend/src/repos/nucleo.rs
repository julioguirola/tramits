use serde::Serialize;
use sqlx::{Arguments, Error, FromRow, Pool, Postgres};

#[derive(FromRow, Serialize)]
pub struct Nucleo {
    pub id: i32,
    pub direccion: String,
    pub bodega_id: i32,
}

pub async fn get_nucleos(
    page: i64,
    limit: i64,
    bodega_id: Option<i32>,
    db: &Pool<Postgres>,
) -> Result<Vec<Nucleo>, Error> {
    let offset = (page - 1) * limit;
    let mut consulta =
        "SELECT id, direccion, bodega_id FROM nucleo WHERE 1=1".to_string();
    let mut argumentos = sqlx::postgres::PgArguments::default();
    let mut i = 1;

    if let Some(bodega) = bodega_id {
        consulta.push_str(&format!(" AND bodega_id = ${i}"));
        let _ = argumentos.add(bodega);
        i += 1;
    }

    consulta.push_str(&format!(" ORDER BY id LIMIT ${i} OFFSET ${}", i + 1));
    let _ = argumentos.add(limit);
    let _ = argumentos.add(offset);

    sqlx::query_as_with::<_, Nucleo, _>(&consulta, argumentos)
        .fetch_all(db)
        .await
}
