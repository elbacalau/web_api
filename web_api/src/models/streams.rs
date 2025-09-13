use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Stream {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub category: String,
    pub is_live: bool,
    pub started_at: Option<NaiveDateTime>,
    pub ended_at: Option<NaiveDateTime> ,
}