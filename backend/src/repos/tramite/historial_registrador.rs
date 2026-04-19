use serde::Serialize;
use sqlx::{Error, Pool, Postgres};

use crate::repos::{tramite::historial::TramiteHistorial, usuario::UsuarioJwt};

#[derive(Serialize)]
pub struct HistorialRegistradorPaginado {
    pub tramites: Vec<TramiteHistorial>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

pub async fn get_historial_finalizado_registrador(
    db: &Pool<Postgres>,
    usr: &UsuarioJwt,
    page: i64,
    limit: i64,
) -> Result<HistorialRegistradorPaginado, Error> {
    let offset = (page - 1) * limit;

    let total: i64 = sqlx::query_scalar(
        "select count(*)
         from tramite
         where registrador_id = $1
           and estado_id in (3, 4);",
    )
    .bind(usr.sub)
    .fetch_one(db)
    .await?;

    let tramites = sqlx::query_as::<_, TramiteHistorial>(
        "select
            t.id,
            tt.nombre as tipo,
            n.direccion as nucleo,
            to_char(t.fecha_solicitud, 'DD/MM/YYYY') as fecha_solicitud,
            to_char(t.fecha_finalizado, 'DD/MM/YYYY') as fecha_finalizado,
            u.email as registrador,
            te.nombre as estado,
            p.nombre as persona_nombre,
            p.apellido as persona_apellido
         from tramite t
         join tramite_tipo tt on tt.id = t.tipo_id
         join tramite_estado te on te.id = t.estado_id
         join nucleo n on n.id = t.nucleo_id
         join persona p on p.id = t.persona_id
         left join usuario u on u.id = t.registrador_id
         where t.registrador_id = $1
           and t.estado_id in (3, 4)
         order by t.fecha_finalizado desc nulls last, t.fecha_solicitud desc
         limit $2 offset $3;",
    )
    .bind(usr.sub)
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await?;

    Ok(HistorialRegistradorPaginado {
        tramites,
        total,
        page,
        limit,
    })
}
