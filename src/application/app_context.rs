use std::sync::Arc;

use crate::{
    domain::repositories::pipeline_repository::PipelineRepository,
    infrastructure::db::{pipeline_repository::PostgresPipelineRepository, postgres::Postgres},
};

use super::{
    ports::pipeline_service::PipelineService, services::pipeline_service::PipelineServiceImpl,
};

pub struct AppContext {
    pub pipeline_service: Arc<dyn PipelineService + Send + Sync>,
}

impl AppContext {
    pub async fn initialize(postgres: Arc<Postgres>) -> Self {
        let pipeline_repository: Arc<Box<dyn PipelineRepository + Send + Sync>> =
            Arc::new(Box::new(PostgresPipelineRepository::new(postgres)));

        let pipeline_service = Arc::new(PipelineServiceImpl::new(Arc::clone(&pipeline_repository)));

        Self { pipeline_service }
    }
}
