mod api;
mod db;
mod models;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use axum::Server; // Works for axum 0.6

// Basic root handler
async fn root() -> &'static str {
    "Hello from GymBuddy API!"
}

#[tokio::main]
async fn main() {

    //Database connection
    let _pool = db::connection::establish_connection().await;


    println!("Database connected successfully!");

    // Create router with root route and merge API routes
    let app = Router::new()
        .route("/", get(root))
        .merge(api::create_api_router());

    // Localhost
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("GymBuddy API running at http://{}", addr);

    // Start Axum 0.6 server
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
