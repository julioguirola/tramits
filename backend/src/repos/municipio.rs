use serde::Serialize;
use sqlx::{Arguments, Error, FromRow, Pool, Postgres};

#[derive(FromRow, Serialize)]
pub struct Municipio {
    pub id: i32,
    pub nombre: String,
    pub provincia_id: i32,
}

pub async fn get_municipios(
    page: i64,
    limit: i64,
    provincia_id: Option<i32>,
    db: &Pool<Postgres>,
) -> Result<Vec<Municipio>, Error> {
    let offset = (page - 1) * limit;
    let mut consulta = "SELECT id, nombre, provincia_id FROM municipio WHERE 1=1".to_string();
    let mut argumentos = sqlx::postgres::PgArguments::default();
    let mut i = 1;

    if let Some(provincia) = provincia_id {
        consulta.push_str(&format!(" AND provincia_id = ${i}"));
        let _ = argumentos.add(provincia);
        i += 1;
    }

    consulta.push_str(&format!(" ORDER BY id LIMIT ${i} OFFSET ${}", i + 1));
    let _ = argumentos.add(limit);
    let _ = argumentos.add(offset);

    sqlx::query_as_with::<_, Municipio, _>(&consulta, argumentos)
        .fetch_all(db)
        .await
}
