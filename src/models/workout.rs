use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Workout {
    pub id: i64,
    pub user_id: i64,
    pub date: String,
    pub notes: Option<String>,
}