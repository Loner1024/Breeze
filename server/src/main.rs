use server::services;
use tower_http::cors::{CorsLayer, Any};

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any);
    // build our application with a single route
    let app = Router::new()
        .route("/list", get(services::list_post))
        .layer(cors);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}