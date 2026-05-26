# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-26 · **VDT** [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — PH-S54 verify-dev-stand RAID job store step

## Ролі (VDT)
- Людина: власник / креативний директор — пріоритети, BLOCKED/Deferred
- Ти: оркестратор Rust — один PH-S*, субагенти для explore/shell/модуль
- Правила: virtual-development-team.mdc · poolai-session-iteration.mdc · runtime-stack-policy.mdc

## S0 (MSYS2 UCRT64 bash)
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI
git fetch; git status -sb; git log -1 --oneline
df -h /s | tail -1
HANDOFF · FM §5.11

## Локальний CI (канон)
cargo fmt --all
cargo test-ci

## Стан
- **PH-S03…S53:** ✅
- **Черга §5.11:** PH-S54…S61 (8 відкритих)
- **BLOCKED:** PH-S35/S16/S02 LAN · **Deferred:** PH-S36/S01/S15 Cloud SDK (FM-041)

## PH-S54 — наступна сесія
1. `bin/verify-dev-stand.sh`: env-gated step — create job, restart coordinator, assert persisted
2. Reuse `jobs_raid` / `job_store_raid_persistence` patterns

## Завершення сесії
1. FM §5.11 (PH-S54 → ✅) + HANDOFF
2. Оновити цей NEXT_SESSION_PROMPT (PH-S55)
3. git push (MSYS2) + самарі

## Не повторювати
PH-S03…S53 · admin jobs UI · jobs RAID E2E · veth/macvlan linux.rs

## Черга §5.11 (відкриті)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S54** | verify-dev-stand RAID step |
| 2–8 | **PH-S55…S61** | run-poolai, grid, CI docs, runbooks, RUN_PARAMETERS, openapi gate, VM isolation docs |

**Поза чергою:** PH-S35 LAN · PH-S36 Cloud SDK
```
