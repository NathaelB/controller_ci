use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pipeline {
    pub id: i64,
    pub repository_url: String
}

impl Pipeline {
    pub fn new(id: i64, repository_url: String) -> Self {
        Self { id, repository_url }
    }
}