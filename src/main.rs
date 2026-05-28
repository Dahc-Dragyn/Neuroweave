mod ipc;
mod core;
mod router;
mod executor;
mod confidence;
mod subroutines;
mod dispatcher;
mod mcp_server;

use tokio::fs;
use ipc::IntentPayload;
use router::evaluate_intent;
use dispatcher::dispatch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"--mcp".to_string()) {
        // Run in MCP server daemon mode
        mcp_server::run_mcp_server().await;
    } else {
        // Run standard End-to-End Validation check
        println!("Initializing Neuroweave Chassis (Phase 5: E2E Validation)...");

        let target_file = "target_workspace.rs";

        // 1. Initialize dummy target workspace file
        println!("\n[E2E Verification] Step 1: Writing dirty file '{}'", target_file);
        fs::write(target_file, "fn main(){println!(\"garbage format\");}").await?;

        // 2. Test Payload A: Legitimate lexical formatting directive
        let payload_a = IntentPayload {
            session_id: "e2e-session-a".to_string(),
            intent_hash: "e2e-hash-a".to_string(),
            raw_prompt: "Quick, format and lint the target workspace.".to_string(),
        };

        println!("\n--- [E2E Test A: Local Bypass Run] ---");
        println!("Prompt: \"{}\"", payload_a.raw_prompt);
        let profile_a = evaluate_intent(&payload_a);
        println!("Confidence: {}, Ambiguity: {:?}", profile_a.confidence_score, profile_a.ambiguity_flags);
        
        let output_a = dispatch(&profile_a, &payload_a).await?;
        println!("Dispatch Output:\n{:#?}", output_a);

        // Read the physical file to prove it was formatted
        let formatted_content = fs::read_to_string(target_file).await?;
        println!("\nPhysical File State after Run A:\n{}", formatted_content);

        // 3. Test Payload B: Subjective prompt that triggers semantic drift
        let payload_b = IntentPayload {
            session_id: "e2e-session-b".to_string(),
            intent_hash: "e2e-hash-b".to_string(),
            raw_prompt: "Format this file and modernize the architecture.".to_string(),
        };

        println!("\n--- [E2E Test B: LLM Escalation Run] ---");
        println!("Prompt: \"{}\"", payload_b.raw_prompt);
        let profile_b = evaluate_intent(&payload_b);
        println!("Confidence: {}, Ambiguity: {:?}", profile_b.confidence_score, profile_b.ambiguity_flags);

        let output_b = dispatch(&profile_b, &payload_b).await?;
        println!("Dispatch Output:\n{:#?}", output_b);

        // 4. Clean up by deleting the workspace file
        println!("\n[E2E Verification] Step 4: Deleting target file '{}'", target_file);
        fs::remove_file(target_file).await?;

        println!("\nNeuroweave End-to-End Verification Pipeline complete.");
    }
    
    Ok(())
}
