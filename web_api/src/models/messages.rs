use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::DateTime;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: u64,
    pub stream_id: u64,
    pub user_id: u64,
    pub content: String,
    pub sent_at: Option<DateTime<Utc>>,
}
