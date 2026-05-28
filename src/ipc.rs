use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntentPayload {
    pub session_id: String,
    pub intent_hash: String,
    pub raw_prompt: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskOutput {
    pub resolved_locally: bool,
    pub execution_ms: u64,
    pub message: String,
}
