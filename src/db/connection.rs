use sqlx::SqlitePool;
use dotenvy::dotenv;
use std::env;

pub async fn establish_connection() -> SqlitePool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to database")
}
