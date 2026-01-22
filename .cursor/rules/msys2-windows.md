# MSYS2 & Windows Development Environment

## ⚠️ CRITICAL: MSYS2 Only — No PowerShell, No cmd

**Terminal**: `C:\msys64\usr\bin\bash.exe` (MSYS2 UCRT64)  
**Do NOT use**: PowerShell, cmd.exe for development or git.

### Project patches (конфіг)

- **`rust-toolchain.toml`** — Rust 1.92.0, `x86_64-pc-windows-gnu`, rustfmt, clippy
- **`.cursor`** — Cursor rules, commands, hooks (`rules/`, `commands/`)
- **`.vscode`** — Terminal profile "bash (MSYS2)", Rust Analyzer
- **`scripts/`** — Bash scripts only (`.sh`). Run via MSYS2 bash.

### Terminal setup

- **Profile**: "bash (MSYS2)" in `.vscode/settings.json`
- **Path**: `C:\msys64\usr\bin\bash.exe` with `-l`
- **Env**: `MSYSTEM=UCRT64`, `CHERE_INVOKING=1`  
- **PATH**: `C:\msys64\ucrt64\bin;C:\msys64\usr\bin` (and cargo)

### Always use MSYS2 bash for

- All `cargo` commands (check, build, test, fmt, clippy)
- All `git` operations: **copy-paste блок** з `.cursor/commands/git-push.md` (без .sh)
- Running `scripts/*.sh` when needed (optional for git)
- Any dev task

### Paths in bash

- Use `/` and MSYS2 paths: `/s/rust/poolAI` for `S:\rust\poolAI`
- Scripts: `./scripts/script.sh` or `bash scripts/script.sh`

### Common issues

1. **`gcc` / `dlltool` not found**: `export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"`
2. **Wrong terminal**: Select "bash (MSYS2)" in Cursor/VS Code terminal dropdown.
3. **Git push / auth**: Use MSYS2 bash; ensure remote is HTTPS or SSH as preferred.
