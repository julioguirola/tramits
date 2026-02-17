use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use sqlx::{Error, Pool, Postgres, types::Uuid};
use tracing::error;
pub async fn crear_usuario(
    db: Pool<Postgres>,
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
        .execute(&db)
        .await
        .map_err(|e| {
            error!("{}", e);
            e
        })?;

    Ok(())
}
