use sqlx::{sqlite::SqlitePoolOptions, SqlitePool, FromRow};
use crate::models::user::User;

pub async fn establish_connection() -> SqlitePool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to database")
}

pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            email TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await?;
    Ok(())
}
pub async fn get_all_users(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>(
        "SELECT id, username, email FROM users"
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

