use sqlx::{Pool, Postgres};

struct Usuario {
    id: Option<String>,
    email: Option<String>,
    pass_word: Option<String>,
    persona_id: Option<String>,
}

pub struct UsuarioService {
    pub db_pool: Pool<Postgres>
}

impl UsuarioService {
    pub async fn crear_usuario (&self) -> Result<(), sqlx::Error> {
        sqlx::query("Select 1;").execute(&self.db_pool).await?;
        Ok(())
    }
}

