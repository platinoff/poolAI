# Cursor 3.13.10 — service band (PoolAI)

**Дата:** 2026-07-24 (service band PH-SVC45…SVC54) · **Cursor:** 3.13.10 · **Rust:** 1.92.0 (rustup GNU) · **Git:** 2.50.0 (MSYS2)

**Операційний зріз:** §5.12 **10** (band 76 **PH-S1399…S1408** open) · vision queue prune closed ≤2000 + eye filter · наступна product-сесія **`абракадабра`**.

**Попередній зріз:** [`CURSOR_UPDATE_RESEARCH_2026-07-22.md`](./CURSOR_UPDATE_RESEARCH_2026-07-22.md) (Cursor 3.12.30).

---

## 1. Локальний baseline (перевірено 2026-07-24)

| Інструмент | Версія | Було (2026-07-22) | Дія |
|------------|--------|-------------------|-----|
| **Cursor** | **3.13.10** (`x64`) | 3.12.30 | ✅ baseline + research |
| **rustc** | 1.92.0 (rustup `stable-x86_64-pc-windows-gnu`) | 1.92.0 | без змін |
| **cargo** | 1.92.0 | 1.92.0 | без змін |
| **git** | 2.50.0 (MSYS2 UCRT64) | 2.50.0 | без змін |
| **OS** | Windows 11 build 26200 | — | без змін |

**Шлях Cursor CLI:** `C:\Program Files\cursor\resources\app\bin\cursor.cmd` (не в MSYS2 `PATH` — перевіряти через `package.json` `version`).

---

## 2. Changelog vs локальний desktop build

| Джерело | Що каже | Висновок для VDT |
|---------|---------|------------------|
| [cursor.com/changelog](https://cursor.com/changelog) | Останній **нумерований** IDE реліз — **3.11** (Jul 10). Jul 22 = **Cursor Router** (Auto model modes). Jul 17 = Slack-only | Канон agent UX: 3.11 + Router |
| Локальний `package.json` | **3.13.10** | Desktop patch line після 3.12.x; **не** окремий numbered feature write-up на changelog |
| Forum (Jul 22) | Agents Window **Changes** empty on Windows (`cwd=`) — fix у **3.13** line | Очікувати working Changes tab після апдейту |
| [Run Modes docs](https://cursor.com/docs/agent/security/run-modes) | **Auto-review** (з 3.6) — default для нових інсталів; `permissions.json` steers classifier | Адаптувати PoolAI rules + project `permissions.json` |

**Не вигадувати** agent-features з patch bump 3.12→3.13. Drain / MSYS2 / Rust-first / vision close / `абракадабра` — без змін.

---

## 3. Auto-execution (Run Modes) — адаптація правил

| Mode | Поведінка | PoolAI канон |
|------|-----------|--------------|
| **Ask** | кожен Shell/MCP/Fetch — approval | лише якщо власник хоче ручний gate |
| **Auto-review** | allowlist → sandbox → classifier | **рекомендовано** для drain / `абракадабра` |
| **Allowlist** | лише allowlist (+ optional sandbox) | вузький повторюваний набір |
| **Run Everything** | нуль prompts | **не** для PoolAI без явного OWNER |

**Канон сесії:**

1. Settings → Agents → Approvals & Execution → **Auto-review** (не Run Everything).
2. Project [`.cursor/permissions.json`](../../.cursor/permissions.json) — allow MSYS2/`cargo`/`git` drain; block force-push / secrets / history rewrite.
3. YOLO / legacy Auto-Run → замінено Run Modes (3.6+); у правилах писати **Auto-review**, не YOLO.
4. Cloud Agents **не** використовують Run Modes (окрема VM).
5. `абракадабра` drain лишається **Agent** mode (Shift+Tab); Run Mode ≠ chat Mode.

---

## 4. Уточнення правил (docs-backed)

| Тема | Канон | Адаптація PoolAI |
|------|-------|------------------|
| Run Modes | Auto-review default | `cursor-environment-baseline`, `poolai-agent-roles`, `autonomous-orchestrator` |
| permissions.json | allow/block plain-English | `.cursor/permissions.json` (project) |
| Cursor Router (Jul 22) | Auto → Cost/Balance/Intelligence | ops note; не змінює VDT |
| Changes tab (Win) | fixed in 3.13 | ops note — verify after update |
| Vision queue | prune closed PH-S ≤2000; eye filter open / open+queued | `poolai-vision-sync` + `vision.js` |

---

## 5. Чекліст правил Cursor

| # | Перевірка | Статус |
|---|-----------|--------|
| 1 | Local Cursor → baseline rule | ✅ **3.13.10** |
| 2 | `rustc` / `cargo` / `git` (MSYS2) | ✅ 1.92.0 / 1.92.0 / 2.50.0 |
| 3 | MSYS2 bash для git/cargo | ✅ без змін |
| 4 | Auto-review + permissions.json | ✅ |
| 5 | Screenshot P0 (`--debug`) PH-SVC41…43 | ✅ already on main |
| 6 | Vision prune ≤2000 + eye | ✅ |
| 7 | FM §5.16 PH-SVC45…SVC54 | ✅ |
| 8 | HANDOFF / NEXT / README / INDEX zriz | ✅ |
| 9 | `poolai-vision-sync --check` | ✅ (після sync) |

---

## 6. Service band PH-SVC45…SVC54 (2026-07-24)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-SVC45** | Cursor 3.13.10 research | цей файл |
| **PH-SVC46** | `cursor-environment-baseline.mdc` | 3.13.10 + Auto-review |
| **PH-SVC47** | `.cursor/permissions.json` + agent-roles | Auto-review steer for drain |
| **PH-SVC48** | Vision eye filter + prune ≤2000 | `vision.js` / `poolai-vision-sync` |
| **PH-SVC49** | Screenshot CI verify (code) | `--debug` absent; PH-SVC34 GH still open |
| **PH-SVC50** | HANDOFF + NEXT_SESSION | service zriz; next = `абракадабра` |
| **PH-SVC51** | README / INDEX / ENV pointer | Jul 24 research |
| **PH-SVC52** | `file_list.csv` + `.cursor/CHANGELOG` | row + changelog |
| **PH-SVC53** | `poolai-vision-sync --check` | drift gate green |
| **PH-SVC54** | git push + самарі | service commit на `main` |

---

## 7. Посилання

- [`.cursor/rules/cursor-environment-baseline.mdc`](../../.cursor/rules/cursor-environment-baseline.mdc)
- [`.cursor/permissions.json`](../../.cursor/permissions.json)
- [Cursor Run Modes](https://cursor.com/docs/agent/security/run-modes)
- [Cursor changelog](https://cursor.com/changelog)
- [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md)
- [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.16
