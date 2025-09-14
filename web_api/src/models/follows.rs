use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Error};
use chrono::{DateTime, Utc};

use crate::db::DbPool;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Follow {
    pub follower_id: u64,
    pub followed_id: u64,
    pub created_at: Option<DateTime<Utc>>,
}

impl Follow {
    pub async fn create(follower_id: u64, followed_id: u64, pool: &DbPool) -> Result<Follow, Error> {
        let query = "INSERT INTO follows (follower_id, followed_id) VALUES (?, ?)";
        sqlx::query(query)
            .bind(follower_id)
            .bind(followed_id)
            .execute(pool)
            .await?;

        Ok(Follow {
            follower_id,
            followed_id,
            created_at: Some(Utc::now()),
        })
    }

    pub async fn delete(follower_id: u64, followed_id: u64, pool: &DbPool) -> Result<(), Error> {
        let query = "DELETE FROM follows WHERE follower_id = ? AND followed_id = ?";
        sqlx::query(query)
            .bind(follower_id)
            .bind(followed_id)
            .execute(pool)
            .await?;
        
        Ok(())
    }

    pub async fn exists(follower_id: u64, followed_id: u64, pool: &DbPool) -> Result<bool, Error> {
        let query = "SELECT COUNT(*) FROM follows WHERE follower_id = ? AND followed_id = ?";
        let count: (i64,) = sqlx::query_as(query)
            .bind(follower_id)
            .bind(followed_id)
            .fetch_one(pool)
            .await?;
        
        Ok(count.0 > 0)
    }

    pub async fn get_follower_count(user_id: u64, pool: &DbPool) -> Result<u64, Error> {
        let query = "SELECT COUNT(*) FROM follows WHERE followed_id = ?";
        let count: (i64,) = sqlx::query_as(query)
            .bind(user_id)
            .fetch_one(pool)
            .await?;
        
        Ok(count.0 as u64)
    }
}