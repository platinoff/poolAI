# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-27 · **VDT** [`.cursor/rules/poolai-agent-roles.mdc`](../../.cursor/rules/poolai-agent-roles.mdc) · [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — ітераційна сесія PH-S96 (VDT, один спринт)

## Ролі (VDT)
| Роль | Хто | Дія |
|------|-----|-----|
| Власник / креативний директор | Людина | Пріоритети, BLOCKED/Deferred, push за бажанням |
| Оркестратор | Ти (Composer) | Один PH-S*; Rust/JS UI/docs; FM/HANDOFF/NEXT_SESSION; commit scope |
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

## Локальний CI (канон)
cargo fmt --all
K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci
cd e2e && npm run test:ci   # після src/ui/

## Стан (2026-05-27)
- HEAD: `c3355c06` — PH-S94 job lease fields wire stub
- Закрито: PH-S03…S95 + PH-S76 + PH-S77 + PH-S90
- Відкритий sprint: PH-S96 (перший у §5.12)
- BLOCKED: PH-S35 / PH-S16 / PH-S02 (LAN)
- Deferred: PH-S36 / PH-S01 / PH-S15 (Cloud SDK, FM-041)

## PH-S96 — scope цієї сесії
1. `src/ui/admin/jobs.rs` — read-only колонки lease_owner / lease_epoch / lease_expires_at (Galaxy §4.3.1)
2. i18n EN/UK; Playwright smoke в `e2e/tests/admin.spec.ts`
3. FM §5.12 (PH-S96 → ✅) + HANDOFF + цей prompt (наступний PH-S97)

## Режим виконання
1. Взяти PH-S96 з черги (не повторювати закриті)
2. Мінімальний scope; MSYS2 для git/cargo
3. Commit лише файлів спринту
4. Push + самарі: що зроблено · тести · hash · наступний PH-S97

## Не повторювати
PH-S03…S95 · PATCH lease CAS (PH-S95) · job lease wire stub (PH-S94) · admin updates-compat · grid pricing

## Черга §5.12 — відкриті
| # | Sprint | Фокус | Тип |
|---|--------|--------|-----|
| 1 | **PH-S96** | Admin jobs lease columns | ui + e2e |
| 2 | **PH-S97** | Job lease TTL env default stub | code |

Поза чергою: PH-S35/S16/S02 LAN · PH-S36/S01/S15 Cloud SDK
```
