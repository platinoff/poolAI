# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-28 · **HEAD:** `347536be` (PH-S108) · VDT — [`.cursor/rules/poolai-agent-roles.mdc`](../../.cursor/rules/poolai-agent-roles.mdc) · [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

## Copy-paste для наступної сесії

```
PoolAI — ітераційна сесія PH-S110 (VDT, один спринт)

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
cargo run --bin poolai-openapi-gap-audit   # після API

## Стан
- **Закрито:** PH-S03…S109 (смуга PH-S100…S109 — 10/10 ✅)
- **Відкритий:** **PH-S110** (1 з 3)
- **BLOCKED:** PH-S35/S16/S02 · **Deferred:** PH-S36/S01/S15

## PH-S110 — scope
1. `src/grid/dispatch.rs` — grid `Result` ingest: optional `lease_epoch` CAS vs job record → reject on mismatch
2. Unit tests in `dispatch.rs` or integration test
3. FM §5.12 + HANDOFF + цей prompt

## Не повторювати
PH-S94…S109 lease wire MVP · jobs_lease E2E · grid ingest→leased docs

## Черга §5.12 (3)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S110** | Grid result lease_epoch CAS |
| 2 | PH-S111 | `POOLAI_JOB_LEASE_RENEW_INTERVAL_SECS` env |
| 3 | PH-S112 | Grid Job envelope E2E |
```

---

## Короткий зріз

| | |
|---|---|
| **Наступний** | PH-S110 — Grid result ingest lease_epoch CAS |
| **Handoff** | [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) |
