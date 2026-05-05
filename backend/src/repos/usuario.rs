use std::time::{SystemTime, UNIX_EPOCH};

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use sqlx::{Arguments, Error, Pool, Postgres, prelude::FromRow, types::Uuid};
use tracing::error;

use hmac::{Hmac, Mac};
use jwt::{Error as JwtError, SignWithKey};

use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsuarioJwt {
    pub sub: Uuid,
    pub email: String,
    pub rol: String,
    pub iat: u64,
    pub exp: u64,
}

impl UsuarioJwt {
    pub fn new(sub: Uuid, email: String, rol: String) -> Option<UsuarioJwt> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
        Some(UsuarioJwt {
            sub,
            email,
            rol,
            iat: now,
            exp: now + 24 * 60 * 60,
        })
    }
}

#[derive(Serialize, FromRow)]
pub struct Usuario {
    id: Uuid,
    pass_word: String,
    rol: String,
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

#[derive(Serialize, FromRow, Debug)]
pub struct UsuarioListado {
    pub id: Uuid,
    pub email: String,
    pub nombre: String,
    pub apellido: String,
    pub rol: String,
    pub oficina: Option<String>,
    pub municipio: String,
    pub provincia: String,
}

pub async fn get_usuario_actual(
    db: &Pool<Postgres>,
    usr: &UsuarioJwt,
) -> Result<UsuarioInfo, Error> {
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
         where u.id = $1;",
    )
    .bind(usr.sub)
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

pub async fn get_rol_usuario(db: &Pool<Postgres>, user_id: &Uuid) -> Result<String, Error> {
    sqlx::query_scalar(
        "select ur.nombre from usuario u join usuario_rol ur on ur.id = u.rol_id where u.id = $1;",
    )
    .bind(user_id)
    .fetch_one(db)
    .await
}

pub async fn login_usuario(
    email: &str,
    pass_word: &str,
    db: &Pool<Postgres>,
    secret: &str,
) -> Result<(String, bool), Error> {
    let result = sqlx::query_as::<_, Usuario>(
        "select u.id, u.pass_word, ur.nombre as rol 
         from usuario u 
         join usuario_rol ur on ur.id = u.rol_id 
         where u.email = $1;",
    )
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
        if let Some(user_jwt) = UsuarioJwt::new(result.id, String::from(email), result.rol) {
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

pub async fn listar_usuarios(
    db: &Pool<Postgres>,
    limit: usize,
    offset: usize,
    provincia_id: Option<i32>,
    municipio_id: Option<i32>,
    oficina_id: Option<i32>,
) -> Result<Vec<UsuarioListado>, Error> {
    let mut query = String::from(
        "select u.id, u.email, p.nombre, p.apellido, ur.nombre as rol,
                case when u.oficina_id is not null then o_reg.nombre else o_pers.nombre end as oficina,
                case when u.oficina_id is not null then m_reg.nombre else m_pers.nombre end as municipio,
                case when u.oficina_id is not null then pr_reg.nombre else pr_pers.nombre end as provincia
         from usuario u
         join persona p on p.id = u.persona_id
         join usuario_rol ur on ur.id = u.rol_id
         left join oficina o_reg on o_reg.id = u.oficina_id
         left join municipio m_reg on m_reg.id = o_reg.municipio_id
         left join provincia pr_reg on pr_reg.id = m_reg.provincia_id
         left join nucleo n on n.id = p.nucleo_id
         left join bodega b on b.id = n.bodega_id
         left join oficina o_pers on o_pers.id = b.oficina_id
         left join municipio m_pers on m_pers.id = o_pers.municipio_id
         left join provincia pr_pers on pr_pers.id = m_pers.provincia_id
         where 1=1",
    );

    let mut args = sqlx::postgres::PgArguments::default();
    let mut param_count = 1;

    if let Some(provincia) = provincia_id {
        query.push_str(&format!(
            " and ((u.oficina_id is not null and pr_reg.id = ${}) or (u.oficina_id is null and pr_pers.id = ${}))",
            param_count, param_count + 1
        ));
        let _ = args.add(provincia);
        let _ = args.add(provincia);
        param_count += 2;
    }
    if let Some(municipio) = municipio_id {
        query.push_str(&format!(
            " and ((u.oficina_id is not null and m_reg.id = ${}) or (u.oficina_id is null and m_pers.id = ${}))",
            param_count, param_count + 1
        ));
        let _ = args.add(municipio);
        let _ = args.add(municipio);
        param_count += 2;
    }
    if let Some(oficina) = oficina_id {
        query.push_str(&format!(
            " and ((u.oficina_id is not null and o_reg.id = ${}) or (u.oficina_id is null and o_pers.id = ${}))",
            param_count, param_count + 1
        ));
        let _ = args.add(oficina);
        let _ = args.add(oficina);

        param_count += 2;
    }

    query.push_str(&format!(
        " order by p.apellido, p.nombre limit ${} offset ${};",
        param_count,
        param_count + 1
    ));
    let _ = args.add(limit as i64);
    let _ = args.add(offset as i64);

    sqlx::query_as_with::<_, UsuarioListado, _>(&query, args)
        .fetch_all(db)
        .await
}

pub async fn contar_usuarios(
    db: &Pool<Postgres>,
    provincia_id: Option<i32>,
    municipio_id: Option<i32>,
    oficina_id: Option<i32>,
) -> Result<i64, Error> {
    let mut query = String::from(
        "select count(*)
         from usuario u
         join persona p on p.id = u.persona_id
         join usuario_rol ur on ur.id = u.rol_id
         left join oficina o_reg on o_reg.id = u.oficina_id
         left join municipio m_reg on m_reg.id = o_reg.municipio_id
         left join provincia pr_reg on pr_reg.id = m_reg.provincia_id
         left join nucleo n on n.id = p.nucleo_id
         left join bodega b on b.id = n.bodega_id
         left join oficina o_pers on o_pers.id = b.oficina_id
         left join municipio m_pers on m_pers.id = o_pers.municipio_id
         left join provincia pr_pers on pr_pers.id = m_pers.provincia_id
         where 1=1",
    );
    let mut args = sqlx::postgres::PgArguments::default();
    let mut param_count = 1;

    if let Some(provincia) = provincia_id {
        query.push_str(&format!(
            " and ((u.oficina_id is not null and pr_reg.id = ${}) or (u.oficina_id is null and pr_pers.id = ${}))",
            param_count, param_count + 1
        ));
        let _ = args.add(provincia);
        let _ = args.add(provincia);
        param_count += 2;
    }
    if let Some(municipio) = municipio_id {
        query.push_str(&format!(
            " and ((u.oficina_id is not null and m_reg.id = ${}) or (u.oficina_id is null and m_pers.id = ${}))",
            param_count, param_count + 1
        ));
        let _ = args.add(municipio);
        let _ = args.add(municipio);
        param_count += 2;
    }
    if let Some(oficina) = oficina_id {
        query.push_str(&format!(
            " and ((u.oficina_id is not null and o_reg.id = ${}) or (u.oficina_id is null and o_pers.id = ${}))",
            param_count, param_count + 1
        ));
        let _ = args.add(oficina);
        let _ = args.add(oficina);
    }

    query.push_str(";");
    let count: (i64,) = sqlx::query_as_with::<_, (i64,), _>(&query, args)
        .fetch_one(db)
        .await?;
    Ok(count.0)
}
