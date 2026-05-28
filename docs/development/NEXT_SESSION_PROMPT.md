# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-28 · **PH-S116 ✅** worker renew ticker · **PH-S117 ✅** grid Result lease E2E

---

## Copy-paste для наступної сесії

```
PoolAI — ітераційна сесія PH-S118 (VDT, один спринт)

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

## Стан черги §5.12 (8 відкритих)
| Sprint | Тема |
|--------|------|
| PH-S118 | Jobs lease negative paths E2E (409/410) |
| PH-S119 | Admin jobs lease column polish (`lease_epoch`, tooltip, i18n) |
| PH-S120 | Solana adapter vision + DIGEST crosslink |
| PH-S121 | Galaxy §4.3 worker heartbeat wire note |
| PH-S122 | OpenAPI jobs/grid lease schemas audit (gap 0) |
| PH-S123 | Grid pricing E2E negative fallback |
| PH-S124 | OTel lease span attrs docs (FM-038) |

**Закрито (не повторювати):** PH-S03…S111 · PH-S112–S117 · PH-S113…S115 (vision UI rev 48)
**BLOCKED:** PH-S35/S16/S02 · **Deferred:** PH-S36/S01/S15

## PH-S118 — scope (наступний)
1. `e2e/tests/jobs_lease.spec.ts` (або новий spec) — wrong owner, expired lease, renew without acquire → 409/410
2. FM §5.12 PH-S118 → ✅; HANDOFF; vision manifest

## Vision (docs/vision, manifest rev 41)
- `vision2.png` galaxy wallpaper 15%; constellation map; legend chips = layer focus
- Fullscreen ☰ → Explorer overlay; **◎ Sprint** = `active_sprint` (PH-S118)
- `.\bin\open-docs-vision.ps1` → Reload after pull
```

---

## Короткий зріз

| **Наступний** | PH-S118 — Jobs lease negative paths E2E |
| **Черга** | 8 відкритих: PH-S118…S124 |
| **Handoff** | [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) |
| **FM** | [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12 |
| **Vision** | [`docs/vision/`](../vision/) manifest rev 41 · UI rev 48 |
