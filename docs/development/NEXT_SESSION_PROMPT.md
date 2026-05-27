# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-27 · **VDT** [`.cursor/rules/poolai-agent-roles.mdc`](../../.cursor/rules/poolai-agent-roles.mdc) · [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — ітераційна сесія PH-S92 (VDT, один спринт)

## Ролі (VDT)
| Роль | Хто | Дія |
|------|-----|-----|
| Власник / креативний директор | Людина | Пріоритети, BLOCKED/Deferred, push за бажанням |
| Оркестратор | Ти (Composer) | Один PH-S*; Rust/docs; FM/HANDOFF/NEXT_SESSION; commit scope |
| Субагенти | explore · shell · generalPurpose | docs search, cargo test-ci, один модуль |

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
| §5.12 | PH-S* спринти (≤10 відкритих) |
| NEXT_SESSION_PROMPT | Copy-paste старт сесії |

## Локальний CI (канон)
cargo fmt --all
K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci
# після API: cargo run --bin poolai-openapi-gap-audit

## Стан (2026-05-27)
- HEAD: (після PH-S91 commit) — pricing L1 fresh-served metric
- Закрито: PH-S03…S91 + PH-S76 + PH-S77 + PH-S90
- Відкритий sprint: PH-S92 (перший у §5.12)
- BLOCKED: PH-S35 / PH-S16 / PH-S02 (LAN)
- Deferred: PH-S36 / PH-S01 / PH-S15 (Cloud SDK, FM-041)

## PH-S92 — scope цієї сесії
1. `galaxy_pricing_oracle.rs` — parse `POOLAI_GALAXY_PRICING_PROVIDERS` allow-list JSON (§4.2.5)
2. No live HTTP fetch; unit tests for parser
3. FM §5.12 (PH-S92 → ✅) + HANDOFF + цей prompt (наступний PH-S93)

## Режим виконання
1. Взяти PH-S92 з черги (не повторювати закриті)
2. Мінімальний scope; MSYS2 для git/cargo
3. Commit лише файлів спринту (без git add -A)
4. Push (MSYS2) + самарі: що зроблено · тести · hash · наступний PH-S*

## Не повторювати
PH-S03…S91 + PH-S76 + PH-S77 + PH-S90 · fresh/stale metrics · l1_cache metadata · release manifest sample · grid pricing E2E · verify-release fixtures

## Черга §5.12 — наступні 10 спринтів розробки (відкриті)
| # | Sprint | Фокус | Джерело (концепт) | Тип |
|---|--------|--------|-------------------|-----|
| 1 | **PH-S92** | Pricing providers env catalog stub | §4.2.5 `POOLAI_GALAXY_PRICING_PROVIDERS` parser | code |
| 2 | **PH-S93** | Admin UI updates & compatibility | §9.8 protocol/verify-release panel | ui + e2e |
| 3 | **PH-S94** | Job lease fields wire stub | §4.3.1 `lease_owner/epoch/expires_at` | code |
| 4 | *(≤10 rule)* | — | — | — |
| 5–10 | *(≤10 rule)* | — | — | — |

Поза чергою: PH-S35/S16/S02 LAN · PH-S36/S01/S15 Cloud SDK
```
