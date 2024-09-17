use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: i64,
    pub action_id: i64,
    pub command: String,
}

impl Command {
    pub fn new(id: i64, action_id: i64, command: String) -> Self {
        Self {
            id,
            action_id,
            command,
        }
    }
}


#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error)
}