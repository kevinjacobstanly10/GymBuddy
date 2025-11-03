use axum::{routing::get, Json, Router};
use serde::Serialize;
use crate::models::user::User;
use crate::db::connection::establish_connection;
use sqlx::query_as;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
}

async fn list_users() -> Json<Vec<User>> {
    // Connect to the database
    let pool = establish_connection().await;

    // Query all users from the "users" table
    let users = query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .unwrap_or_default(); // Return empty Vec if error occurs

    Json(users)
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "GymBuddy API is healthy 💪!".to_string(),
    })
}

pub fn create_api_router() -> Router {
    Router::new().route("/api/users", get(list_users))
}
