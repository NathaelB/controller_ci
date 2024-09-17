use async_trait::async_trait;

use crate::domain::entities::command::{Command, CommandError};

#[async_trait]
pub trait CommandRepository: Send + Sync {
  async fn find_all(&self) -> Result<Vec<Command>, CommandError>;
  async fn create(&self, action_id: i64, command: String) -> Result<Command, CommandError>;
}