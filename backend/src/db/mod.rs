use crate::config::EnvConfig;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn init_db(config: &EnvConfig) -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_user, config.db_password, config.db_host, config.db_port, config.db_name,
    );

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
