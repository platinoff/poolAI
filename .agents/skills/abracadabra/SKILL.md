---
name: abracadabra
description: >-
  Trigger word «абракадабра» starts a VDT drain session. FIRST ask the owner to
  choose the project (poolai | gsv), THEN run project-scan → drain → one commit
  + push. Use when the owner literally writes «абракадабра» in a new session
  (Cursor or OpenCode).
metadata:
  audience: poolai-vdt
  clients: cursor-opencode
---

# «абракадабра» — VDT drain session

Works the same in **Cursor** and **OpenCode**. Canon lives here
(`.agents/skills/abracadabra/`); client copies under `.cursor/skills/` and
`.opencode/skills/` must stay identical.

## Step 0 — Choose the project (ALWAYS first)

When the owner writes `абракадабра`, **before anything else** ask which project
to drain. Use the host question UI (one click, two options):

- **Cursor:** `AskQuestion`
- **OpenCode:** `question`

| Option | Project | Canon docs | Flow |
|--------|---------|-----------|------|
| **poolai** | PoolAI (repo root, `src/`, `tests/`) | `docs/development/NEXT_SESSION_PROMPT.md`, `docs/catalog/FUNCTION_MANAGEMENT.md` §5.12 | S0 disk → project scan → drain band (FM §5.12) → vision close → test → one commit + push |
| **gsv** | GSV (`GSV/` — separate Rust-first project) | `GSV/docs/NEXT_SESSION_PROMPT.md`, `GSV/docs/GSV_ROLES.md`, `GSV/docs/gsv/GSV_TECH_ROADMAP.md` | S0 disk (GSV target) → project scan (warnings first) → drain next band (FM §5.102 / GSV roadmap) → Speeds + Rust panel → vision-sync → one commit + push |

## poolai flow

1. S0 disk: `df -h /s` + `bash scripts/check_target_disk.sh` → `cargo clean` if needed → `git fetch` → HANDOFF → FM §1–§5.1 → NEXT_SESSION → `poolai-vision-sync --check` ok.
2. Project scan (if §5.12 < 10): warnings/diagnostics first (`rust_diagnostics.json`, clippy warnings, compile errors) → concept → FM §5.1 → architect → roadmaps → gaps → code → 10 PH-S* into §5.12.
3. Drain all open PH-S* (no mid-drain push).
4. Vision close: FM §5.12 ✅ + HANDOFF + NEXT → one `poolai-vision-sync` → rev from manifest → `--check`.
5. Test: one `cargo fmt --all` → one `cargo test-ci` → `record-test-ci-speed.sh` + `record-rust-diagnostics.sh`.
6. Git (end of session): one commit → `git push origin main` + summary in chat.

## gsv flow

1. S0 disk (GSV has its own `target/`): disk check + `git fetch` + GSV HANDOFF + FM §5.102.
2. Project scan: warnings/diagnostics first (`poolai-rust-diagnostics --print` covers whole repo incl. GSV, clippy) → `GSV/docs/gsv/GSV_TECH_ROADMAP.md` unchecked rows → FM §5.102 → concept `GSV/docs/gsv/` → gaps → 10 PH-S* into FM §5.102 (GSV band numbering).
3. Drain next band (≤10 open PH-S*; no mid-drain push).
4. Vision close: FM §5.102 ✅ + GSV HANDOFF + GSV NEXT → one `poolai-vision-sync` → rev from manifest → `--check`.
5. Test: GSV test flow (`cargo test` in `GSV/` — stop `gsv-server` first) → Speeds + Rust panel.
6. Git (end of session): one commit → `git push origin main` + summary.

## Hard rules (both projects)

- **No** `git add -A`; stage only sprint files.
- **No** push mid-drain / mid-scan; push + summary always last step.
- **No** parallel `cargo` (file lock).
- **Never** stage: `.env*`, `*.pem`/`*.key`, `certs/*.pem`, `data/audit/*` (except `.gitkeep`), `comitmsg/*.txt`.
- Vision rev read from `GSV/docs/vision/manifest.json` after `poolai-vision-sync`.
- Warnings >0 or errors >0 fixable → 1–3 PH-S* at the top of the band (Source: `rust_diagnostics` / lint code).
- Shell is **MSYS2 bash**, not PowerShell: `C:\msys64\usr\bin\bash.exe -lc '…'`.

## See also

- `AGENTS.md` § «Тригер абракадабра» (OpenCode always-on canon)
- `.cursor/rules/poolai-session-iteration.mdc` § «Тригер абракадабра» (Cursor)
- `.cursor/rules/virtual-development-team.mdc` (always-on VDT)
- poolai: `docs/development/NEXT_SESSION_PROMPT.md`, `docs/catalog/FUNCTION_MANAGEMENT.md`
- gsv: `GSV/docs/NEXT_SESSION_PROMPT.md`, `GSV/docs/GSV_ROLES.md`, `GSV/docs/gsv/GSV_TECH_ROADMAP.md`
