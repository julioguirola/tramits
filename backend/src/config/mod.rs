use std::env;

pub struct EnvConfig {
    pub db_host: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub db_port: String,
    pub port: String,
    pub spa_url: String,
    pub environment: String,
    pub jwt_secret: String,
    pub admin_email: String,
    pub admin_password: String,
    pub consumer_email: String,
    pub consumer_password: String,
    pub registrar_email: String,
    pub registrar_password: String,
    pub redis_url: String,
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
        let jwt_secret =
            env::var("JWT_SECRET").unwrap_or_else(|_| panic!("JWT_SECRET must be set"));
        let environment =
            env::var("ENVIRONMENT").unwrap_or_else(|_| panic!("ENVIRONMENT must be set"));
        let admin_email =
            env::var("ADMIN_EMAIL").unwrap_or_else(|_| panic!("ADMIN_EMAIL must be set"));
        let admin_password =
            env::var("ADMIN_PASSWORD").unwrap_or_else(|_| panic!("ADMIN_PASSWORD must be set"));
        let consumer_email =
            env::var("CONSUMER_EMAIL").unwrap_or_else(|_| panic!("CONSUMER_EMAIL must be set"));
        let consumer_password = env::var("CONSUMER_PASSWORD")
            .unwrap_or_else(|_| panic!("CONSUMER_PASSWORD must be set"));
        let registrar_email =
            env::var("REGISTRAR_EMAIL").unwrap_or_else(|_| panic!("REGISTRAR_EMAIL must be set"));
        let registrar_password = env::var("REGISTRAR_PASSWORD")
            .unwrap_or_else(|_| panic!("REGISTRAR_PASSWORD must be set"));
        let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| panic!("REDIS_URL must be set"));

        EnvConfig {
            db_host,
            db_user,
            db_password,
            db_name,
            db_port,
            port,
            spa_url,
            environment,
            jwt_secret,
            admin_email,
            admin_password,
            consumer_email,
            consumer_password,
            registrar_email,
            registrar_password,
            redis_url,
        }
    }
}
