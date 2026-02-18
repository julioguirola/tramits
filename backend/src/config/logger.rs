use axum::{extract::Request, middleware::Next, response::Response};
use tracing::warn;

pub async fn logger_m(req: Request, next: Next) -> Response {
    warn!("{} {}", req.method(), req.uri());
    next.run(req).await
}
