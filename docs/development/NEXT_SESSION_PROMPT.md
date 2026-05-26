# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-26 · **VDT** [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — PH-S55 run-poolai RAID jobs preset

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
- **PH-S03…S54:** ✅
- **Черга §5.11:** PH-S55…S61 (7 відкритих)
- **BLOCKED:** PH-S35/S16/S02 LAN · **Deferred:** PH-S36/S01/S15 Cloud SDK (FM-041)

## PH-S55 — наступна сесія
1. `bin/run-poolai.sh` / `.ps1`: preset або documented one-liner з `POOLAI_JOB_STORE=raid` + `POOLAI_RAID_BASE_PATH`
2. `RUN_LOCAL.md` — single/lan quick start для RAID jobs

## Завершення сесії
1. FM §5.11 (PH-S55 → ✅) + HANDOFF
2. Оновити цей NEXT_SESSION_PROMPT (PH-S56)
3. git push (MSYS2) + самарі

## Не повторювати
PH-S03…S54 · verify-dev-stand RAID step · admin jobs UI · jobs RAID E2E

## Черга §5.11 (відкриті)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S55** | run-poolai RAID preset |
| 2–7 | **PH-S56…S61** | grid, CI docs, runbooks, RUN_PARAMETERS, openapi gate, VM isolation docs |

**Поза чергою:** PH-S35 LAN · PH-S36 Cloud SDK
```
