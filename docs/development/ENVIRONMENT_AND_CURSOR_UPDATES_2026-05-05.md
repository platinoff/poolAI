# Environment and Cursor updates verification (2026-05-05)

> **Актуальний зріз (2026-07-17):** [`CURSOR_UPDATE_RESEARCH_2026-07-17.md`](./CURSOR_UPDATE_RESEARCH_2026-07-17.md) — Cursor **3.12.17**, service band PH-SVC01…SVC10. Попередній зріз: [`CURSOR_UPDATE_RESEARCH_2026-06-20.md`](./CURSOR_UPDATE_RESEARCH_2026-06-20.md).

## Local software baseline on this machine

- OS: Windows 11 build `26200`
- `rustc`: `1.92.0 (ded5c06cf 2025-12-08)`
- `cargo`: `1.92.0 (344c4567c 2025-10-21)`
- `git`: `2.50.0` (MSYS2 UCRT64; було `2.54.0.windows.1` у старому зрізі)
- `cursor --version`: **3.12.17** (`x64`, `C:\Program Files\cursor\...`)

## Latest Cursor updates reviewed for development workflow

Based on official Cursor changelog pages reviewed during service band 2026-07-17:

- **3.11** (Jul 10): side chats, agent transcript search, redesigned repo pickers, cloud agent conversation hooks.
- **3.10** (Jun 30): Team MCPs in team marketplaces; organization groups.
- **3.9** (Jun 22–29): Customize page (plugins, skills, MCP); Cursor iOS + cloud agents.
- **3.12.x** (Jul 17): Slack multi-repo + plan-before-start (ops, поза repo drain).

## Project-level Cursor rule tuning applied

Added rules:

- `.cursor/rules/cursor-environment-baseline.mdc`
- `.cursor/rules/rust-toolchain-baseline.mdc`
- `.cursor/rules/ci-scripts-maintenance.mdc`

Purpose:

- Keep agent behavior aligned with installed local toolchain and current Cursor behavior.
- Ensure config/script/CI edits include post-update verification and cleanup discipline.

**Service band 2026-07-17:** baseline rule → Cursor 3.12.17; FM §5.16 journal; HANDOFF/README/INDEX zriz.

## Operational recommendation

- Re-run version checks after system package updates:
  - `cursor --version` (Windows path or full `cursor.cmd`)
  - `rustc --version`
  - `cargo --version`
  - `git --version`
- If any major version changes, review `.cursor/rules/*.mdc` and adjust workflow assumptions.
- Product drain unchanged: **`абракадабра`** → PH-S950…S959 band 30.
