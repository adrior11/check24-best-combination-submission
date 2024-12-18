use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultMessagePayload<T> {
    pub task_id: String,
    pub results: T,
}
