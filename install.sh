#!/bin/bash
# install.sh - Automated Installer for Neuroweave (Linux/macOS)
# Enforces zero-friction deployment for any Antigravity developer.

set -e

# Clear screen and show header
clear
echo "=================================================="
echo -e "\033[36m     ⚡ NEUROWEAVE DETACHED BYPASS INSTALLER ⚡   \033[0m"
echo "=================================================="
echo ""

# 1. Environment & Prerequisite Audit
echo -e "\033[33m[1/4] Auditing toolchain prerequisites...\033[0m"

if ! command -v cargo &> /dev/null; then
    echo -e "\033[31m❌ Rust Toolchain ('cargo') not found! Please install Rust from https://rustup.rs/ first.\033[0m"
    exit 1
fi
echo -e "\033[32m  ✓ Rust/Cargo toolchain detected.\033[0m"

if ! command -v python3 &> /dev/null && ! command -v python &> /dev/null; then
    echo -e "\033[31m❌ Python (for telemetry tracker) not found! Please install Python 3.x first.\033[0m"
    exit 1
fi
echo -e "\033[32m  ✓ Python runtime detected.\033[0m"

# Get correct python binary name
PYTHON_BIN="python3"
if ! command -v python3 &> /dev/null; then
    PYTHON_BIN="python"
fi

# 2. Production Build Execution
echo -e "\n\033[33m[2/4] Compiling optimized single-binary appliance...\033[0m"
cargo build --release
echo -e "\033[32m  ✓ Successfully built single-binary target/release/neuroweave\033[0m"

# 3. Locate & Configure Antigravity Config
echo -e "\n\033[33m[3/4] Registering Neuroweave MCP Server...\033[0m"

EXE_PATH="$(pwd)/target/release/neuroweave"
mcpConfigBlock="    \"neuroweave_basal_ganglia\": {
      \"command\": \"$EXE_PATH\",
      \"args\": [\"--mcp\"],
      \"env\": {}
    }"

# Standard paths to scan
POSSIBLE_PATHS=(
    "$HOME/.gemini/antigravity-ide/mcp_config.json"
    "$HOME/.gemini/antigravity-ide/config.json"
    "$HOME/.config/antigravity-ide/mcp_config.json"
)

INJECTED=false
for path in "${POSSIBLE_PATHS[@]}"; do
    if [ -f "$path" ]; then
        echo -e "\033[90m  🔍 Found Antigravity configuration at: $path\033[0m"
        if grep -q "neuroweave_basal_ganglia" "$path"; then
            echo -e "\033[32m  ✓ Neuroweave is already registered in this configuration!\033[0m"
            INJECTED=true
            break
        fi

        # Injection logic: Insert nested inside mcpServers block if it exists
        if grep -q '"mcpServers"\s*:\s*{' "$path"; then
            sed -i.bak "s/\(\"mcpServers\"\s*:\s*{\)/\\1\n$mcpConfigBlock,/" "$path"
        else
            # Generic insert before the last closing brace
            sed -i.bak "s/\(}\s*$\)/,\n$mcpConfigBlock\n\\1/" "$path"
        fi
        echo -e "\033[32m  ✓ Automatically registered Neuroweave as an active MCP server!\033[0m"
        INJECTED=true
        break
    fi
done

if [ "$INJECTED" = false ]; then
    echo -e "\n\033[35m📢 [Manual Configuration Required] 📢\033[0m"
    echo "We could not automatically locate/edit your active mcp_config.json."
    echo "Please add the following block under your 'mcpServers' object manually:"
    echo -e "\033[90m--------------------------------------------------\033[0m"
    echo -e "\033[36m$mcpConfigBlock\033[0m"
    echo -e "\033[90m--------------------------------------------------\033[0m"
fi

# 4. Success Handoff
echo -e "\n\033[33m[4/4] Finalizing setup...\033[0m"
echo -e "\n=================================================="
echo -e "\033[32m   🎉 NEUROWEAVE INSTALLED SUCCESSFULLY! 🎉\033[0m"
echo "=================================================="
echo "  • Daemon Mode   : target/release/neuroweave --mcp"
echo "  • Stats Log     : savings_log.json"
echo -e "  • View Ledger   : $PYTHON_BIN telemetry_tracker.py --stats"
echo -e "==================================================\n"
