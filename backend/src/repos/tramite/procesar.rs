use sqlx::{Error, Pool, Postgres, prelude::FromRow, types::Uuid};

use crate::repos::usuario::UsuarioJwt;

#[derive(FromRow)]
struct TramiteParaProcesar {
    estado_id: i32,
    registrador_id: Option<Uuid>,
    tramite_oficina: i32,
    registrador_oficina: Option<i32>,
}

pub enum ProcesarBajaError {
    NoEncontrado,
    NoPendiente,
    YaAsignado,
    SinOficina,
    FueraDeOficina,
    Db(Error),
}

pub async fn procesar_solicitud(
    db: &Pool<Postgres>,
    usr: &UsuarioJwt,
    tramite_id: Uuid,
) -> Result<(), ProcesarBajaError> {
    let tramite = sqlx::query_as::<_, TramiteParaProcesar>(
        "select t.estado_id, t.registrador_id,
                b.oficina_id as tramite_oficina,
                u.oficina_id as registrador_oficina
         from tramite t
         join nucleo n on n.id = t.nucleo_id
         join bodega b on b.id = n.bodega_id
         join usuario u on u.id = $2
         where t.id = $1;",
    )
    .bind(tramite_id)
    .bind(usr.sub)
    .fetch_optional(db)
    .await
    .map_err(ProcesarBajaError::Db)?;

    let Some(tramite) = tramite else {
        return Err(ProcesarBajaError::NoEncontrado);
    };

    if tramite.estado_id != 1 {
        return Err(ProcesarBajaError::NoPendiente);
    }
    if tramite.registrador_id.is_some() {
        return Err(ProcesarBajaError::YaAsignado);
    }

    let Some(registrador_oficina) = tramite.registrador_oficina else {
        return Err(ProcesarBajaError::SinOficina);
    };

    if registrador_oficina != tramite.tramite_oficina {
        return Err(ProcesarBajaError::FueraDeOficina);
    }

    let updated = sqlx::query(
        "update tramite
         set registrador_id = $1,
             estado_id = 2
         where id = $2
           and estado_id = 1
           and registrador_id is null;",
    )
    .bind(usr.sub)
    .bind(tramite_id)
    .execute(db)
    .await
    .map_err(ProcesarBajaError::Db)?;

    if updated.rows_affected() == 0 {
        return Err(ProcesarBajaError::YaAsignado);
    }

    Ok(())
}
