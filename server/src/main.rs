use server::services;
use tower_http::cors::{CorsLayer, Any};

use axum::{
    routing::get,
    Router,
};
use sea_orm::{Database, DbErr};

#[tokio::main]
async fn main() {
    connect_db().await.expect("connect db error");


    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any);
    // build our application with a single route
    let app = Router::new()
        .route("/list", get(services::list_post))
        .route("/post/:id", get(services::post))
        .layer(cors);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn connect_db() -> Result<(), DbErr> {
    const DATABASE_URL: &str = "postgres://postgres:123456@localhost:5432/blog";
    let db = Database::connect(DATABASE_URL).await?;

    Ok(())
}