import sys
import json
import os
import time

LOG_FILE = "savings_log.json"

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
    print(f"   Bypass Status: {resolved}")
    print(f"   Tokens Saved: {tokens if resolved else 0} | Cost Saved: ${cost if resolved else 0.0:.6f} | Energy Saved: {energy if resolved else 0.0} Wh")

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python telemetry_tracker.py '<raw_prompt>' '<json_task_output>'")
        sys.exit(1)
        
    prompt_arg = sys.argv[1]
    task_output_arg = sys.argv[2]
    
    try:
        # Clean Windows shell quote escaping artifacts
        cleaned_json = task_output_arg.strip("'\"").replace('\\"', '"')
        if "'" in cleaned_json and '"' not in cleaned_json:
            cleaned_json = cleaned_json.replace("'", '"')
            
        task_out = json.loads(cleaned_json)
        update_savings(task_out, prompt_arg)
    except Exception as e:
        print(f"❌ Failed to parse telemetry args: {e}")
        print(f"   Received raw arg: {task_output_arg}")
