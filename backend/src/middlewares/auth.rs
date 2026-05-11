use crate::{
    AppState,
    repos::usuario::UsuarioJwt,
    tipos::{Respuesta, Ress},
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
use sqlx::Row;
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::{error, warn};

fn unauthorized() -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!(Ress::<u8> {
            message: Respuesta::Info.as_str(),
            description: "Usuario no autenticado",
            data: None,
        })),
    )
        .into_response()
}

pub async fn auth_m(State(estado): State<Arc<AppState>>, mut req: Request, next: Next) -> Response {
    let Some(cookie_header) = req.headers().get(COOKIE) else {
        return unauthorized();
    };

    let Ok(cookie_str) = cookie_header.to_str() else {
        return unauthorized();
    };

    let Some(token) = cookie_str.split(';').find_map(|part| {
        let part = part.trim();
        part.strip_prefix("jwt=")
    }) else {
        return unauthorized();
    };

    let key: Result<Hmac<Sha256>, _> =
        Hmac::new_from_slice(estado.env_config.jwt_secret.as_bytes());

    match key {
        Ok(k) => {
            let claim: Result<UsuarioJwt, _> = token.verify_with_key(&k);

            if claim.is_err() {
                return unauthorized();
            }
            let Ok(usr) = claim else {
                return unauthorized();
            };
            let Ok(t) = SystemTime::now().duration_since(UNIX_EPOCH) else {
                return unauthorized();
            };

            if t.as_secs() > usr.exp {
                warn!("Token expirado para: {}", &usr.email);
                return unauthorized();
            }

            let active = sqlx::query("select activo from usuario where id = $1")
                .bind(usr.sub)
                .fetch_optional(&estado.db)
                .await;
            let Ok(Some(row)) = active else {
                return unauthorized();
            };
            let activo: bool = row.get("activo");
            if !activo {
                return unauthorized();
            }

            req.extensions_mut().insert(usr);
        }
        Err(e) => {
            error!("{}", e);
            return unauthorized();
        }
    }

    next.run(req).await
}
