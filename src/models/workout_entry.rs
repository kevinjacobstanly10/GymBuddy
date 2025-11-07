use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct WorkoutEntry {
    pub id: i64,
    pub workout_id: i64,
    pub exercise_id: i64,
    pub sets: i32,
    pub reps: i32,
    pub weight: Option<f32>, // Just in case bodyweight exercise
}

#[derive(Deserialize, Debug)]
pub struct NewWorkoutEntry {
    pub workout_id: i64,
    pub exercise_id: i64,
    pub sets: i32,
    pub reps: i32,
    pub weight: Option<f32>,
}