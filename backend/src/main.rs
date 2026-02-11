use axum::{Router, routing::get};

mod db;
mod config;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    let config = config::EnvConfig::new();
    let db = db::DataBase::new(&config, false).await?;

    let routes = Router::new()
        .route("/", get(|| async {"Hola "}));


    let listener = tokio::net::TcpListener::bind("0.0.0.0:6006").await.unwrap();
    axum::serve(listener, routes).await.unwrap();
    Ok(())
}