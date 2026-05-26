# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-26 · **VDT** [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — PH-S52 E2E jobs + RAID persistence smoke

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
# Linux VM isolation: cargo test --test vm_isolation_integration --features vm-isolation-linux

## Стан
- **PH-S03…S51:** ✅
- **Черга §5.11:** PH-S52…S61 (10 відкритих)
- **BLOCKED:** PH-S35/S16/S02 LAN · **Deferred:** PH-S36/S01/S15 Cloud SDK (FM-041)

## PH-S52 — наступна сесія
1. E2E або API helper: `POST /api/v1/jobs` → restart coordinator з `POOLAI_JOB_STORE=raid` → `GET /jobs/{id}`
2. `e2e/tests/` + `helpers.ts`; опційно reuse `job_store_raid_persistence.rs` patterns
3. `cd e2e && npm run test:ci` якщо Playwright scope

## Завершення сесії
1. FM §5.11 (PH-S52 → ✅) + HANDOFF
2. Оновити цей NEXT_SESSION_PROMPT (PH-S53)
3. git push (MSYS2) + самарі

## Не повторювати
PH-S03…S51 · veth/macvlan linux.rs hardening · OpenAPI jobs / RAID store docs

## Черга §5.11 (відкриті)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S52** | E2E jobs + RAID smoke |
| 2 | **PH-S53** | Admin jobs UI |
| 3–10 | **PH-S54…S61** | verify-dev-stand, run-poolai, grid, CI docs, runbooks, RUN_PARAMETERS, openapi gate, VM isolation docs |

**Поза чергою:** PH-S35 LAN · PH-S36 Cloud SDK
```
