# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-27 · **VDT** [`.cursor/rules/poolai-agent-roles.mdc`](../../.cursor/rules/poolai-agent-roles.mdc) · [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — ітераційна сесія PH-S88 (VDT, один спринт)

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
# docs-only sprint — test-ci не обов’язковий, якщо Rust не чіпали

## Стан (2026-05-27)
- HEAD: (після PH-S87 commit) — INDEX security hub cross-link
- Закрито: PH-S03…S87 + PH-S76 + PH-S77 + PH-S90
- Відкритий sprint: PH-S88 (перший у §5.12)
- BLOCKED: PH-S35 / PH-S16 / PH-S02 (LAN)
- Deferred: PH-S36 / PH-S01 / PH-S15 (Cloud SDK, FM-041)

## PH-S88 — scope цієї сесії
1. `docs/development/` — operator-facing minimal `release-manifest.json` sample для `poolai-verify-release` (Galaxy §9.2)
2. Cross-link з `tests/fixtures/release/dev/` (PH-S85); без дублювання Galaxy governance prose
3. FM §5.12 (PH-S88 → ✅) + HANDOFF + цей prompt (наступний PH-S89)

## Режим виконання
1. Взяти PH-S88 з черги (не повторювати закриті)
2. Мінімальний scope; MSYS2 для git/cargo
3. Commit лише файлів спринту (без git add -A)
4. Push (MSYS2) + самарі: що зроблено · тести · hash · наступний PH-S*

## Не повторювати
PH-S03…S87 + PH-S76 + PH-S77 + PH-S90 · INDEX hub cross-link · grid pricing E2E · verify-release fixtures · security docs hub body (PH-S69…S77)

## Черга §5.12 — наступні 10 спринтів розробки (відкриті)
| # | Sprint | Фокус | Джерело (концепт) | Тип |
|---|--------|--------|-------------------|-----|
| 1 | **PH-S88** | Release manifest sample JSON | Galaxy §9.2 manifest schema | docs |
| 2 | **PH-S89** | L1 stale TTL metadata / metrics | §4.2.3 fresh vs stale distinction | code |
| 3 | **PH-S91** | Pricing fresh-served metric | §4.2.5 `galaxy_pricing_fresh_served` | code |
| 4 | **PH-S92** | Pricing providers env catalog stub | §4.2.5 `POOLAI_GALAXY_PRICING_PROVIDERS` parser | code |
| 5 | **PH-S93** | Admin UI updates & compatibility | §9.8 protocol/verify-release panel | ui + e2e |
| 6 | **PH-S94** | Job lease fields wire stub | §4.3.1 `lease_owner/epoch/expires_at` | code |
| 7 | *(≤10 rule)* | — | — | — |
| 8 | *(≤10 rule)* | — | — | — |
| 9 | *(≤10 rule)* | — | — | — |
| 10 | *(≤10 rule)* | — | — | — |

Поза чергою: PH-S35/S16/S02 LAN · PH-S36/S01/S15 Cloud SDK
```
