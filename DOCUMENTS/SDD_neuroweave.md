Let’s lock it in. The PRD gave us the What and the Why. The Software Design Document (SDD) is the How.

We are building Neuroweave to be the primary intercept layer—essentially the first line of defense for the entire agent ecosystem, resolving operational issues locally before requiring a high-latency LLM escalation.

Here is the architectural blueprint for Neuroweave.

Software Design Document (SDD)
Project: Neuroweave ("The Basal Ganglia Engine")
Environment: Antigravity Agent System (v2.0+)
Core Paradigm: Single-Binary Appliance Pattern (Rust)

1. System Architecture Overview
Neuroweave operates as an asynchronous, deterministic middleware layer sitting precisely between the developer’s workspace (the IDE/CLI) and the stochastic LLM reasoning engine. It intercepts all incoming agent intents, evaluates them for known operational patterns, and executes pre-compiled subroutines if a match is found.

1.1 High-Level Data Flow
Input Event: Developer issues a command via prompt or workspace delta.

Interception: Neuroweave intercepts the payload before it hits the LLM API boundary.

Evaluation: The payload is hashed and evaluated against the Subroutine Registry.

Branching Logic:

Cache Hit (Deterministic): Request routed to the Execution Compiler. Rust subroutine executes local file I/O. Success flag returned to IDE.

Cache Miss (Stochastic): Request routed to the LLM for active cognitive reasoning.

2. Component Design & Specifications
2.1 The Intent Intersection Gate (The Router)
This module acts as the triage center. It must be brutally fast and computationally lightweight.

Technology: Pure Rust utilizing Regex and abstract syntax tree (AST) delta comparisons.

Core Function: fn evaluate_intent(prompt: &str, workspace_diff: &Diff) -> IntentRoute

Mechanism: It strips the prompt of conversational noise (e.g., "Could you please format...") to extract the core directive (format). It then checks the active workspace context. If the directive and the context map to a known type-state, it returns a Route::Subroutine(TaskID). Otherwise, it returns Route::EscalateLLM.

2.2 The Subroutine Registry (Muscle Memory)
A statically compiled, in-memory registry. It does not rely on external databases or network calls, adhering strictly to the appliance pattern.

Technology: Rust HashMap populated at compile time, utilizing the Typestate Pattern to ensure memory safety.

Core Traits: Every subroutine must implement the BasalTask trait, guaranteeing standard execution and rollback methods.

Rust
pub trait BasalTask {
    fn execute(&self, context: &WorkspaceContext) -> Result<TaskOutput, ExecutionError>;
    fn rollback(&self, context: &WorkspaceContext) -> Result<(), RollbackError>;
}
Initial Subroutine Manifest:

sys_format_lint: Invokes local formatting tools (e.g., Ruff, Prettier) directly.

sys_scaffold_module: Injects standard boilerplate code based on project architecture.

sys_clean_imports: Identifies and removes orphaned imports via AST parsing.

2.3 The Execution Compiler (The Builder)
This is the physical actuator. When a subroutine is called, this module handles the actual file manipulation and state changes.

Technology: std::fs for zero-cost file I/O, wrapped in a transactional execution model.

Error Handling: Strict operational constraints. If a local Rust execution fails (e.g., file lock, permission error), it must not fail silently. It immediately rolls back the partial changes and triggers Route::EscalateLLM with the error trace attached, allowing the active AI to problem-solve the failure.

3. Interfaces & Data Structures
3.1 Standardized IPC (Inter-Process Communication) Payload
To communicate with the Antigravity frontend and the LLM backend, Neuroweave uses a strictly typed JSON payload.

Field	Type	Description
session_id	String	Unique identifier for the active coding session.
intent_hash	String	SHA-256 hash of the parsed user directive.
resolved_locally	Boolean	true if handled by Neuroweave; false if sent to LLM.
execution_ms	Integer	Latency tracker for telemetry.
4. Non-Functional Requirements
To maintain its status as an energy-optimizing engine, Neuroweave must adhere to the following strict boundaries:

Startup Time: < 50ms. Must boot instantly alongside the agent.

Memory Footprint: < 30MB at idle.

Concurrency: Must utilize Tokio to handle multiple asynchronous file I/O requests without blocking the main agent UI thread.

Dependency Minimization: No bloated third-party frameworks. Core logic must be built using the standard library and foundational crates (Serde, Tokio) to ensure the compiled binary remains ultra-lightweight.