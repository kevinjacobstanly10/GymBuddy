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
    dotenvy::dotenv().ok(); // Load .env variables (like DATABASE_URL)
    
    let pool = establish_connection().await;

    //println!("DATABASE_URL = {:?}", std::env::var("DATABASE_URL"));

    // Quick debug: check users table
    //let rows: Result<Vec<(i64, String, String)>, sqlx::Error> = 
    //sqlx::query_as("SELECT id, username, email FROM users")
    //    .fetch_all(&pool)
    //    .await;

    //println!("Rows fetched: {:?}", rows);

    let app = api::routes::create_api_router().with_state(pool.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("GymBuddy API running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
