use crate::ipc::{IntentPayload, TaskOutput};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrustRadius {
    pub max_files: usize,
    pub allow_ast_mutation: bool,
    pub allow_cross_module_changes: bool,
    pub rollback_required: bool,
}

pub trait BasalTask {
    async fn execute(&self, payload: &IntentPayload) -> Result<TaskOutput, String>;
    
    #[allow(dead_code)]
    async fn rollback(&self, payload: &IntentPayload) -> Result<(), String>;
    
    #[allow(dead_code)]
    fn trust_radius(&self) -> TrustRadius;
}
