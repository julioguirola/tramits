use sqlx::{Error, Pool, Postgres};

pub async fn crear_usuario(
    db: Pool<Postgres>,
    email: String,
    pass_word: String,
    persona_id: String,
) -> Result<(), Error> {
    sqlx::query("insert into usuario (email, pass_word, persona_id) values ($1, $2, $3);")
        .bind(email)
        .bind(pass_word)
        .bind(persona_id)
        .execute(&db)
        .await?;
    Ok(())
}
