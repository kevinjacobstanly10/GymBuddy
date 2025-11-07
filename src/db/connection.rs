use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use crate::models::{
    user::{NewUser, User},
    workout::{Workout, NewWorkout},
    exercise::{Exercise, NewExercise},
};


pub async fn establish_connection() -> SqlitePool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to database")
}

pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Create users table
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            email TEXT NOT NULL
        );
        ",
    )
    .execute(pool)
    .await?;

    // Create workouts table
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS workouts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            notes TEXT,
            FOREIGN KEY(user_id) REFERENCES users(id)
        );
        ",
    )
    .execute(pool)
    .await?;

    // Create exercises table
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS exercises (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            muscle_group TEXT NOT NULL,
            description TEXT
        );
        ",
    )
    .execute(pool)
    .await?;

    // Create workout_entries table (linking workouts ↔ exercises)
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS workout_entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            workout_id INTEGER NOT NULL,
            exercise_id INTEGER NOT NULL,
            sets INTEGER NOT NULL,
            reps INTEGER NOT NULL,
            weight REAL,
            FOREIGN KEY(workout_id) REFERENCES workouts(id),
            FOREIGN KEY(exercise_id) REFERENCES exercises(id)
        );
        ",
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

pub async fn update_user_db(pool: &SqlitePool, id: i64, updated_user: &NewUser) -> Result<User, sqlx::Error> {
    sqlx::query(
        "UPDATE users SET username = ?, email = ? WHERE id = ?"
    )
    .bind(&updated_user.username)
    .bind(&updated_user.email)
    .bind(id)
    .execute(pool)
    .await?;

    // Fetch the updated user
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, email FROM users WHERE id = ?"
    )
    .bind(id)
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

// WORKOUTS CRUD 
pub async fn get_all_workouts(pool: &SqlitePool) -> Result<Vec<Workout>, sqlx::Error> {
    let workouts = sqlx::query_as::<_, Workout>(
        "SELECT id, user_id, date, notes FROM workouts"
    )
    .fetch_all(pool)
    .await?;
    Ok(workouts)
}

pub async fn create_workout_db(pool: &SqlitePool, workout: &NewWorkout) -> Result<Workout, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO workouts (user_id, date, notes) VALUES (?, ?, ?)"
    )
    .bind(workout.user_id)
    .bind(&workout.date)
    .bind(&workout.notes)
    .execute(pool)
    .await?;

    let last_id = result.last_insert_rowid();

    let new_workout = sqlx::query_as::<_, Workout>(
        "SELECT id, user_id, date, notes FROM workouts WHERE id = ?"
    )
    .bind(last_id)
    .fetch_one(pool)
    .await?;

    Ok(new_workout)
}

// Delete a workout by ID
pub async fn delete_workout_db(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM workouts WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

// ---------------- EXERCISES ----------------

pub async fn get_all_exercises(pool: &SqlitePool) -> Result<Vec<Exercise>, sqlx::Error> {
    let exercises =
        sqlx::query_as::<_, Exercise>("SELECT id, name, muscle_group, description FROM exercises")
            .fetch_all(pool)
            .await?;
    Ok(exercises)
}

pub async fn get_exercise_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Exercise>, sqlx::Error> {
    let exercise = sqlx::query_as::<_, Exercise>(
        "SELECT id, name, muscle_group, description FROM exercises WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(exercise)
}

pub async fn create_exercise_db(
    pool: &SqlitePool,
    exercise: &NewExercise,
) -> Result<Exercise, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO exercises (name, muscle_group, description) VALUES (?, ?, ?)",
    )
    .bind(&exercise.name)
    .bind(&exercise.muscle_group)
    .bind(&exercise.description)
    .execute(pool)
    .await?;
    let last_id = result.last_insert_rowid();

    let new_exercise = sqlx::query_as::<_, Exercise>(
        "SELECT id, name, muscle_group, description FROM exercises WHERE id = ?",
    )
    .bind(last_id)
    .fetch_one(pool)
    .await?;
    Ok(new_exercise)
}

pub async fn delete_exercise_db(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM exercises WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

// Get a single workout by ID
pub async fn get_workout_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Workout>, sqlx::Error> {
    let workout = sqlx::query_as::<_, Workout>(
        "SELECT id, user_id, date, notes FROM workouts WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(workout)
}
// Get a single user by ID
pub async fn get_user(pool: &SqlitePool, id: i64) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, email FROM users WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(user)
}