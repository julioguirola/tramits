use axum::{
    Router, routing::{post}
};

mod db;
mod models;

use models::usuario::UsuarioService;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    let pool = db::init_db(false).await?;
    let user_rep = UsuarioService{db_pool: pool};

    let crear_usuario = async || {
        "Hello world"
    };
    
    let routes = Router::new()
        .route("/usuario", post(crear_usuario));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6006").await.unwrap();
    axum::serve(listener, routes).await.unwrap();
    Ok(())
}