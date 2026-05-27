# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-27 · **VDT** [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — PH-S80 pricing oracle L3 hard stop (code queue)

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
- HEAD: (після push PH-S77) — security docs Galaxy §9.2/§9.3/§9.6 canonical pointer hub
- **PH-S03…S79 + PH-S76 + PH-S77:** ✅
- **Черга §5.12:** PH-S80…S89 (10 відкритих)
- **BLOCKED:** PH-S35/S16/S02 LAN · **Deferred:** PH-S36/S01/S15 Cloud SDK (FM-041)

## PH-S80 — наступна сесія
1. `src/grid/galaxy_pricing_oracle.rs` — L3 hard stop when L1 stale + L2 fallback unavailable
2. Wire `503 pricing_unavailable` on `GET /api/v1/grid/pricing` and document Galaxy §4.2.4 alignment
3. Unit tests for L3 path; `cargo test-ci` + openapi-gap if API response shape changes
4. Синхронізувати FM §5.12 + HANDOFF + цей prompt

## Завершення сесії
1. FM §5.12 (PH-S80 → ✅) + HANDOFF
2. Оновити цей NEXT_SESSION_PROMPT
3. git push (MSYS2) + самарі

## Не повторювати
PH-S03…S79 + PH-S76 + PH-S77 · pricing oracle stub/L2/API snapshot wire/env fallback · security docs canonical pointer hub · advisory operator actions · DIGEST Galaxy zріз · verify-release · protocol_version wire

## Черга §5.12 (відкриті)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S80** | Pricing oracle L3 hard stop (code) |
| 2 | **PH-S81** | Pricing oracle FORCE_FALLBACK env wire |
| 3 | **PH-S82** | Admin UI grid pricing snapshot panel |
| 4 | **PH-S83** | Galaxy pricing stale-served metric |
| 5 | **PH-S84** | Galaxy §4.2.3 wire note sync (docs) |
| 6 | **PH-S85** | verify-release dev fixtures + RUN_LOCAL |
| 7 | **PH-S86** | Grid pricing E2E smoke |
| 8 | **PH-S87** | INDEX security docs cross-link |
| 9 | **PH-S88** | Release manifest sample JSON |
| 10 | **PH-S89** | Pricing oracle L1 stale TTL metadata |

**Поза чергою:** PH-S35 LAN · PH-S36 Cloud SDK
```
