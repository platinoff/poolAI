# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-26 · **VDT** [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — PH-S63 Galaxy Grid open-source governance (concept)

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
- **PH-S03…S62:** ✅
- **Черга §5.11:** PH-S63…S64 (2 відкриті; Galaxy Grid concept)
- **BLOCKED:** PH-S35/S16/S02 LAN · **Deferred:** PH-S36/S01/S15 Cloud SDK (FM-041)

## PH-S63 — наступна сесія
1. `docs/concept/POOLAI_GALAXY_GRID.md` (або окремий governance doc): signed releases + protocol versioning
2. Оновлення без «root супер-адміна»: підписи, compat matrix, opt-in auto-update (concept)

## Завершення сесії
1. FM §5.11 (PH-S63 → ✅) + HANDOFF
2. Оновити цей NEXT_SESSION_PROMPT (PH-S64)
3. git push (MSYS2) + самарі

## Не повторювати
PH-S03…S62 · Galaxy Grid concept blocks (fee, pricing, Telegram, lease, locality, verify)

## Черга §5.11 (відкриті)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S63** | Governance + signed releases |
| 2 | **PH-S64** | Docs sync (README/INDEX pointers) |

**Поза чергою:** PH-S35 LAN · PH-S36 Cloud SDK
```
