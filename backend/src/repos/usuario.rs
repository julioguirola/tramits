use std::time::{SystemTime, UNIX_EPOCH};

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use sqlx::{Error, Pool, Postgres, prelude::FromRow, types::Uuid};
use tracing::error;

use hmac::{Hmac, Mac};
use jwt::{Error as JwtError, SignWithKey, VerifyWithKey};

use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Serialize, Deserialize, Debug)]
pub struct UsuarioJwt {
    pub sub: Uuid,
    pub email: String,
    pub iat: u64,
    pub exp: u64,
}

impl UsuarioJwt {
    pub fn new(sub: Uuid, email: String) -> Option<UsuarioJwt> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
        Some(UsuarioJwt {
            sub,
            email,
            iat: now,
            exp: now + 24 * 60 * 60,
        })
    }
}

#[derive(Serialize, FromRow)]
pub struct Usuario {
    id: Uuid,
    pass_word: String,
}

pub fn jwt(claim: &UsuarioJwt, secret: &str) -> Result<String, JwtError> {
    let key: Hmac<Sha256> =
        Hmac::new_from_slice(secret.as_bytes()).map_err(|_| JwtError::InvalidSignature)?;

    claim.sign_with_key(&key)
}

#[derive(Serialize, FromRow)]
pub struct UsuarioInfo {
    pub email: String,
    pub nombre: String,
    pub apellido: String,
    pub rol: String,
    pub nucleo: Option<String>,
    pub numero_nucleo: Option<i32>,
    pub bodega: Option<String>,
    pub oficina: Option<String>,
    pub municipio: String,
    pub provincia: String,
}

pub fn claims_from_cookie(cookie_header: &str, secret: &str) -> Option<UsuarioJwt> {
    let token = cookie_header
        .split(';')
        .find_map(|part| part.trim().strip_prefix("jwt="))?;

    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).ok()?;
    token.verify_with_key(&key).ok()
}

pub async fn get_usuario_actual(
    db: &Pool<Postgres>,
    secret: &str,
    cookie_header: &str,
) -> Result<UsuarioInfo, Error> {
    let claims = claims_from_cookie(cookie_header, secret)
        .ok_or_else(|| Error::Protocol("Token inválido".into()))?;

    sqlx::query_as::<_, UsuarioInfo>(
        "select u.email, p.nombre, p.apellido, t.nombre as rol,
                n.direccion as nucleo, b.nombre as bodega, o.nombre as oficina,
                m.nombre as municipio, pr.nombre as provincia, n.id as numero_nucleo
         from usuario u
         join persona p on p.id = u.persona_id
         join usuario_rol t on t.id = u.rol_id
         join municipio m on m.id = p.municipio_id
         join provincia pr on pr.id = m.provincia_id
         left join nucleo n on n.id = p.nucleo_id
         left join bodega b on b.id = n.bodega_id
         left join oficina o on o.id = b.oficina_id
         where u.email = $1;",
    )
    .bind(&claims.email)
    .fetch_one(db)
    .await
}

pub async fn crear_usuario(
    db: &Pool<Postgres>,
    email: &String,
    pass_word: &String,
    persona_id: &Uuid,
) -> Result<Uuid, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(pass_word.as_bytes(), &salt)
        .map_err(|e| {
            error!("Error hasheando contraseña: {}", e);
            Error::Protocol(e.to_string())
        })?
        .to_string();
    let user_id: Uuid = sqlx::query_scalar(
        "insert into usuario (email, pass_word, persona_id) values ($1, $2, $3) returning id;",
    )
    .bind(email)
    .bind(hash)
    .bind(persona_id)
    .fetch_one(db)
    .await
    .map_err(|e| {
        error!("{}", e);
        e
    })?;

    Ok(user_id)
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
        if let Some(user_jwt) = UsuarioJwt::new(result.id, String::from(email)) {
            match jwt(&user_jwt, secret) {
                Ok(s) => Ok((s, true)),
                Err(e) => {
                    error!("{}", e);
                    Err(Error::Protocol(e.to_string()))
                }
            }
        } else {
            Err(Error::Protocol(String::from(
                "Error instanciando UsuarioJwt",
            )))
        }
    } else {
        Ok((String::default(), false))
    }
}
