use async_trait::async_trait;

use crate::domain::entities::action::Action;

#[async_trait]
pub trait SchedulerService: Send + Sync {
    async fn send_action(
        &self,
        action: Action,
        repo_url: String,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
