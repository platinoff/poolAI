# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-25 · **HEAD** (після PH-S46) · **VDT rules** [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — PH-S41 macvlan network isolation (Linux).

## Ролі (VDT)
- Людина: власник / креативний директор — пріоритети, BLOCKED/Deferred
- Ти: оркестратор Rust — один PH-S*, субагенти для explore/shell/модуль
- Правила: virtual-development-team.mdc · poolai-session-iteration.mdc · runtime-stack-policy.mdc

## S0 (MSYS2 UCRT64 bash)
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI
git fetch; git status -sb; git log -1 --oneline
df -h /s | tail -1   # Use% ≥99% або Avail <5G → cargo clean
HANDOFF · FM §5.11 · vm/isolation/linux.rs

## Локальний CI (канон — GitHub CI ігнорувати для «готово»)
cargo fmt --all
cargo test-ci
# за scope Linux VM only: cargo test vm::isolation --features …
# за змін API: cargo run --bin poolai-openapi-gap-audit  # 0 errors

## Стан
- **PH-S03…S47, PH-S37 infra, PH-S39, PH-S44, PH-S42, PH-S43, PH-S45, PH-S38, PH-S46:** ✅
- **Черга §5.11:** PH-S41 (єдиний відкритий кодовий)
- **BLOCKED:** PH-S35/S16 LAN (2 хости) · **Deferred:** PH-S36/S01 Cloud SDK (FM-041)

## PH-S41 — ця сесія
- macvlan network isolation (Linux) — `src/vm/isolation/linux.rs`, FM §5.10
- Джерела: FM §5.11, Architect / UI plans за scope VM network

## Завершення сесії
1. Закрити PH-S41 у FM §5.11 + HANDOFF
2. Оновити NEXT_SESSION_PROMPT → research sprint (черга <3) або наступний PH-S*
3. git push (зовнішній MSYS2) + самарі: hash, subject, test-ci, known issues
4. Не стаджити: data/audit/*.log, .commit-msg-*, bin/commit-*.sh, target/

## Не повторювати
PH-S03…S47 · PH-S46 Solana adapter · PH-S37/39/44/42/43/45/38 · повний cargo test-ci --verbose без змін коду

## Наступні спринти (§5.11)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S41** | macvlan (Linux) ← ПОТОЧНИЙ |

**Поза чергою:** PH-S35 LAN · PH-S36 Cloud SDK · PH-S40 hardware VM (великий scope)
```
