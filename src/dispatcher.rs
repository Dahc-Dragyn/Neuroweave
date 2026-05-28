use crate::confidence::ConfidenceProfile;
use crate::ipc::{IntentPayload, TaskOutput};
use crate::router::TaskID;
use crate::core::BasalTask;

pub async fn dispatch(profile: &ConfidenceProfile, payload: &IntentPayload) -> Result<TaskOutput, String> {
    // Security check: Trust boundaries must be strictly enforced
    if profile.confidence_score < 0.90 || !profile.ambiguity_flags.is_empty() {
        return Ok(TaskOutput {
            resolved_locally: false,
            execution_ms: 0,
            message: profile
                .escalation_reason
                .clone()
                .unwrap_or_else(|| "Low confidence or ambiguity detected. Escalating to LLM.".to_string()),
        });
    }

    // Match matched target subroutine
    match profile.task_id.as_ref().unwrap() {
        TaskID::FormatLint => {
            let task = crate::subroutines::FormatLintTask;
            task.execute(payload).await
        }
        _ => {
            Ok(TaskOutput {
                resolved_locally: false,
                execution_ms: 0,
                message: "Subroutine not yet implemented in registry. Escalating to LLM.".to_string(),
            })
        }
    }
}
