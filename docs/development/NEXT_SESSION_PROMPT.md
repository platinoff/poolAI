# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-27 · **VDT** [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — PH-S81 pricing oracle FORCE_FALLBACK env wire (code queue)

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
- HEAD: (після push PH-S80) — pricing oracle L3 hard stop + 503 pricing_unavailable
- **PH-S03…S80 + PH-S76 + PH-S77:** ✅
- **Черга §5.12:** PH-S81…S89 (9 відкритих)
- **BLOCKED:** PH-S35/S16/S02 LAN · **Deferred:** PH-S36/S01/S15 Cloud SDK (FM-041)

## PH-S81 — наступна сесія
1. `galaxy_pricing_oracle.rs` — `POOLAI_GALAXY_PRICING_FORCE_FALLBACK=1` always L2 path + log/metric
2. Ensure HTTP `/api/v1/grid/pricing` honors force-fallback via `from_env()` oracle
3. Unit tests; `cargo test-ci`
4. FM §5.12 + HANDOFF + цей prompt

## Завершення сесії
1. FM §5.12 (PH-S81 → ✅) + HANDOFF
2. Оновити цей NEXT_SESSION_PROMPT
3. git push (MSYS2) + самарі

## Не повторювати
PH-S03…S80 + PH-S76 + PH-S77 · L3 hard stop / pricing_unavailable 503 · security docs hub · pricing API snapshot/env fallback · advisory operator actions · DIGEST Galaxy zріз

## Черга §5.12 (відкриті)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S81** | Pricing oracle FORCE_FALLBACK env wire |
| 2 | **PH-S82** | Admin UI grid pricing snapshot panel |
| 3 | **PH-S83** | Galaxy pricing stale-served metric |
| 4 | **PH-S84** | Galaxy §4.2.3 wire note sync (docs) |
| 5 | **PH-S85** | verify-release dev fixtures + RUN_LOCAL |
| 6 | **PH-S86** | Grid pricing E2E smoke |
| 7 | **PH-S87** | INDEX security docs cross-link |
| 8 | **PH-S88** | Release manifest sample JSON |
| 9 | **PH-S89** | Pricing oracle L1 stale TTL metadata |

**Поза чергою:** PH-S35 LAN · PH-S36 Cloud SDK
```
