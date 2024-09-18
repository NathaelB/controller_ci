use crate::domain::entities::command::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, PartialEq, Deserialize)]
pub enum ActionType {
    Container,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy, Eq)]
pub enum ActionStatus {
    Pending,
    Running,
    Completed,
    Error,
}

#[derive(Debug, Clone)]
pub struct ActionRequest {
    pub action_id: u32,
    pub commands: Vec<String>,
    pub container_uri: Option<String>,
    pub repo_url: String,
}

#[derive(Debug, Clone)]
pub struct ActionResponse {
    pub action_id: u32,
    pub log: String,
    pub result: Option<ActionResult>,
}

#[derive(Debug, Clone)]
pub struct ActionResult {
    pub completion: ActionStatus,
    pub exit_code: Option<i32>,
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
