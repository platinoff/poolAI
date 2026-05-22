# Промпт наступної автономної сесії (PoolAI)

**Оновлено:** 2026-05-22 · **Фаза:** Legacy backlog → **FM-035…042** · Post-Horizon **FM-020…033 ✅**

---

## Промпт

```
PoolAI — розробка FM-035 (наступна в §5.1). Post-Horizon + FM-033 закрито. HEAD 1b1681aa+.

## S0 — зріз

1. git fetch && git status -sb
2. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 (черга FM-035…042)
3. runtime-stack-policy.mdc · autonomous-orchestrator.mdc

Не повторювати: FM-020…033; Solana adapter FM-033 (on-chain + devnet RPC).

## Мета сесії — FM-035

Real model loading (libtorch/onnx path, not metadata-only)
(`ARCHITECT_PLAN_EXO_INTEGRATION`, `runtime/instance.rs`).

## Черга після FM-035 (одна FM / сесію)

| # | FM | Фокус |
|---|-----|--------|
| 2 | FM-034 | Job scheduler → VM/worker |
| 3 | FM-036 | Tensor sharding runtime |
| 4–8 | FM-040,037,039,038,042 | UI audit, topology graph, Playwright CI, OTel, perf |

Ops BLOCKED: FM-003 §4 LAN (2 хости) — лише verify-lan-prep / runbook.

Завершення: src/ → cargo fmt → cargo test-ci; push MSYS2 з Summary (git-push.md).
```

**§5.1 канон:** [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md). **Legacy audit:** §5.8.
