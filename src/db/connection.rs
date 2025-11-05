use sqlx::{sqlite::SqlitePoolOptions, SqlitePool, FromRow};
use crate::models::user::{User, NewUser};

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

pub async fn create_user_db(pool: &SqlitePool, new_user: &NewUser) -> Result<User, sqlx::Error> {
    // Step 1: Insert the user
    let result = sqlx::query(
        "INSERT INTO users (username, email) VALUES (?, ?)"
    )
    .bind(&new_user.username)
    .bind(&new_user.email)
    .execute(pool)
    .await?;

    // Step 2: Get the last inserted row ID
    let last_id = result.last_insert_rowid();

    // Step 3: Fetch the user row
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, email FROM users WHERE id = ?"
    )
    .bind(last_id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn delete_user_db(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}


