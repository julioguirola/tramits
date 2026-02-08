mod db;
mod models;
mod http;
mod config;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    // let pool = db::init_db(false).await?;

    // let routes = Router::new()
    //     .route("/hello", post(|| async { "Hello" }));

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:6006").await.unwrap();
    // axum::serve(listener, routes).await.unwrap();
    Ok(())
}