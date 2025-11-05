use axum::{
    extract::State,
    routing::get,
    Json,            
    Router,
};
use serde::Serialize;
use sqlx::SqlitePool;
use crate::db::connection::get_all_users;
use crate::models::user::User;


#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
}

pub async fn list_users(State(pool): State<SqlitePool>) -> Json<Vec<User>> {
    match get_all_users(&pool).await {
        Ok(users) => {
            println!("Users fetched: {:?}", users);
            Json(users)
        },
        Err(e) => {
            eprintln!("Error fetching users: {:?}", e);
            Json(vec![])
        }
    }
}

pub fn create_api_router() -> Router<SqlitePool> {
    Router::new()
        .route("/api/users", get(list_users))
        .route("/health", get(health_check))
}

// Route handler: health check
async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "GymBuddy API is healthy 💪!".to_string(),
    })
}
