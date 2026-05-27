# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-27 · **VDT** [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — PH-S65 Galaxy Grid protocol_version wire (research queue)

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
- **PH-S03…S64:** ✅ (Galaxy Grid concept arc закрито)
- **Черга §5.12:** PH-S65…S69 (5 відкритих; Galaxy wire/ops)
- **BLOCKED:** PH-S35/S16/S02 LAN · **Deferred:** PH-S36/S01/S15 Cloud SDK (FM-041)

## PH-S65 — наступна сесія
1. OpenAPI: `protocol_version` / `build_id` на worker register
2. Handler: compat matrix check (Galaxy §9.3) — `accepted` | `upgrade_required` | `unsupported`
3. Unit/integration tests

## Research (якщо черга <3)
rg "\- \[ \]" docs/development/NEXT_STEPS_ARCHITECT_*.md
DOCS_LEGACY_AUDIT §5.3 · rg "TODO|FIXME" src/ → FM §5.12 до ≤10 PH-S*

## Завершення сесії
1. FM §5.12 (PH-S65 → ✅) + HANDOFF
2. Оновити цей NEXT_SESSION_PROMPT
3. git push (MSYS2) + самарі

## Не повторювати
PH-S03…S64 · Galaxy Grid concept-only blocks · PH-S64 docs pointers

## Черга §5.12 (відкриті)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S65** | protocol_version wire |
| 2 | **PH-S66** | poolai verify-release CLI |
| 3 | **PH-S67** | DIGEST Galaxy modules |
| 4 | **PH-S68** | pricing oracle Rust stub |
| 5 | **PH-S69** | SECURITY_HARDENING ↔ Galaxy §9 |

**Поза чергою:** PH-S35 LAN · PH-S36 Cloud SDK
```
