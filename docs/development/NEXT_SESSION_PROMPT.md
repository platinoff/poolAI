# Промпт наступної автономної сесії (PoolAI)

**Оновлено:** 2026-05-22 · **Фаза:** Legacy backlog → **FM-034…042** · Post-Horizon **FM-020…035 ✅**

---

## Промпт

```
PoolAI — розробка FM-034 (наступна в §5.1). Post-Horizon + FM-035 закрито.

## S0 — зріз

1. git fetch && git status -sb
2. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 (черга FM-034…042)
3. runtime-stack-policy.mdc · autonomous-orchestrator.mdc

Не повторювати: FM-020…035; Solana adapter FM-033; FM-035 model_loader.

## Мета сесії — FM-034

Job scheduler → VM/worker binding (beyond in-process tick)
(`src/runtime/scheduler.rs`, Architect P6).

## Черга після FM-034 (одна FM / сесію)

| # | FM | Фокус |
|---|-----|--------|
| 2 | FM-036 | Tensor sharding runtime |
| 3–7 | FM-040,037,039,038,042 | UI audit, topology graph, Playwright CI, OTel, perf |

Ops BLOCKED: FM-003 §4 LAN (2 хости) — лише verify-lan-prep / runbook.

Завершення: src/ → cargo fmt → cargo test-ci; push MSYS2 з Summary (git-push.md).
```

**§5.1 канон:** [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md). **Legacy audit:** §5.8.
