use serde::Serialize;
use sqlx::{Arguments, Error, FromRow, Pool, Postgres};

#[derive(FromRow, Serialize)]
pub struct Provincia {
    pub id: i32,
    pub nombre: String,
}

pub async fn get_provincias(
    page: i64,
    limit: i64,
    db: &Pool<Postgres>,
) -> Result<Vec<Provincia>, Error> {
    let offset = (page - 1) * limit;
    let mut consulta = "SELECT id, nombre FROM provincia".to_string();
    let mut argumentos = sqlx::postgres::PgArguments::default();

    consulta.push_str(" ORDER BY id LIMIT $1 OFFSET $2");
    let _ = argumentos.add(limit);
    let _ = argumentos.add(offset);

    sqlx::query_as_with::<_, Provincia, _>(&consulta, argumentos)
        .fetch_all(db)
        .await
}
