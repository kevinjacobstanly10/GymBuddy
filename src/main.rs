mod api;
mod db;
mod models;

use axum::{routing::get, Router};
use std::net::SocketAddr;

// Simple route handler
async fn root() -> &'static str {
    "Hello from GymBuddy API!"
}

#[tokio::main]
async fn main() {
    // Create the app and define routes
    let app = Router::new().route("/", get(root));

    // Define where the server listens
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("GymBuddy API running at http://{}", addr);

    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
