# Cursor 3.12.17 — post-update research (PoolAI service band)

**Дата:** 2026-07-17 (service band PH-SVC01…SVC10) · **Cursor:** 3.12.17 · **Rust:** 1.92.0 · **Git:** 2.50.0

**Операційний зріз:** active §5.12 **PH-S950…S959** (band 30 FUNCTIONALITY_DIGEST) · master backlog **61** pending · vision **rev 297** · rust_ratio **94.91%** (PH-S945).

**Попередній зріз:** [`CURSOR_UPDATE_RESEARCH_2026-06-20.md`](./CURSOR_UPDATE_RESEARCH_2026-06-20.md) (Cursor 3.8.11).

**Наступна product-сесія:** **`абракадабра`** — drain band 30 → promote PH-S960…S969.

---

## 1. Локальний baseline (перевірено 2026-07-17)

| Інструмент | Версія | Було в доках | Дія |
|------------|--------|--------------|-----|
| **Cursor** | **3.12.17** (`x64`) | 3.8.11 (HANDOFF) / 3.7.42 (baseline rule) | ✅ оновлено rules + HANDOFF |
| **rustc** | 1.92.0 | 1.92.0 | без змін |
| **cargo** | 1.92.0 | 1.92.0 | без змін |
| **git** | 2.50.0 | 2.54.0.windows.1 | ✅ baseline rule (факт MSYS2) |
| **OS** | Windows 11 build 26200 | — | без змін |

**Шлях Cursor CLI:** `C:\Program Files\cursor\resources\app\bin\cursor.cmd` (не в MSYS2 `PATH` — перевіряти через Windows або повний шлях).

---

## 2. Що змінилось у Cursor 3.9 → 3.12 (вплив на PoolAI VDT)

| Версія | Область | Вплив на PoolAI |
|--------|---------|-----------------|
| **3.9** (Jun 22–29) | Customize page (plugins, skills, MCP, rules, hooks); iOS + cloud agents; Remote Control | Repo skills у `~/.cursor/skills-cursor/` — канон без дублювання в rules |
| **3.10** (Jun 30) | Team MCPs у team marketplaces; org groups | Ops — поза drain; не змінює MSYS2 bash policy |
| **3.11** (Jul 10) | **Side chats** (`/side`, `/btw`); **transcript search** (Cmd+K); cloud agent hooks (`beforeSubmitPrompt`, `afterAgentThought`, `subagentStart`, …) | Дослідження/уточнення — не блокує main drain; hooks — опційно для cloud, локальний `hooks.json` без змін |
| **3.12.x** (Jul 17) | Slack: plan-before-start, multi-repo env, cross-channel | Поза PoolAI repo workflow |

**Не потребує змін у drain:** MSYS2 bash для `git`/`cargo`, Rust-first tests, vision close band, `абракадабра` порядок.

**Рекомендація VDT:** side chats — для research tangents під час drain; transcript search — швидкий пошук минулих PH-S* сесій.

---

## 3. Чекліст правил Cursor (актуальний)

| # | Перевірка | Статус |
|---|-----------|--------|
| 1 | `cursor --version` → baseline rule | ✅ **3.12.17** |
| 2 | `rustc` / `cargo` / `git` | ✅ 1.92.0 / 1.92.0 / 2.50.0 |
| 3 | MSYS2 bash для git/cargo (не PowerShell) | ✅ |
| 4 | `poolai-vision-sync --check` | ✅ rev **297**, next **PH-S950** |
| 5 | FM §5.12 active 10 + §5.14 master backlog | ✅ PH-S950…S959 `[ ]`, **61** pending |
| 6 | HANDOFF / README / INDEX zriz | ✅ service band 2026-07-17 |
| 7 | Service band §5.16 journal | ✅ PH-SVC01…SVC10 |

---

## 4. Service band PH-SVC01…SVC10 (2026-07-17)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-SVC01** | Cursor 3.12 research | цей файл |
| **PH-SVC02** | `cursor-environment-baseline.mdc` | 3.12.17 + git 2.50.0 |
| **PH-SVC03** | `.cursor/CHANGELOG` + `poolai-agent-roles.mdc` | 3.11 side chats note |
| **PH-SVC04** | HANDOFF + NEXT_SESSION | cursor pointer + service zriz |
| **PH-SVC05** | README release/Next Focus | rev 297, §5.12 active 10 |
| **PH-SVC06** | ENVIRONMENT_AND_CURSOR_UPDATES | pointer → цей research |
| **PH-SVC07** | `file_list.csv` | CURSOR_UPDATE_RESEARCH_2026-07-17 |
| **PH-SVC08** | `poolai-vision-sync --check` | drift gate green |
| **PH-SVC09** | FM §5.16 service journal | рядки ✅ |
| **PH-SVC10** | git push + самарі | service commit на `main` |

---

## 5. Посилання

- [`.cursor/rules/cursor-environment-baseline.mdc`](../../.cursor/rules/cursor-environment-baseline.mdc)
- [`.cursor/CHANGELOG.md`](../../.cursor/CHANGELOG.md)
- [Cursor changelog](https://cursor.com/changelog)
- [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md)
- [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.16
