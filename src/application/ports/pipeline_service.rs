use crate::domain::entities::piepline::{Pipeline, PipelineError};
use async_trait::async_trait;

#[async_trait]
pub trait PipelineService {
    async fn find_all(&self) -> Vec<Pipeline>;
    async fn create_pipeline(&self, repository_url: &String) -> Result<Pipeline, PipelineError>;
}
