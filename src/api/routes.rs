use axum::{routing::get, Json, Router};
use serde::Serialize;
use crate::models::user::User;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
}

async fn list_users() -> Json<Vec<User>> {
    let users = vec![
        User { id: 1, username: "Kevin".to_string() },
        User { id: 2, username: "Jacob".to_string() },
    ];
    Json(users)
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "GymBuddy API is healthy 💪!".to_string(),
    })
}

pub fn create_api_router() -> Router {
    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/users", get(list_users))
}