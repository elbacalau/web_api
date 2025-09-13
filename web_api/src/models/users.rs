use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::DateTime;
use chrono::Utc;
use sqlx::Error;
use crate::db::DbPool;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub async fn get_by_id(id: u64, pool: &DbPool) -> Result<User, Error> {
        let query = "SELECT * FROM users WHERE id = ?";
        let user: User = sqlx::query_as(query).bind(id).fetch_one(pool).await?;
        Ok(user)
    }

    pub async fn create(self, pool: &DbPool) -> Result<User, Error> {
        let query = "INSERT INTO users (username, email, password_hash) VALUES (?, ?, ?)";
        let result = sqlx::query(query)
            .bind(&self.username)
            .bind(&self.email)
            .bind(&self.password_hash)
            .execute(pool)
            .await?;
        
        let id = result.last_insert_id();
        
        
        Self::get_by_id(id, pool).await
    }
}