use sqlx::{Error, Pool, Postgres, types::Uuid};

use crate::repos::usuario::UsuarioJwt;

pub enum CancelarTramiteError {
    NoEncontrado,
    NoAutorizado,
    NoPendiente,
    Db(Error),
}

pub async fn cancelar_tramite(
    db: &Pool<Postgres>,
    usr: &UsuarioJwt,
    tramite_id: Uuid,
) -> Result<(), CancelarTramiteError> {
    let persona_id: Uuid = sqlx::query_scalar("select persona_id from usuario where id = $1;")
        .bind(usr.sub)
        .fetch_one(db)
        .await
        .map_err(CancelarTramiteError::Db)?;

    let tramite = sqlx::query_as::<_, (Uuid, i32)>(
        "select persona_id, estado_id from tramite where id = $1;",
    )
    .bind(tramite_id)
    .fetch_optional(db)
    .await
    .map_err(CancelarTramiteError::Db)?;

    let Some((tramite_persona, estado_id)) = tramite else {
        return Err(CancelarTramiteError::NoEncontrado);
    };

    if tramite_persona != persona_id {
        return Err(CancelarTramiteError::NoAutorizado);
    }

    if estado_id != 1 {
        return Err(CancelarTramiteError::NoPendiente);
    }

    let updated = sqlx::query(
        "update tramite
         set estado_id = 5,
             fecha_finalizado = current_date
         where id = $1
           and estado_id = 1
           and persona_id = $2;",
    )
    .bind(tramite_id)
    .bind(persona_id)
    .execute(db)
    .await
    .map_err(CancelarTramiteError::Db)?;

    if updated.rows_affected() == 0 {
        return Err(CancelarTramiteError::NoPendiente);
    }

    Ok(())
}
