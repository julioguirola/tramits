use crate::{AppState, repos::usuario::claims_from_cookie};
use axum::{
    body::Body,
    extract::{Request, State},
    http::{StatusCode, header::COOKIE},
    middleware::Next,
    response::{IntoResponse, Response},
};
use deadpool_redis::redis::AsyncCommands;
use std::sync::Arc;
use tracing::{error, warn};

const CACHE_TTL: u64 = 3600; // 5 minutos

pub async fn cache_m(State(state): State<Arc<AppState>>, req: Request, next: Next) -> Response {
    let cookie_header = req
        .headers()
        .get(COOKIE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let Some(claims) = claims_from_cookie(&cookie_header, &state.env_config.jwt_secret) else {
        return next.run(req).await;
    };

    let cache_key = format!("usuario_me:{}", claims.email);

    let mut conn: deadpool_redis::Connection = match state.redis.get().await {
        Ok(c) => c,
        Err(e) => {
            warn!("Redis no disponible, saltando cache: {}", e);
            return next.run(req).await;
        }
    };

    if let Ok(cached) = conn.get::<_, String>(&cache_key).await {
        return (
            StatusCode::OK,
            [("content-type", "application/json")],
            Body::from(cached),
        )
            .into_response();
    }
    let res = next.run(req).await;
    if res.status() == StatusCode::OK {
        let (parts, body) = res.into_parts();
        let bytes = match axum::body::to_bytes(body, usize::MAX).await {
            Ok(b) => b,
            Err(e) => {
                error!("Error leyendo body para cache: {}", e);
                return Response::from_parts(parts, Body::empty());
            }
        };
        if let Ok(body_str) = std::str::from_utf8(&bytes)
            && let Err(e) = conn
                .set_ex::<_, _, ()>(&cache_key, body_str, CACHE_TTL)
                .await
        {
            error!("Error guardando en cache: {}", e);
        }
        return Response::from_parts(parts, Body::from(bytes));
    }
    res
}
