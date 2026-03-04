use crate::{
    AppState,
    config::tipos::{Respuesta, Ress},
};
use axum::{
    Json,
    extract::{Request, State},
    http::{StatusCode, header::COOKIE},
    middleware::Next,
    response::{IntoResponse, Response},
};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use serde_json::json;
use sha2::Sha256;
use std::{collections::HashMap, sync::Arc};

fn forbidden() -> Response {
    (
        StatusCode::FORBIDDEN,
        Json(json!(Ress::<u8> {
            message: Respuesta::Info,
            description: "Usuario no autenticado",
            data: None,
        })),
    )
        .into_response()
}

pub async fn auth_m(State(estado): State<Arc<AppState>>, req: Request, next: Next) -> Response {
    let Some(cookie) = req.headers().get(COOKIE) else {
        return forbidden();
    };

    let Ok(cookie_str) = cookie.to_str() else {
        return forbidden();
    };

    let Some((_, token)) = cookie_str.split_once('=') else {
        return forbidden();
    };

    let key: Hmac<Sha256> = Hmac::new_from_slice(estado.env_config.jwt_secret.as_bytes()).unwrap();

    let claim: Result<HashMap<String, String>, _> = token.verify_with_key(&key);

    if claim.is_err() {
        return forbidden();
    }

    next.run(req).await
}
