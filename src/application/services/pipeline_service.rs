use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::ports::pipeline_service::PipelineService,
    domain::{
        entities::pipeline::{Pipeline, PipelineError},
        repositories::pipeline_repository::PipelineRepository,
    },
};

pub struct PipelineServiceImpl {
    repository: Arc<Box<dyn PipelineRepository + Send + Sync>>,
}

impl PipelineServiceImpl {
    pub fn new(repository: Arc<Box<dyn PipelineRepository + Send + Sync>>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl PipelineService for PipelineServiceImpl {
    async fn find_all(&self) -> Vec<Pipeline> {
        self.repository.find_all().await.unwrap_or_else(|_| vec![])
    }

    async fn create_pipeline(&self, repository_url: &String, name: &String) -> Result<Pipeline, PipelineError> {
        self.repository.create(repository_url, name).await
    }

    async fn find_by_id(&self, pipeline_id: i64) -> Result<Pipeline, PipelineError> {
        self.repository.find_by_id(pipeline_id).await
    }
}
