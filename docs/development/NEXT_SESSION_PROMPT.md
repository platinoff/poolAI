# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-27 · **VDT** [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — PH-S75 security audit-event pointers (research queue)

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
- HEAD: (після push PH-S68) — pricing oracle Rust stub
- **PH-S03…S74:** ✅
- **Черга §5.12:** PH-S75..PH-S77 (3 відкриті)
- **BLOCKED:** PH-S35/S16/S02 LAN · **Deferred:** PH-S36/S01/S15 Cloud SDK (FM-041)

## PH-S75 — наступна сесія
1. `docs/security/SECURITY_HARDENING.md` — додати concise pointers на audit events (`release_verify_ok|fail`, `protocol_negotiation_rejected`, `advisory_acknowledged`)
2. Зв’язати події з operator triage flow (без нового prose-концепту)
3. FM §5.12 + HANDOFF + `NEXT_SESSION_PROMPT` sync

## Research (черга <3 — доповнити до ≤10 PH-S*)
rg "\- \[ \]" docs/development/NEXT_STEPS_ARCHITECT_*.md
DOCS_LEGACY_AUDIT §5.3 · rg "TODO|FIXME" src/ → FM §5.12

## Завершення сесії
1. FM §5.12 (PH-S75 → ✅) + HANDOFF
2. Оновити цей NEXT_SESSION_PROMPT
3. git push (MSYS2) + самарі

## Не повторювати
PH-S03…S74 · pricing oracle stub · DIGEST Galaxy zріз · verify-release · protocol_version wire

## Черга §5.12 (відкриті)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S75** | Security hardening audit-event pointers |
| 2 | **PH-S76** | Release advisory operator actions pointer |
| 3 | **PH-S77** | Security docs canonical pointer cleanup |

**Поза чергою:** PH-S35 LAN · PH-S36 Cloud SDK
```
