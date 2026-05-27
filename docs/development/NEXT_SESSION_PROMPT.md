# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-26 · **VDT** [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — PH-S61 Galaxy Grid seeds/locality (concept)

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
- **PH-S03…S60:** ✅
- **Черга §5.11:** PH-S61…S64 (4 відкриті; Galaxy Grid concept)
- **BLOCKED:** PH-S35/S16/S02 LAN · **Deferred:** PH-S36/S01/S15 Cloud SDK (FM-041)

## PH-S61 — наступна сесія
1. `docs/concept/POOLAI_GALAXY_GRID.md`: seeds/locality — placement + prefetch RAM/VRAM policy
2. Telemetry signals, keep hot layers local, task-driven prefetch (concept)

## Завершення сесії
1. FM §5.11 (PH-S61 → ✅) + HANDOFF
2. Оновити цей NEXT_SESSION_PROMPT (PH-S62)
3. git push (MSYS2) + самарі

## Не повторювати
PH-S03…S60 · fee split · pricing oracle · Telegram seats/wallet · unified worker DTO · lease/re-migrate

## Черга §5.11 (відкриті)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S61** | Seeds/locality + prefetch policy |
| 2–4 | **PH-S62…S64** | Edge verification, governance, docs sync |

**Поза чергою:** PH-S35 LAN · PH-S36 Cloud SDK
```
