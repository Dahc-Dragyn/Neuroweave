import subprocess
import json
import os
import stat
import sys
import time

exe_path = os.path.join(os.getcwd(), "target", "release", "neuroweave.exe")
target_file = os.path.join(os.getcwd(), "target_workspace.rs")

if not os.path.exists(exe_path):
    print(f"❌ Error: Executable not found at {exe_path}")
    sys.exit(1)

# Ensure a clean slate
if os.path.exists(target_file):
    os.chmod(target_file, stat.S_IWRITE)
    os.remove(target_file)

# Create the dummy file with UTF-8
with open(target_file, "w", encoding="utf-8") as f:
    f.write("fn main(){println!(\"chaos test\");}")

print("🚀 Activating Neuroweave Continuous Chaos Stream...")

try:
    process = subprocess.Popen(
        [exe_path, "--mcp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        bufsize=1
    )
    
    def send_tool_call(session_id):
        mcp_tool_call = {
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "name": "neuroweave_intercept",
                "arguments": {
                    "session_id": session_id,
                    "intent_hash": "hash_chaos",
                    "raw_prompt": "Quick, format and lint the target workspace."
                }
            },
            "id": 1
        }
        process.stdin.write(json.dumps(mcp_tool_call) + "\n")
        process.stdin.flush()
        
        response_line = process.stdout.readline()
        if not response_line:
            return None
        return json.loads(response_line)

    # --- TEST 1: The Baseline Success ---
    print("\n--- Phase 1: Baseline Format (File is Writeable) ---")
    resp1 = send_tool_call("chaos-1")
    if resp1 and "result" in resp1:
        content = json.loads(resp1["result"]["content"][0]["text"])
        print(f"✅ PASS: Server responded. resolved_locally: {content.get('resolved_locally')}")
    else:
        print(f"❌ FAIL: Unexpected response: {resp1}")

    # --- TEST 2: The Physical Lock ---
    print("\n--- Phase 2: OS Friction (File is locked/Read-Only) ---")
    os.chmod(target_file, stat.S_IREAD) # Lock the file
    
    resp2 = send_tool_call("chaos-2")
    if resp2 and "error" in resp2:
        print(f"✅ PASS: Server caught the OS error gracefully and did not crash.")
        print(f"   Error Message: {resp2['error']['message']}")
    else:
        print(f"❌ FAIL: Server either crashed or failed to return an error object. Output: {resp2}")

    # --- TEST 3: The Persistence Check ---
    print("\n--- Phase 3: Stream Recovery (File Unlocked) ---")
    os.chmod(target_file, stat.S_IWRITE) # Unlock the file
    
    resp3 = send_tool_call("chaos-3")
    if resp3 and "result" in resp3:
        content = json.loads(resp3["result"]["content"][0]["text"])
        print(f"✅ PASS: Server is still alive and successfully processed the next command!")
    else:
        print(f"❌ FAIL: Server dropped the connection or failed to recover. Output: {resp3}")

    print("\n🏁 Chaos Matrix Complete. Terminating Server.")
    process.terminate()

except Exception as e:
    print(f"❌ Critical Test Failure: {e}")
    if process:
        process.kill()
finally:
    if os.path.exists(target_file):
        os.chmod(target_file, stat.S_IWRITE)
        os.remove(target_file)
