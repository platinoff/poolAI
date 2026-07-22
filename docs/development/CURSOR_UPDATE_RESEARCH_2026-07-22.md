# Cursor 3.12.30 — service band re-check (PoolAI)

**Дата:** 2026-07-22 (service band PH-SVC21…SVC30) · **Cursor:** 3.12.30 · **Rust:** 1.92.0 (rustup GNU) · **Git:** 2.50.0 (MSYS2)

**Операційний зріз:** §5.12 **0** (band 61 **PH-S1249…S1258** ✅) · horizon band 62 → **PH-S1259…S1268** · vision queue/feed fix (enterprise bands) · наступна product-сесія **`абракадабра`**.

**Попередній зріз:** [`CURSOR_UPDATE_RESEARCH_2026-07-21.md`](./CURSOR_UPDATE_RESEARCH_2026-07-21.md) (Cursor 3.12.29).

---

## 1. Локальний baseline (перевірено 2026-07-22)

| Інструмент | Версія | Було (2026-07-21) | Дія |
|------------|--------|-------------------|-----|
| **Cursor** | **3.12.30** (`x64`) | 3.12.29 | ✅ baseline + research |
| **rustc** | 1.92.0 (rustup `stable-x86_64-pc-windows-gnu`) | 1.92.0 | без змін |
| **cargo** | 1.92.0 | 1.92.0 | без змін |
| **git** | 2.50.0 (MSYS2 UCRT64) | 2.50.0 | без змін (host Windows git може бути новіший — канон MSYS2) |
| **OS** | Windows 11 build 26200 | — | без змін |

**Шлях Cursor CLI:** `C:\Program Files\cursor\resources\app\bin\cursor.cmd` (не в MSYS2 `PATH` — перевіряти через `package.json` `version`).

---

## 2. Changelog vs локальний desktop build

| Джерело | Що каже | Висновок для VDT |
|---------|---------|------------------|
| [cursor.com/changelog](https://cursor.com/changelog) | Останній **нумерований** IDE реліз — **3.11** (Jul 10): side chats, transcript search, cloud conversation hooks | Канон agent UX для правил |
| Changelog Jul 17 | **Slack** only (plan-before-start, multi-repo, cross-channel) | Поза PoolAI drain |
| Локальний `package.json` | **3.12.30** (було 3.12.29) | Patch desktop build ID; **не** нова публічна agent-feature смуга |
| Forum (Jul 21+) | Agents Window flicker з темою **Cursor Dark High Contrast** | Ops note — switch theme; не змінює VDT drain |

**Не вигадувати** «3.12 agent features» з patch bump. Drain / MSYS2 / Rust-first / vision close / `абракадабра` — без змін.

---

## 3. Уточнення правил (docs-backed)

| Тема | Канон | Адаптація PoolAI |
|------|-------|------------------|
| Side chats | `/side`, `/btw`, plus; **local-only** | research tangents без зупинки drain |
| Transcript search | Agents Window Cmd/Ctrl+K; in-chat Cmd/Ctrl+F | пошук минулих PH-S* / `абракадабра` |
| Modes | Shift+Tab / mode picker | drain = **Agent**; Plan лише для неоднозначного scope |
| Multitask | `/multitask` + фонові Task | довгі `cargo test-ci` — `shell` subagent |
| Cloud hooks | `beforeSubmitPrompt`, `afterAgentResponse`, `afterAgentThought`, `stop`, `subagentStart`, … | опційно cloud; локальний `hooks.json` без змін |
| Vision queue/feed | `poolai-vision-sync` мержить §5.12 **+** enterprise `queue — band` секції | після sync Sprint queue + Feed показують **останні закриті PH-S*** |

---

## 4. Чекліст правил Cursor

| # | Перевірка | Статус |
|---|-----------|--------|
| 1 | Local Cursor → baseline rule | ✅ **3.12.30** |
| 2 | `rustc` / `cargo` / `git` (MSYS2) | ✅ 1.92.0 / 1.92.0 / 2.50.0 |
| 3 | MSYS2 bash для git/cargo | ✅ без змін |
| 4 | Changelog alignment (3.11 numbered vs 3.12.x local) | ✅ |
| 5 | `poolai-agent-roles` pointer → цей research | ✅ |
| 6 | FM §5.16 PH-SVC21…SVC30 | ✅ |
| 7 | HANDOFF / NEXT / README / INDEX zriz | ✅ |
| 8 | Vision queue/feed merge enterprise bands | ✅ |
| 9 | `poolai-vision-sync --check` | ✅ (після sync) |

---

## 5. Service band PH-SVC21…SVC30 (2026-07-22)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-SVC21** | Cursor 3.12.30 research | цей файл |
| **PH-SVC22** | `cursor-environment-baseline.mdc` | 3.12.30 + changelog note |
| **PH-SVC23** | `.cursor/CHANGELOG` + `poolai-agent-roles.mdc` | pointer + High Contrast flicker note |
| **PH-SVC24** | HANDOFF + NEXT_SESSION | service zriz; next = `абракадабра` |
| **PH-SVC25** | README Next Focus | cursor 3.12.30 note |
| **PH-SVC26** | ENVIRONMENT_AND_CURSOR_UPDATES | pointer → цей research |
| **PH-SVC27** | `file_list.csv` | CURSOR_UPDATE_RESEARCH_2026-07-22 |
| **PH-SVC28** | Vision sync: enterprise band → queue/feed | `last_sprint_closed` / Feed = latest PH-S* |
| **PH-SVC29** | INDEX + docs/README cross-links | zriz Jul 22 |
| **PH-SVC30** | git push + самарі | service commit на `main` |

---

## 6. Посилання

- [`.cursor/rules/cursor-environment-baseline.mdc`](../../.cursor/rules/cursor-environment-baseline.mdc)
- [`.cursor/CHANGELOG.md`](../../.cursor/CHANGELOG.md)
- [Cursor changelog](https://cursor.com/changelog)
- [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md)
- [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.16
