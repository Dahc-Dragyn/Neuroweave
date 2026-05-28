import subprocess
import json
import sys
import os

mcp_tool_call = {
    "jsonrpc": "2.0",
    "method": "tools/call",
    "params": {
        "name": "neuroweave_intercept",
        "arguments": {
            "session_id": "test-session-999",
            "intent_hash": "6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b",
            "raw_prompt": "Quick, format and lint the target workspace."
        }
    },
    "id": 1
}

print("🚀 Activating Neuroweave MCP Server Daemon via Stdio...")

exe_path = os.path.join(os.getcwd(), "target", "release", "neuroweave.exe")

if not os.path.exists(exe_path):
    print(f"❌ Error: Executable not found at {exe_path}")
    print("Ensure you have run `cargo build --release` first.")
    sys.exit(1)

try:
    process = subprocess.Popen(
        [exe_path, "--mcp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE
    )
    
    input_data = json.dumps(mcp_tool_call) + "\n"
    print("📥 Sending Live Tool Call Payload...")
    
    stdout_bytes, stderr_bytes = process.communicate(input=input_data.encode('utf-8'), timeout=5)
    
    print("\n🖥️  [Raw Server Stdio Response]:")
    print(stdout_bytes.decode('utf-8', errors='ignore'))
    
    stderr = stderr_bytes.decode('utf-8', errors='ignore')
    if stderr and "error" in stderr.lower():
        print(f"⚠️ Compiler/Runtime Stderr Output:\n{stderr}")

except subprocess.TimeoutExpired as e:
    print("❌ Error: Timeout reached. Process hung.")
    # process.communicate() attaches the captured output to the exception object on timeout
    if e.stdout:
        print(f"\n[Swallowed Stdout]:\n{e.stdout.decode('utf-8', errors='ignore')}")
    if e.stderr:
        print(f"\n[Swallowed Stderr / Telemetry]:\n{e.stderr.decode('utf-8', errors='ignore')}")
    process.kill()
except Exception as e:
    print(f"❌ Failed to execute test script: {e}")
