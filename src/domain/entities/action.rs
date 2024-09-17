use crate::domain::entities::command::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, PartialEq, Deserialize)]
pub enum ActionType {
    Container,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionStatus {
    Pending,
    Scheduled,
    Running,
    Completed,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: i64,
    pub pipeline_id: i64,
    pub name: String,
    pub r#type: ActionType,
    pub container_uri: String,
    pub commands: Vec<Command>,
    pub status: ActionStatus,
}

impl Action {
    pub fn new(
        id: i64,
        pipeline_id: i64,
        name: String,
        status: ActionStatus,
        r#type: ActionType,
        container_uri: String,
        commands: Vec<Command>,
    ) -> Self {
        Self {
            id,
            pipeline_id,
            name,
            status,
            r#type,
            container_uri,
            commands,
        }
    }
}
