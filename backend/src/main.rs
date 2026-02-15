use std::sync::Arc;

use axum::{
    Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::post,
};

mod config;
mod db;
mod http;
mod repos;

use config::EnvConfig;
use sqlx::{Pool, Postgres};
use tracing::warn;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::http::usuario;

#[derive(Clone)]
struct AppState {
    env_config: Arc<EnvConfig>,
    db: Pool<Postgres>,
}

async fn middle(req: Request, next: Next) -> Response {
    warn!("Request a: {} {}", req.method(), req.uri());
    next.run(req).await
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let config = EnvConfig::new();
    let db = db::init_db(&config, false).await?;

    let env_config = Arc::new(config);

    let shared_state = Arc::new(AppState { env_config, db });

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    let routes = Router::new()
        .route("/usuario", post(usuario::crear_usuario_h))
        .layer(middleware::from_fn(middle))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6006").await.unwrap();
    axum::serve(listener, routes).await.unwrap();
    Ok(())
}
