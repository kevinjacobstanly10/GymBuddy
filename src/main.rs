mod api;
mod db;
mod models;

use axum::Router;
use std::net::SocketAddr;
use db::connection::establish_connection;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

// Basic root handler
async fn root() -> &'static str {
    "Hello from GymBuddy API!"
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = establish_connection().await;

    // Initialize database tables
    db::connection::init_db(&pool)
        .await
        .expect("Failed to initialize database tables");

    let app = api::routes::create_api_router().with_state(pool.clone());

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("GymBuddy API running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
