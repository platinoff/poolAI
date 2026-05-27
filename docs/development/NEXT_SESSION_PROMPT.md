# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-27 · **VDT** [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — PH-S64 Galaxy Grid docs sync (canonical pointers)

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
cargo test-ci   # якщо зміни в src/; для чистих docs — опційно

## Стан
- **PH-S03…S63:** ✅
- **Черга §5.11:** PH-S64 (1 відкритий; Galaxy Grid docs sync)
- **BLOCKED:** PH-S35/S16/S02 LAN · **Deferred:** PH-S36/S01/S15 Cloud SDK (FM-041)

## PH-S64 — наступна сесія
1. Короткі посилання на `docs/concept/POOLAI_GALAXY_GRID.md` у README, `docs/README.md`, `docs/INDEX_2026-03-17.md`, `docs/STRUCTURE.md` (без розростання таблиць)
2. Перевірити vision manifest після docs sync

## Завершення сесії
1. FM §5.11 (PH-S64 → ✅) + HANDOFF
2. Оновити цей NEXT_SESSION_PROMPT (research sprint / нова черга)
3. git push (MSYS2) + самарі

## Не повторювати
PH-S03…S63 · Galaxy Grid concept blocks (fee, pricing, Telegram, lease, locality, verify, governance)

## Черга §5.11 (відкриті)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S64** | Docs sync — README/INDEX/STRUCTURE pointers |

**Поза чергою:** PH-S35 LAN · PH-S36 Cloud SDK
```
