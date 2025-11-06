use axum::{
    extract::{Path, State},
    routing::{get, post, delete,put},
    Json, Router,
};
use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;
use crate::models::user::User;
use crate::db::connection::{get_all_users,create_user_db, update_user_db, delete_user_db};
use crate::models::user::{NewUser};
use crate::db::connection::{get_all_workouts, get_all_exercises};
use crate::models::{workout::Workout, exercise::Exercise};
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

pub async fn update_user(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
    Json(updated_user): Json<NewUser>,
) -> Json<User> {
    let id = id.trim().parse::<i64>().expect("Invalid ID format");

    let user = update_user_db(&pool, id, &updated_user)
        .await
        .expect("Failed to update user");
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

// GET all workouts
pub async fn list_workouts(State(pool): State<SqlitePool>) -> Json<Vec<Workout>> {
    let workouts = get_all_workouts(&pool).await.unwrap_or_default();
    Json(workouts)
}

// GET all exercises
pub async fn list_exercises(State(pool): State<SqlitePool>) -> Json<Vec<Exercise>> {
    let exercises = get_all_exercises(&pool).await.unwrap_or_default();
    Json(exercises)
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
        .route("/api/workouts", get(list_workouts))
        .route("/api/exercises", get(list_exercises))
        .route("/health", get(health_check))
}