use serde::Serialize;
use sqlx::{Arguments, Error, Pool, Postgres, prelude::FromRow, types::Uuid};

use crate::repos::usuario::UsuarioJwt;

#[derive(Serialize, FromRow)]
pub struct TramiteHistorial {
    pub id: Uuid,
    pub tipo: String,
    pub nucleo: String,
    pub fecha_solicitud: String,
    pub fecha_finalizado: Option<String>,
    pub registrador: Option<String>,
    pub registrador_id: Option<Uuid>,
    pub estado: String,
    pub persona_nombre: Option<String>,
    pub persona_apellido: Option<String>,
}

pub async fn get_historial_tramites(
    db: &Pool<Postgres>,
    usr: &UsuarioJwt,
    estado_id: Option<i32>,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Vec<TramiteHistorial>, Error> {
    let persona_id: Uuid = sqlx::query_scalar("select persona_id from usuario where id = $1;")
        .bind(usr.sub)
        .fetch_one(db)
        .await?;

    let mut query = String::from(
        "select 
            t.id,
            tt.nombre as tipo,
            n.direccion as nucleo,
            to_char(t.fecha_solicitud, 'DD/MM/YYYY') as fecha_solicitud,
            to_char(t.fecha_finalizado, 'DD/MM/YYYY') as fecha_finalizado,
            case when pr.id is null then null else pr.nombre || ' ' || pr.apellido end as registrador,
            t.registrador_id,
            te.nombre as estado,
            p.nombre as persona_nombre,
            p.apellido as persona_apellido
         from tramite t
         join tramite_tipo tt on tt.id = t.tipo_id
         join tramite_estado te on te.id = t.estado_id
         join nucleo n on n.id = t.nucleo_id
         join persona p on p.id = t.persona_id
         left join usuario u on u.id = t.registrador_id
         left join persona pr on pr.id = u.persona_id
         where t.persona_id = $1",
    );

    let mut args = sqlx::postgres::PgArguments::default();
    let _ = args.add(persona_id);
    let mut param_count = 1;

    if let Some(estado) = estado_id {
        param_count += 1;
        query.push_str(&format!(" and t.estado_id = ${}", param_count));
        let _ = args.add(estado);
    }

    query.push_str(" order by t.fecha_solicitud desc");

    if let Some(lim) = limit {
        param_count += 1;
        query.push_str(&format!(" limit ${}", param_count));
        let _ = args.add(lim as i64);
    }

    if let Some(off) = offset {
        param_count += 1;
        query.push_str(&format!(" offset ${}", param_count));
        let _ = args.add(off as i64);
    }

    query.push_str(";");

    let tramites = sqlx::query_as_with::<_, TramiteHistorial, _>(&query, args)
        .fetch_all(db)
        .await?;

    Ok(tramites)
}

pub async fn get_todas_solicitudes(
    db: &Pool<Postgres>,
    usr: &UsuarioJwt,
    estado_id: Option<i32>,
    asignadas: Option<bool>,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Vec<TramiteHistorial>, Error> {
    let mut query = String::from(
        "select 
            t.id,
            tt.nombre as tipo,
            n.direccion as nucleo,
            to_char(t.fecha_solicitud, 'DD/MM/YYYY') as fecha_solicitud,
            to_char(t.fecha_finalizado, 'DD/MM/YYYY') as fecha_finalizado,
            case when pr.id is null then null else pr.nombre || ' ' || pr.apellido end as registrador,
            t.registrador_id,
            te.nombre as estado,
            p.nombre as persona_nombre,
            p.apellido as persona_apellido
         from tramite t
         join tramite_tipo tt on tt.id = t.tipo_id
         join tramite_estado te on te.id = t.estado_id
         join nucleo n on n.id = t.nucleo_id
         join bodega b on b.id = n.bodega_id
         join persona p on p.id = t.persona_id
         left join usuario u on u.id = t.registrador_id
         left join persona pr on pr.id = u.persona_id
         where 1=1",
    );

    let mut args = sqlx::postgres::PgArguments::default();
    let mut param_count = 0;

    if usr.rol == "Registrador" {
        let oficina_id: Option<i32> =
            sqlx::query_scalar("select oficina_id from usuario where id = $1;")
                .bind(usr.sub)
                .fetch_one(db)
                .await?;

        if let Some(oficina) = oficina_id {
            param_count += 1;
            query.push_str(&format!(" and b.oficina_id = ${}", param_count));
            let _ = args.add(oficina);
        }

        if asignadas.unwrap_or(false) {
            param_count += 1;
            query.push_str(&format!(" and t.registrador_id = ${}", param_count));
            let _ = args.add(usr.sub);
        }
    }

    if let Some(estado) = estado_id {
        param_count += 1;
        query.push_str(&format!(" and t.estado_id = ${}", param_count));
        let _ = args.add(estado);
    }

    query.push_str(" order by t.fecha_solicitud desc");

    if let Some(lim) = limit {
        param_count += 1;
        query.push_str(&format!(" limit ${}", param_count));
        let _ = args.add(lim as i64);
    }

    if let Some(off) = offset {
        param_count += 1;
        query.push_str(&format!(" offset ${}", param_count));
        let _ = args.add(off as i64);
    }

    query.push_str(";");

    let tramites = sqlx::query_as_with::<_, TramiteHistorial, _>(&query, args)
        .fetch_all(db)
        .await?;

    Ok(tramites)
}

pub async fn get_historial_tramites_count(
    db: &Pool<Postgres>,
    usr: &UsuarioJwt,
    estado_id: Option<i32>,
) -> Result<i64, Error> {
    let persona_id: Uuid = sqlx::query_scalar("select persona_id from usuario where id = $1;")
        .bind(usr.sub)
        .fetch_one(db)
        .await?;

    let mut query = String::from("select count(*) from tramite t where t.persona_id = $1");
    let mut args = sqlx::postgres::PgArguments::default();
    let _ = args.add(persona_id);

    if let Some(estado) = estado_id {
        query.push_str(" and t.estado_id = $2");
        let _ = args.add(estado);
    }

    query.push_str(";");

    let count: i64 = sqlx::query_as_with::<_, (i64,), _>(&query, args)
        .fetch_one(db)
        .await?
        .0;

    Ok(count)
}

pub async fn get_todas_solicitudes_count(
    db: &Pool<Postgres>,
    usr: &UsuarioJwt,
    estado_id: Option<i32>,
    asignadas: Option<bool>,
) -> Result<i64, Error> {
    let mut query = String::from(
        "select count(*) from tramite t
         join nucleo n on n.id = t.nucleo_id
         join bodega b on b.id = n.bodega_id
         where 1=1",
    );

    let mut args = sqlx::postgres::PgArguments::default();
    let mut param_count = 0;

    if usr.rol == "Registrador" {
        let oficina_id: Option<i32> =
            sqlx::query_scalar("select oficina_id from usuario where id = $1;")
                .bind(usr.sub)
                .fetch_one(db)
                .await?;

        if let Some(oficina) = oficina_id {
            param_count += 1;
            query.push_str(&format!(" and b.oficina_id = ${}", param_count));
            let _ = args.add(oficina);
        }

        if asignadas.unwrap_or(false) {
            param_count += 1;
            query.push_str(&format!(" and t.registrador_id = ${}", param_count));
            let _ = args.add(usr.sub);
        }
    }

    if let Some(estado) = estado_id {
        param_count += 1;
        query.push_str(&format!(" and t.estado_id = ${}", param_count));
        let _ = args.add(estado);
    }

    query.push_str(";");

    let count: i64 = sqlx::query_as_with::<_, (i64,), _>(&query, args)
        .fetch_one(db)
        .await?
        .0;

    Ok(count)
}
