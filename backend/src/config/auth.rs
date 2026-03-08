use crate::{
    AppState,
    config::tipos::{Respuesta, Ress},
    repos::usuario::UsuarioJwt,
};
use axum::{
    Json,
    extract::{Request, State},
    http::{StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::{IntoResponse, Response},
};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use serde_json::json;
use sha2::Sha256;
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

pub async fn auth_m(State(estado): State<Arc<AppState>>, req: Request, next: Next) -> Response {
    let Some(authorization) = req.headers().get(AUTHORIZATION) else {
        return unauthorized();
    };

    let Ok(authorization_str) = authorization.to_str() else {
        return unauthorized();
    };

    let Some((_, token)) = authorization_str.split_once(' ') else {
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

            if let Ok(usr) = claim
                && let Ok(t) = SystemTime::now().duration_since(UNIX_EPOCH)
                && t.as_secs() > usr.exp
            {
                warn!("Token expirado para: {}", &usr.email);
                return unauthorized();
            }
        }
        Err(e) => {
            error!("{}", e);
            return unauthorized();
        }
    }

    next.run(req).await
}
