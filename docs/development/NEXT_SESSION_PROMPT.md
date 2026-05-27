# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-27 · **VDT** [`.cursor/rules/poolai-agent-roles.mdc`](../../.cursor/rules/poolai-agent-roles.mdc) · [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — ітераційна сесія PH-S94 (VDT, один спринт)

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
# після API/UI: e2e npm run test:ci

## Стан (2026-05-27)
- HEAD: *(після PH-S93 commit)* — admin updates & compatibility panel
- Закрито: PH-S03…S93 + PH-S76 + PH-S77 + PH-S90
- Відкритий sprint: PH-S94 (перший у §5.12)
- BLOCKED: PH-S35 / PH-S16 / PH-S02 (LAN)
- Deferred: PH-S36 / PH-S01 / PH-S15 (Cloud SDK, FM-041)

## PH-S94 — scope цієї сесії
1. `src/job/` — optional lease fields wire stub: `lease_owner`, `lease_epoch`, `lease_expires_at` (Galaxy §4.3.1)
2. Backward compatible JSON/SQLite/RAID store paths; unit tests
3. FM §5.12 (PH-S94 → ✅) + HANDOFF + цей prompt (наступний спринт з §5.12 або research)

## Режим виконання
1. Взяти PH-S94 з черги (не повторювати закриті)
2. Мінімальний scope; MSYS2 для git/cargo
3. Commit лише файлів спринту (без git add -A)
4. Push (MSYS2) + самарі: що зроблено · тести · hash · наступний sprint

## Не повторювати
PH-S03…S93 + PH-S76 + PH-S77 + PH-S90 · admin updates-compat UI · grid pricing · pricing oracle metrics/catalog

## Черга §5.12 — відкриті спринти
| # | Sprint | Фокус | Джерело (концепт) | Тип |
|---|--------|--------|-------------------|-----|
| 1 | **PH-S94** | Job lease fields wire stub | §4.3.1 `lease_owner/epoch/expires_at` | code |
| 2 | *(≤10 rule)* | — | — | — |

Поза чергою: PH-S35/S16/S02 LAN · PH-S36/S01/S15 Cloud SDK
```
