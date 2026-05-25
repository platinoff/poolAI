# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-25 · **HEAD** (після PH-S43) · **VDT rules** [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — PH-S45 E2E stability (vm modal + axe audit).

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
HANDOFF · FM §5.11 · concept root (напрям) · ARCHITECTURE_BEST_PRACTICES

## Локальний CI (канон — GitHub CI ігнорувати для «готово»)
cargo fmt --all
cargo test-ci
# за scope Raft: cargo test-raft-ci
# за змін API: cargo run --bin poolai-openapi-gap-audit  # 0 errors
# за змін UI/e2e: cd e2e && npm run test:ci
# бенч лише якщо спринт чіпає perf → BENCHMARKS.md / poolai_health_load

## Стан
- **PH-S03…S47, PH-S37 infra, PH-S39, PH-S44, PH-S42, PH-S43:** ✅
- **Черга §5.11:** PH-S45 → S38 → S46 → S41
- **BLOCKED:** PH-S35/S16 LAN (2 хости) · **Deferred:** PH-S36/S01 Cloud SDK (FM-041)

## PH-S45 — ця сесія
- E2E stability: VM create modal + axe `/ui/admin/audit` (`admin.spec.ts`, `a11y.spec.ts`, `E2E_PLAYWRIGHT.md`)
- Джерела: FM §5.10, PH-S44 CI gate scope

## Завершення сесії
1. Закрити PH-S45 у FM §5.11 + HANDOFF
2. Оновити NEXT_SESSION_PROMPT → наступний PH-S38
3. git push (зовнішній MSYS2) + самарі: hash, subject, test-ci, known issues
4. Не стаджити: data/audit/*.log, .commit-msg-*, bin/commit-*.sh, target/

## Не повторювати
PH-S03…S47 · PH-S37/PH-S39/PH-S44/PH-S42/PH-S43 · повний `cargo test-ci --verbose` без змін коду

## Наступні спринти (§5.11, max 10 у черзі)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S45** | E2E vm modal + axe audit ← ПОТОЧНИЙ |
| 2 | **PH-S38** | Job scheduler + on-chain epics |
| 3 | **PH-S46** | Solana on-chain program |
| 4 | **PH-S41** | macvlan (Linux) |

**Поза чергою:** PH-S35 LAN · PH-S36 Cloud SDK · PH-S40 hardware VM (великий scope)
```
