# Промпт наступної автономної сесії (PoolAI)

**Оновлено:** 2026-05-20 · **Фаза:** **Post-Horizon** (FM-020…) · **A+B+C:** **100%**

---

## Промпт

```
PoolAI — Post-Horizon: FM-031 (оркестратор + менеджер функціоналу).

## S0 — зріз

1. git fetch && git status -sb
2. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1, §5.7
3. AUTO_RUN_SESSION_2026_POST_HORIZON.md §4
4. autonomous-orchestrator.mdc · runtime-stack-policy.mdc

Не повторювати: FM-001…030 baseline.

## Мета (одна FM)

| FM | Фокус | Критерій |
|----|--------|----------|
| — | FM-020…030 | ✅ |
| 12 | FM-031 | WCAG expand admin URLs |

Почни з FM-031. Див. `ADMIN_A11Y_RUNBOOK.md`, `e2e/tests/a11y.spec.ts`, FM-019.

Завершення: cargo fmt → cargo test-ci (якщо `src/`); оновити FM/HANDOFF/CHANGELOG/AUTO_DEV_PATTERNS; commit+push MSYS2 з Summary.
```

**Наступна сесія:** **FM-031** (WCAG expand admin URLs).
