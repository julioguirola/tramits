use serde::Serialize;
use sqlx::{Arguments, Error, FromRow, Pool, Postgres};

#[derive(FromRow, Serialize)]
pub struct Oficina {
    pub id: i32,
    pub nombre: String,
    pub municipio_id: i32,
}

pub async fn get_oficinas(
    page: i64,
    limit: i64,
    municipio_id: Option<i32>,
    db: &Pool<Postgres>,
) -> Result<Vec<Oficina>, Error> {
    let offset = (page - 1) * limit;
    let mut consulta = "SELECT id, nombre, municipio_id FROM oficina WHERE 1=1".to_string();
    let mut argumentos = sqlx::postgres::PgArguments::default();
    let mut i = 1;

    if let Some(municipio) = municipio_id {
        consulta.push_str(&format!(" AND municipio_id = ${i}"));
        let _ = argumentos.add(municipio);
        i += 1;
    }

    consulta.push_str(&format!(" ORDER BY id LIMIT ${i} OFFSET ${}", i + 1));
    let _ = argumentos.add(limit);
    let _ = argumentos.add(offset);

    sqlx::query_as_with::<_, Oficina, _>(&consulta, argumentos)
        .fetch_all(db)
        .await
}
