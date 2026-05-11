use std::sync::Arc;

use serde::Deserialize;
use sqlx::{Error, prelude::FromRow, types::Uuid};

use crate::{
    AppState,
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

#[derive(FromRow, Debug)]
struct EmailUser {
    nombre: String,
    apellido: String,
    email: String,
}

pub async fn gestionar_tramite(
    state: Arc<AppState>,
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
    .fetch_optional(&state.db)
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

    let mut tx = state.db.begin().await.map_err(GestionarTramiteError::Db)?;

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
    let user_envia = sqlx::query_as::<_, EmailUser>(
        "select p.nombre, p.apellido, u.email
         from usuario u
         join persona p on u.persona_id = p.id
         where u.id = $1;",
    )
    .bind(usr.sub)
    .fetch_one(&state.db)
    .await;

    let user_recibe = sqlx::query_as::<_, EmailUser>(
        "select p.nombre, p.apellido, u.email
         from tramite t
         join persona p on t.persona_id = p.id
         join usuario u on p.id = u.persona_id
         where t.id = $1 and u.rol_id = 1;",
    )
    .bind(tramite_id)
    .fetch_one(&state.db)
    .await;

    if let Ok(usr_env) = user_envia
        && let Ok(usr_rc) = user_recibe
    {
        let email_type = match accion {
            AccionGestion::Completar => EmailType::TramiteCompletado,
            AccionGestion::Rechazar => EmailType::TramiteRechazado,
        };

        let _ = send_email(
            String::from(usr_env.nombre) + " " + &String::from(usr_env.apellido),
            String::from(usr.email.clone()),
            String::from(usr_rc.nombre) + " " + &String::from(usr_rc.apellido),
            String::from(usr_rc.email),
            email_type,
            &state.env_config,
        );
    }

    Ok(())
}
