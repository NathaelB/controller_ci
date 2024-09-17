use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::ports::pipeline_service::PipelineService,
    domain::{
        entities::piepline::{Pipeline, PipelineError},
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

    async fn create_pipeline(&self, repository_url: &String) -> Result<Pipeline, PipelineError> {
        self.repository.create(repository_url).await
    }
}
