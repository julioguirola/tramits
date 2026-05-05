use std::sync::Arc;

use axum::{
    Extension,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Json as Js, Response},
};
use serde::Serialize;
use serde_json::json;
use sqlx::{Arguments, FromRow, Pool, Postgres};
use tracing::error;

use crate::{
    AppState,
    repos::usuario::UsuarioJwt,
    tipos::{Respuesta, Ress},
};

#[derive(serde::Deserialize)]
pub struct EstadisticasQuery {
    pub provincia_id: Option<i32>,
    pub municipio_id: Option<i32>,
    pub oficina_id: Option<i32>,
}

#[derive(Serialize, FromRow)]
pub struct TramiteTipoCount {
    pub nombre: String,
    pub count: i64,
}

#[derive(Serialize, FromRow)]
pub struct TramiteMesCount {
    pub mes: String,
    pub count: i64,
}

#[derive(Serialize)]
pub struct EstadisticasOficina {
    pub total_tramites: i64,
    pub pendientes: i64,
    pub en_proceso: i64,
    pub completados: i64,
    pub rechazados: i64,
    pub cancelados: i64,
    pub total_bodegas: i64,
    pub total_nucleos: i64,
    pub total_personas_atendidas: i64,
    pub tramites_por_tipo: Vec<TramiteTipoCount>,
    pub tramites_por_mes: Vec<TramiteMesCount>,
}

pub async fn get_estadisticas_h(
    State(state): State<Arc<AppState>>,
    Extension(usr): Extension<UsuarioJwt>,
    Query(params): Query<EstadisticasQuery>,
) -> Response {
    if usr.rol != "Registrador" && usr.rol != "Administrador" {
        return (
            StatusCode::FORBIDDEN,
            Js(json!(Ress::<u8> {
                message: Respuesta::Error.as_str(),
                description: "No autorizado para ver estadísticas",
                data: None
            })),
        )
            .into_response();
    }

    let filtros = if usr.rol == "Administrador" {
        Some(&params)
    } else {
        None
    };
    let result = get_estadisticas_oficina(&state.db, &usr, filtros).await;

    match result {
        Ok(stats) => (
            StatusCode::OK,
            Js(json!(Ress::<EstadisticasOficina> {
                message: Respuesta::Success.as_str(),
                description: "Estadísticas obtenidas",
                data: Some(stats)
            })),
        )
            .into_response(),
        Err(e) => {
            error!("{}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Js(json!(Ress::<u8> {
                    message: Respuesta::Error.as_str(),
                    description: "Error obteniendo estadísticas",
                    data: None
                })),
            )
                .into_response()
        }
    }
}

async fn get_estadisticas_oficina(
    db: &Pool<Postgres>,
    usr: &UsuarioJwt,
    filtros: Option<&EstadisticasQuery>,
) -> Result<EstadisticasOficina, sqlx::Error> {
    let total = count_tramites(db, usr, filtros, None).await?;
    let pendientes = count_tramites(db, usr, filtros, Some("t.estado_id = 1")).await?;
    let en_proceso = count_tramites(db, usr, filtros, Some("t.estado_id = 2")).await?;
    let completados = count_tramites(db, usr, filtros, Some("t.estado_id = 3")).await?;
    let rechazados = count_tramites(db, usr, filtros, Some("t.estado_id = 4")).await?;
    let cancelados = count_tramites(db, usr, filtros, Some("t.estado_id = 5")).await?;
    let total_bodegas = count_bodegas(db, usr, filtros).await?;
    let total_nucleos = count_nucleos(db, usr, filtros).await?;
    let total_personas = count_personas(db, usr, filtros).await?;
    let tramites_por_tipo = get_tramites_por_tipo(db, usr, filtros).await?;
    let tramites_por_mes = get_tramites_por_mes(db, usr, filtros).await?;

    Ok(EstadisticasOficina {
        total_tramites: total,
        pendientes,
        en_proceso,
        completados,
        rechazados,
        cancelados,
        total_bodegas,
        total_nucleos,
        total_personas_atendidas: total_personas,
        tramites_por_tipo,
        tramites_por_mes,
    })
}

async fn count_tramites(
    db: &Pool<Postgres>,
    usr: &UsuarioJwt,
    filtros: Option<&EstadisticasQuery>,
    estado_filter: Option<&str>,
) -> Result<i64, sqlx::Error> {
    let mut query = String::from(
        "select count(*) from tramite t join nucleo n on n.id = t.nucleo_id join bodega b on b.id = n.bodega_id join oficina o on o.id = b.oficina_id join municipio m on m.id = o.municipio_id join provincia pr on pr.id = m.provincia_id where 1=1",
    );
    let mut args = sqlx::postgres::PgArguments::default();
    let mut param_count = 1;

    if usr.rol == "Registrador" {
        let oficina_id: Option<i32> =
            sqlx::query_scalar("select oficina_id from usuario where id = $1;")
                .bind(usr.sub)
                .fetch_one(db)
                .await?;

        if let Some(oficina) = oficina_id {
            query.push_str(&format!(" and o.id = ${}", param_count));
            let _ = args.add(oficina);
            param_count += 1;
        }
    } else if let Some(filtros) = filtros {
        apply_filtros(&mut query, &mut args, &mut param_count, filtros);
    }

    if let Some(filter) = estado_filter {
        query.push_str(&format!(
            " and {}",
            filter.replace("$1", &format!("${}", param_count))
        ));
    }

    query.push_str(";");
    let count: (i64,) = sqlx::query_as_with::<_, (i64,), _>(&query, args)
        .fetch_one(db)
        .await?;
    Ok(count.0)
}

async fn count_bodegas(
    db: &Pool<Postgres>,
    usr: &UsuarioJwt,
    filtros: Option<&EstadisticasQuery>,
) -> Result<i64, sqlx::Error> {
    let mut query = String::from(
        "select count(distinct b.id) from bodega b join oficina o on o.id = b.oficina_id join municipio m on m.id = o.municipio_id join provincia pr on pr.id = m.provincia_id where 1=1",
    );
    let mut args = sqlx::postgres::PgArguments::default();
    let mut param_count = 1;

    if usr.rol == "Registrador" {
        let oficina_id: Option<i32> =
            sqlx::query_scalar("select oficina_id from usuario where id = $1;")
                .bind(usr.sub)
                .fetch_one(db)
                .await?;

        if let Some(oficina) = oficina_id {
            query.push_str(&format!(" and o.id = ${}", param_count));
            let _ = args.add(oficina);
        }
    } else if let Some(filtros) = filtros {
        apply_filtros(&mut query, &mut args, &mut param_count, filtros);
    }

    query.push_str(";");
    let count: (i64,) = sqlx::query_as_with::<_, (i64,), _>(&query, args)
        .fetch_one(db)
        .await?;
    Ok(count.0)
}

async fn count_nucleos(
    db: &Pool<Postgres>,
    usr: &UsuarioJwt,
    filtros: Option<&EstadisticasQuery>,
) -> Result<i64, sqlx::Error> {
    let mut query = String::from(
        "select count(distinct n.id) from nucleo n join bodega b on b.id = n.bodega_id join oficina o on o.id = b.oficina_id join municipio m on m.id = o.municipio_id join provincia pr on pr.id = m.provincia_id where 1=1",
    );
    let mut args = sqlx::postgres::PgArguments::default();
    let mut param_count = 1;

    if usr.rol == "Registrador" {
        let oficina_id: Option<i32> =
            sqlx::query_scalar("select oficina_id from usuario where id = $1;")
                .bind(usr.sub)
                .fetch_one(db)
                .await?;

        if let Some(oficina) = oficina_id {
            query.push_str(&format!(" and o.id = ${}", param_count));
            let _ = args.add(oficina);
        }
    } else if let Some(filtros) = filtros {
        apply_filtros(&mut query, &mut args, &mut param_count, filtros);
    }

    query.push_str(";");
    let count: (i64,) = sqlx::query_as_with::<_, (i64,), _>(&query, args)
        .fetch_one(db)
        .await?;
    Ok(count.0)
}

async fn count_personas(
    db: &Pool<Postgres>,
    usr: &UsuarioJwt,
    filtros: Option<&EstadisticasQuery>,
) -> Result<i64, sqlx::Error> {
    let mut query = String::from(
        "select count(distinct t.persona_id) from tramite t join nucleo n on n.id = t.nucleo_id join bodega b on b.id = n.bodega_id join oficina o on o.id = b.oficina_id join municipio m on m.id = o.municipio_id join provincia pr on pr.id = m.provincia_id where 1=1",
    );
    let mut args = sqlx::postgres::PgArguments::default();
    let mut param_count = 1;

    if usr.rol == "Registrador" {
        let oficina_id: Option<i32> =
            sqlx::query_scalar("select oficina_id from usuario where id = $1;")
                .bind(usr.sub)
                .fetch_one(db)
                .await?;

        if let Some(oficina) = oficina_id {
            query.push_str(&format!(" and o.id = ${}", param_count));
            let _ = args.add(oficina);
        }
    } else if let Some(filtros) = filtros {
        apply_filtros(&mut query, &mut args, &mut param_count, filtros);
    }

    query.push_str(";");
    let count: (i64,) = sqlx::query_as_with::<_, (i64,), _>(&query, args)
        .fetch_one(db)
        .await?;
    Ok(count.0)
}

async fn get_tramites_por_tipo(
    db: &Pool<Postgres>,
    usr: &UsuarioJwt,
    filtros: Option<&EstadisticasQuery>,
) -> Result<Vec<TramiteTipoCount>, sqlx::Error> {
    let mut query = String::from(
        "select tt.nombre, count(t.id) as count from tramite t join tramite_tipo tt on tt.id = t.tipo_id join nucleo n on n.id = t.nucleo_id join bodega b on b.id = n.bodega_id join oficina o on o.id = b.oficina_id join municipio m on m.id = o.municipio_id join provincia pr on pr.id = m.provincia_id where 1=1",
    );
    let mut args = sqlx::postgres::PgArguments::default();
    let mut param_count = 1;

    if usr.rol == "Registrador" {
        let oficina_id: Option<i32> =
            sqlx::query_scalar("select oficina_id from usuario where id = $1;")
                .bind(usr.sub)
                .fetch_one(db)
                .await?;

        if let Some(oficina) = oficina_id {
            query.push_str(&format!(" and o.id = ${}", param_count));
            let _ = args.add(oficina);
        }
    } else if let Some(filtros) = filtros {
        apply_filtros(&mut query, &mut args, &mut param_count, filtros);
    }

    query.push_str(" group by tt.nombre order by count desc limit 10;");
    Ok(sqlx::query_as_with::<_, TramiteTipoCount, _>(&query, args)
        .fetch_all(db)
        .await?)
}

async fn get_tramites_por_mes(
    db: &Pool<Postgres>,
    usr: &UsuarioJwt,
    filtros: Option<&EstadisticasQuery>,
) -> Result<Vec<TramiteMesCount>, sqlx::Error> {
    let mut query = String::from(
        "select to_char(t.fecha_solicitud, 'YYYY-MM') as mes, count(t.id) as count from tramite t join nucleo n on n.id = t.nucleo_id join bodega b on b.id = n.bodega_id join oficina o on o.id = b.oficina_id join municipio m on m.id = o.municipio_id join provincia pr on pr.id = m.provincia_id where 1=1",
    );
    let mut args = sqlx::postgres::PgArguments::default();
    let mut param_count = 1;

    if usr.rol == "Registrador" {
        let oficina_id: Option<i32> =
            sqlx::query_scalar("select oficina_id from usuario where id = $1;")
                .bind(usr.sub)
                .fetch_one(db)
                .await?;

        if let Some(oficina) = oficina_id {
            query.push_str(&format!(" and o.id = ${}", param_count));
            let _ = args.add(oficina);
        }
    } else if let Some(filtros) = filtros {
        apply_filtros(&mut query, &mut args, &mut param_count, filtros);
    }

    query.push_str(" group by to_char(t.fecha_solicitud, 'YYYY-MM') order by mes asc limit 13;");
    Ok(sqlx::query_as_with::<_, TramiteMesCount, _>(&query, args)
        .fetch_all(db)
        .await?)
}

fn apply_filtros(
    query: &mut String,
    args: &mut sqlx::postgres::PgArguments,
    param_count: &mut i32,
    filtros: &EstadisticasQuery,
) {
    if let Some(oficina) = filtros.oficina_id {
        query.push_str(&format!(" and o.id = ${}", *param_count));
        let _ = args.add(oficina);
        *param_count += 1;
    }
    if let Some(municipio) = filtros.municipio_id {
        query.push_str(&format!(" and m.id = ${}", *param_count));
        let _ = args.add(municipio);
        *param_count += 1;
    }
    if let Some(provincia) = filtros.provincia_id {
        query.push_str(&format!(" and pr.id = ${}", *param_count));
        let _ = args.add(provincia);
        *param_count += 1;
    }
}
