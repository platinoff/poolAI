# Cursor 3.8.11 — post-update research (PoolAI)

**Дата:** 2026-06-21 (hygiene sync) · **Cursor:** 3.8.11 · **Rust:** 1.92.0 · **Git:** 2.54.0.windows.1

**Операційний зріз:** band 11 **PH-S760…S769 ✅** · active §5.12 **PH-S770…S779** (band 12) · master backlog **241** pending · vision **rev 277** · rust_ratio **94.63%**.

**Наступна сесія:** **`абракадабра`** — drain band 12 → promote PH-S780…S789.

---

## 1. Що змінилось у Cursor 3.8.x

| Область | Зміна | Вплив на PoolAI VDT |
|---------|--------|---------------------|
| **Automations** | `/automate` skill — plain-language → automation draft | Опційно: triage failed CI (не замінює `абракадабра`) |
| **GitHub triggers** | issue/PR/workflow triggers | Ops — поза drain |
| **3.7 carry-over** | Task subagents, Multitask Mode, SwitchMode | Канон — [`poolai-session-iteration.mdc`](../../.cursor/rules/poolai-session-iteration.mdc) |

**Не потребує змін:** `hooks.json`, MSYS2 bash policy, Rust-first tests, vision close band порядок.

---

## 2. Чекліст правил Cursor (актуальний)

| # | Перевірка | Статус |
|---|-----------|--------|
| 1 | `cursor --version` → baseline rule | ✅ **3.8.11** |
| 2 | `rustc` / `cargo` / `git` | ✅ 1.92.0 / 1.92.0 / 2.54.0 |
| 3 | MSYS2 bash для git/cargo (не PowerShell) | ✅ |
| 4 | `poolai-vision-sync --check` | ✅ rev **277**, next **PH-S770** |
| 5 | FM §5.12 active 10 + §5.14 master backlog | ✅ PH-S770…S779 `[ ]`, **241** pending |
| 6 | HANDOFF trim (5 band + archive pointer) | ✅ 2026-06-21 hygiene |
| 7 | `PH_S_MASTER_BACKLOG_351.md` regen | ✅ bands 1–11 drained, 12 active |

---

## 3. Docs hygiene (2026-06-21)

| Артефакт | Дія |
|----------|-----|
| `HANDOFF_NEW_SESSION.md` | Trim: 5 останніх band + archive → FM §5.12 |
| `PH_S_MASTER_BACKLOG_351.md` | `bash scripts/generate-ph-s-master-backlog-351.sh` |
| `bin/git-commit-push.sh`, `bin/test-vm-create-api.sh` | `.gitignore` (agent debris) |
| FM §5.12 journal | Без обрізки (канон); archive band — майбутній PH-S* |

**Канон drain:** [`NEXT_SESSION_PROMPT.md`](./NEXT_SESSION_PROMPT.md) · [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md).

---

## 4. Відомий ops-борг (не блокує drain)

- OpenAPI gap audit: **4** pre-existing routes (PH-S841 band)
- rust_ratio **94.63%** — hold advisory below 95% (formal band 90–95% ok)

---

## 5. Посилання

- [`.cursor/rules/cursor-environment-baseline.mdc`](../../.cursor/rules/cursor-environment-baseline.mdc)
- [`.cursor/CHANGELOG.md`](../../.cursor/CHANGELOG.md)
- [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md)
