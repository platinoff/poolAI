# Промпт наступної автономної сесії (PoolAI)

**Оновлено:** 2026-05-20 · **Фаза:** **Post-Horizon** (FM-020…) · **A+B+C:** **100%**

---

## Промпт

```
PoolAI — Post-Horizon: FM-029…031 (оркестратор + менеджер функціоналу).

## S0 — зріз

1. git fetch && git status -sb
2. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1, §5.7
3. AUTO_RUN_SESSION_2026_POST_HORIZON.md §4
4. autonomous-orchestrator.mdc · runtime-stack-policy.mdc

Не повторювати: FM-001…028 baseline.

## Мета (одна FM)

| FM | Фокус | Критерій |
|----|--------|----------|
| — | FM-020…028 | ✅ |
| 10 | FM-029 | Job store SQLite optional feature |
| 11 | FM-030 | Monitoring persistence MVP |
| 12 | FM-031 | WCAG expand admin URLs |

Почни з FM-029. Див. JOB_LAYER_CONCEPT §6, `src/job/store.rs`.

Завершення: cargo fmt → cargo test-ci (якщо `src/`); оновити FM/HANDOFF/CHANGELOG/AUTO_DEV_PATTERNS; commit+push MSYS2 з Summary.
```

**Наступна сесія:** **FM-029** (Job store SQLite).
