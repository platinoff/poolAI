# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-27 · **VDT** [`.cursor/rules/poolai-agent-roles.mdc`](../../.cursor/rules/poolai-agent-roles.mdc) · [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — ітераційна сесія PH-S98 (VDT, один спринт)

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
- HEAD: (після PH-S97) — job lease TTL env stub
- Закрито: PH-S03…S97 + PH-S76 + PH-S77 + PH-S90
- Відкритий sprint: PH-S98 (перший після research replenish)
- BLOCKED: PH-S35 / PH-S16 / PH-S02 (LAN)
- Deferred: PH-S36 / PH-S01 / PH-S15 (Cloud SDK, FM-041)

## PH-S98 — scope цієї сесії
1. `src/job/` — lease acquire at schedule or explicit API (Galaxy §4.3.1)
2. Populate `lease_owner` / `lease_epoch` / `lease_expires_at` using `JobLeaseConfig::from_env()`
3. FM §5.12 (PH-S98 → ✅) + HANDOFF + цей prompt

## Режим виконання
1. Взяти PH-S98 з черги
2. Мінімальний scope; MSYS2 для git/cargo
3. Commit лише файлів спринту
4. Push + самарі

## Не повторювати
PH-S03…S97 · TTL env stub · admin jobs lease UI · PATCH lease CAS · job lease wire stub

## Черга §5.12 — відкриті
| # | Sprint | Фокус | Тип |
|---|--------|--------|-----|
| 1 | **PH-S98** | Lease acquire at schedule/API | code |
| 2 | PH-S99 | Lease renew / heartbeat | code |
| 3 | PH-S100 | JobStatus::Leased + lifecycle | code |
| 4 | PH-S101 | Failover / re-migrate stub | code |
| 5 | PH-S102 | Live pricing provider fetch | code |
| 6 | PH-S103 | X-PoolAI-Protocol middleware | code |

Поза чергою: PH-S35/S16/S02 LAN · PH-S36/S01/S15 Cloud SDK
```
