use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "GymBuddy API is healthy 💪!".to_string(),
    })
}

pub fn create_api_router() -> Router {
    Router::new().route("/api/health", get(health_check))
}
