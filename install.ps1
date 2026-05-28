# install.ps1 - Automated Installer for Neuroweave (Windows)
# Enforces zero-friction deployment for any Antigravity developer.

$ErrorActionPreference = "Stop"

Clear-Host
Write-Host "==================================================" -ForegroundColor Cyan
Write-Host "     ⚡ NEUROWEAVE DETACHED BYPASS INSTALLER ⚡   " -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan
Write-Host ""

# 1. Environment & Prerequisite Audit
Write-Host "[1/4] Auditing toolchain prerequisites..." -ForegroundColor Yellow

$cargoCheck = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $cargoCheck) {
    Write-Host "❌ Rust Toolchain ('cargo') not found! Please install Rust from https://rustup.rs/ first." -ForegroundColor Red
    Exit 1
}
Write-Host "  ✓ Rust/Cargo toolchain detected." -ForegroundColor Green

$pythonCheck = Get-Command python -ErrorAction SilentlyContinue
if (-not $pythonCheck) {
    Write-Host "❌ Python (for telemetry tracker) not found! Please install Python 3.x first." -ForegroundColor Red
    Exit 1
}
Write-Host "  ✓ Python runtime detected." -ForegroundColor Green

# 2. Production Build Execution
Write-Host "`n[2/4] Compiling optimized single-binary appliance..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Compilation failed! Please inspect the Rust compiler logs above." -ForegroundColor Red
    Exit 1
}
Write-Host "  ✓ Successfully built single-binary target\release\neuroweave.exe" -ForegroundColor Green

# 3. Locate & Configure Antigravity Config
Write-Host "`n[3/4] Registering Neuroweave MCP Server..." -ForegroundColor Yellow

$exePath = (Get-Item "target/release/neuroweave.exe").FullName.Replace("\", "\\")
$mcpConfigBlock = @"
    "neuroweave_basal_ganglia": {
      "command": "$exePath",
      "args": ["--mcp"],
      "env": {}
    }
"@

# Standard paths to scan
$userProfile = $env:USERPROFILE
$possiblePaths = @(
    "$userProfile\.gemini\antigravity-ide\mcp_config.json",
    "$userProfile\.gemini\antigravity-ide\config.json",
    "$env:APPDATA\antigravity-ide\mcp_config.json"
)

$injected = $false
foreach ($path in $possiblePaths) {
    if (Test-Path $path) {
        Write-Host "  🔍 Found Antigravity configuration at: $path" -ForegroundColor Gray
        try {
            $content = Get-Content $path -Raw
            if ($content -match "neuroweave_basal_ganglia") {
                Write-Host "  ✓ Neuroweave is already registered in this configuration!" -ForegroundColor Green
                $injected = $true
                break
            }
            
            # Injection logic: Insert nested inside mcpServers block if it exists
            if ($content -match '"mcpServers"\s*:\s*\{') {
                $content = $content -replace '("mcpServers"\s*:\s*\{)', "`$1`n$mcpConfigBlock,"
            } else {
                # Generic insert before the last closing brace
                $content = $content -replace '(\}\s*$)', ",`n$mcpConfigBlock`n`$1"
            }
            Set-Content $path $content
            Write-Host "  ✓ Automatically registered Neuroweave as an active MCP server!" -ForegroundColor Green
            $injected = $true
            break
        } catch {
            Write-Host "  ⚠️ Could not modify config automatically: $_. Freezing registration." -ForegroundColor Yellow
        }
    }
}

if (-not $injected) {
    Write-Host "`n📢 [Manual Configuration Required] 📢" -ForegroundColor Magenta
    Write-Host "We could not automatically locate/edit your active mcp_config.json."
    Write-Host "Please add the following block under your 'mcpServers' object manually:"
    Write-Host "--------------------------------------------------" -ForegroundColor DarkGray
    Write-Host $mcpConfigBlock -ForegroundColor Cyan
    Write-Host "--------------------------------------------------" -ForegroundColor DarkGray
}

# 4. Success Handoff
Write-Host "`n[4/4] Finalizing setup..." -ForegroundColor Yellow
Write-Host "`n==================================================" -ForegroundColor Green
Write-Host "   🎉 NEUROWEAVE INSTALLED SUCCESSFULLY! 🎉" -ForegroundColor Green
Write-Host "==================================================" -ForegroundColor Green
Write-Host "  • Daemon Mode   : target\release\neuroweave.exe --mcp"
Write-Host "  • Stats Log     : savings_log.json"
Write-Host "  • View Ledger   : python telemetry_tracker.py --stats"
Write-Host "==================================================`n"
