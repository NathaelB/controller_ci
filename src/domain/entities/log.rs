use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Log {
    pub id: i64,
    pub action_id: i64,
    pub data: String,
}

impl Log {
    pub fn new(id: i64, action_id: i64, data: String) -> Self {
        Self {
            id,
            action_id,
            data,
        }
    }
}
