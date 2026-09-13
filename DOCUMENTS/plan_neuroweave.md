Here is the proposed phased rollout for Project Neuroweave.

Phase 1: Foundation and IPC (The Chassis)
Before we intercept anything, we need the raw engine block and the communication protocol.

Objective: Bootstrap the Rust environment, set up the asynchronous runtime (Tokio), and define the strict JSON structures for Inter-Process Communication (IPC).

Key Deliverables: * Core main.rs daemon structure.

ipc.rs defining the IntentPayload and TaskOutput structs (serialized via serde).

The core BasalTask trait that all future subroutines will implement.

Phase 2: The Intent Intersection Gate (The Router)
We build the triage center. This phase is purely about parsing text and making routing decisions without touching the file system yet.

Objective: Ingest the JSON payload, parse the developer's prompt, and match it against a hardcoded hash-map of known tasks.

Key Deliverables:

router.rs module.

Regex and keyword extraction to strip conversational noise.

The evaluate_intent function that returns either a Route::Subroutine(TaskID) or Route::EscalateLLM.

Phase 3: The Execution Compiler & Transactional I/O (The Actuator)
This is where Neuroweave gets physical access to your workspace. We must build the failsafe mechanisms here.

Objective: Safely execute local file manipulations with absolute transactional integrity.

Key Deliverables:

executor.rs module utilizing std::fs.

Implementation of the rollback() mechanism. If a file is locked or a permission errors out, the system must revert the file state to its pre-execution hash before escalating to the LLM.

Phase 4: The Subroutine Registry (Muscle Memory)
With the router and executor built, we wire up the first actual development tasks. We will start with the safest, highest-yield operations.

Objective: Populate the in-memory registry with our first deterministic Rust modules.

Key Deliverables:

sys_format_lint: A subroutine that intercepts formatting requests and pipes them directly to local tools (like Ruff or Prettier).

sys_clean_imports: A subroutine to regex-strip orphaned imports based on standard AST rules.

Phase 5: Antigravity SDK Hook & Telemetry (The Wiretap)
We integrate Neuroweave into your local development environment and ensure it meets our strict non-functional constraints.

Objective: Hook the binary into the Antigravity pre-turn execution flow and measure performance.

Key Deliverables:

Integration script for the Antigravity Agent Runtime.

Latency benchmarking (ensuring cache-miss LLM handoffs occur in < 50ms).

Memory profiling to ensure the background daemon stays under 30MB.