# Автономний прогін (PoolAI) — 2026-06-15 (S5)

**Попередній:** [`AUTO_RUN_SESSION_2026-06-14.md`](./AUTO_RUN_SESSION_2026-06-14.md) (FM-019 docs S4 ✅ `c1b2b24e`).

**Ціль:** **FM-019** — розширити `ADMIN_URLS` у `bin/pa11y-ci.sh`: `/ui`, `/ui/admin/config`.

**Критерії:**
- [x] `bin/pa11y-ci.sh` — `/ui` + `/ui/admin/config` у `ADMIN_URLS`
- [x] `tests/pa11y_ci_script.rs` — assert нові URL
- [x] `PA11Y_ADMIN_STRICT=1 bash bin/pa11y-ci.sh --start` — **0 errors** (6 auth URLs + login)
- [x] `cargo test --test pa11y_ci_script`
- [x] `cargo test-ci`
- [x] runbook / FM / HANDOFF / CHANGELOG sync
- [ ] push

## S5 — виконання (2026-05-18)

**Поза обсягом:** FM-004, FM-006, FM-009, FM-010.

**BLOCKED:** FM-003 §4 LAN.

## Стартовий промпт (оркестратор)

```
S0: git fetch && git status -sb; HANDOFF + FUNCTION_MANAGEMENT §5.1.
Пріоритет: FM-019 pa11y — ADMIN_URLS + /ui, /ui/admin/config; 0 errors strict.
Не робити: FM-004/006/009/010.
Після коду: cargo fmt → cargo test-ci → push MSYS2.
```
