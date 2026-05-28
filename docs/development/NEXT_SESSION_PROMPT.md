# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-27 · **VDT** [`.cursor/rules/poolai-agent-roles.mdc`](../../.cursor/rules/poolai-agent-roles.mdc) · [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

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

## Стан (2026-05-27)
- Закрито: PH-S03…S99 + PH-S76 + PH-S77 + PH-S90
- Відкритий sprint: PH-S100
- BLOCKED: PH-S35 / PH-S16 / PH-S02 (LAN)
- Deferred: PH-S36 / PH-S01 / PH-S15 (Cloud SDK, FM-041)

## PH-S100 — scope цієї сесії
1. `src/job/` — `JobStatus::Leased` + lifecycle (Galaxy §4.3.2)
2. Backward compatible JSON/SQLite; unit tests
3. FM §5.12 (PH-S100 → ✅) + HANDOFF + цей prompt

## Режим виконання
1. Взяти PH-S100 з черги
2. Мінімальний scope; MSYS2 для git/cargo
3. Commit лише файлів спринту
4. Push + самарі

## Не повторювати
PH-S03…S99 · TTL env · acquire/renew API · admin lease UI · PATCH CAS stub

## Черга §5.12 — відкриті
| # | Sprint | Фокус | Тип |
|---|--------|--------|-----|
| 1 | **PH-S100** | JobStatus::Leased + lifecycle | code |
| 2 | PH-S101 | Failover / re-migrate stub | code |
| 3 | PH-S102 | Live pricing provider fetch | code |
| 4 | PH-S103 | X-PoolAI-Protocol middleware | code |

Поза чергою: PH-S35/S16/S02 LAN · PH-S36/S01/S15 Cloud SDK
```
