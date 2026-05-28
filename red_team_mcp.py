import subprocess
import json
import os
import sys

# Define our test matrix: 3 distinct payloads
payloads = [
    {
        "name": "Payload 1: The Happy Path",
        "prompt": "Quick, format and lint the target workspace.",
        "expected_resolved": True
    },
    {
        "name": "Payload 2: Semantic Drift Trap",
        "prompt": "Format this file and optimize the architecture.",
        "expected_resolved": False
    },
    {
        "name": "Payload 3: Out-of-Bounds / Complete Cache Miss",
        "prompt": "Draft a new backend schema for the TinyTruce simulation.",
        "expected_resolved": False
    }
]

exe_path = os.path.join(os.getcwd(), "target", "release", "neuroweave.exe")

if not os.path.exists(exe_path):
    print(f"❌ Error: Executable not found at {exe_path}")
    sys.exit(1)

print("🚀 Activating Neuroweave Continuous Stdio Stream...")

try:
    # Open the process ONCE. We will pipe multiple commands into this single instance.
    process = subprocess.Popen(
        [exe_path, "--mcp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE, # We won't block on stderr to avoid deadlocks here, just reading stdout
        text=True,
        bufsize=1
    )
    
    for i, test in enumerate(payloads):
        print(f"\n--- Testing {test['name']} ---")
        
        mcp_tool_call = {
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "name": "neuroweave_intercept",
                "arguments": {
                    "session_id": f"red-team-session-{i}",
                    "intent_hash": f"hash_mock_{i}",
                    "raw_prompt": test["prompt"]
                }
            },
            "id": i + 1
        }
        
        input_data = json.dumps(mcp_tool_call) + "\n"
        process.stdin.write(input_data)
        process.stdin.flush()
        
        # Read the single line response
        response_line = process.stdout.readline()
        
        if not response_line:
            print("❌ Error: Server dropped the connection unexpectedly.")
            break
            
        try:
            response_json = json.loads(response_line)
            # Extract the embedded JSON string from the MCP response
            content_str = response_json["result"]["content"][0]["text"]
            task_output = json.loads(content_str)
            
            actual_resolved = task_output.get("resolved_locally")
            
            if actual_resolved == test["expected_resolved"]:
                print(f"✅ PASS: resolved_locally is {actual_resolved} as expected.")
                print(f"   Message: {task_output.get('message')}")
            else:
                print(f"❌ FAIL: Expected {test['expected_resolved']} but got {actual_resolved}.")
                print(f"   Raw Output: {task_output}")
                
        except Exception as e:
            print(f"❌ Parse Error on response: {e}")
            print(f"Raw Line: {response_line}")

    print("\n🏁 Red Team Matrix Complete. Terminating Server.")
    process.terminate()

except Exception as e:
    print(f"❌ Critical Test Failure: {e}")
    if process:
        process.kill()
