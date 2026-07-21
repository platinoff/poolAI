# Environment and Cursor updates verification (2026-05-05)

> **Актуальний зріз (2026-07-21):** [`CURSOR_UPDATE_RESEARCH_2026-07-21.md`](./CURSOR_UPDATE_RESEARCH_2026-07-21.md) — Cursor **3.12.29**, service band PH-SVC11…SVC20. Попередній: [`CURSOR_UPDATE_RESEARCH_2026-07-17.md`](./CURSOR_UPDATE_RESEARCH_2026-07-17.md) (3.12.17).

## Local software baseline on this machine

- OS: Windows 11 build `26200`
- `rustc`: `1.92.0 (ded5c06cf 2025-12-08)`
- `cargo`: `1.92.0 (344c4567c 2025-10-21)`
- `git`: `2.50.0` (MSYS2 UCRT64; було `2.54.0.windows.1` у старому зрізі)
- Cursor desktop: **3.12.29** (`C:\Program Files\cursor\resources\app\package.json`)

## Latest Cursor updates reviewed for development workflow

Based on official Cursor changelog + local package check during service band 2026-07-21:

- **Local desktop:** **3.12.29** (patch from 3.12.17) — build ID, not a new public agent-feature dump.
- **3.11** (Jul 10): latest **numbered** IDE release — side chats, agent transcript search, redesigned repo pickers, cloud agent conversation hooks.
- **3.10** (Jun 30): Team MCPs in team marketplaces; organization groups.
- **3.9** (Jun 22–29): Customize page (plugins, skills, MCP); Cursor iOS + cloud agents.
- **Jul 17 changelog:** Slack multi-repo + plan-before-start (ops, поза repo drain).

## Project-level Cursor rule tuning applied

Added rules:

- `.cursor/rules/cursor-environment-baseline.mdc`
- `.cursor/rules/rust-toolchain-baseline.mdc`
- `.cursor/rules/ci-scripts-maintenance.mdc`

Purpose:

- Keep agent behavior aligned with installed local toolchain and current Cursor behavior.
- Ensure config/script/CI edits include post-update verification and cleanup discipline.

**Service band 2026-07-21:** baseline rule → Cursor 3.12.29; FM §5.16 PH-SVC11…SVC20; HANDOFF/README/INDEX zriz.

## Operational recommendation

- Re-run version checks after system package updates:
  - Cursor: `C:\Program Files\cursor\resources\app\package.json` `version` (or `cursor.cmd --version` outside MSYS2)
  - `rustc --version`
  - `cargo --version`
  - `git --version`
- If any major version changes, review `.cursor/rules/*.mdc` and adjust workflow assumptions.
- Product drain unchanged: **`абракадабра`** → band 59 **PH-S1229…S1238**.
