use crate::{
    application::ports::action_service::ActionService,
    domain::{
        entities::action::{Action, ActionError, ActionType},
        repositories::action_repository::ActionRepository,
    },
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct ActionServiceImpl {
    repository: Arc<Box<dyn ActionRepository + Send + Sync>>,
}

impl ActionServiceImpl {
    pub fn new(repository: Arc<Box<dyn ActionRepository + Send + Sync>>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ActionService for ActionServiceImpl {
    async fn create(
        &self,
        pipeline_id: i64,
        name: String,
        container_uri: String,
        r#type: ActionType,
        status: String,
    ) -> Result<Action, ActionError> {
        self.repository
            .create(pipeline_id, name, container_uri, r#type, status)
            .await
    }
}
