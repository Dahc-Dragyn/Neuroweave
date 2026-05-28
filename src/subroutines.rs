use crate::core::{BasalTask, TrustRadius};
use crate::ipc::{IntentPayload, TaskOutput};
use crate::executor::FileTransaction;

pub struct FormatLintTask;

impl BasalTask for FormatLintTask {
    async fn execute(&self, _payload: &IntentPayload) -> Result<TaskOutput, String> {
        let start = std::time::Instant::now();
        let target_file = "target_workspace.rs";

        // 1. Begin transactional tracking
        let transaction = FileTransaction::begin(target_file)
            .await
            .map_err(|e| format!("Failed to initiate transaction on {}: {}", target_file, e))?;

        // 2. Perform deterministic write / formatting simulation
        let formatted_content = "// [Neuroweave Muscle Memory] Formatting applied deterministically.\nfn main() { println!(\"Formatted!\"); }";
        
        transaction.write(formatted_content)
            .await
            .map_err(|e| format!("Failed to write format changes: {}", e))?;

        let duration = start.elapsed().as_millis() as u64;

        Ok(TaskOutput {
            resolved_locally: true,
            execution_ms: duration,
            message: "Deterministic formatting applied successfully via local muscle memory.".to_string(),
        })
    }

    async fn rollback(&self, _payload: &IntentPayload) -> Result<(), String> {
        let target_file = "target_workspace.rs";
        let transaction = FileTransaction::begin(target_file)
            .await
            .map_err(|e| format!("Failed to rollback: could not read original state: {}", e))?;

        transaction.rollback()
            .await
            .map_err(|e| format!("Failed to rollback file changes: {}", e))?;

        Ok(())
    }

    fn trust_radius(&self) -> TrustRadius {
        TrustRadius {
            max_files: 1,
            allow_ast_mutation: false,
            allow_cross_module_changes: false,
            rollback_required: true,
        }
    }
}
