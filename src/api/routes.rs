use axum::{
    extract::{Path, State},
    routing::{get, post, delete, put},
    Json, Router,
};
use serde::Serialize;
use sqlx::SqlitePool;

use crate::db::connection::*;
use crate::models::{
    user::{User, NewUser},
    workout::{Workout, NewWorkout},
    exercise::{Exercise, NewExercise},
    workout_entry::{WorkoutEntry, NewWorkoutEntry},
};

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
// ---------------- EXERCISES ----------------
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

// ---------------- WORKOUTS ----------------

pub async fn list_workouts(State(pool): State<SqlitePool>) -> Json<Vec<Workout>> {
    match get_all_workouts(&pool).await {
        Ok(workouts) => Json(workouts),
        Err(e) => {
            eprintln!("Error fetching workouts: {:?}", e);
            Json(vec![])
        }
    }
}

pub async fn get_workout(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Json<Option<Workout>> {
    match get_workout_by_id(&pool, id).await {
        Ok(workout) => Json(workout),
        Err(_) => Json(None),
    }
}
pub async fn create_workout(State(pool): State<SqlitePool>, Json(new_workout): Json<NewWorkout>) -> Json<Workout> {
    let workout = create_workout_db(&pool, &new_workout).await.expect("Failed to create workout");
    Json(workout)
}

pub async fn delete_workout(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Json<String> {
    match delete_workout_db(&pool, id).await {
        Ok(_) => Json(format!("Workout with id {} deleted", id)),
        Err(e) => Json(format!("Failed to delete workout: {:?}", e)),
    }
}

// ---------------- EXERCISES ----------------

pub async fn list_exercises(State(pool): State<SqlitePool>) -> Json<Vec<Exercise>> {
    match get_all_exercises(&pool).await {
        Ok(exercises) => Json(exercises),
        Err(e) => {
            eprintln!("Error fetching exercises: {:?}", e);
            Json(vec![])
        }
    }
}

pub async fn get_exercise(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Json<Option<Exercise>> {
    let exercise = get_exercise_by_id(&pool, id).await.unwrap();
    Json(exercise)
}

// POST: add single or multiple exercises
pub async fn create_exercise(
    State(pool): State<SqlitePool>,
    Json(exercises): Json<Vec<NewExercise>>, // <-- accepts an array now
) -> Json<Vec<Exercise>> {
    let mut inserted = Vec::new();

    for new_exercise in exercises {
        match create_exercise_db(&pool, &new_exercise).await {
            Ok(ex) => inserted.push(ex),
            Err(e) => eprintln!("Error inserting exercise: {:?}", e),
        }
    }

    Json(inserted)
}
pub async fn delete_exercise(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Json<String> {
    match delete_exercise_db(&pool, id).await {
        Ok(_) => Json(format!("Exercise with id {} deleted", id)),
        Err(e) => Json(format!("Failed to delete exercise: {:?}", e)),
    }
}

// ---------------- WORKOUT ENTRIES ----------------

pub async fn list_workout_entries(State(pool): State<SqlitePool>) -> Json<Vec<WorkoutEntry>> {
    match get_all_workout_entries(&pool).await {
        Ok(entries) => Json(entries),
        Err(e) => {
            eprintln!("Error fetching entries: {:?}", e);
            Json(vec![])
        }
    }
}

pub async fn create_workout_entry(
    State(pool): State<SqlitePool>,
    Json(new_entry): Json<NewWorkoutEntry>,
) -> Json<WorkoutEntry> {
    let entry = create_workout_entry_db(&pool, &new_entry)
        .await
        .expect("Failed to insert workout entry");
    Json(entry)
}

pub async fn delete_workout_entry(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Json<String> {
    match delete_workout_entry_db(&pool, id).await {
        Ok(_) => Json(format!("Workout entry with id {} deleted", id)),
        Err(e) => Json(format!("Failed to delete workout entry: {:?}", e)),
    }
}

// ---------------- HEALTH CHECK ----------------

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "GymBuddy API is healthy 💪!".to_string(),
    })
}

// GET user by id
pub async fn get_user_by_id(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Json<Option<User>> {
    match get_user(&pool, id).await {
        Ok(user) => Json(user),
        Err(_) => Json(None),
    }
}
// GET workout by id
pub async fn get_workout_by_id_route(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Json<Option<Workout>> {
    match get_workout_by_id(&pool, id).await {
        Ok(workout) => Json(workout),
        Err(_) => Json(None),
    }
}

// ---------------- ROUTER SETUP ----------------

pub fn create_api_router() -> Router<SqlitePool> {
    Router::new()
        // Users
        .route("/api/users", get(list_users).post(create_user))
        .route("/api/users/:id", get(get_user_by_id))
        // Workouts
        .route("/api/workouts", get(list_workouts).post(create_workout))
        .route("/api/workouts/:id", get(get_workout_by_id_route).delete(delete_workout))
        // Exercises
        .route("/api/exercises", get(list_exercises).post(create_exercise))
        // Workout Entries
        .route("/api/workout_entries", get(list_workout_entries).post(create_workout_entry))
        .route("/api/workout_entries/:id", delete(delete_workout_entry))
        // Health check
        .route("/health", get(health_check))
}
