# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-28 · **HEAD:** (PH-S110 pending push) · VDT

---

## Copy-paste для наступної сесії

```
PoolAI — ітераційна сесія PH-S111 (VDT, один спринт)

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

## Стан
- **Закрито:** PH-S03…S110
- **Відкритий:** **PH-S111** (1 з 2)
- **BLOCKED:** PH-S35/S16/S02 · **Deferred:** PH-S36/S01/S15

## PH-S111 — scope
1. `src/job/lease_config.rs` — `POOLAI_JOB_LEASE_RENEW_INTERVAL_SECS` optional override (default `lease_ttl/3`)
2. Unit tests; HANDOFF §2a env row
3. FM §5.12 + HANDOFF + цей prompt

## Не повторювати
PH-S110 grid result CAS · PH-S94…S109 lease MVP

## Черга §5.12 (2)
| 1 | **PH-S111** | Renew interval env |
| 2 | PH-S112 | Grid Job envelope E2E |
```

---

## Короткий зріз

| **Наступний** | PH-S111 — `POOLAI_JOB_LEASE_RENEW_INTERVAL_SECS` |
| **Handoff** | [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) |
