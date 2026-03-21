use axum::{extract::Request, middleware::Next, response::Response};
use tracing::info;

pub async fn logger_m(req: Request, next: Next) -> Response {
    info!("{} {}", req.method(), req.uri());
    next.run(req).await
}
