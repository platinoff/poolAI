# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-28 · **HEAD:** (після PH-S105 commit) · VDT — [`.cursor/rules/poolai-agent-roles.mdc`](../../.cursor/rules/poolai-agent-roles.mdc) · [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc) · [`.cursor/rules/poolai-session-iteration.mdc`](../../.cursor/rules/poolai-session-iteration.mdc)

---

```
PoolAI — ітераційна сесія PH-S106 (VDT, один спринт)

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
- **Черга:** ≤10 відкритих PH-S* у §5.12 (зараз 4: PH-S106…S109).

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

## Стан (2026-05-28)
- Закрито: PH-S03…S105 + PH-S76 + PH-S77 + PH-S90
- Відкритий sprint: **PH-S106** (1 з 4 у §5.12)
- BLOCKED: PH-S35 / PH-S16 / PH-S02 (LAN)
- Deferred: PH-S36 / PH-S01 / PH-S15 (Cloud SDK, FM-041)

## PH-S106 — scope цієї сесії
1. `src/bin/poolai-worker.rs` — lease renew client stub (`POST /api/v1/jobs/{id}/lease/renew`) when worker holds active task lease
2. Unit tests for renew loop/client behavior; no full failover logic changes
3. FM §5.12 (PH-S106 → ✅) + HANDOFF + цей prompt

## Режим виконання
1. Взяти PH-S106 з черги §5.12
2. MSYS2 для git/cargo; staging лише scope спринту
3. Commit + push + самарі
4. Не починати PH-S107 у тій самій сесії

## Не повторювати
PH-S03…S105 · TTL env · lease acquire/renew API · `JobStatus::Leased`/`Migrating` · failover requeue stub · live pricing HTTP fetch · protocol middleware · admin lease columns/badge · PATCH lease CAS stub

## Черга §5.12 — 4 відкритих
| # | Sprint | Фокус | Тип |
|---|--------|--------|-----|
| 1 | **PH-S106** | `poolai-worker` lease renew client | code |
| 2 | PH-S107 | Jobs lease E2E acquire+renew | e2e |
| 3 | PH-S108 | Grid ingest → Leased on acquire | code |
| 4 | PH-S109 | Galaxy §4.3 lease wire docs sync | docs |

Поза чергою: PH-S35/S16/S02 LAN · PH-S36/S01/S15 Cloud SDK
```
