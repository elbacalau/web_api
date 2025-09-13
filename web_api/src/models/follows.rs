use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::DateTime;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Follow {
    pub follower_id: u64,
    pub followed_id: u64,
    pub created_at: Option<DateTime<Utc>>,
}