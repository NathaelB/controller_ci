use crate::domain::entities::action::{Action, ActionError, ActionType};
use async_trait::async_trait;

#[async_trait]
pub trait ActionService: Send + Sync {
    async fn create(
        &self,
        pipeline_id: i64,
        name: String,
        container_uri: String,
        r#type: ActionType,
        status: String,
    ) -> Result<Action, ActionError>;
}
