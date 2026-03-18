use axum::{
    Router,
    http::StatusCode,
    middleware::{self},
    routing::{any, get, post},
};
use deadpool_redis::{Config as RedisConfig, Pool as RedisPool, Runtime};
use std::sync::Arc;
use tracing::info;
mod config;
mod db;
mod http;
mod repos;

use crate::http::{bodega, municipio, nucleo, oficina, persona, provincia, tramite, usuario};
use config::EnvConfig;
use config::auth::auth_m;
use config::cache::cache_m;
use config::cors::cors_m;
use config::logger::logger_m;
use sqlx::{Pool, Postgres};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(Clone)]
struct AppState {
    env_config: Arc<EnvConfig>,
    db: Pool<Postgres>,
    redis: RedisPool,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    let config = EnvConfig::new();
    let db = db::init_db(&config, config.migrate).await?;

    let redis = RedisConfig::from_url(&config.redis_url)
        .create_pool(Some(Runtime::Tokio1))
        .expect("Error creando pool de Redis");

    let shared_state = Arc::new(AppState {
        env_config: Arc::new(config),
        db,
        redis,
    });
    let state_clone = shared_state.clone();

    let router_auth_cache = Router::new()
        .route("/usuario/me", get(usuario::me_h))
        .route("/provincia", get(provincia::get_provincias_h))
        .route("/municipio", get(municipio::get_municipios_h))
        .route("/oficina", get(oficina::get_oficinas_h))
        .route("/bodega", get(bodega::get_bodegas_h))
        .route("/nucleo", get(nucleo::get_nucleos_h))
        .layer(middleware::from_fn_with_state(shared_state.clone(), auth_m))
        .layer(middleware::from_fn_with_state(
            shared_state.clone(),
            cache_m,
        ));

    let routes_auth = Router::new()
        .route("/logout", post(usuario::logout_h))
        .route("/tramite", post(tramite::crear_tramite_h))
        .layer(middleware::from_fn_with_state(shared_state.clone(), auth_m));

    let routes = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/persona", get(persona::get_personas_h))
        .route("/usuario", post(usuario::crear_usuario_h))
        .route("/login", post(usuario::login_usuario_h))
        .merge(routes_auth)
        .merge(router_auth_cache)
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
