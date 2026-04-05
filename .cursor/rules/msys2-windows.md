# MSYS2 & Windows Development Environment

## ⚠️ CRITICAL: MSYS2 bash only — no PowerShell, no cmd

**Terminal**: `C:\msys64\usr\bin\bash.exe` (MSYS2 UCRT64) for **all** dev work: `cargo`, `git`, scripts, and any verification you run locally.  
CI on GitHub uses its own runner environment; locally, stay in MSYS2 bash (see `rust-architect.md`, `git-push.md`).

### Windows 11 (24H2+ / builds 26100+; e.g. 26200)

- **Windows Defender** can slow or lock files under `target/` — optional exclusion for the repo `target` folder (or whole repo) if builds/tests stall.
- **Long paths**: if `cargo` reports path length issues, enable long paths in Windows or keep the repo on a shorter path (e.g. `S:\rust\poolAI` is fine).
- **`rust-toolchain.toml`** pins **`x86_64-pc-windows-gnu`**; the active **host** may still be MSVC (`rustc -vV` → `host: …-msvc`). For GNU linking you need MSYS2 `gcc`/`x86_64-w64-mingw32-gcc` on `PATH` (see `.cargo/config.toml`). If the host is MSVC-only, align with `rustup override` / installed toolchains.

### Project patches (конфіг)

- **`rust-toolchain.toml`** — Rust 1.92.0, `x86_64-pc-windows-gnu`, rustfmt, clippy
- **`.cursor`** — Cursor rules, commands, `hooks.json` (`rules/`, `commands/`); без `.ps1`
- **`.vscode`** — Terminal profile "bash (MSYS2)", Rust Analyzer
- **`scripts/`** — Bash scripts only (`.sh`). Run via MSYS2 bash.

### Terminal setup

- **Profile**: "bash (MSYS2)" in `.vscode/settings.json`
- **Path**: `C:\msys64\usr\bin\bash.exe` with `-l`
- **Env**: `MSYSTEM=UCRT64`, `CHERE_INVOKING=1`  
- **PATH**: `C:\msys64\ucrt64\bin;C:\msys64\usr\bin` (and cargo)

### Always use MSYS2 bash for

- All `cargo` commands (check, build, test, fmt, clippy)
- All `git` operations (copy-paste block in `.cursor/commands/git-push.md`; no `.sh` wrappers for git)
- Running `scripts/*.sh` when needed

### Tooling Checks (rustc/cargo/clippy/cl)

- Version sanity (run once per session if something fails):
  - `rustc --version`
  - `cargo --version`
  - `cargo clippy --version`
- MSRV note (cloud-sdk/AWS SDK): `cloud_mock_integration` requires `rustc >= 1.88`. If your `rustc --version` is lower (e.g. 1.87.0), install `rustup`+the toolchain from `rust-toolchain.toml` or upgrade the MSYS2 Rust package before running cloud tests.
- MSVC `cl.exe` note:
  - Default toolchain is GNU (`x86_64-pc-windows-gnu`); `cl.exe` is usually not needed.
  - If you switch to an MSVC host/toolchain, use a Visual Studio / MSVC environment where `cl` is on `PATH`, then run `cargo` from that same environment (still avoid ad-hoc PowerShell/cmd-only workflows for this repo).

### Paths in bash

- Use `/` and MSYS2 paths: `/s/rust/poolAI` for `S:\rust\poolAI`
- Scripts: `./scripts/script.sh` or `bash scripts/script.sh`

### Disk space (`target/`)

- Full debug builds can use **many GB**. To reclaim space: `cargo clean` (then the next build is cold).
- Optional: set `CARGO_TARGET_DIR` to another drive if `S:` is tight.

### Common issues

1. **`gcc` / `dlltool` not found**: `export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"`
2. **Wrong terminal**: Select "bash (MSYS2)" in Cursor/VS Code terminal dropdown.
3. **Git push / auth**: Use MSYS2 bash; ensure remote is HTTPS or SSH as preferred.
