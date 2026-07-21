# Cursor 3.12.29 — service band re-check (PoolAI)

**Дата:** 2026-07-21 (service band PH-SVC11…SVC20) · **Cursor:** 3.12.29 · **Rust:** 1.92.0 · **Git:** 2.50.0

**Операційний зріз:** §5.12 **0** (band 58 **PH-S1219…S1228** ✅) · horizon band 59 → **PH-S1229…S1238** · vision **rev 342** (`--check` ok) · rust_ratio **94.83%**.

**Попередній зріз:** [`CURSOR_UPDATE_RESEARCH_2026-07-17.md`](./CURSOR_UPDATE_RESEARCH_2026-07-17.md) (Cursor 3.12.17).

**Наступна product-сесія:** **`абракадабра`** — project scan → promote band 59.

---

## 1. Локальний baseline (перевірено 2026-07-21)

| Інструмент | Версія | Було (2026-07-17) | Дія |
|------------|--------|-------------------|-----|
| **Cursor** | **3.12.29** (`x64`) | 3.12.17 | ✅ baseline + research |
| **rustc** | 1.92.0 | 1.92.0 | без змін |
| **cargo** | 1.92.0 | 1.92.0 | без змін |
| **git** | 2.50.0 | 2.50.0 | без змін |
| **OS** | Windows 11 build 26200 | — | без змін |

**Шлях Cursor CLI:** `C:\Program Files\cursor\resources\app\bin\cursor.cmd` (не в MSYS2 `PATH` — перевіряти через `package.json` version або повний шлях).

---

## 2. Changelog vs локальний desktop build

| Джерело | Що каже | Висновок для VDT |
|---------|---------|------------------|
| [cursor.com/changelog](https://cursor.com/changelog) | Останній **нумерований** IDE реліз — **3.11** (Jul 10): side chats, transcript search, cloud conversation hooks | Канон agent UX для правил |
| Changelog Jul 17 | **Slack** only (plan-before-start, multi-repo, cross-channel) — **без** desktop agent feature dump | Поза PoolAI drain |
| Локальний `package.json` | **3.12.29** (було 3.12.17) | Patch desktop build ID; **не** нова публічна agent-feature смуга |

**Не вигадувати** «3.12 agent features» з patch bump. Drain / MSYS2 / Rust-first / vision close / `абракадабра` — без змін.

---

## 3. Уточнення правил (docs-backed)

| Тема | Канон у docs | Адаптація PoolAI |
|------|--------------|------------------|
| Side chats | `/side`, `/btw`, plus; **local-only** (не Cloud Agents) | research tangents без зупинки drain |
| Transcript search | Agents Window Cmd/Ctrl+K; in-chat Cmd/Ctrl+F | пошук минулих PH-S* / `абракадабра` |
| Modes | Shift+Tab / mode picker: Agent, Ask, Plan, Debug | drain = **Agent**; Plan лише для неоднозначного scope |
| Multitask | `/multitask` + plan Build in Parallel | фонові Task subagents під час drain |
| Cloud hooks | `beforeSubmitPrompt`, `afterAgentResponse`, `afterAgentThought`, `stop`, `subagentStart`, … | опційно cloud; локальний `hooks.json` без змін |
| Subagents | product: explore / bash / browser; Task runtime: explore / shell / generalPurpose / … | `bugbot` / `security-review` / `best-of-n-runner` — лише за явним запитом |

---

## 4. Чекліст правил Cursor

| # | Перевірка | Статус |
|---|-----------|--------|
| 1 | Local Cursor → baseline rule | ✅ **3.12.29** |
| 2 | `rustc` / `cargo` / `git` | ✅ 1.92.0 / 1.92.0 / 2.50.0 |
| 3 | MSYS2 bash для git/cargo | ✅ без змін |
| 4 | Changelog alignment note (3.11 numbered vs 3.12.x local) | ✅ |
| 5 | `poolai-agent-roles` side chats local-only + mode picker | ✅ |
| 6 | FM §5.16 PH-SVC11…SVC20 | ✅ |
| 7 | HANDOFF / NEXT / README / INDEX zriz | ✅ |
| 8 | `poolai-vision-sync --check` | ✅ (після sync) |

---

## 5. Service band PH-SVC11…SVC20 (2026-07-21)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-SVC11** | Cursor 3.12.29 research | цей файл |
| **PH-SVC12** | `cursor-environment-baseline.mdc` | 3.12.29 + changelog note |
| **PH-SVC13** | `.cursor/CHANGELOG` + `poolai-agent-roles.mdc` | local-only side chats; mode picker |
| **PH-SVC14** | HANDOFF + NEXT_SESSION | service zriz; next = `абракадабра` |
| **PH-SVC15** | README Next Focus | cursor 3.12.29 note |
| **PH-SVC16** | ENVIRONMENT_AND_CURSOR_UPDATES | pointer → цей research |
| **PH-SVC17** | `file_list.csv` | CURSOR_UPDATE_RESEARCH_2026-07-21 |
| **PH-SVC18** | `poolai-vision-sync --check` | drift gate green |
| **PH-SVC19** | INDEX + docs/README cross-links | zriz Jul 21 |
| **PH-SVC20** | git push + самарі | service commit на `main` |

---

## 6. Посилання

- [`.cursor/rules/cursor-environment-baseline.mdc`](../../.cursor/rules/cursor-environment-baseline.mdc)
- [`.cursor/CHANGELOG.md`](../../.cursor/CHANGELOG.md)
- [Cursor changelog](https://cursor.com/changelog)
- [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md)
- [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.16
