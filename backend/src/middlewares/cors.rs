use std::sync::Arc;

use crate::AppState;
use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderValue, Method, StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use tracing::debug;

pub async fn cors_m(State(estado): State<Arc<AppState>>, req: Request, next: Next) -> Response {
    debug!("CORS Middleware: Processing request for {}", req.uri());

    debug!("SPA URL env {}", estado.env_config.spa_url);

    if req.method() == Method::OPTIONS {
        let option_res = Response::builder()
            .status(StatusCode::NO_CONTENT)
            .header(
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                &estado.env_config.spa_url,
            )
            .header(
                header::ACCESS_CONTROL_ALLOW_METHODS,
                "GET, POST, PUT, DELETE, OPTIONS",
            )
            .header(
                header::ACCESS_CONTROL_ALLOW_HEADERS,
                "content-type, authorization",
            )
            .header(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
            .body(Body::empty());
        if let Ok(res) = option_res {
            return res;
        } else {
            return (StatusCode::NO_CONTENT).into_response();
        }
    }
    let mut response = next.run(req).await;

    let headers = response.headers_mut();
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_str(&estado.env_config.spa_url).expect("Invalid Header Value"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("content-type, authorization"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("true"),
    );

    response
}
