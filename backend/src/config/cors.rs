use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderValue, Method, StatusCode, header},
    middleware::Next,
    response::Response,
};

use crate::AppState;

pub async fn cors_m(State(estado): State<Arc<AppState>>, req: Request, next: Next) -> Response {
    if req.method() == Method::OPTIONS {
        return Response::builder()
            .status(StatusCode::NO_CONTENT)
            .header(
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                &estado.env_config.spa_url,
            )
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
        HeaderValue::from_str(&estado.env_config.spa_url).expect("Invalid Header Value"),
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
