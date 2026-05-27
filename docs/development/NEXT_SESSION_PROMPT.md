# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-27 · **VDT** [`.cursor/rules/poolai-agent-roles.mdc`](../../.cursor/rules/poolai-agent-roles.mdc) · [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — ітераційна сесія PH-S83 (VDT, один спринт)

## Ролі (VDT)
| Роль | Хто | Дія |
|------|-----|-----|
| Власник / креативний директор | Людина | Пріоритети, BLOCKED/Deferred, push за бажанням |
| Оркестратор | Ти (Composer) | Один PH-S*; архітектура Rust; FM/HANDOFF/NEXT_SESSION; commit scope |
| Субагенти | explore · shell · generalPurpose | Вузько: docs search, cargo test-ci, один модуль |

Оркестратор НЕ делегує: git push, закриття §5.12, оновлення цього prompt.

Правила: poolai-agent-roles.mdc · virtual-development-team.mdc · poolai-session-iteration.mdc · runtime-stack-policy.mdc

## S0 (MSYS2 UCRT64 bash — обов’язково)
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI
git fetch; git status -sb; git log -1 --oneline
df -h /s | tail -1
Прочитати: HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.12 · цей файл

## Локальний CI (канон)
cargo fmt --all
cargo test-ci
cargo run --bin poolai-openapi-gap-audit   # після змін API

## Стан (2026-05-27)
- HEAD: (після PH-S82) — Admin UI grid pricing panel
- Закрито: PH-S03…S82 + PH-S76 + PH-S77 + PH-S90
- Відкритий sprint: PH-S83 (перший у §5.12)
- BLOCKED: PH-S35 / PH-S16 / PH-S02 (LAN)
- Deferred: PH-S36 / PH-S01 / PH-S15 (Cloud SDK, FM-041)

## PH-S83 — scope цієї сесії
1. `galaxy_pricing_oracle.rs` — `galaxy_pricing_stale_served` counter on L1 stale path
2. Unit or integration test
3. `cargo fmt` → `cargo test-ci`
4. FM §5.12 (PH-S83 → ✅) + HANDOFF + цей prompt

## Режим виконання
1. Взяти PH-S83 з черги (не повторювати закриті)
2. Мінімальний scope; MSYS2 для git/cargo
3. Commit лише файлів спринту (без git add -A)
4. Push (MSYS2) + самарі: що зроблено · тести · hash · наступний PH-S*

## Не повторювати
PH-S03…S82 + PH-S76 + PH-S77 + PH-S90 · Admin UI grid pricing · FORCE_FALLBACK wire · L3 pricing_unavailable · Cursor rules refactor · pricing API wire/fallback

## Черга §5.12 (відкриті)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | PH-S83 | stale-served metric |
| 2 | PH-S84 | Galaxy §4.2.3 wire note |
| 3 | PH-S85 | verify-release fixtures |
| 4 | PH-S86 | Grid pricing E2E |
| 5 | PH-S87 | INDEX security cross-link |
| 6 | PH-S88 | Release manifest sample |
| 7 | PH-S89 | L1 stale TTL metadata |

Поза чергою: PH-S35 LAN · PH-S36 Cloud SDK
```
