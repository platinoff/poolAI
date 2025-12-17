# PR: Stage 3 — UI Read-only + Runtime Hardening

**Branch**: `stage3/ui-readonly-runtime-hardening`  
**Type**: Stage 3 milestone / stabilization  

## Summary (Rust architect)
- Delivered a **read-only operations dashboard** under `/ui` (auto-refresh, no writes).
- Hardened runtime worker bootstrap by introducing a real `poolai-worker` binary and removing PATH dependency.
- Improved Windows-gnu stability by avoiding native compression deps (zstd/bzip2) where possible.
- Synchronized plan/concepts with the real state of the code.

## What changed
- **UI**: `/ui`, `/ui/status`, `/ui/health`, `/ui/metrics`, `/ui/workers`, `/ui/libs`, `/ui/vm`, `/ui/raid`
  - fixed `/ui/` → `/ui` redirect to avoid trailing-slash 404.
- **Runtime**:
  - added `src/bin/poolai-worker.rs`
  - worker spawns sibling binary next to `poolai.exe` (fallback + warning)
  - `default-run = "poolai"` so `cargo run` stays simple
  - `kill_on_drop(true)` to prevent orphan processes / Windows exe locking.

## Verification
- `cargo check`
- `cargo run` then open `http://localhost:8080/ui`

## Next step (per plan)
**Libs production-min**: atomic install + manifest + constraints + minimal tests.


