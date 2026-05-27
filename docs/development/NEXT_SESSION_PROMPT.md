# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-27 · **VDT** [`.cursor/rules/poolai-agent-roles.mdc`](../../.cursor/rules/poolai-agent-roles.mdc) · [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — ітераційна сесія PH-S81 (VDT, один спринт)

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

## Канон черги FM
| Розділ | Зміст |
|--------|--------|
| §5.1 | FM-* тікети |
| §5.12 | PH-S* спринти (≤10 відкритих) |
| NEXT_SESSION_PROMPT | Copy-paste старт сесії |

AUTO_RUN — лише за явним запитом (не підміняє §5.12).

## Локальний CI (канон)
cargo fmt --all
cargo test-ci
cargo run --bin poolai-openapi-gap-audit   # після змін API

## Стан (2026-05-27)
- HEAD: (після push PH-S90) — Cursor VDT agent roles + §5.12 sync
- Закрито: PH-S03…S80 + PH-S76 + PH-S77 + PH-S90
- Відкритий sprint: PH-S81 (перший у §5.12)
- BLOCKED: PH-S35 / PH-S16 / PH-S02 (LAN)
- Deferred: PH-S36 / PH-S01 / PH-S15 (Cloud SDK, FM-041)

## PH-S81 — scope цієї сесії
1. `src/grid/galaxy_pricing_oracle.rs` — `POOLAI_GALAXY_PRICING_FORCE_FALLBACK=1` → завжди L2; log/metric
2. HTTP `/api/v1/grid/pricing` — oracle `from_env()` у prod path
3. Unit tests; `cargo test-ci`
4. FM §5.12 (PH-S81 → ✅) + HANDOFF + цей prompt

## Режим виконання
1. Взяти PH-S81 з черги (не повторювати закриті)
2. Мінімальний scope; MSYS2 для git/cargo
3. Commit лише файлів спринту (без git add -A)
4. Push (MSYS2) + самарі: що зроблено · тести · hash · наступний PH-S*

## Не повторювати
PH-S03…S80 + PH-S76 + PH-S77 + PH-S90 · L3 pricing_unavailable · Cursor rules refactor · security docs hub · pricing API wire/fallback · verify-release · protocol_version wire

## Черга §5.12 (відкриті)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | PH-S81 | FORCE_FALLBACK env wire |
| 2 | PH-S82 | Admin UI grid pricing panel |
| 3 | PH-S83 | stale-served metric |
| 4 | PH-S84 | Galaxy §4.2.3 wire note |
| 5 | PH-S85 | verify-release fixtures |
| 6 | PH-S86 | Grid pricing E2E |
| 7 | PH-S87 | INDEX security cross-link |
| 8 | PH-S88 | Release manifest sample |
| 9 | PH-S89 | L1 stale TTL metadata |

Поза чергою: PH-S35 LAN · PH-S36 Cloud SDK
```
