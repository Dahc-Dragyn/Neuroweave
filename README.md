# Neuroweave: The Basal Ganglia Engine (v1.0-stable)

Neuroweave is a high-performance, asynchronous **Deterministic Local Bypass Engine** compiled in Rust. It functions as a native Model Context Protocol (MCP) server daemon to intercept development intents, offloading repetitive, non-subjective workspace tasks directly to zero-cost, transactional local subroutines.

## ⚡ One-Step Quick Start Installation

Project Neuroweave is designed for zero-friction integration for any developer using the Antigravity Agent. You don't have to manually build binaries or configure files.

### 💻 Windows Installation (PowerShell)
Simply run the following command in the project root to compile the engine and automatically register it as an Antigravity MCP Server:
```powershell
./install.ps1
```

### 🍎 macOS / 🐧 Linux Installation (Bash)
Run the automated installation script:
```bash
chmod +x install.sh && ./install.sh
```

The installer will automatically detect your active Antigravity/Gemini configuration, compile the optimized release binary, and link everything up instantly!

---

## 🧠 The Problem It Solves

Stochastic Large Language Models (LLMs) are highly capable but computationally expensive, slow, and prone to **"Semantic Drift"** when evaluating subjective directives.
1.  **Token Waste**: Asking an LLM to repeatedly perform deterministic tasks (such as code formatting, import sorting, or API scaffolding) drains token quotas unnecessarily.
2.  **Turnaround Latency**: Outbound network requests to upstream LLMs routinely incur multi-second latencies for operations that can run locally in less than 5 milliseconds.
3.  **Intent Ambiguity Drift**: Stochastic models risk over-interpreting or corrupting code when processing subjective prompts like *"improve the code"* or *"modernize the layout"*.

Neuroweave acts as the primary firewall, triaging prompts locally. If an intent is clean and recognized, it executes it with zero API cost and near-instant latency. If the prompt contains subjective language or unknown blast radiuses, it escalates seamlessly to the LLM.

---

## 🛠️ Architecture & Core Components

Neuroweave is engineered around the **Single-Binary Appliance Pattern**, boasting an idle memory footprint of `< 30MB` and cold-boot times under `50ms`.

```mermaid
graph TD
    User([Developer Prompt]) --> Intercept[Agent Client Hook]
    Intercept --> Router[router::evaluate_intent]
    
    subgraph Intent Intersection Gate
        Router --> Drift{Layer 4: Semantic Drift?}
        Drift -->|Subjective Language| ZeroConf[Confidence: 0.0 / Escalate]
        Drift -->|Safe Directives| Lexical[Layer 1: Lexical Match / TaskID]
    end
    
    ZeroConf --> Dispatcher[dispatcher::dispatch]
    Lexical --> Dispatcher
    
    subgraph Actuation Pipeline
        Dispatcher --> Security{Trust Boundary >= 0.90?}
        Security -->|No| LLMEscalation[Escalate to LLM]
        Security -->|Yes| Subroutine[subroutines::BasalTask]
        Subroutine --> Transaction[executor::FileTransaction]
        Transaction --> Rollback{Success?}
        Rollback -->|No| Revert[Abort & Rollback file delta]
        Rollback -->|Yes| Success[resolved_locally: true]
    end
```

### 1. The Intent Intersection Gate (`src/router.rs`)
The triage center. It executes compiled `OnceLock` regex filters. It strictly processes **Layer 4 Semantic Drift** (e.g. checking for danger words like *"modernize"*, *"simplify"*, *"improve"*) first. If drift is detected, it zeroes the confidence and flags it. Otherwise, it proceeds to **Layer 1 Lexical matching** (e.g. mapping `"format"` and `"lint"` to safe task IDs).

### 2. The Confidence Engine & Trust Radius (`src/confidence.rs` & `src/core.rs`)
Defines the `ConfidenceProfile` and `TrustRadius` of tasks. Every local task declares its blast radius parameters—defining constraints on maximum mutable files, permitted AST mutations, cross-module writes, and mandatory rollbacks.

### 3. The Transactional Actuator (`src/executor.rs` & `src/subroutines.rs`)
Guarantees absolute workspace safety. File edits are managed in a transactional context via `FileTransaction`. In the event of an OS-level friction error (such as folder permissions or lock limits), it immediately executes a `rollback()` to the pre-transaction state and escalates the execution back to the LLM with the error trace attached.

### 4. The Dispatcher (`src/dispatcher.rs`)
Enforces strict security trust boundaries. Any interaction falling below `0.90` confidence or yielding active ambiguity flags is blocked from local physical file I/O and routed back safely to standard LLM execution.

---

## 🎯 Trigger Keywords & Triage Rules

To use Neuroweave effectively, it is helpful to know exactly which keywords activate the local bypass engine, and which subjective words will cause the gate to escalate the prompt to the cloud LLM.

### 🟢 Local Bypass Triggers (Muscle Memory)
These keywords immediately map to ultra-fast local subroutines:
* **Code Formatting / Linting (`FormatLint` Subroutine)**: 
  * Trigger words: `format`, `lint` (e.g., *"Quick, format the codebase."*)
* **Boilerplate Scaffolding (`ScaffoldModule` Subroutine)**: 
  * Trigger words: `scaffold`, `init` (e.g., *"Scaffold the new controller."*)
* **Import Optimization (`CleanImports` Subroutine)**: 
  * Trigger words: Both `clean` and `imports` present in the prompt (e.g., *"Clean unused imports."*)

### ⚠️ Cloud Escalation Words (Semantic Drift)
If any of these subjective "danger words" appear in your prompt, Neuroweave automatically **escalates the prompt to the cloud LLM** to prevent local semantic drift and guarantee reasoning safety:
* `improve`
* `refactor`
* `simplify`
* `modernize`
* `clean this up`
* `optimize`
* `architect`
* `design`
* `rework`
* `bulletproof`
* `harden`
* `audit`
* `review`
* `evaluate`
* `fix`
* `debug`
* `resolve`
* `advise`
* `integrate`

*(For example, asking "format this codebase" runs locally in 2ms. Asking "improve and format this codebase" will safely escalate to the cloud LLM for review.)*

---

## 🚀 Execution & Usage

Neuroweave is configured to compile as a standalone static executable with zero external runtimes.

### Build release binary:
```bash
cargo build --release
```

### Run E2E verification test harness:
```bash
target/release/neuroweave.exe
```

### Activate stdio JSON-RPC MCP daemon mode:
```bash
target/release/neuroweave.exe --mcp
```

---

## 📊 Telemetry & Savings Ledger

To track and audit the cost, latency, and energy reductions achieved by Project Neuroweave, the engine operates a self-sustaining, file-based telemetry logging pipeline.

### 🔄 Payload Mechanism
To avoid shell-escaping limitations under complex CLI environments (such as PowerShell quote-stripping), all processes logging savings utilize a transactional file-based mechanism:

1. The caller (the MCP server, pre-turn hooks, or validation scripts) writes the execution output to a temporary JSON file named `last_action_payload.json` in the root workspace folder:
   ```json
   {
       "raw_prompt": "Quick, format the codebase.",
       "task_output": {
           "resolved_locally": true,
           "execution_ms": 2,
           "message": "Deterministic formatting applied.",
           "tokens_saved": 1500,
           "cost_saved_usd": 0.0001125,
           "energy_saved_wh": 0.01
       }
   }
   ```
2. The caller executes the Python tracker script:
   ```bash
   python telemetry_tracker.py
   ```
3. The tracker parses the payload, updates the persistent ledger (`savings_log.json`), and automatically deletes `last_action_payload.json` to keep the workspace clean.

### 📈 Metrics Tracking (`savings_log.json`)
The ledger tracks cumulative historical metrics across the following schema:
* `total_runs`: Total intercepted prompts triaged by Neuroweave.
* `total_bypasses`: Total prompts resolved locally without invoking upstream cognitive APIs.
* `total_tokens_saved`: Total tokens saved by bypassing standard cloud reasoning models (e.g., $0.000000075 per token basis).
* `total_cost_saved_usd`: Total USD saved based on active LLM-Flash-Lite pricing structures.
* `total_energy_saved_wh`: Total estimated energy footprint saved (e.g., 0.01 Wh per local vs cloud inference standard metric).
