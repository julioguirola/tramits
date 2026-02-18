use std::env;
pub mod cors;
pub mod logger;
pub struct EnvConfig {
    pub db_host: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub db_port: String,
    pub port: String,
    pub spa_url: String,
    pub environment: String,
}

impl EnvConfig {
    pub fn new() -> EnvConfig {
        let db_host = env::var("DB_HOST").unwrap_or_else(|_| panic!("DB_HOST must be set"));
        let db_user = env::var("DB_USER").unwrap_or_else(|_| panic!("DB_USER must be set"));
        let db_password =
            env::var("DB_PASSWORD").unwrap_or_else(|_| panic!("DB_PASSWORD must be set"));
        let db_name = env::var("DB_NAME").unwrap_or_else(|_| panic!("DB_NAME must be set"));
        let db_port = env::var("DB_PORT").unwrap_or_else(|_| panic!("DB_PORT must be set"));
        let port = env::var("PORT").unwrap_or_else(|_| panic!("PORT must be set"));
        let spa_url = env::var("SPA_URL").unwrap_or_else(|_| panic!("SPA_URL must be set"));
        let environment =
            env::var("ENVIRONMENT").unwrap_or_else(|_| panic!("ENVIRONMENT must be set"));

        EnvConfig {
            db_host,
            db_user,
            db_password,
            db_name,
            db_port,
            port,
            spa_url,
            environment,
        }
    }
}
