# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-27 · **VDT** [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — PH-S76 release advisory operator actions pointer (docs queue)

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
- HEAD: (після push PH-S79) — grid pricing API env fallback wire fix (`GalaxyPricingOracle::from_env()`)
- **PH-S03…S79:** ✅
- **Черга §5.12:** PH-S76..PH-S77 (2 відкриті)
- **BLOCKED:** PH-S35/S16/S02 LAN · **Deferred:** PH-S36/S01/S15 Cloud SDK (FM-041)

## PH-S76 — наступна сесія
1. `docs/security/SECURITY_HARDENING.md` — додати короткий operator actions pointer для release advisory (Galaxy §9.6)
2. Зв’язати з уже наявними signed advisory / update policy секціями без дублювання prose
3. Синхронізувати FM §5.12 + HANDOFF + цей prompt

## Research (черга <3 — доповнити до ≤10 PH-S*)
rg "\- \[ \]" docs/development/NEXT_STEPS_ARCHITECT_*.md
DOCS_LEGACY_AUDIT §5.3 · rg "TODO|FIXME" src/ → FM §5.12

## Завершення сесії
1. FM §5.12 (PH-S76 → ✅) + HANDOFF
2. Оновити цей NEXT_SESSION_PROMPT
3. git push (MSYS2) + самарі

## Не повторювати
PH-S03…S79 · pricing oracle stub/L2/API snapshot wire/env fallback fix · DIGEST Galaxy zріз · verify-release · protocol_version wire

## Черга §5.12 (відкриті)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S76** | Release advisory operator actions pointer |
| 2 | **PH-S77** | Security docs canonical pointer cleanup |

**Поза чергою:** PH-S35 LAN · PH-S36 Cloud SDK
```
