mod config;
mod db;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let config = config::EnvConfig::new();
    let migration_query = include_str!("db/migration.sql");
    let pool = db::init_db(&config).await?;
    sqlx::raw_sql(migration_query).execute(&pool).await?;
    Ok(())
}
