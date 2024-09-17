use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pipeline {
    pub id: i64,
    pub repository_url: String,
}

impl Pipeline {
    pub fn new(id: i64, repository_url: String) -> Self {
        Self { id, repository_url }
    }

    pub fn repository_url(&self) -> &String {
        &self.repository_url
    }
}

#[derive(Debug, Error)]
pub enum CreatePipelineError {
    #[error("Error while creating pipeline: {0}")]
    Error(String),
}

#[derive(Debug, Error)]
pub enum PipelineError {
    #[error("Error while creating pipeline: {0}")]
    CreateError(String),
    #[error("Pipeline not found")]
    NotFound,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error), // Erreurs liées à la base de données

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
