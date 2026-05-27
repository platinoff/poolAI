# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-26 · **VDT** [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — PH-S62 Galaxy Grid edge verification (concept)

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
- **PH-S03…S61:** ✅
- **Черга §5.11:** PH-S62…S64 (3 відкриті; Galaxy Grid concept)
- **BLOCKED:** PH-S35/S16/S02 LAN · **Deferred:** PH-S36/S01/S15 Cloud SDK (FM-041)

## PH-S62 — наступна сесія
1. `docs/concept/POOLAI_GALAXY_GRID.md`: edge verification baseline (untrusted telegram_edge)
2. Sampling / replay / replication — без ZK (concept)

## Завершення сесії
1. FM §5.11 (PH-S62 → ✅) + HANDOFF
2. Оновити цей NEXT_SESSION_PROMPT (PH-S63)
3. git push (MSYS2) + самарі

## Не повторювати
PH-S03…S61 · fee split · pricing oracle · Telegram seats · seeds/locality · lease/re-migrate

## Черга §5.11 (відкриті)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S62** | Edge verification baseline |
| 2–3 | **PH-S63…S64** | Governance, docs sync |

**Поза чергою:** PH-S35 LAN · PH-S36 Cloud SDK
```
