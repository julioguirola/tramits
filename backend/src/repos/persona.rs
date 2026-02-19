use crate::http::persona::CrearPersonaDto;
use serde::Serialize;
use sqlx::{Arguments, Error, FromRow, Pool, Postgres};
use tracing::error;
use uuid::Uuid;

#[derive(FromRow, Serialize)]
pub struct Persona {
    id: Uuid,
    nombre: String,
    apellido: String,
    carnet: String,
    direccion: String,
}

pub async fn get_personas(
    params: &CrearPersonaDto,
    db: &Pool<Postgres>,
) -> Result<Vec<Persona>, Error> {
    let mut consulta = "select p.id, p.nombre, p.apellido, p.carnet, n.direccion
        from persona p
        join nucleo n
        on p.nucleo_id = n.id
        where 1=1"
        .to_string();
    let mut argumentos = sqlx::postgres::PgArguments::default();
    let i = 1;

    if let Some(carnet) = &params.carnet {
        consulta.push_str(&format!(" AND carnet = ${i}"));
        let _ = argumentos.add(&carnet.to_string());
    }

    let personas = sqlx::query_as_with::<_, Persona, _>(&consulta, argumentos)
        .fetch_all(db)
        .await
        .map_err(|e| {
            error!("{}", e);
            e
        })?;

    Ok(personas)
}
