use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct WorkoutEntry {
    pub id: i64,
    pub workout_id: i64,
    pub exercise_id: i64,
    pub sets: i32,
    pub reps: i32,
    pub weight: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct NewWorkoutEntry {
    pub workout_id: i64,
    pub exercise_id: i64,
    pub sets: i32,
    pub reps: i32,
    pub weight: Option<f32>,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct WorkoutEntryDetailed { // Used to return more details than just id
    pub id: i64,
    pub workout_id: i64,
    pub exercise_id: i64,
    pub exercise_name: String,
    pub muscle_group: String,
    pub sets: i32,
    pub reps: i32,
    pub weight: Option<f32>,
}