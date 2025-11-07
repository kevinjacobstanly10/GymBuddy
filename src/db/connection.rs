use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use crate::models::{
    user::{NewUser, User},
    workout::{Workout, NewWorkout},
    exercise::{Exercise, NewExercise},
    workout_entry::{WorkoutEntry, NewWorkoutEntry, WorkoutEntryDetailed},
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

pub async fn update_workout_entry_db(
    pool: &SqlitePool,
    id: i64,
    updated_entry: &NewWorkoutEntry,
) -> Result<WorkoutEntry, sqlx::Error> {
    sqlx::query(
        "
        UPDATE workout_entries 
        SET workout_id = ?, exercise_id = ?, sets = ?, reps = ?, weight = ?
        WHERE id = ?
        ",
    )
    .bind(updated_entry.workout_id)
    .bind(updated_entry.exercise_id)
    .bind(updated_entry.sets)
    .bind(updated_entry.reps)
    .bind(updated_entry.weight)
    .bind(id)
    .execute(pool)
    .await?;

    // Fetch the updated record
    let updated = sqlx::query_as::<_, WorkoutEntry>(
        "
        SELECT id, workout_id, exercise_id, sets, reps, weight
        FROM workout_entries
        WHERE id = ?
        ",
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(updated)
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

// WORKOUT ENTRIES CRUD

// Get all workout entries
pub async fn get_all_workout_entries(pool: &SqlitePool) -> Result<Vec<WorkoutEntry>, sqlx::Error> {
    let entries = sqlx::query_as::<_, WorkoutEntry>(
        "SELECT id, workout_id, exercise_id, sets, reps, weight FROM workout_entries"
    )
    .fetch_all(pool)
    .await?;
    Ok(entries)
}

// Create new workout entry
pub async fn create_workout_entry_db(
    pool: &SqlitePool,
    new_entry: &NewWorkoutEntry,
) -> Result<WorkoutEntry, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO workout_entries (workout_id, exercise_id, sets, reps, weight)
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind(new_entry.workout_id)
    .bind(new_entry.exercise_id)
    .bind(new_entry.sets)
    .bind(new_entry.reps)
    .bind(new_entry.weight)
    .execute(pool)
    .await?;

    let last_id = result.last_insert_rowid();

    let entry = sqlx::query_as::<_, WorkoutEntry>(
        "SELECT id, workout_id, exercise_id, sets, reps, weight FROM workout_entries WHERE id = ?"
    )
    .bind(last_id)
    .fetch_one(pool)
    .await?;

    Ok(entry)
}

pub async fn update_workout_db(
    pool: &SqlitePool,
    id: i64,
    updated_workout: &NewWorkout,
) -> Result<Workout, sqlx::Error> {
    sqlx::query(
        "UPDATE workouts SET user_id = ?, date = ?, notes = ? WHERE id = ?"
    )
    .bind(updated_workout.user_id)
    .bind(&updated_workout.date)
    .bind(&updated_workout.notes)
    .bind(id)
    .execute(pool)
    .await?;

    let workout = sqlx::query_as::<_, Workout>(
        "SELECT id, user_id, date, notes FROM workouts WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(workout)
}

// Delete a workout entry
pub async fn delete_workout_entry_db(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM workout_entries WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

// Get all entries with exercise info
pub async fn get_detailed_workout_entries(
    pool: &SqlitePool,
) -> Result<Vec<WorkoutEntryDetailed>, sqlx::Error> {
    let entries = sqlx::query_as::<_, WorkoutEntryDetailed>(
        "
        SELECT 
            we.id,
            we.workout_id,
            we.exercise_id,
            e.name AS exercise_name,
            e.muscle_group AS muscle_group,
            we.sets,
            we.reps,
            we.weight
        FROM workout_entries we
        JOIN exercises e ON we.exercise_id = e.id
        ORDER BY we.workout_id;
        "
    )
    .fetch_all(pool)
    .await?;

    Ok(entries)
}

pub async fn get_workout_entries_by_workout_id(
    pool: &SqlitePool,
    workout_id: i64,
) -> Result<Vec<WorkoutEntryDetailed>, sqlx::Error> {
    let entries = sqlx::query_as::<_, WorkoutEntryDetailed>(
        "
        SELECT 
            we.id,
            we.workout_id,
            we.exercise_id,
            e.name AS exercise_name,
            e.muscle_group AS muscle_group,
            we.sets,
            we.reps,
            we.weight
        FROM workout_entries we
        JOIN exercises e ON we.exercise_id = e.id
        WHERE we.workout_id = ?
        ORDER BY we.id;
        "
    )
    .bind(workout_id)
    .fetch_all(pool)
    .await?;

    Ok(entries)
}

// Calculates per-muscle-group volume and top exercises
pub async fn get_workout_progress(pool: &SqlitePool, user_id: i64) -> Result<serde_json::Value, sqlx::Error> {
    let workouts = sqlx::query!(
        "SELECT id, date FROM workouts WHERE user_id = ?",
        user_id
    )
    .fetch_all(pool)
    .await?;

    let mut progress = Vec::new();

    for workout in workouts {
        let entries = sqlx::query!(
            "
            SELECT e.name, e.muscle_group, we.sets, we.reps, we.weight
            FROM workout_entries we
            JOIN exercises e ON we.exercise_id = e.id
            WHERE we.workout_id = ?
            ",
            workout.id
        )
        .fetch_all(pool)
        .await?;

        let mut total_sets = 0;
        let mut total_reps = 0;
        let mut total_volume = 0.0;
        let mut muscle_groups = std::collections::HashMap::new();
        let mut top_exercises = Vec::new();

        for row in entries {
            let sets = row.sets;
            let reps = row.reps;
            let weight = row.weight.unwrap_or(0.0);
            let volume = sets as f64 * reps as f64 * weight;

            total_sets += sets;
            total_reps += reps;
            total_volume += volume;

            *muscle_groups.entry(row.muscle_group.clone()).or_insert(0.0) += volume;

            top_exercises.push(serde_json::json!({
                "name": row.name,
                "volume": volume
            }));
        }

        // Sort top exercises by volume descending
        top_exercises.sort_by(|a, b| b["volume"].as_f64().partial_cmp(&a["volume"].as_f64()).unwrap());

        progress.push(serde_json::json!({
            "workout_id": workout.id,
            "date": workout.date,
            "total_sets": total_sets,
            "total_reps": total_reps,
            "total_volume": total_volume,
            "muscle_groups": muscle_groups,
            "top_exercises": top_exercises
        }));
    }

    Ok(serde_json::json!(progress))
}

// For summary, total volume, most muscle group trained
pub async fn get_workout_summary(
    pool: &SqlitePool,
    workout_id: i64,
) -> Result<serde_json::Value, sqlx::Error> {
    let entries = sqlx::query!(
        "
        SELECT e.name, we.sets, we.reps, we.weight
        FROM workout_entries we
        JOIN exercises e ON we.exercise_id = e.id
        WHERE we.workout_id = ?
        ",
        workout_id
    )
    .fetch_all(pool)
    .await?;

    let mut total_sets = 0;
    let mut total_reps = 0;
    let mut total_volume = 0.0;

    let exercises: Vec<_> = entries
        .into_iter()
        .map(|row| {
            let sets = row.sets;
            let reps = row.reps;
            let weight = row.weight.unwrap_or(0.0);
            let volume = sets as f64 * reps as f64 * weight;

            total_sets += sets;
            total_reps += reps;
            total_volume += volume;

            serde_json::json!({
                "name": row.name,
                "sets": sets,
                "reps": reps,
                "weight": weight,
                "volume": volume
            })
        })
        .collect();

    Ok(serde_json::json!({
        "workout_id": workout_id,
        "total_sets": total_sets,
        "total_reps": total_reps,
        "total_volume": total_volume,
        "exercises": exercises
    }))
}

// Overall progress
pub async fn get_user_progress(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Vec<serde_json::Value>, sqlx::Error> {
    // Get all workouts for the user
    let workouts = sqlx::query!(
        "SELECT id, date FROM workouts WHERE user_id = ? ORDER BY date ASC",
        user_id
    )
    .fetch_all(pool)
    .await?;

    let mut progress = Vec::new();

    for workout in workouts {
        // Reuse the summary function logic per workout
        let summary = get_workout_summary(pool, workout.id).await?;
        progress.push(serde_json::json!({
            "workout_id": workout.id,
            "date": workout.date,
            "total_sets": summary["total_sets"],
            "total_reps": summary["total_reps"],
            "total_volume": summary["total_volume"]
        }));
    }

    Ok(progress)
}