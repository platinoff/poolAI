# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-27 · **VDT** [`.cursor/rules/poolai-agent-roles.mdc`](../../.cursor/rules/poolai-agent-roles.mdc) · [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — ітераційна сесія PH-S97 (VDT, один спринт)

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

## Стан (2026-05-27)
- HEAD: `c2a96e36` — PH-S95 PATCH lease epoch CAS
- Закрито: PH-S03…S96 + PH-S76 + PH-S77 + PH-S90
- Відкритий sprint: PH-S97 (останній у §5.12)
- BLOCKED: PH-S35 / PH-S16 / PH-S02 (LAN)
- Deferred: PH-S36 / PH-S01 / PH-S15 (Cloud SDK, FM-041)

## PH-S97 — scope цієї сесії
1. `src/job/` — `POOLAI_JOB_LEASE_TTL_SECS` env parse + default (Galaxy §4.3.1 `lease_ttl`)
2. Doc pointer in HANDOFF §2a; unit tests (no renew/failover wire)
3. FM §5.12 (PH-S97 → ✅) + HANDOFF + цей prompt (research replenish після закриття)

## Режим виконання
1. Взяти PH-S97 з черги
2. Мінімальний scope; MSYS2 для git/cargo
3. Commit лише файлів спринту
4. Push + самарі

## Не повторювати
PH-S03…S96 · admin jobs lease UI · PATCH lease CAS · job lease wire stub

## Черга §5.12 — відкриті
| # | Sprint | Фокус | Тип |
|---|--------|--------|-----|
| 1 | **PH-S97** | Job lease TTL env default stub | code |

Поза чергою: PH-S35/S16/S02 LAN · PH-S36/S01/S15 Cloud SDK
```
