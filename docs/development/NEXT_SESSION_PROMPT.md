# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-27 · **VDT** [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — PH-S68 pricing oracle Rust stub (research queue)

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
HANDOFF · FM §5.12

## Локальний CI (канон)
cargo fmt --all
cargo test-ci
cargo run --bin poolai-openapi-gap-audit   # після змін API

## Стан
- HEAD: (після push PH-S67) — DIGEST Galaxy modules
- **PH-S03…S67:** ✅
- **Черга §5.12:** PH-S68…S69 (2 відкритих)
- **BLOCKED:** PH-S35/S16/S02 LAN · **Deferred:** PH-S36/S01/S15 Cloud SDK (FM-041)

## PH-S68 — наступна сесія
1. Pricing oracle Rust stub (Galaxy §4.2) — unit keys, `POOLAI_GALAXY_PRICE_*` env
2. Unit tests (mock market min, floor 0.9×, cache TTL)
3. Cross-link DIGEST + concept §4.2

## Research (якщо черга <3)
rg "\- \[ \]" docs/development/NEXT_STEPS_ARCHITECT_*.md
DOCS_LEGACY_AUDIT §5.3 · rg "TODO|FIXME" src/ → FM §5.12 до ≤10 PH-S*

## Завершення сесії
1. FM §5.12 (PH-S68 → ✅) + HANDOFF
2. Оновити цей NEXT_SESSION_PROMPT
3. git push (MSYS2) + самарі

## Не повторювати
PH-S03…S67 · DIGEST Galaxy zріз · verify-release · protocol_version wire

## Черга §5.12 (відкриті)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S68** | pricing oracle Rust stub |
| 2 | **PH-S69** | SECURITY_HARDENING ↔ Galaxy §9 |

**Поза чергою:** PH-S35 LAN · PH-S36 Cloud SDK
```
