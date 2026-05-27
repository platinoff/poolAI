# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-27 · **VDT** [`.cursor/rules/poolai-agent-roles.mdc`](../../.cursor/rules/poolai-agent-roles.mdc) · [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — ітераційна сесія PH-S82 (VDT, один спринт)

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
cd e2e && npm run test:ci                  # після src/ui/ або e2e/

## Стан (2026-05-27)
- HEAD: (після PH-S81) — pricing oracle FORCE_FALLBACK wire
- Закрито: PH-S03…S81 + PH-S76 + PH-S77 + PH-S90
- Відкритий sprint: PH-S82 (перший у §5.12)
- BLOCKED: PH-S35 / PH-S16 / PH-S02 (LAN)
- Deferred: PH-S36 / PH-S01 / PH-S15 (Cloud SDK, FM-041)

## PH-S82 — scope цієї сесії
1. `src/ui/` — read-only admin panel для grid pricing snapshot (`GET /api/v1/grid/pricing`)
2. Playwright smoke (task/model/unit query params)
3. `cargo fmt` → `cargo test-ci` → `e2e npm run test:ci`
4. FM §5.12 (PH-S82 → ✅) + HANDOFF + цей prompt

## Режим виконання
1. Взяти PH-S82 з черги (не повторювати закриті)
2. Мінімальний scope; MSYS2 для git/cargo
3. Commit лише файлів спринту (без git add -A)
4. Push (MSYS2) + самарі: що зроблено · тести · hash · наступний PH-S*

## Не повторювати
PH-S03…S81 + PH-S76 + PH-S77 + PH-S90 · FORCE_FALLBACK wire · L3 pricing_unavailable · Cursor rules refactor · security docs hub · pricing API wire/fallback · verify-release · protocol_version wire

## Черга §5.12 (відкриті)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | PH-S82 | Admin UI grid pricing panel |
| 2 | PH-S83 | stale-served metric |
| 3 | PH-S84 | Galaxy §4.2.3 wire note |
| 4 | PH-S85 | verify-release fixtures |
| 5 | PH-S86 | Grid pricing E2E |
| 6 | PH-S87 | INDEX security cross-link |
| 7 | PH-S88 | Release manifest sample |
| 8 | PH-S89 | L1 stale TTL metadata |

Поза чергою: PH-S35 LAN · PH-S36 Cloud SDK
```
