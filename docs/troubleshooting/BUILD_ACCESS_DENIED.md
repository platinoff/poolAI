# Fix: Access Denied Error During Build

## Problem

When running `cargo build` or `cargo run`, you may encounter:

```
error: failed to remove file `S:\rust\poolAI\target\release\poolai.exe`
Caused by:
  Access is denied. (os error 5)
```

## Cause

The executable file is locked by a running process. This typically happens when:
- A previous instance of `poolai.exe` or `poolai-worker.exe` is still running
- An antivirus or security tool is scanning the file
- Another process has the file open

## Solution

### Step 1: Stop Running Processes

In PowerShell:
```powershell
# Stop all poolai processes
Stop-Process -Name "poolai" -Force -ErrorAction SilentlyContinue
Stop-Process -Name "poolai-worker" -Force -ErrorAction SilentlyContinue
```

Or check what's running:
```powershell
Get-Process | Where-Object { $_.ProcessName -like "*poolai*" }
```

### Step 2: Clean Build Directory

```powershell
cargo clean
```

This removes all build artifacts and forces a fresh build.

### Step 3: Rebuild

```powershell
cargo build --release --features enterprise
```

## Alternative: Manual File Removal

If `cargo clean` doesn't work, manually remove the locked files:

```powershell
# Remove locked executables
Remove-Item "target\release\poolai.exe" -Force -ErrorAction SilentlyContinue
Remove-Item "target\release\poolai-worker.exe" -Force -ErrorAction SilentlyContinue

# Then rebuild
cargo build --release --features enterprise
```

## Prevention

1. Always stop the application gracefully before rebuilding
2. Use `Ctrl+C` to stop running processes
3. Check for background processes before building

## Related Issues

- If you see `dlltool.exe: program not found`, see [DLLTOOL_FIX.md](./DLLTOOL_FIX.md)
- Ensure MSYS2 `ucrt64\bin` is in your PATH
