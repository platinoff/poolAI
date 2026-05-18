# Автономний прогін (PoolAI) — 2026-06-08

**Попередній:** [`AUTO_RUN_SESSION_2026-06-07.md`](./AUTO_RUN_SESSION_2026-06-07.md) (менеджер функціоналу: §5.3 audit + підготовка сесії).

**Ціль:** звірка FM ↔ docs після FM-019; **наступний кодовий спринт** — на вибір оркестратора з §5.1 (рекомендація: **P4** `poolai_health_load` на ref-host **або** docs-only якщо немає стенду).

**Критерії:**
- [x] `FUNCTION_MANAGEMENT.md` §5.3 оновлено (2026-06-07 audit)
- [x] HANDOFF, STABLE_STATE, README Next Focus синхронізовано
- [ ] Наступний FM-спринт (код) — **не в цій сесії** (лише підготовка)
- [x] docs-only push (менеджер функціоналу)

**BLOCKED:** FM-003 §4 LAN (2 хости).

**Поза обсягом:** FM-004, FM-006, FM-009, FM-010.

## Стартовий промпт (оркестратор)

```
S0: git fetch && git status -sb; HANDOFF + FUNCTION_MANAGEMENT §5.1–5.3 + AUTO_RUN_SESSION_2026-06-08.
Пріоритет: P4 poolai_health_load (якщо сервер піднято) АБО FM-003 runbook refresh (BLOCKED).
Не робити: FM-004/006/009/010 без явного запиту. Після коду: cargo fmt, cargo test-ci, push MSYS2 (-c commit.template=).
```
