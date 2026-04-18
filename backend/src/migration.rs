mod config;
mod db;
use tracing::info;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    info!("Iniciando proceso de migracion");
    let config = config::EnvConfig::new();
    let migration_query = include_str!("db/migration.sql");
    info!("Conectando a la base de datos");
    let pool = db::init_db(&config).await.map_err(|e| {
        tracing::error!("Error conectando a la base de datos en migracion: {}", e);
        e
    })?;
    info!("Ejecutando migration.sql");
    sqlx::raw_sql(migration_query)
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Error ejecutando migration.sql: {}", e);
            e
        })?;
    info!("Migracion completada");
    Ok(())
}
