# Промпт наступної автономної сесії (PoolAI)

**Оновлено:** 2026-05-20 · **Фаза:** Legacy backlog → **FM-032…042** · Post-Horizon **FM-020…031 ✅**

---

## Промпт

```
PoolAI — розробка FM-032 (наступна в §5.1). Post-Horizon закрито. HEAD f00bb1d4+.

## S0 — зріз

1. git fetch && git status -sb
2. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 (черга FM-032…042)
3. runtime-stack-policy.mdc · autonomous-orchestrator.mdc

Не повторювати: FM-020…031; grid import cleanup (f00bb1d4).

## Мета сесії — FM-032

OpenAPI: додати body schemas `VmNetwork`, `NetworkIsolationConfig` у docs/openapi.yaml
(типи вже в src/vm/mod.rs). Перевірка: cargo run --bin poolai-openapi-gap-audit.

## Черга після FM-032 (одна FM / сесію)

| # | FM | Фокус |
|---|-----|--------|
| 2 | FM-033 | Solana on-chain + real devnet RPC |
| 3 | FM-035 | Real model loading (EXO) |
| 4 | FM-034 | Job scheduler → VM/worker |
| 5 | FM-036 | Tensor sharding runtime |
| 6–10 | FM-040,037,039,038,042 | UI audit, topology graph, Playwright CI, OTel, perf |

Ops BLOCKED: FM-003 §4 LAN (2 хости) — лише verify-lan-prep / runbook.

Завершення: src/ → cargo fmt → cargo test-ci; push MSYS2 з Summary (не git commit з Cursor — див. git-push.md).
```

**§5.1 канон:** [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md). **Legacy audit:** §5.8.
