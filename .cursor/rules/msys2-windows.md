# MSYS2 & Windows Development Environment

## 🪟 MSYS2 & Windows Development Environment

### ⚠️ CRITICAL: MSYS2 Configuration for Rust Development

**Project uses MSYS2 UCRT64** as the primary development environment on Windows.

### Terminal Configuration
- **Primary Terminal**: MSYS2 UCRT64 bash (`C:\msys64\usr\bin\bash.exe`)
- **Terminal Profile**: "bash (MSYS2)" configured in `.vscode/settings.json`
- **Environment Variables**: 
  - `MSYSTEM=UCRT64`
  - `CHERE_INVOKING=1`
  - PATH includes: `C:\msys64\ucrt64\bin;C:\msys64\usr\bin`

### When to Use MSYS2 vs PowerShell
1. **Use MSYS2 bash** for:
   - Building with `enterprise,https,jwt` features (requires `gcc.exe`)
   - Running cargo commands that need native C/C++ libraries
   - Executing shell scripts (`.sh` files)
   - When encountering `gcc.exe not found` errors

2. **Use PowerShell** for:
   - Git operations (may have authentication issues with MSYS2)
   - Quick checks (`cargo check`, `cargo fmt`, `cargo clippy`)
   - When MSYS2 causes PATH conflicts

### Building with HTTPS/JWT Features
**CRITICAL**: When building with `enterprise,https,jwt` features:
1. **Option A**: Use MSYS2 bash terminal
   ```bash
   # In MSYS2 bash
   ./scripts/build-with-https.sh
   ```

2. **Option B**: Set environment variables in PowerShell
   ```powershell
   $env:PATH = "C:\msys64\ucrt64\bin;C:\msys64\usr\bin;$env:PATH"
   $env:CC = "gcc"
   $env:CXX = "g++"
   cargo build --features enterprise,https,jwt
   ```

### Git Operations with MSYS2
**IMPORTANT**: MSYS2 may cause git authentication issues:
- **If `git push` fails**: Remove MSYS2 from PATH temporarily
  ```powershell
   # Save original PATH
   $originalPath = $env:PATH
   # Remove MSYS2 paths
   $env:PATH = ($env:PATH -split ';' | Where-Object { $_ -notlike '*msys64*' }) -join ';'
   # Run git push
   git push
   # Restore PATH
   $env:PATH = $originalPath
   ```

- **Better approach**: Use PowerShell for all git operations when possible

### Directory Structure for MSYS2
- **Scripts**: All in `scripts/` directory (use relative paths: `./scripts/script.sh`)
- **Documentation**: All in `docs/` directory
- **MSYS2 paths**: Use forward slashes `/` even on Windows when in MSYS2 bash

### Common Issues & Solutions
1. **`gcc.exe not found`**: 
   - Ensure MSYS2 is in PATH: `export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"`
   - Or use `scripts/build-with-https.sh`

2. **Git authentication fails**:
   - Use PowerShell instead of MSYS2 bash for git commands
   - Or remove MSYS2 from PATH temporarily

3. **Terminal not using MSYS2**:
   - Check `.vscode/settings.json` has correct terminal profile
   - Manually select "bash (MSYS2)" profile in VS Code/Cursor
