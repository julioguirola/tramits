use axum::{
    Router,
    http::StatusCode,
    middleware::{self},
    routing::{any, get, post},
};
use std::sync::Arc;
mod config;
mod db;
mod http;
mod repos;
use crate::http::usuario;
use config::EnvConfig;
use config::cors::cors_m;
use config::logger::logger_m;
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
    let db = db::init_db(&config, false).await?;

    let shared_state = Arc::new(AppState {
        env_config: Arc::new(config),
        db,
    });
    let state_port = shared_state.clone();

    let routes = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/usuario", post(usuario::crear_usuario_h))
        .route("/{*path}", any(|| async { StatusCode::NO_CONTENT }))
        .layer(middleware::from_fn(logger_m))
        .layer(middleware::from_fn_with_state(shared_state.clone(), cors_m))
        .with_state(shared_state);

    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &state_port.env_config.port))
            .await
            .unwrap();
    axum::serve(listener, routes).await.unwrap();
    Ok(())
}
