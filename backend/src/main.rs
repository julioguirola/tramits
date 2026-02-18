use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    extract::Request,
    http::{HeaderValue, Method, StatusCode, header},
    middleware::{self, Next},
    response::Response,
    routing::{any, get, post},
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

async fn logger_m(req: Request, next: Next) -> Response {
    warn!("{} {}", req.method(), req.uri());
    next.run(req).await
}

async fn cors_m(req: Request, next: Next) -> Response {
    if req.method() == Method::OPTIONS {
        return Response::builder()
            .status(StatusCode::NO_CONTENT)
            .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .header(
                header::ACCESS_CONTROL_ALLOW_METHODS,
                "GET, POST, PUT, DELETE, OPTIONS",
            )
            .header(header::ACCESS_CONTROL_ALLOW_HEADERS, "*")
            .body(Body::empty())
            .unwrap();
    }
    let mut response = next.run(req).await;

    let headers = response.headers_mut();
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("*"),
    );

    response
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
        .layer(middleware::from_fn(cors_m))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", state_port.env_config.port))
        .await
        .unwrap();
    axum::serve(listener, routes).await.unwrap();
    Ok(())
}
