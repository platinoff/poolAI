# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-28 · **HEAD:** (PH-S107 ✅, pending commit) · VDT — [`.cursor/rules/poolai-agent-roles.mdc`](../../.cursor/rules/poolai-agent-roles.mdc) · [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc) · [`.cursor/rules/poolai-session-iteration.mdc`](../../.cursor/rules/poolai-session-iteration.mdc)

---

## Copy-paste для наступної сесії

```
PoolAI — ітераційна сесія PH-S108 (VDT, один спринт)

## Ролі (VDT)
| Роль | Хто | Дія |
|------|-----|-----|
| Власник / креативний директор | Людина | Пріоритети, BLOCKED/Deferred, push за бажанням |
| Оркестратор | Ти (Composer) | Один PH-S*; Rust/e2e/docs; FM/HANDOFF/NEXT_SESSION; commit scope |
| Субагенти | explore · shell · generalPurpose | docs search, cargo test-ci, один модуль |

Оркестратор НЕ делегує: git push, закриття §5.12, оновлення цього prompt.

Правила: poolai-agent-roles.mdc · virtual-development-team.mdc · poolai-session-iteration.mdc · runtime-stack-policy.mdc

## Режим ітерації (канон)

- **Один PH-S* за сесію** — мінімальний scope; локальний CI перед push.
- **Закриття спринту:** PH-S* → ✅ FM §5.12 + HANDOFF + цей файл (наступний PH-S*).
- **Черга:** ≤10 відкритих PH-S* у §5.12 (зараз **2**: PH-S108…S109).

## S0 (MSYS2 UCRT64 bash — обов’язково)
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI
git fetch; git status -sb; git log -1 --oneline
df -h /s | tail -1
Прочитати: HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.12 · GALAXY_GRID_ROADMAP_2026-05-27.md · цей файл

## Локальний CI (канон)
cargo fmt --all
K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci
cargo run --bin poolai-openapi-gap-audit   # після API
cd e2e && npm run test:ci                # після e2e / src/ui/

## Стан (2026-05-28)
- **HEAD:** (PH-S107 ✅ pending commit)
- **Закрито:** PH-S03…S107 + PH-S76 + PH-S77 + PH-S90
- **Відкритий sprint:** **PH-S108** (1 з 2 у §5.12)
- **BLOCKED:** PH-S35 / PH-S16 / PH-S02 (LAN)
- **Deferred:** PH-S36 / PH-S01 / PH-S15 (Cloud SDK, FM-041)

## PH-S108 — scope цієї сесії
1. `src/grid/dispatch.rs` + `src/job/` — після grid job ingest + schedule, status `leased` коли lease active
2. Unit/integration tests; `cargo test-ci`
3. FM §5.12 (PH-S108 → ✅) + HANDOFF + цей prompt + INDEX/README sync

## Режим виконання
1. Взяти PH-S108 з черги §5.12
2. MSYS2 для git/cargo; staging лише scope спринту
3. Commit + push + самарі
4. Не починати PH-S109 у тій самій сесії

## Не повторювати
PH-S03…S107 · TTL env · lease acquire/renew API · worker renew stub · `JobStatus::Leased`/`Migrating` · failover requeue stub · live pricing HTTP fetch · protocol middleware · admin lease columns/badge · PATCH lease CAS · jobs_lease E2E

## Черга §5.12 — 2 відкритих
| # | Sprint | Фокус | Тип |
|---|--------|--------|-----|
| 1 | **PH-S108** | Grid ingest → Leased on acquire | code |
| 2 | PH-S109 | Galaxy §4.3 lease wire docs sync | docs |

## Смуга PH-S100…S109 (10 спринтів Galaxy lease/protocol — орієнтир)
| Sprint | Статус | Фокус |
|--------|--------|--------|
| PH-S100 | ✅ | `JobStatus::Leased` + lifecycle |
| PH-S101 | ✅ | Failover requeue stub (expired leased → rebind) |
| PH-S102 | ✅ | Live pricing provider HTTP fetch |
| PH-S103 | ✅ | `X-PoolAI-Protocol` middleware |
| PH-S104 | ✅ | `JobStatus::Migrating` + lifecycle |
| PH-S105 | ✅ | Admin jobs lease active/expired badge |
| PH-S106 | ✅ | `poolai-worker` lease renew client stub |
| PH-S107 | ✅ | Jobs lease E2E acquire+renew |
| **PH-S108** | **відкрито** | Grid ingest → `leased` on acquire |
| PH-S109 | відкрито | §4.3 lease wire docs sync |

Поза чергою: PH-S35/S16/S02 LAN · PH-S36/S01/S15 Cloud SDK
```

---

## Короткий зріз (людина)

| | |
|---|---|
| **Наступний спринт** | PH-S108 — Grid ingest → Leased on acquire |
| **Після нього** | PH-S109 (§4.3 lease wire docs sync) |
| **Канон черги** | [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12 |
| **Роадмеп** | [`GALAXY_GRID_ROADMAP_2026-05-27.md`](./GALAXY_GRID_ROADMAP_2026-05-27.md) |
| **Handoff** | [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) |
