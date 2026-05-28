# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-28 · **HEAD:** `731c8dcd` (PH-S111) · після docs-sync vision **PH-S113…S115** (локально, до push)

---

## Copy-paste для наступної сесії

```
PoolAI — ітераційна сесія PH-S112 (VDT, один спринт)

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

## Стан черги §5.12 (10 відкритих)
| Sprint | Тема |
|--------|------|
| PH-S112 | Grid Job envelope E2E — POST grid Job + peer → GET `leased` + lease fields |
| PH-S116 | Worker periodic lease renew loop (`poolai-worker`, PH-S111) |
| PH-S117 | Grid result `lease_epoch` E2E (409 stale epoch) |
| PH-S118 | Jobs lease negative paths E2E (409/410) |
| PH-S119 | Admin jobs lease column polish (`lease_epoch`, tooltip, i18n) |
| PH-S120 | Solana adapter vision + DIGEST crosslink |
| PH-S121 | Galaxy §4.3 worker heartbeat wire note |
| PH-S122 | OpenAPI jobs/grid lease schemas audit (gap 0) |
| PH-S123 | Grid pricing E2E negative fallback |
| PH-S124 | OTel lease span attrs docs (FM-038) |

**Закрито (не повторювати):** PH-S03…S111 · PH-S113…S115 (vision map rev 38)
**BLOCKED:** PH-S35/S16/S02 · **Deferred:** PH-S36/S01/S15

## PH-S112 — scope (наступний)
1. `e2e/tests/grid_job_lease.spec.ts` — grid envelope Job + `schedule_with_grid_peer` path
2. `e2e/package.json` — spec у `test:ci`; `bin/e2e-playwright.sh --start`
3. FM §5.12 PH-S112 → ✅; HANDOFF; INDEX/README

## Vision (docs/vision, rev 38)
- L0–L5: concept → ops → catalog → code → lib roots → workspace TOML
- ⊟ Folders: collapse 5+ files per `src/*/` hub; ◎ Sprint: dim out-of-scope
- Pan/zoom: wheel ~6%/event, buttons 16%; `.\bin\open-docs-vision.ps1`

## Після PH-S112
Закрити один спринт → оновити FM §5.12, HANDOFF, manifest `revision++`, `extensions.json` `active_sprint`; replenish до ≤10 відкритих за потреби.
```

---

## Короткий зріз

| **Наступний** | PH-S112 — Grid Job envelope E2E |
| **Черга** | 10 відкритих: PH-S112, PH-S116…S124 |
| **Handoff** | [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) |
| **FM** | [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12 |
| **Vision** | [`docs/vision/`](../vision/) manifest rev 38 |
| **Інвентар** | [`file_list.csv`](../../file_list.csv) · `git ls-files` |
