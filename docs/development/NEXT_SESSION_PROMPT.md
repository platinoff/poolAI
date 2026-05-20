# Промпт наступної автономної сесії (PoolAI)

**Оновлено:** 2026-05-20 · **Фаза:** Post-Horizon **FM-020…031 ✅** · **A+B+C:** **100%**

---

## Промпт

```
PoolAI — Post-Horizon закрито (FM-020…031). Maintenance / ops.

## S0 — зріз

1. git fetch && git status -sb
2. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1
3. runtime-stack-policy.mdc · autonomous-orchestrator.mdc

Не повторювати: FM-020…031 (job/memory/grid/SQLite/monitoring/a11y).

## Мета (на вибір)

| Пріоритет | Фокус | Стан |
|-----------|--------|------|
| Ops | FM-003 §4 LAN sign-off | **BLOCKED** (2 хости) |
| QA | cargo test-ci зріз | періодично |
| Docs | Architect P6 залишок / on-chain | за планом |

Завершення: якщо `src/` — cargo fmt → cargo test-ci; commit+push з Summary при змінах коду.
```

**Post-Horizon черга:** закрита. **Ops:** FM-003 §4 LAN (**BLOCKED**, 2 фізичні хости).
