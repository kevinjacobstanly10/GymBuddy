use axum::{
    extract::{Path, State},
    routing::{get, post, delete},
    Json, Router,
};
use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;
use crate::models::user::User;
use crate::db::connection::{get_all_users,create_user_db, delete_user_db};
use crate::models::user::{NewUser};
use sqlx::query_as;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
}

// GET: list all users
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

// Add new user
pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(new_user): Json<NewUser>,
) -> Json<User> {
    let user = create_user_db(&pool, &new_user)
        .await
        .expect("Failed to insert user");

    Json(user)
}

pub async fn delete_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Json<String> {
    match delete_user_db(&pool, id).await {
        Ok(_) => Json(format!("User with id {} deleted", id)),
        Err(e) => Json(format!("Failed to delete user: {:?}", e)),
    }
}
// Health check
async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "GymBuddy API is healthy 💪!".to_string(),
    })
}

// Router setup
pub fn create_api_router() -> Router<SqlitePool> {
    Router::new()
        .route("/api/users", get(list_users).post(create_user))
        .route("/api/users/:id", delete(delete_user)) 
        .route("/health", get(health_check))
}
