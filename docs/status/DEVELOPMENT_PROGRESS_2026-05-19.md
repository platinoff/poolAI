# Прогрес розробки PoolAI (менеджер функціоналу)

**Оновлено:** 2026-05-19 · **Гілка:** `main` · **Зріз комітів:** `285b898d` (S26)  
**Метод:** звірка FM-001…019, Architect P1–P6, `STABLE_STATE`, `FUNCTIONALITY_DIGEST`, відкриті `[ ]` у канонічних планах, `rg TODO` у `src/`, legacy docs (`docs/archive/`, січень–квітень 2026) — **не канон** для черги.

---

## Зведені показники (0–100%)

| Шар | % | Що вимірює |
|-----|---|------------|
| **A. Продукт у scope автопрогону** | **93%** | FM-001…019 без Concept-only; Deferred не в чисельнику |
| **B. Architect P1–P5 (інженерія)** | **97%** | План `NEXT_STEPS_ARCHITECT_2026-03-17.md` — закриті пріоритети 1–5 |
| **C. Повна візія (з P6 + concept)** | **79%** | Включно Grid/Solana, SIMD, cloud-sdk deep, LAN sign-off |

**Рекомендований офіційний зріз для HANDOFF / README:** **93%** (шар A) — «що реально ведемо до production без двох хостів і без on-chain».

---

## Методика шару A (FM-*)

**Чисельник (15 пунктів):** FM-001, 002, 003, 005, 007, 008, 011–019.  
**Поза чисельником:** FM-004, FM-006 (Deferred), FM-009, FM-010 (Concept-only).

| FM | Стан | Вага % | Примітка |
|----|------|--------|----------|
| FM-001 | Implemented | 100 | AppState injection tests |
| FM-002 | Implemented | 100 | Service layer |
| FM-003 | Partial | 80 | dev stand ✅; **§4 LAN BLOCKED** (ніколи не sign-off) |
| FM-005 | Implemented | 100 | JSON errors |
| FM-007–008 | Implemented | 100 | Distributed RAID wire |
| FM-011 | Implemented | 100 | `cargo test-ci` |
| FM-012 | Implemented | 100 | OAuth/Telegram/i18n |
| FM-013–015 | Implemented | 100 | 27 admin contract tests (S25–S26) |
| FM-016 | Implemented | 100 | Virtual nodes + worker + tgbot |
| FM-017–018 | Implemented | 100 | discovery errors + a11y baseline |
| FM-019 | Partial | 88 | pa11y CI ✅; axe Playwright / повний WCAG — backlog |

**Розрахунок:** (13×100 + 80 + 88) / 15 = **92.5%** → округлення **93%**.

---

## Методика шару B (Architect)

| Пріоритет | % | Відкрито |
|-----------|---|----------|
| P1 AppState/DI | 100 | — |
| P2 Service layer | 100 | — |
| P2b TurboQuant | 90 | 1 чекбокс: LAN replication + TQ01 (**BLOCKED**, 2 хости) |
| P3 ErrorContext | 100 | — |
| P4 Benchmarks | 95 | `poolai_health_load` ✅; LAN table — ops |
| P5 Cleanup/TODO | 98 | Azure/GCP SDK опційно |
| P6 Grid/Job/Memory | 0 | Concept-only |

**P1–P5 середнє:** (100+100+90+100+95+98) / 6 ≈ **97%**.

---

## Що ніколи не було зроблено (підтверджено доки + код)

### BLOCKED / інфраструктура

| ID | Пункт | Чому «ніколи» |
|----|--------|----------------|
| **FM-003 §4** | Реальний LAN: реплікація + TQ01 на 2 вузлах | Немає 2 фізичних хостів; harness ✅, sign-off у runbook — ні |

### Deferred (свідомо не стартували в автопрогоні)

| ID | Пункт | Де в коді |
|----|--------|-----------|
| **FM-004** | SIMD / нативний TurboQuant | Немає `portable_simd` / ISA paths |
| **FM-006** | Azure/GCP deep під `cloud-sdk` | `TODO` у `azure.rs`, `gcp.rs` |

### Concept-only (немає цільової реалізації в `src/`)

| ID | Пункт |
|----|--------|
| **FM-009** | Grid protocol wire envelope |
| **FM-010** | Solana / on-chain адаптер |
| **P6** | Grid / Job / Memory layers (`docs/concept/`) |

### Partial / backlog (є MVP, не закрито повністю)

| Область | Стан | Джерело |
|---------|------|---------|
| OpenAPI повний `rg` audit | Partial ✅ S14–S21 | enterprise + ai-ml; дрібні прогалини можливі |
| Playwright E2E | Smoke ✅ S23 | Розширені сценарії admin CRUD — ні |
| FM-019 axe у Playwright | Ні | pa11y CI ✅; axe — backlog |
| BurstRAID metrics v0.2+ | Ні | Stale `RUST_ARCHITECT_STATUS` |
| VM Windows AppContainer / post-spawn limits | Stub | `vm/isolation/windows.rs`, `vm/resources.rs` |
| Hardware VM isolation | Ні | `vm/mod.rs` |
| Raft membership з log/snapshot | Ні | `raid/raft.rs` |
| ML pipeline ops hardening | Partial | `PIPELINE_MANAGEMENT.md` — метрики кроків, стенд |

### Legacy docs (історичні `[ ]` — **не повторювати**)

Файли з невиконаними чекбоксами, **замінені каноном**:

- `docs/development/UI_BUGFIXES_AND_OAUTH_PLAN.md` → FM-012/FM-019 ✅
- `docs/development/STATUS_UPDATE_2026-01-16.md` → cloud-sdk Deferred
- `docs/development/CONCEPT_PENDING_FEATURES.md` → STABLE + DIGEST
- `docs/status/PERCENTAGE_PLAN.md` → **цей файл** + §5.5 FM
- `docs/development/UI_IMPROVEMENTS_PLAN.md`, `ADMIN_PANEL_STATUS.md` → архів

Повний аудит січень–квітень: [`AUTO_RUN_SESSION_2026-06-23.md`](../development/AUTO_RUN_SESSION_2026-06-23.md).

---

## Етапи розробки (оновлений зріз)

| Етап | Статус | % етапу |
|------|--------|---------|
| MVP (Stage 1) | ✅ | 100 |
| Foundation (Stage 2) | ✅ | 100 |
| Advanced (Stage 3) | ✅ | 100 |
| Enterprise + Admin UI | ✅ | 100 |
| Architect P1–P5 | ✅ майже | 97 |
| Ops / LAN / P4 benchmarks | ◆ BLOCKED ops | 85 |
| UI quality P1 contracts | ✅ S25–S26 | 100 |
| UI a11y CI (FM-019) | Partial | 88 |
| Horizon P6 / Grid / Solana | Concept | 0 |

---

## Рекомендована черга (після аудиту)

| Порядок | Спринт | Умова |
|--------|--------|--------|
| 1 | **S27** Playwright E2E розширення (admin CRUD smoke) | За [`E2E_PLAYWRIGHT.md`](../development/E2E_PLAYWRIGHT.md) |
| 2 | **S28** OpenAPI gap audit (`rg` routes vs yaml) | Без 2 хостів |
| — | **FM-003 §4** LAN sign-off | **BLOCKED** (2 хости) |
| — | FM-004/006/009/010 | Лише за явним запитом |

---

## Посилання

- Крок 12: [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.1, §5.3, §5.5  
- Наступна сесія: [`NEXT_SESSION_PROMPT.md`](../development/NEXT_SESSION_PROMPT.md)  
- Автопрогін: [`AUTO_RUN_SESSION_2026-07-01.md`](../development/AUTO_RUN_SESSION_2026-07-01.md)
