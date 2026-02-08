use sqlx::{Pool, Postgres};

pub struct UsuarioService<'a> {
    db_pool: &'a Pool<Postgres>
}

impl UsuarioService<'_> {
    pub fn new (db_pool: &Pool<Postgres>) -> UsuarioService<'_> {
        UsuarioService { db_pool }
    }
}

