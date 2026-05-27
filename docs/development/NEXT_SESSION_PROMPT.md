# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-27 · **VDT** [`.cursor/rules/poolai-agent-roles.mdc`](../../.cursor/rules/poolai-agent-roles.mdc) · [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — ітераційна сесія (VDT, research / §5.12 replenish)

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
- HEAD: *(після PH-S94 commit)* — job lease fields wire stub
- Закрито: PH-S03…S94 + PH-S76 + PH-S77 + PH-S90
- §5.12: 0 відкритих → **research replenish** (≤10 rule)
- BLOCKED: PH-S35 / PH-S16 / PH-S02 (LAN)
- Deferred: PH-S36 / PH-S01 / PH-S15 (Cloud SDK, FM-041)

## Research — scope наступної сесії
1. `rg "\- \[ \]" docs/development/NEXT_STEPS_ARCHITECT_*.md`
2. [`DOCS_LEGACY_AUDIT`](./DOCS_LEGACY_AUDIT_2026-05-19.md) §5.3
3. `rg "TODO|FIXME" src/` → нові PH-S* у FM §5.12 (code-first, ≤10 відкритих)
4. Один новий PH-S* з черги після replenish — мінімальний scope

## Режим виконання
1. Спочатку replenish §5.12 (якщо <3 відкритих), потім один PH-S*
2. MSYS2 для git/cargo; commit лише scope спринту
3. Push + самарі: що зроблено · тести · hash

## Не повторювати
PH-S03…S94 · admin updates-compat · grid pricing oracle · job lease wire stub (PH-S94)

Поза чергою: PH-S35/S16/S02 LAN · PH-S36/S01/S15 Cloud SDK
```
