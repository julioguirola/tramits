use serde::Deserialize;
use sqlx::{Error, Pool, Postgres, prelude::FromRow, types::Uuid};

use crate::{
    mail::{EmailType, send_email},
    repos::usuario::UsuarioJwt,
};

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum AccionGestion {
    Completar,
    Rechazar,
}

#[derive(FromRow)]
struct TramiteEnProceso {
    tipo_id: i32,
    estado_id: i32,
    registrador_id: Option<Uuid>,
    persona_id: Uuid,
    nucleo_id: i32,
}

pub enum GestionarTramiteError {
    NoEncontrado,
    NoAsignado,
    NoEnProceso,
    YaFinalizado,
    NoEsTuSolicitud,
    NoActualizado,
    Db(Error),
}

pub async fn gestionar_tramite(
    db: &Pool<Postgres>,
    usr: &UsuarioJwt,
    tramite_id: Uuid,
    accion: AccionGestion,
) -> Result<(), GestionarTramiteError> {
    let tramite = sqlx::query_as::<_, TramiteEnProceso>(
        "select tipo_id, estado_id, registrador_id, persona_id, nucleo_id
         from tramite
         where id = $1;",
    )
    .bind(tramite_id)
    .fetch_optional(db)
    .await
    .map_err(GestionarTramiteError::Db)?;

    let Some(tramite) = tramite else {
        return Err(GestionarTramiteError::NoEncontrado);
    };

    if tramite.registrador_id.is_none() {
        return Err(GestionarTramiteError::NoAsignado);
    }
    if tramite.registrador_id != Some(usr.sub) {
        return Err(GestionarTramiteError::NoEsTuSolicitud);
    }
    if tramite.estado_id == 3 || tramite.estado_id == 4 || tramite.estado_id == 5 {
        return Err(GestionarTramiteError::YaFinalizado);
    }
    if tramite.estado_id != 2 {
        return Err(GestionarTramiteError::NoEnProceso);
    }

    let mut tx = db.begin().await.map_err(GestionarTramiteError::Db)?;

    match accion {
        AccionGestion::Completar => {
            if tramite.tipo_id == 2 {
                let persona_update =
                    sqlx::query("update persona set nucleo_id = null where id = $1;")
                        .bind(tramite.persona_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(GestionarTramiteError::Db)?;

                if persona_update.rows_affected() == 0 {
                    return Err(GestionarTramiteError::NoActualizado);
                }
            } else if tramite.tipo_id == 1 {
                let persona_update =
                    sqlx::query("update persona set nucleo_id = $1 where id = $2;")
                        .bind(tramite.nucleo_id)
                        .bind(tramite.persona_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(GestionarTramiteError::Db)?;

                if persona_update.rows_affected() == 0 {
                    return Err(GestionarTramiteError::NoActualizado);
                }
            }

            let tramite_update = sqlx::query(
                "update tramite
                 set estado_id = 3,
                     fecha_finalizado = current_date
                 where id = $1
                   and estado_id = 2
                   and registrador_id = $2;",
            )
            .bind(tramite_id)
            .bind(usr.sub)
            .execute(&mut *tx)
            .await
            .map_err(GestionarTramiteError::Db)?;

            if tramite_update.rows_affected() == 0 {
                return Err(GestionarTramiteError::NoActualizado);
            }
        }
        AccionGestion::Rechazar => {
            let tramite_update = sqlx::query(
                "update tramite
                 set estado_id = 4,
                     fecha_finalizado = current_date
                 where id = $1
                   and estado_id = 2
                   and registrador_id = $2;",
            )
            .bind(tramite_id)
            .bind(usr.sub)
            .execute(&mut *tx)
            .await
            .map_err(GestionarTramiteError::Db)?;

            if tramite_update.rows_affected() == 0 {
                return Err(GestionarTramiteError::NoActualizado);
            }
        }
    }

    tx.commit().await.map_err(GestionarTramiteError::Db)?;
    Ok(())
}
