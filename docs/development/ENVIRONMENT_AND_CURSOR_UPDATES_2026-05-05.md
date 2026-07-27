# Environment and Cursor updates verification (2026-05-05)

> **Актуальний зріз (2026-07-27):** Cursor desktop [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md) (**3.13.21**, Auto-review, vision Speeds/eye verify) · service PH-SVC75…SVC84. GH tokens: [`GITHUB_APP_INSTALLATION_TOKENS_2026-07-25.md`](./GITHUB_APP_INSTALLATION_TOKENS_2026-07-25.md) · PH-SVC65…SVC74. Попередній Cursor: [`CURSOR_UPDATE_RESEARCH_2026-07-24.md`](./CURSOR_UPDATE_RESEARCH_2026-07-24.md) (3.13.10).

## Local software baseline on this machine

- OS: Windows 11 build `26200`
- `rustc`: `1.92.0 (ded5c06cf 2025-12-08)` (rustup GNU; MSYS2 pacman may show 1.87 — ignore for VDT)
- `cargo`: `1.92.0 (344c4567c 2025-10-21)`
- `git`: `2.50.0` (MSYS2 UCRT64; було `2.54.0.windows.1` у старому зрізі)
- Cursor desktop: **3.13.21** (`C:\Program Files\cursor\resources\app\package.json`)

## Latest Cursor updates reviewed for development workflow

Based on official Cursor changelog + local package check during service band 2026-07-27:

- **Local desktop:** **3.13.21** (from 3.13.10) — patch line; Windows Agents Changes-tab / switch-agent / subagent nav fixes expected; not a new numbered feature dump.
- **3.11** (Jul 10): latest **numbered** IDE release — side chats, agent transcript search, redesigned repo pickers, cloud agent conversation hooks.
- **Jul 22 changelog:** Cursor Router (Auto Cost/Balance/Intelligence) — for long drain prefer Balance/Intelligence.
- **Run Modes:** prefer **Auto-review**; project [`.cursor/permissions.json`](../../.cursor/permissions.json); no project `sandbox.json` on Windows VDT without OWNER.
- **Vision tools:** Speeds panel + eye filter + prune ≤2000 verified (`poolai-vision-sync --check` ok).
- **3.10** (Jun 30): Team MCPs in team marketplaces; organization groups.
- **Jul 17 changelog:** Slack multi-repo + plan-before-start (ops, поза repo drain).

## Project-level Cursor rule tuning applied

Added rules:

- `.cursor/rules/cursor-environment-baseline.mdc`
- `.cursor/rules/rust-toolchain-baseline.mdc`
- `.cursor/rules/ci-scripts-maintenance.mdc`
- `.cursor/permissions.json` (Auto-review steer)

Purpose:

- Keep agent behavior aligned with installed local toolchain and current Cursor behavior.
- Ensure config/script/CI edits include post-update verification and cleanup discipline.

**Service band 2026-07-24:** baseline → Cursor 3.13.10; Auto-review + permissions; vision eye + prune closed ≤2000; FM §5.16 PH-SVC45…SVC54.

**Service band 2026-07-25:** GitHub App installation / Actions token format (JWT `ghs_` ~520) → rules, SECRETS §5, permissions; FM §5.16 PH-SVC65…SVC74.

**Service band 2026-07-27:** baseline → Cursor 3.13.21; rules/agents/permissions + vision tools verify; FM §5.16 PH-SVC75…SVC84. Product drain unchanged: **`абракадабра`** → band 87.

## Operational recommendation

- Re-run version checks after system package updates:
  - Cursor: `C:\Program Files\cursor\resources\app\package.json` `version` (or `cursor.cmd --version` outside MSYS2)
  - `rustc --version`
  - `cargo --version`
  - `git --version`
- If any major version changes, review `.cursor/rules/*.mdc` and adjust workflow assumptions.
- Product drain unchanged: **`абракадабра`** → band 62 **PH-S1259…S1268**.
