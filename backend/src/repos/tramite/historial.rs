use serde::Serialize;
use sqlx::{Error, Pool, Postgres, prelude::FromRow, types::Uuid};

use crate::repos::usuario::UsuarioJwt;

#[derive(Serialize, FromRow)]
pub struct TramiteHistorial {
    pub id: Uuid,
    pub tipo: String,
    pub nucleo: String,
    pub fecha_solicitud: String,
    pub fecha_completado: Option<String>,
    pub registrador: Option<String>,
    pub estado: String,
}

pub async fn get_historial_tramites(
    db: &Pool<Postgres>,
    usr: &UsuarioJwt,
) -> Result<Vec<TramiteHistorial>, Error> {
    let persona_id: Uuid = sqlx::query_scalar("select persona_id from usuario where email = $1;")
        .bind(&usr.email)
        .fetch_one(db)
        .await?;

    let tramites = sqlx::query_as::<_, TramiteHistorial>(
        "select 
            t.id,
            tt.nombre as tipo,
            n.direccion as nucleo,
            to_char(t.fecha_solicitud, 'DD/MM/YYYY') as fecha_solicitud,
            to_char(t.fecha_completado, 'DD/MM/YYYY') as fecha_completado,
            u.email as registrador,
            te.nombre as estado
         from tramite t
         join tramite_tipo tt on tt.id = t.tipo_id
         join tramite_estado te on te.id = t.estado_id
         join nucleo n on n.id = t.nucleo_id
         left join usuario u on u.id = t.usuario_id
         where t.persona_id = $1
         order by t.fecha_solicitud desc;",
    )
    .bind(persona_id)
    .fetch_all(db)
    .await?;

    Ok(tramites)
}
