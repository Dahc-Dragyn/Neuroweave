import sys
import json
import os
import time

LOG_FILE = "savings_log.json"
PAYLOAD_FILE = "last_action_payload.json"

def init_log():
    if not os.path.exists(LOG_FILE):
        initial_data = {
            "total_runs": 0,
            "total_bypasses": 0,
            "total_tokens_saved": 0,
            "total_cost_saved_usd": 0.0,
            "total_energy_saved_wh": 0.0,
            "runs": []
        }
        with open(LOG_FILE, "w", encoding="utf-8") as f:
            json.dump(initial_data, f, indent=4)

def update_savings(task_output, raw_prompt):
    init_log()
    
    with open(LOG_FILE, "r", encoding="utf-8") as f:
        log_data = json.load(f)
        
    resolved = task_output.get("resolved_locally", False)
    tokens = task_output.get("tokens_saved", 0)
    cost = task_output.get("cost_saved_usd", 0.0)
    energy = task_output.get("energy_saved_wh", 0.0)
    execution_ms = task_output.get("execution_ms", 0)
    
    log_data["total_runs"] += 1
    if resolved:
        log_data["total_bypasses"] += 1
        log_data["total_tokens_saved"] += tokens
        log_data["total_cost_saved_usd"] += cost
        log_data["total_energy_saved_wh"] += energy
        
    run_entry = {
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()), 
        "prompt": raw_prompt,
        "resolved_locally": resolved,
        "execution_ms": execution_ms,
        "tokens_saved": tokens if resolved else 0,
        "cost_saved_usd": cost if resolved else 0.0,
        "energy_saved_wh": energy if resolved else 0.0
    }
    
    log_data["runs"].append(run_entry)
    
    with open(LOG_FILE, "w", encoding="utf-8") as f:
        json.dump(log_data, f, indent=4)
        
    print("📈 Telemetry Log Updated Successfully:")
    print(f"   Prompt: {raw_prompt}")
    print(f"   Bypass Status: {resolved}")
    print(f"   Tokens Saved: {tokens if resolved else 0} | Cost Saved: ${cost if resolved else 0.0:.6f} | Energy Saved: {energy if resolved else 0.0} Wh")

def print_dashboard():
    init_log()
    if not os.path.exists(LOG_FILE):
        print("📉 No telemetry log found. Run some bypass tasks first!")
        return
        
    with open(LOG_FILE, "r", encoding="utf-8") as f:
        data = json.load(f)
        
    runs = data.get("total_runs", 0)
    bypasses = data.get("total_bypasses", 0)
    rate = (bypasses / runs * 100) if runs > 0 else 0.0
    
    print("\n" + "="*50)
    print("     ⚡ NEUROWEAVE BASAL GANGLIA SAVINGS LEDGER ⚡")
    print("="*50)
    print(f"  Total Prompts Intercepted : {runs}")
    print(f"  Total Local Bypasses      : {bypasses}")
    print(f"  Local Bypass Efficiency   : {rate:.1f}%")
    print("-"*50)
    print("  💎 Cumulative Resource Savings:")
    print(f"    • Cloud Tokens Saved    : {data.get('total_tokens_saved', 0):,}")
    print(f"    • Net Cost Saved (USD)  : ${data.get('total_cost_saved_usd', 0.0):.6f}")
    print(f"    • Hardware Energy Saved : {data.get('total_energy_saved_wh', 0.0):.4f} Wh")
    print("="*50 + "\n")

if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] in ("--stats", "-s"):
        print_dashboard()
        sys.exit(0)
        
    if not os.path.exists(PAYLOAD_FILE):
        print(f"❌ Error: Payload file '{PAYLOAD_FILE}' not found in the current directory.")
        print("💡 To view cumulative savings, run: python telemetry_tracker.py --stats")
        sys.exit(1)
        
    try:
        with open(PAYLOAD_FILE, "r", encoding="utf-8") as f:
            payload = json.load(f)
            
        # Support either flat structure or nested task_output
        raw_prompt = payload.get("raw_prompt", "Unknown Action")
        
        if "task_output" in payload:
            task_output = payload["task_output"]
        else:
            task_output = payload
            
        update_savings(task_output, raw_prompt)
        
        # Delete the payload file after successful update
        os.remove(PAYLOAD_FILE)
        print(f"🗑️ Cleaned up '{PAYLOAD_FILE}'.")
        
    except Exception as e:
        print(f"❌ Failed to process telemetry payload: {e}")
        sys.exit(1)
