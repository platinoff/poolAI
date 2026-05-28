# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-28 · **PH-S119 ✅** admin jobs lease polish

---

## Copy-paste для наступної сесії

```
PoolAI — ітераційна сесія PH-S120 (VDT, один спринт)

Правила: poolai-agent-roles.mdc · virtual-development-team.mdc · poolai-session-iteration.mdc · docs-vision.mdc

## S0
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI
git fetch; git status -sb; git log -1 --oneline
HANDOFF · FM §5.12 · GALAXY_GRID_ROADMAP · NEXT_SESSION_PROMPT · docs/vision/

## Локальний CI
cargo fmt --all
K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci
cd e2e && npm run test:ci   # після e2e scope

## Стан черги §5.12 (6 відкритих)
| Sprint | Тема |
|--------|------|
| PH-S120 | Solana adapter vision + DIGEST crosslink |
| PH-S121 | Galaxy §4.3 worker heartbeat wire note |
| PH-S122 | OpenAPI jobs/grid lease schemas audit (gap 0) |
| PH-S123 | Grid pricing E2E negative fallback |
| PH-S124 | OTel lease span attrs docs (FM-038) |

**Закрито (не повторювати):** PH-S03…S119 · PH-S113…S115 (vision UI rev 49)
**BLOCKED:** PH-S35/S16/S02 · **Deferred:** PH-S36/S01/S15

## PH-S120 — scope (наступний)
1. `docs/vision/` — `poolai-solana-adapter` node edges; FM-033 / DIGEST crosslink
2. FM §5.12 PH-S120 → ✅; HANDOFF; vision manifest

## Vision (manifest rev 43)
- Header: rev · last sprint · git HEAD · → PH-S120
- `.\bin\open-docs-vision.ps1` → hard-refresh after pull
```

---

## Короткий зріз

| **Наступний** | PH-S120 — Solana adapter vision crosslink |
| **Черга** | 6 відкритих: PH-S120…S124 |
| **Handoff** | [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) |
| **FM** | [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12 |
