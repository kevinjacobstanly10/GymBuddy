use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Exercise {
    pub id: i64,
    pub name: String,
    pub muscle_group: String,
    pub description: Option<String>,
}