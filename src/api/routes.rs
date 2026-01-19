use axum::{
    extract::{Path, State},
    routing::{get, post, delete, put},
    Json, Router,
};
use serde::Serialize;
use sqlx::SqlitePool;
use crate::db::connection::*;
use crate::models::{
    user::{User, NewUser, LoginUser},
    workout::{Workout, NewWorkout},
    exercise::{Exercise, NewExercise},
    workout_entry::{WorkoutEntry, NewWorkoutEntry, WorkoutEntryDetailed},
};
use crate::auth::{hash_password, verify_password};
use crate::jwt::{generate_jwt, verify_jwt};
use crate::middleware::auth::AuthUser;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
}

#[derive(sqlx::FromRow)]
struct UserRow {
    id: i64,
    password_hash: String,
}

#[derive(Serialize)]
struct WeeklyAnalytics {
    total_volume: f64,
    top_muscle: String,
}


// GET: list all users
pub async fn list_users(
    auth: AuthUser,
    State(pool): State<SqlitePool>
) -> Json<Vec<User>> {
    let users = get_all_users(&pool).await.unwrap_or_default();
    Json(users)
}

// ---------------- EXERCISES ----------------
// Add new user
pub async fn create_user(
    auth: AuthUser,
    State(pool): State<SqlitePool>,
    Json(new_user): Json<NewUser>,
) -> Json<User> {
    let user = create_user_db(&pool, &new_user)
        .await
        .expect("Failed to insert user");

    Json(user)
}

pub async fn update_user(
    auth: AuthUser,
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
    Json(updated_user): Json<NewUser>,
) -> Json<User> {
    let id = id.trim().parse::<i64>().unwrap();

    let user = update_user_db(&pool, id, &updated_user)
        .await
        .expect("Failed to update user");

    Json(user)
}

pub async fn delete_user(
    auth: AuthUser,
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Json<String> {
    delete_user_db(&pool, id).await.unwrap();
    Json(format!("User {} deleted", id))
}

// ---------------- WORKOUTS ----------------

pub async fn list_workouts(
    auth: AuthUser,
    State(pool): State<SqlitePool>
) -> Json<Vec<Workout>> {
    let workouts = get_all_workouts(&pool)
        .await
        .unwrap_or_default();

    Json(workouts)
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

pub async fn create_workout(
    auth: AuthUser,
    State(pool): State<SqlitePool>,
    Json(mut new_workout): Json<NewWorkout>
) -> Json<Workout> {

    tracing::info!(
        "Creating workout for user {}",
        auth.user_id
    );

    new_workout.user_id = auth.user_id;

    let workout = create_workout_db(&pool, &new_workout)
        .await
        .expect("Failed to create workout");

    Json(workout)
}

pub async fn delete_workout(
    auth: AuthUser,
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Json<String> {
    delete_workout_db(&pool, id)
        .await
        .expect("Failed to delete workout");

    Json(format!("Workout {} deleted", id))
}


// ---------------- EXERCISES ----------------

pub async fn list_exercises(State(pool): State<SqlitePool>) -> Json<Vec<Exercise>> {
    match get_all_exercises(&pool).await {
        Ok(exercises) => Json(exercises),
        Err(e) => {
            tracing::error!("Error fetching exercises: {:?}", e);
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
            Err(e) => tracing::error!("Error fetching exercises: {:?}", e),
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

pub async fn update_workout_entry(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(updated_entry): Json<NewWorkoutEntry>,
) -> Json<WorkoutEntry> {
    let entry = update_workout_entry_db(&pool, id, &updated_entry)
        .await
        .expect("Failed to update workout entry");
    Json(entry)
}

// ---------------- WORKOUT ENTRIES ----------------

pub async fn list_workout_entries(State(pool): State<SqlitePool>) -> Json<Vec<WorkoutEntry>> {
    match get_all_workout_entries(&pool).await {
        Ok(entries) => Json(entries),
        Err(e) => {
            tracing::error!("Error fetching exercises: {:?}", e);
            Json(vec![])
        }
    }
}

pub async fn create_workout_entry(
    State(pool): State<SqlitePool>,
    Json(new_entry): Json<NewWorkoutEntry>,
) -> Json<serde_json::Value> {

    match create_workout_entry_db(&pool, &new_entry).await {
        Ok(entry) => Json(serde_json::json!({
            "status": "success",
            "entry": entry
        })),
        Err(e) => Json(serde_json::json!({
            "status": "error",
            "message": e.to_string()
        })),
    }
}


pub async fn update_workout(
    auth: AuthUser,
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Json(updated): Json<NewWorkout>
) -> Json<Workout> {

    let workout = update_workout_db(&pool, id, &updated)
        .await
        .expect("Failed to update workout");

    Json(workout)
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
    auth: AuthUser,
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Json<Option<User>> {
    let user = get_user(&pool, id).await.unwrap();
    Json(user)
}
// GET workout by id
pub async fn get_workout_by_id_route(
    auth: AuthUser,
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Json<Option<Workout>> {
    let workout = get_workout_by_id(&pool, id)
        .await
        .unwrap();

    Json(workout)
}

// To get more detailed info of workout (name, muscle group, etc)
pub async fn list_workout_entries_detailed(
    State(pool): State<SqlitePool>,
) -> Json<Vec<WorkoutEntryDetailed>> {
    match get_detailed_workout_entries(&pool).await {
        Ok(entries) => Json(entries),
        Err(e) => {
            tracing::error!("Error fetching exercises: {:?}", e);
            Json(vec![])
        }
    }
}

pub async fn get_entries_for_workout(
    auth: AuthUser,
    Path(workout_id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Json<Vec<WorkoutEntryDetailed>> {
    let entries = get_workout_entries_by_workout_id(&pool, workout_id)
        .await
        .unwrap_or_default();

    Json(entries)
}


// Summary report route
pub async fn get_workout_summary_route(
    auth: AuthUser,
    Path(workout_id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Json<serde_json::Value> {
    let summary = get_workout_summary(&pool, workout_id)
        .await
        .expect("Failed to fetch summary");

    Json(summary)
}

// Overall progress route
pub async fn get_user_progress_route(
    auth: AuthUser,
    Path(user_id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Json<serde_json::Value> {
    let progress = get_workout_progress(&pool, user_id)
        .await
        .expect("Failed to fetch user progress");

    Json(progress)
}

// POST /api/register
pub async fn register_user(
    State(pool): State<SqlitePool>,
    Json(new_user): Json<NewUser>,
) -> Json<serde_json::Value> {

    tracing::info!(
        "Registering new user: {}",
        new_user.email
    );

    let password_hash = hash_password(&new_user.password);

    let result = sqlx::query(
        "INSERT INTO users (username, email, password_hash) VALUES (?, ?, ?)"
    )
    .bind(&new_user.username)
    .bind(&new_user.email)
    .bind(password_hash)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => {
            tracing::info!("User registered successfully: {}", new_user.email);
            Json(serde_json::json!({"status": "success"}))
        }
        Err(e) => {
            tracing::error!(
                "Failed to register user {}: {:?}",
                new_user.email,
                e
            );
            Json(serde_json::json!({"status": "error"}))
        }
    }
}

pub async fn login_user(
    State(pool): State<SqlitePool>,
    Json(login): Json<LoginUser>,
) -> Json<serde_json::Value> {

    tracing::info!("Login attempt for {}", login.email);

    let user = sqlx::query_as::<_, (i64, String)>(
        "SELECT id, password_hash FROM users WHERE email = ?"
    )
    .bind(&login.email)
    .fetch_optional(&pool)
    .await
    .unwrap();

    if let Some((id, password_hash)) = user {
        if verify_password(&password_hash, &login.password) {
            tracing::info!("Login successful for {}", login.email);
            let token = generate_jwt(id);
            return Json(serde_json::json!({ "token": token }));
        }
    }

    tracing::warn!("Invalid login attempt for {}", login.email);
    Json(serde_json::json!({ "error": "Invalid credentials" }))
}


// Analytics
pub async fn weekly_analytics(
    auth: AuthUser,
    State(pool): State<SqlitePool>,
) -> Json<WeeklyAnalytics> {

     tracing::info!(
        "Fetching weekly analytics for user {}",
        auth.user_id
    );

    let volume: (Option<f64>,) = sqlx::query_as(
        r#"
        SELECT SUM(we.sets * we.reps * COALESCE(we.weight,0))
        FROM workout_entries we
        JOIN workouts w ON we.workout_id = w.id
        WHERE w.user_id = ?
          AND w.date >= date('now','-7 days')
        "#
    )
    .bind(auth.user_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    let muscle: (Option<String>,) = sqlx::query_as(
        r#"
        SELECT e.muscle_group
        FROM workout_entries we
        JOIN exercises e ON we.exercise_id = e.id
        JOIN workouts w ON we.workout_id = w.id
        WHERE w.user_id = ?
        GROUP BY e.muscle_group
        ORDER BY SUM(we.sets * we.reps * COALESCE(we.weight,0)) DESC
        LIMIT 1
        "#
    )
    .bind(auth.user_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(WeeklyAnalytics {
        total_volume: volume.0.unwrap_or(0.0),
        top_muscle: muscle.0.unwrap_or("N/A".into()),
    })
}


// ---------------- ROUTER SETUP ----------------

pub fn create_api_router() -> Router<SqlitePool> {
    let protected = Router::new()
        .route("/api/users", get(list_users).post(create_user))
        .route("/api/users/:id", get(get_user_by_id))
        .route("/api/users/:id/progress", get(get_user_progress_route))

        .route("/api/exercises", get(list_exercises).post(create_exercise))

        .route(
            "/api/workouts",
            get(list_workouts).post(create_workout),
        )
        .route(
            "/api/workouts/:id",
            get(get_workout_by_id_route)
                .put(update_workout)
                .delete(delete_workout),
        )
        .route("/api/workouts/:id/entries", get(get_entries_for_workout))
        .route("/api/workouts/:id/summary", get(get_workout_summary_route))

        .route(
            "/api/workout_entries",
            get(list_workout_entries_detailed).post(create_workout_entry),
        )
        .route(
            "/api/workout_entries/:id",
            put(update_workout_entry).delete(delete_workout_entry),
        )
        .route("/api/analytics/weekly", get(weekly_analytics));

    Router::new()
        // Public
        .route("/api/register", post(register_user))
        .route("/api/login", post(login_user))
        .route("/health", get(health_check))

        // Protected (AuthUser extractor works here)
        .merge(protected)
}
