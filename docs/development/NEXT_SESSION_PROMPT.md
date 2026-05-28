# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-27 · **HEAD:** `ab51763b` (PH-S99) · VDT — [`.cursor/rules/poolai-agent-roles.mdc`](../../.cursor/rules/poolai-agent-roles.mdc) · [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc) · [`.cursor/rules/poolai-session-iteration.mdc`](../../.cursor/rules/poolai-session-iteration.mdc)

---

```
PoolAI — ітераційна сесія PH-S100 (VDT, один спринт)

## Ролі (VDT)
| Роль | Хто | Дія |
|------|-----|-----|
| Власник / креативний директор | Людина | Пріоритети, BLOCKED/Deferred, push за бажанням |
| Оркестратор | Ти (Composer) | Один PH-S*; Rust/docs; FM/HANDOFF/NEXT_SESSION; commit scope |
| Субагенти | explore · shell · generalPurpose | docs search, cargo test-ci, один модуль |

Оркестратор НЕ делегує: git push, закриття §5.12, оновлення цього prompt.

Правила: poolai-agent-roles.mdc · virtual-development-team.mdc · poolai-session-iteration.mdc · runtime-stack-policy.mdc

## Режим ітерації (канон)

- **Один PH-S* за сесію** — мінімальний scope; локальний CI перед push.
- **Закриття спринту:** PH-S* → ✅ FM §5.12 + HANDOFF + цей файл (наступний PH-S*).
- **Опційно (окремий коміт):** docs-sync після серії коду — README, `file_list.csv`, DIGEST, INDEX (див. poolai-session-iteration § «Приклад PH-S97…S99»).
- **Черга:** тримати **≤10** відкритих PH-S* у §5.12; replenish виконано (PH-S100…S109).

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
cargo run --bin poolai-openapi-gap-audit   # після API
cd e2e && npm run test:ci                  # після src/ui/ або e2e scope

## Стан (2026-05-27)
- HEAD: `ab51763b` — PH-S99 lease renew heartbeat API
- Закрито: PH-S03…S99 + PH-S76 + PH-S77 + PH-S90
- Відкритий sprint: PH-S100 (1 з 10 у §5.12)
- BLOCKED: PH-S35 / PH-S16 / PH-S02 (LAN)
- Deferred: PH-S36 / PH-S01 / PH-S15 (Cloud SDK, FM-041)

## PH-S100 — scope цієї сесії
1. `src/job/` — додати `JobStatus::Leased` (Galaxy §4.3.2)
2. `allows_transition` + backward compatible JSON/SQLite deserialize
3. Опційно: перехід у `Leased` при успішному lease acquire (мінімально, без Migrating)
4. FM §5.12 (PH-S100 → ✅) + HANDOFF + цей prompt

## Режим виконання
1. Взяти PH-S100 з черги §5.12
2. MSYS2 для git/cargo; staging лише scope спринту
3. Commit + push + самарі
4. Не починати PH-S101 у тій самій сесії (наступний чат → PH-S101)

## Не повторювати
PH-S03…S99 · TTL env · lease acquire/renew API · admin lease columns (PH-S96) · PATCH lease CAS stub

## Черга §5.12 — 10 відкритих (research replenish 2026-05-27)
| # | Sprint | Фокус | Тип |
|---|--------|--------|-----|
| 1 | **PH-S100** | `JobStatus::Leased` + lifecycle | code |
| 2 | PH-S101 | Failover / re-migrate stub | code |
| 3 | PH-S102 | Live pricing provider HTTP fetch | code |
| 4 | PH-S103 | `X-PoolAI-Protocol` middleware | code |
| 5 | PH-S104 | `JobStatus::Migrating` + lifecycle | code |
| 6 | PH-S105 | Admin jobs lease active/expired badge | code |
| 7 | PH-S106 | `poolai-worker` lease renew client | code |
| 8 | PH-S107 | Jobs lease E2E acquire+renew | e2e |
| 9 | PH-S108 | Grid ingest → Leased on acquire | code |
| 10 | PH-S109 | Galaxy §4.3 lease wire docs sync | docs |

Поза чергою: PH-S35/S16/S02 LAN · PH-S36/S01/S15 Cloud SDK

## Приклад закритої смуги (орієнтир для агента)
PH-S97 TTL env → PH-S98 acquire → PH-S99 renew → (опц.) docs-sync коміт → replenish §5.12 до 10 PH-S*.
Деталі: `.cursor/rules/poolai-session-iteration.mdc` § «Приклад ітерації».
```
