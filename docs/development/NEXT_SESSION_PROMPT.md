# Промпт наступної автономної сесії (PoolAI)

**Оновлено:** 2026-05-22 · **Фаза:** Legacy backlog → **FM-033…042** · Post-Horizon **FM-020…032 ✅**

---

## Промпт

```
PoolAI — розробка FM-033 (наступна в §5.1). Post-Horizon + FM-032 закрито. HEAD e49e92ef+.

## S0 — зріз

1. git fetch && git status -sb
2. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 (черга FM-033…042)
3. runtime-stack-policy.mdc · autonomous-orchestrator.mdc

Не повторювати: FM-020…032; OpenAPI VM network schemas (FM-032).

## Мета сесії — FM-033

Solana: on-chain program prototype + real devnet RPC submit
(після FM-024 stub; `SOLANA_ADAPTER_CONCEPT` §8, `crates/poolai-solana-adapter/`).

## Черга після FM-033 (одна FM / сесію)

| # | FM | Фокус |
|---|-----|--------|
| 2 | FM-035 | Real model loading (EXO) |
| 3 | FM-034 | Job scheduler → VM/worker |
| 4 | FM-036 | Tensor sharding runtime |
| 5–9 | FM-040,037,039,038,042 | UI audit, topology graph, Playwright CI, OTel, perf |

Ops BLOCKED: FM-003 §4 LAN (2 хости) — лише verify-lan-prep / runbook.

Завершення: src/ → cargo fmt → cargo test-ci; push MSYS2 з Summary (не git commit з Cursor — див. git-push.md).
```

**§5.1 канон:** [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md). **Legacy audit:** §5.8.
