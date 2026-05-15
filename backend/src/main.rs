use axum::{
    Router,
    http::StatusCode,
    middleware::{self},
    routing::{any, get, post},
};
use deadpool_redis::{Config as RedisConfig, Runtime};
use std::sync::Arc;
use tracing::info;
mod config;
mod db;
mod http;
mod mail;
mod middlewares;
mod repos;
mod tipos;
use crate::http::{
    bodega, municipio, nucleo, oficina, password_recovery, persona, provincia, tramite, usuario,
};
use crate::middlewares::{auth::auth_m, cors::cors_m, logger::logger_m};
use config::EnvConfig;
use redis::cmd;
use sqlx::{Pool, Postgres};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(Clone)]
struct AppState {
    env_config: Arc<EnvConfig>,
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    let config = EnvConfig::new();
    let db = db::init_db(&config).await?;

    let redis = RedisConfig::from_url(&config.redis_url)
        .create_pool(Some(Runtime::Tokio1))
        .expect("Error creando pool de Redis");

    if config.environment == "dev" {
        let mut conn = redis.get().await.unwrap();
        cmd("FLUSHDB").query_async::<()>(&mut conn).await.unwrap();
    }

    let shared_state = Arc::new(AppState {
        env_config: Arc::new(config),
        db,
    });
    let state_clone = shared_state.clone();

    // let router_auth_cache = Router::new()
    //     .layer(middleware::from_fn_with_state(
    //         shared_state.clone(),
    //         cache_m,
    //     ))
    //     .layer(middleware::from_fn_with_state(shared_state.clone(), auth_m));

    let routes_auth = Router::new()
        .route("/tramite/alta", post(tramite::alta::crear_alta_h))
        .route("/tramite/baja", post(tramite::baja::crear_baja_h))
        .route("/tramite/libreta", post(tramite::libreta::crear_libreta_h))
        .route(
            "/tramite/{id}/procesar",
            post(tramite::procesar::procesar_solicitud_h),
        )
        .route(
            "/tramite/{id}/gestionar",
            post(tramite::gestionar::gestionar_tramite_h),
        )
        .route(
            "/tramite/{id}/correo",
            post(tramite::correo::enviar_correo_tramite_h),
        )
        .route(
            "/tramite/{id}/correo-registrador",
            post(tramite::correo::enviar_correo_registrador_h),
        )
        .route(
            "/tramite/{id}/cancelar",
            post(tramite::cancelar::cancelar_tramite_h),
        )
        .route("/tramite", get(tramite::historial::get_historial_h))
        .route(
            "/tramite/historial/registrador",
            get(tramite::historial_registrador::get_historial_registrador_h),
        )
        .route(
            "/tramite/estadisticas",
            get(tramite::estadisticas::get_estadisticas_h),
        )
        .route("/usuario/me", get(usuario::me_h))
        .route("/provincia", get(provincia::get_provincias_h))
        .route("/municipio", get(municipio::get_municipios_h))
        .route("/oficina", get(oficina::get_oficinas_h))
        .route("/bodega", get(bodega::get_bodegas_h))
        .route("/nucleo", get(nucleo::get_nucleos_h))
        .route("/usuarios", get(usuario::listar_usuarios_h))
        .route(
            "/usuarios/sin-nucleo",
            get(usuario::listar_usuarios_sin_nucleo_h),
        )
        .route(
            "/usuarios/estado",
            post(usuario::actualizar_estado_usuario_h),
        )
        .route("/usuarios/rol", post(usuario::actualizar_rol_usuario_h))
        .route("/usuarios/correo", post(usuario::enviar_correo_usuario_h))
        .layer(middleware::from_fn_with_state(shared_state.clone(), auth_m));

    let routes = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/logout", post(usuario::logout_h))
        .route("/persona", get(persona::get_personas_h))
        .route("/usuario", post(usuario::crear_usuario_h))
        .route("/login", post(usuario::login_usuario_h))
        .route(
            "/password-recovery/solicitar",
            post(password_recovery::solicitar_h),
        )
        .route(
            "/password-recovery/restablecer/{uuid}",
            post(password_recovery::restablecer_h),
        )
        .merge(routes_auth)
        .route("/{*path}", any(|| async { StatusCode::NO_CONTENT }))
        .layer(middleware::from_fn_with_state(shared_state.clone(), cors_m))
        .layer(middleware::from_fn(logger_m))
        .with_state(shared_state);

    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &state_clone.env_config.port))
            .await
            .unwrap();
    info!(
        "Servidor {} corriendo en el puerto: {}",
        &state_clone.env_config.environment, &state_clone.env_config.port,
    );
    axum::serve(listener, routes).await.unwrap();
    Ok(())
}
