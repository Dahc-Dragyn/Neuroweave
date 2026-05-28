# Subroutine Expansion Protocol

This document provides a strict, step-by-step developer checklist for adding new deterministic "Muscle Memory" subroutines to the Neuroweave engine. Follow this protocol exactly to ensure that we maintain system safety, compile-time performance, and clean transactional boundaries.

---

## 🛠️ Step-by-Step Integration Protocol

### 1. Step 1: Task ID & Route Registration
Open [src/router.rs](file:///c:/Antigravity%20projects/Rust/neuroweave/src/router.rs) and register your new task identifiers:
*   Add a new variant to the `TaskID` enum representing your task.
    ```rust
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TaskID {
        FormatLint,
        ScaffoldModule,
        CleanImports,
        MyNewDeterministicTask, // <-- Register your task here
    }
    ```
*   Add a new `OnceLock<Regex>` static matcher inside `evaluate_intent` to match your target lexical command keywords (case-insensitive):
    ```rust
    static RE_MY_NEW_TASK: OnceLock<Regex> = OnceLock::new();
    let my_new_task = RE_MY_NEW_TASK.get_or_init(|| Regex::new(r"(?i)my_trigger_keyword|other_keyword").unwrap());
    ```
*   Route the matched regex to your `TaskID` inside `evaluate_intent`, returning a high confidence score:
    ```rust
    } else if my_new_task.is_match(prompt) {
        ConfidenceProfile {
            task_id: Some(TaskID::MyNewDeterministicTask),
            confidence_score: 0.95,
            ambiguity_flags: vec![],
            escalation_reason: None,
        }
    }
    ```

### 2. Step 2: Define the Subroutine Struct
Open [src/subroutines.rs](file:///c:/Antigravity%20projects/Rust/neuroweave/src/subroutines.rs) and declare your task struct:
```rust
pub struct MyNewDeterministicTask;
```

### 3. Step 3: Implement the `BasalTask` Trait
Implement the core trait requirements on your struct inside [src/subroutines.rs](file:///c:/Antigravity%20projects/Rust/neuroweave/src/subroutines.rs). You must strictly outline:
*   **`execute`**: The asynchronous payload execution block. Utilize `FileTransaction` for all file manipulations to guarantee rollback transactionality.
*   **`rollback`**: The recovery script. It must return the workspace to its exact pre-execution state.
*   **`trust_radius`**: Declare your safety constraints explicitly using `TrustRadius` bounds.
```rust
impl BasalTask for MyNewDeterministicTask {
    async fn execute(&self, payload: &IntentPayload) -> Result<TaskOutput, String> {
        let start = std::time::Instant::now();
        let target = "my_target_file.txt";

        // 1. Begin transaction tracking
        let transaction = FileTransaction::begin(target)
            .await
            .map_err(|e| format!("Transaction initiation failed: {}", e))?;

        // 2. Perform write/manipulation
        transaction.write("New deterministic output...")
            .await
            .map_err(|e| format!("File write failed: {}", e))?;

        Ok(TaskOutput {
            resolved_locally: true,
            execution_ms: start.elapsed().as_millis() as u64,
            message: "Completed deterministically via muscle memory.".to_string(),
        })
    }

    async fn rollback(&self, _payload: &IntentPayload) -> Result<(), String> {
        let target = "my_target_file.txt";
        let transaction = FileTransaction::begin(target)
            .await
            .map_err(|e| format!("Rollback initiation failed: {}", e))?;
        transaction.rollback().await.map_err(|e| format!("Rollback failed: {}", e))?;
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
```

### 4. Step 4: Dispatch Wiring
Open [src/dispatcher.rs](file:///c:/Antigravity%20projects/Rust/neuroweave/src/dispatcher.rs) and map your `TaskID` variant inside the `dispatch` coordinator's match block:
```rust
    match profile.task_id.as_ref().unwrap() {
        TaskID::FormatLint => {
            let task = crate::subroutines::FormatLintTask;
            task.execute(payload).await
        }
        TaskID::MyNewDeterministicTask => {
            let task = crate::subroutines::MyNewDeterministicTask;
            task.execute(payload).await
        }
        // ...
    }
```

### 5. Step 5: Compile & Lock "Muscle Memory"
Recompile your single-binary appliance to lock the new muscle memory subroutine securely inside the Rust executable:
```bash
cargo build --release
```
Verify that the compilation succeeds without generating any warnings or dead code traces!
