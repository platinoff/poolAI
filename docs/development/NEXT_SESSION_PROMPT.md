# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-28 · **HEAD:** `ea7b9957` (PH-S110)

---

## Copy-paste для наступної сесії

```
PoolAI — ітераційна сесія PH-S112 (VDT, один спринт)

Правила: poolai-agent-roles.mdc · virtual-development-team.mdc · poolai-session-iteration.mdc

## S0
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI
git fetch; git status -sb; git log -1 --oneline
HANDOFF · FM §5.12 · GALAXY_GRID_ROADMAP · цей файл

## Локальний CI
cargo fmt --all
K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci
cd e2e && npm run test:ci   # після e2e

## Стан
- **Закрито:** PH-S03…S111
- **Відкритий:** **PH-S112** (останній у post-lease черзі)
- **BLOCKED:** PH-S35/S16/S02 · **Deferred:** PH-S36/S01/S15

## PH-S112 — scope
1. `e2e/tests/grid_job_lease.spec.ts` (або розширення) — POST grid envelope Job + peer → GET job `leased` + lease fields
2. `npm run test:ci` includes spec; `bin/e2e-playwright.sh --start`
3. FM §5.12 PH-S112 → ✅; HANDOFF; replenish §5.12 (<10 нових code-first)

## Не повторювати
PH-S108 unit tests · jobs_lease E2E · PH-S110 result CAS
```

---

## Короткий зріз

| **Наступний** | PH-S112 — Grid Job envelope E2E |
| **Handoff** | [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) |
