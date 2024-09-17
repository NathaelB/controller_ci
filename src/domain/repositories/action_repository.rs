use async_trait::async_trait;

use crate::domain::entities::action::Action;

#[async_trait]
pub trait ActionRepository: Send + Sync {
    async fn find_by_pipeline_id(
        &self,
        pipeline_id: i64,
    ) -> Result<Vec<Action>, Box<dyn std::error::Error>>;
}
