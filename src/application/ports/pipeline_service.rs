use crate::domain::entities::pipeline::{Pipeline, PipelineError};
use async_trait::async_trait;

#[async_trait]
pub trait PipelineService {
    async fn find_all(&self) -> Vec<Pipeline>;
    async fn find_by_id(&self, pipeline_id: i64) -> Result<Pipeline, PipelineError>;
    async fn create_pipeline(&self, repository_url: &String, name: &String) -> Result<Pipeline, PipelineError>;
}
