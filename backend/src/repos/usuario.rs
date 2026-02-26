use std::time::{SystemTime, UNIX_EPOCH};

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use sqlx::{Error, Pool, Postgres, prelude::FromRow, types::Uuid};
use tracing::error;

use hmac::{Hmac, Mac};
use jwt::{Error as JwtError, SignWithKey};

use serde::Serialize;
use sha2::Sha256;

#[derive(Serialize)]
pub struct UsuarioJwt<'a> {
    sub: &'a Uuid,
    email: &'a str,
    iat: u64,
    exp: u64,
}

#[derive(Serialize, FromRow)]
pub struct Usuario {
    id: Uuid,
    pass_word: String,
}

fn jwt(claim: &UsuarioJwt, secret: &str) -> Result<String, JwtError> {
    let key: Hmac<Sha256> =
        Hmac::new_from_slice(secret.as_bytes()).map_err(|_| JwtError::InvalidSignature)?;

    claim.sign_with_key(&key)
}

pub async fn crear_usuario(
    db: &Pool<Postgres>,
    email: &String,
    pass_word: &String,
    persona_id: &Uuid,
) -> Result<(), Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(pass_word.as_bytes(), &salt)
        .map_err(|e| {
            error!("Error hasheando contraseña: {}", e);
            Error::Protocol(e.to_string())
        })?
        .to_string();
    sqlx::query("insert into usuario (email, pass_word, persona_id) values ($1, $2, $3);")
        .bind(email)
        .bind(hash)
        .bind(persona_id)
        .execute(db)
        .await
        .map_err(|e| {
            error!("{}", e);
            e
        })?;

    Ok(())
}

pub async fn login_usuario(
    email: &str,
    pass_word: &str,
    db: &Pool<Postgres>,
    secret: &str,
) -> Result<(String, bool), Error> {
    let result =
        sqlx::query_as::<_, Usuario>("select id, pass_word from usuario where email = $1;")
            .bind(email)
            .fetch_one(db)
            .await
            .map_err(|e| {
                error!("{}", e);
                e
            })?;

    let parsed_hash = PasswordHash::new(&result.pass_word).map_err(|e| {
        error!("Error hasheando contraseña: {}", e.to_string());
        Error::Protocol(e.to_string())
    })?;

    if Argon2::default()
        .verify_password(pass_word.as_bytes(), &parsed_hash)
        .is_ok()
    {
        match jwt(
            &UsuarioJwt {
                sub: &result.id,
                email,
                iat: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                exp: 10,
            },
            secret,
        ) {
            Ok(s) => Ok((s, true)),
            Err(e) => {
                error!("{}", e);
                Err(Error::Protocol(e.to_string()))
            }
        }
    } else {
        Ok((String::default(), false))
    }
}
