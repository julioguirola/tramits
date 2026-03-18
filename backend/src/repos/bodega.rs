use serde::Serialize;
use sqlx::{Arguments, Error, FromRow, Pool, Postgres};

#[derive(FromRow, Serialize)]
pub struct Bodega {
    pub id: i32,
    pub nombre: String,
    pub oficina_id: i32,
}

pub async fn get_bodegas(
    page: i64,
    limit: i64,
    oficina_id: Option<i32>,
    db: &Pool<Postgres>,
) -> Result<Vec<Bodega>, Error> {
    let offset = (page - 1) * limit;
    let mut consulta = "SELECT id, nombre, oficina_id FROM bodega WHERE 1=1".to_string();
    let mut argumentos = sqlx::postgres::PgArguments::default();
    let mut i = 1;

    if let Some(oficina) = oficina_id {
        consulta.push_str(&format!(" AND oficina_id = ${i}"));
        let _ = argumentos.add(oficina);
        i += 1;
    }

    consulta.push_str(&format!(" ORDER BY id LIMIT ${i} OFFSET ${}", i + 1));
    let _ = argumentos.add(limit);
    let _ = argumentos.add(offset);

    sqlx::query_as_with::<_, Bodega, _>(&consulta, argumentos)
        .fetch_all(db)
        .await
}
