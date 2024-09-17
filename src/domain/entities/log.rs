use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Log {
    pub id: i64,
    pub action_id: i64,
    pub data: String,
}

impl Log {
    pub fn new(id: i64, action_id: i64, data: String) -> Self {
        Self {
            id,
            action_id,
            data,
        }
    }
}

#[derive(Debug, Error)]
pub enum LogError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
