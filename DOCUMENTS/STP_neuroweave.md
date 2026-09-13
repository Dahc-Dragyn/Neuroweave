Software Testing Plan (STP)
Project: Neuroweave (The Basal Ganglia Engine)
1. Introduction and Objectives
The objective of this STP is to define the testing strategy, phases, and specific test cases required to validate Neuroweave before integration into the Antigravity Agent System.

Primary Testing Goals:

Zero-Damage Guarantee: Prove that the Execution Compiler can perfectly roll back partial changes during local file I/O failures.

Latency Adherence: Validate that the Intent Intersection Gate processes cache-misses in under 50ms to prevent bottlenecking the upstream LLM.

Routing Accuracy: Ensure false positives in intent parsing are eliminated.

2. Test Strategy and Scope
Testing will follow a bottom-up approach, starting with the deterministic Rust modules and moving outward to the asynchronous boundaries where Neuroweave interfaces with the Antigravity SDK and the local file system.

In Scope:

Regex and AST delta parsing within the Intent Intersection Gate.

Memory safety and static compilation of the Subroutine Registry.

Asynchronous file I/O and transactional rollbacks via the Execution Compiler.

Performance benchmarking (latency and memory footprint).

Out of Scope:

Testing the cognitive output of the underlying LLM (e.g., gemini-3.1-flash-lite). If Neuroweave successfully hands off a cache-miss, its job is done.

Antigravity UI rendering of the success/fail artifacts.

3. Test Phases
Phase 1: Unit Testing (Component Level)
Focus: Validating the isolated logic of the Rust binary using standard cargo test.

1.1 Intent Parser Tests: Feed the router 1,000 synthetic developer prompts. Validate that conversational noise is stripped and core directives are correctly mapped to standard intent hashes.

1.2 Type-State Validation: Compile the Subroutine Registry to ensure traits (execute, rollback) are strictly enforced for every loaded module.

1.3 Serialization Tests: Validate that the standardized IPC JSON payload correctly serializes and deserializes between Neuroweave and the mock Antigravity frontend.

Phase 2: Integration Testing (System Level)
Focus: Validating the data flow between Neuroweave, the local file system, and the SDK boundary.

2.1 The Intercept Hook: Mock an Antigravity agent session. Fire requests and verify Neuroweave intercepts the payload before an outbound network request is generated.

2.2 File I/O Execution: Trigger the sys_format_lint subroutine against a dummy workspace. Verify the file is physically modified and the resolved_locally boolean is returned as true.

2.3 Concurrency (Tokio): Fire 50 simultaneous intent payloads at Neuroweave. Verify the async runtime handles the queue without dropping requests or corrupting the session_id.

Phase 3: Fallback and Escapement Testing (Chaos Engineering)
Focus: Proving the engine fails gracefully and escalates appropriately.

3.1 The "Locked File" Scenario: Initiate a sys_scaffold_module subroutine, but manually lock the target directory (read-only permissions) halfway through execution.

Expected Result: The rollback() trait fires, the directory is restored to its original state, and the payload is routed to the LLM with an attached ExecutionError.

3.2 The "Garbage Context" Scenario: Pass a corrupted or unreadable workspace delta to the Intersection Gate.

Expected Result: Immediate cache-miss and escalation to the LLM. No local execution attempted.

Phase 4: Performance and Load Testing
Focus: Enforcing the strict non-functional constraints outlined in the SRS.

4.1 Idle Memory Profiling: Run the Neuroweave daemon in the background for 24 hours. Monitor for memory leaks. Must stay strictly under 30MB.

4.2 Latency Benchmarking (The 50ms Rule): Fire 10,000 highly complex, novel prompts (guaranteed cache-misses). Measure the time from interception to LLM handoff. The 99th percentile (P99) must be under 50ms.

4.3 Cold Start Time: Measure the boot time of the single-binary application. Must initialize in under 50ms.

4. Defect Severity Classifications
Critical (P0): False positive routing (modifying code unexpectedly), failure to roll back on I/O error, or crashing the main Antigravity thread. Halts deployment.

High (P1): Latency exceeds 50ms on cache-misses, memory leaks detected over time, or interception hooks failing to catch active prompts.

Medium (P2): False negatives (failing to recognize a known pattern, resulting in unnecessary API costs), or improper formatting of the JSON IPC payload.

Low (P3): Telemetry errors, minor inefficiencies in AST parsing, or artifact logging delays.

5. Exit Criteria (Definition of Done)
Neuroweave is considered ready for production deployment when:

100% of P0 and P1 defects are resolved.

The rollback() mechanism achieves a 100% success rate during Chaos Engineering tests.

The P99 latency overhead for LLM handoffs remains under 50ms.

The system successfully runs as a background process for 48 hours without exceeding the 30MB memory ceiling.