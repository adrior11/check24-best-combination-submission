use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskMessagePayload {
    pub task_id: String,
    pub game_ids: Vec<u32>,
}
