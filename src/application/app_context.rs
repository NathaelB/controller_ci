use std::sync::Arc;

use crate::{
    domain::repositories::{
        action_repository::ActionRepository, command_repository::CommandRepository,
        pipeline_repository::PipelineRepository,
    },
    infrastructure::db::{
        action_repository::PostgresActionRepository, command_repository::PostgresCommandRepository,
        pipeline_repository::PostgresPipelineRepository, postgres::Postgres,
    },
};

use super::{
    ports::{
        action_service::ActionService, command_service::CommandService,
        pipeline_service::PipelineService,
    },
    services::{
        action_service::ActionServiceImpl, command_service::CommandServiceImpl,
        pipeline_service::PipelineServiceImpl,
    },
};

pub struct AppContext {
    pub pipeline_service: Arc<Box<dyn PipelineService + Send + Sync>>,
    pub action_service: Arc<Box<dyn ActionService + Send + Sync>>,
    pub command_service: Arc<Box<dyn CommandService + Send + Sync>>,
}

impl AppContext {
    pub async fn initialize(postgres: Arc<Postgres>) -> Self {
        let pipeline_repository: Arc<Box<dyn PipelineRepository + Send + Sync>> = Arc::new(
            Box::new(PostgresPipelineRepository::new(Arc::clone(&postgres))),
        );

        let action_repository: Arc<Box<dyn ActionRepository + Send + Sync>> = Arc::new(Box::new(
            PostgresActionRepository::new(Arc::clone(&postgres)),
        ));

        let command_repository: Arc<Box<dyn CommandRepository + Send + Sync>> = Arc::new(Box::new(
            PostgresCommandRepository::new(Arc::clone(&postgres)),
        ));

        let command_service: Arc<Box<dyn CommandService + Send + Sync>> = Arc::new(Box::new(
            CommandServiceImpl::new(Arc::clone(&command_repository)),
        ));
        let action_service: Arc<Box<dyn ActionService + Send + Sync>> = Arc::new(Box::new(
            ActionServiceImpl::new(Arc::clone(&action_repository)),
        ));
        let pipeline_service: Arc<Box<dyn PipelineService + Send + Sync>> =
            Arc::new(Box::new(PipelineServiceImpl::new(
                Arc::clone(&pipeline_repository),
                Arc::clone(&action_service),
            )));

        Self {
            pipeline_service,
            action_service,
            command_service,
        }
    }
}
