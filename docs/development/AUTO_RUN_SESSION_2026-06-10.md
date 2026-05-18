# Автономний прогін (PoolAI) — 2026-06-10

**Попередній:** [`AUTO_RUN_SESSION_2026-06-09.md`](./AUTO_RUN_SESSION_2026-06-09.md) (FM-019 dashboard modals ✅ `08c704fe`).

**Ціль:** **FM-019 backlog** — pa11y/axe CI (`workflow_dispatch`).

**Критерії:**
- [x] `bin/pa11y-ci.sh` — strict `/ui/login`, optional admin/workers
- [x] `.github/workflows/a11y.yml` — `workflow_dispatch`
- [x] `ADMIN_A11Y_RUNBOOK.md` §3 оновлено
- [x] push — `8c5dc1df`

**BLOCKED:** FM-003 §4 LAN (2 хости).

**Поза обсягом:** FM-004, FM-006, FM-009, FM-010.

## Стартовий промпт (оркестратор)

```
S0: git fetch && git status -sb; HANDOFF + FUNCTION_MANAGEMENT §5.1–5.3.
Пріоритет: FM-019 pa11y CI slice. Не робити: FM-004/006/009/010.
Після коду: cargo test-ci (якщо зміни src/), push MSYS2.
```

## S1 — виконання (2026-05-18)

**Артефакти:** `bin/pa11y-ci.sh`, `.github/workflows/a11y.yml`.

**Далі:** [`AUTO_RUN_SESSION_2026-06-11.md`](./AUTO_RUN_SESSION_2026-06-11.md) (auth fixture).
