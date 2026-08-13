# Cursor 3.13.21 — service band (PoolAI)

**Дата:** 2026-07-27 (service band PH-SVC75…SVC84) · **Cursor:** 3.13.21 · **Rust:** 1.92.0 (rustup GNU) · **Git:** 2.50.0 (MSYS2)

**Операційний зріз:** §5.12 **10** (band 87 **PH-S1509…S1518** open) · vision rev **407** (Speeds + eye + prune ≤2000) · наступна product-сесія **`абракадабра`**.

**Попередній зріз:** [`CURSOR_UPDATE_RESEARCH_2026-07-24.md`](./CURSOR_UPDATE_RESEARCH_2026-07-24.md) (Cursor 3.13.10) · GH tokens [`GITHUB_APP_INSTALLATION_TOKENS_2026-07-25.md`](./GITHUB_APP_INSTALLATION_TOKENS_2026-07-25.md).

---

## 1. Локальний baseline (перевірено 2026-07-27)

| Інструмент | Версія | Було (2026-07-24/25) | Дія |
|------------|--------|----------------------|-----|
| **Cursor** | **3.13.21** (`x64`) | 3.13.10 | ✅ baseline + research |
| **rustc** | 1.92.0 (rustup `stable-x86_64-pc-windows-gnu`) | 1.92.0 | без змін |
| **cargo** | 1.92.0 | 1.92.0 | без змін |
| **git** | 2.50.0 (MSYS2 UCRT64) | 2.50.0 | без змін |
| **OS** | Windows 11 build 26200 | — | без змін |

**Шлях Cursor CLI:** `C:\Program Files\cursor\resources\app\bin\cursor.cmd` (не в MSYS2 `PATH` — перевіряти через `package.json` `version`).

**Увага:** MSYS2 `pacman` `rustc`/`cargo` може показувати **1.87.0** — для VDT канон **rustup** `RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu` → **1.92.0**.

---

## 2. Changelog vs локальний desktop build

| Джерело | Що каже | Висновок для VDT |
|---------|---------|------------------|
| [cursor.com/changelog](https://cursor.com/changelog) | Останній **нумерований** IDE — **3.11** (Jul 10). Jul 22 = **Cursor Router**. Jul 17 = Slack-only | Канон agent UX: 3.11 + Router; без нового numbered feature dump |
| Локальний `package.json` | **3.13.21** | Patch line після 3.13.10; **не** окремий public feature write-up |
| Forum (3.13 line) | Win Agents **Changes** (`cwd=`), switch-agent → chat, subagent back-nav, Cloud credits dismiss | Очікувати working UX на **3.13.21** (було announced для 3.13) |
| Remote SSH `crepectl` / GLIBC 2.39 | Ubuntu 22.04 indexing issue (forum) | **N/A** для local Windows PoolAI drain |
| [Run Modes docs](https://cursor.com/docs/agent/security/run-modes) | **Auto-review** + optional `sandbox.json` (macOS/Linux Seatbelt/Landlock) | Windows VDT: MSYS2 часто **поза** sandbox → classifier + `permissions.json` |
| Router blog | Dynamic tool calling (lazy tool schemas) | Не змінює drain; MCP — discover schema before call |

**Не вигадувати** agent-features з patch 3.13.10→3.13.21. Drain / MSYS2 / Rust-first / vision close / `абракадабра` — без змін протоколу.

---

## 3. Auto-execution + Router (адаптація)

| Тема | PoolAI канон |
|------|--------------|
| **Run Mode** | **Auto-review** (не Run Everything) |
| **permissions.json** | allow MSYS2/`cargo`/`git`/vision-sync; block force-push, secrets, `ghs_` length-checks, Python |
| **sandbox.json** | **не** обов’язковий для Windows VDT; Seatbelt/Landlock — macOS/Linux. Не додавати project `sandbox.json` без OWNER |
| **Cursor Router (Auto)** | опційно Cost/Balance/Intelligence; для довгого `абракадабра` drain — **Balance** або **Intelligence** (не Cost) |
| **Chat Mode** | `абракадабра` = **Agent**; Plan лише для неоднозначного scope |

---

## 4. Vision tools state (PoolAI `GSV/docs/vision/`)

Перевірено 2026-07-27 (`poolai-vision-sync --check` → **ok**, rev **407**):

| Компонент | Стан | Канон |
|-----------|------|--------|
| Sprint queue eye 👁 | ✅ | open / open+queued (`vision.js`) |
| Prune closed PH-S ≤2000 | ✅ | `poolai-vision-sync` → `manifest.sprint_queue` |
| Speeds panel | ✅ | `speed_index.json` + [`SPEED_INDEX.md`](./SPEED_INDEX.md); record після drain |
| Auto-reload / `__watch` | ✅ | manifest + speed_index + git_head |
| Open URL | ✅ | `http://127.0.0.1:8765/…` · `bin/open-docs-vision.ps1` — **не** `S:/` |
| Vision close band | ✅ | sync → FM rev = manifest → `--check` → test-ci |

Окремих code-змін vision у цій service-сесії **немає** — стан інструментів уже на main; оновлено лише правила/docs pointers на Cursor **3.13.21**.

---

## 5. Чекліст правил Cursor

| # | Перевірка | Статус |
|---|-----------|--------|
| 1 | Local Cursor → baseline rule | ✅ **3.13.21** |
| 2 | `rustc` / `cargo` / `git` (rustup + MSYS2) | ✅ 1.92.0 / 1.92.0 / 2.50.0 |
| 3 | MSYS2 bash для git/cargo | ✅ без змін |
| 4 | Auto-review + permissions.json | ✅ (+ vision-sync allow note) |
| 5 | GH tokens opaque (PH-SVC65…) | ✅ без регресії |
| 6 | Vision Speeds + eye + prune | ✅ verified |
| 7 | FM §5.16 PH-SVC75…SVC84 | ✅ |
| 8 | HANDOFF / NEXT / README / INDEX zriz | ✅ |
| 9 | `poolai-vision-sync --check` | ✅ rev 407 |

---

## 6. Service band PH-SVC75…SVC84 (2026-07-27)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-SVC75** | Cursor 3.13.21 research | цей файл |
| **PH-SVC76** | `cursor-environment-baseline.mdc` | 3.13.21 + Router/sandbox notes |
| **PH-SVC77** | agent-roles + orchestrator + permissions | desktop 3.13.21; vision allow |
| **PH-SVC78** | Vision tools state verify | Speeds/eye/prune; `--check` ok |
| **PH-SVC79** | HANDOFF + NEXT_SESSION | service zriz; next = `абракадабра` |
| **PH-SVC80** | README / INDEX / ENV pointer | Jul 27 research |
| **PH-SVC81** | `file_list.csv` + `.cursor/CHANGELOG` | CURSOR_UPDATE_2026-07-27 |
| **PH-SVC82** | docs-vision / Speeds cross-link | pointer SPEED_INDEX + 3.13.21 |
| **PH-SVC83** | `poolai-vision-sync --check` | drift gate green |
| **PH-SVC84** | git push + самарі | service commit на `main` |

---

## 7. Посилання

- [`.cursor/rules/cursor-environment-baseline.mdc`](../../.cursor/rules/cursor-environment-baseline.mdc)
- [`.cursor/permissions.json`](../../.cursor/permissions.json)
- [Cursor Run Modes](https://cursor.com/docs/agent/security/run-modes)
- [Cursor changelog](https://cursor.com/changelog)
- [`GSV/docs/vision/README.md`](../GSV/GSV/docs/vision/README.md) · [`SPEED_INDEX.md`](./SPEED_INDEX.md)
- [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md)
- [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.16
