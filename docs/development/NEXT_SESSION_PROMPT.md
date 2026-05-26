# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-25 · **HEAD** (після PH-S41) · **VDT rules** [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — PH-S48 job RAID store.

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
HANDOFF · FM §5.11 · vm/mod.rs

## Локальний CI (канон)
cargo fmt --all
cargo test-ci
# за змін API: cargo run --bin poolai-openapi-gap-audit  # 0 errors

## Стан
- **PH-S03…S47, PH-S41, PH-S46, PH-S37/39/44/42/43/45/38:** ✅
- **Черга §5.11:** PH-S48
- **BLOCKED:** PH-S35/S16 LAN · **Deferred:** PH-S36/S01 Cloud SDK (FM-041)

## PH-S48 — рекомендована наступна сесія
- Job store RAID-backed persistence (Architect deferred)

## Завершення сесії
1. Закрити спринт у FM §5.11 + HANDOFF
2. Оновити NEXT_SESSION_PROMPT
3. git push (MSYS2) + самарі

## Не повторювати
PH-S03…S47 · PH-S41 macvlan · PH-S46 Solana · повний test-ci --verbose без змін коду

## Черга §5.11
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S48** | Job store RAID-backed |

**Поза чергою:** PH-S35 LAN · PH-S36 Cloud SDK
```
