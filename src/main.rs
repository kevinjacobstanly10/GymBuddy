mod api;
mod db;
mod models;
pub mod auth;
mod jwt;
mod middleware;
use axum::Router;
use db::connection::establish_connection;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting GymBuddy API");

    let pool = establish_connection().await;

    db::connection::init_db(&pool)
        .await
        .expect("DB init failed");

    let app: Router = api::routes::create_api_router()
        .with_state(pool);

    let addr = ([127, 0, 0, 1], 3000).into();
    tracing::info!("GymBuddy API running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}