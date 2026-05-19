# Аудит legacy-документації (менеджер функціоналу)

**Дата:** 2026-05-19 · **Після спринтів:** S21–S29 · **Канон пріоритетів:** [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) **§5.1** · **Прогрес:** [`DEVELOPMENT_PROGRESS_2026-05-19.md`](../status/DEVELOPMENT_PROGRESS_2026-05-19.md)

**Призначення:** звірка **старіших** планів/статусів у `docs/` з кодом і FM-*; не читати `[ ]` з архівних файлів для автопрогону.

**Попередній аудит:** [`AUTO_RUN_SESSION_2026-06-23.md`](./AUTO_RUN_SESSION_2026-06-23.md) (S13, січень–квітень 2026).

---

## 1. Канон (єдине джерело пріоритетів)

| Крок | Файл | Роль |
|------|------|------|
| 12 | `catalog/FUNCTION_MANAGEMENT.md` §5.1 | Наступні кроки FM-* |
| 12 | §5.3, §5.5 | «Не зроблено», прогрес % |
| 3 | `development/NEXT_STEPS_ARCHITECT_2026-03-17.md` | P1–P6, 2 відкриті чекбокси (LAN BLOCKED, cloud-sdk Deferred) |
| 4 | `development/HANDOFF_NEW_SESSION.md` | Операційний зріз сесії |
| 7 | `development/AUTO_RUN_SESSION_2026-07-01.md` | Черга спринтів S21+ |
| 11 | `catalog/FUNCTIONALITY_DIGEST_2026-04-06.md` | Витяг можливостей |
| — | `status/STABLE_STATE_SUMMARY.md` | Стабільний стан |

---

## 2. Таблиця legacy-файлів (оновлено після S29)

| Документ | Дата | Стан | Канон / дія |
|----------|------|------|-------------|
| `NEXT_STEPS_ARCHITECT_2026-03-17.md` | 03-17 | **Канон P1–P6** | 2× `[ ]`: LAN (BLOCKED), cloud-sdk (Deferred) |
| `UI_QUALITY_AND_E2E_PLAN_2026-04-06.md` | 04-06 | **Канон UX/E2E** | Playwright S23–S29 ✅; axe — backlog |
| `OPENAPI_GAP_AUDIT_2026-05-19.md` | 05-19 | **Канон OpenAPI** | v1 ✅ S28; backlog `/raid/distributed/*` |
| `E2E_PLAYWRIGHT.md` | 05-19 | **Канон E2E** | 5 specs; raid/topology — backlog |
| `UI_IMPROVEMENTS_PLAN.md` | — | **Archived** | банер ✅; FM §5.4 |
| `UI_BUGFIXES_AND_OAUTH_PLAN.md` | 01-16 | **Archived** | FM-012/FM-019 канон |
| `CONCEPT_PENDING_FEATURES.md` | 01-17 | **Archived** | ML/RAID у коді ✅ |
| `STATUS_UPDATE_2026-01-16.md` | 01-16 | **Stale** | банер ✅; FM-006 Deferred |
| `RUST_ARCHITECT_STATUS_2026-01-19.md` | 01-19 | **Stale** | банер ✅; BurstRAID опційно |
| `RUST_ARCHITECT_NEXT_STEPS_2026-01-19.md` | 01-19 | **Superseded** | банер ✅ → Architect 03-17 |
| `STABLE_STATE_UPDATE_2026-01-19.md` | 01-19 | **Stale** | банер ✅ → `STABLE_STATE_SUMMARY.md` |
| `PERCENTAGE_PLAN.md` | 2025 | **Stale** | банер ✅; % — не канон |
| `ADMIN_PANEL_STATUS.md` | 01-19 | **Stale** | банер 2026-05-19; admin + runbook + E2E |
| `UI_UX_IMPROVEMENTS_PLAN.md` | 01-21 | **Stale** | банер 2026-05-19; `admin/monitoring.rs` |
| `BUTTON_FUNCTIONS_AUDIT_2026-01-19.md` | 01-19 | **Reference** | ручна регресія |
| `docs/openapi.yaml` | — | **Partial ✅** | S14–S28; `/raid/distributed/*` backlog |
| Плоскі `docs/*.md` (~60) | різні | **Archive / ref** | [`STRUCTURE.md`](../STRUCTURE.md) §3; не кроки 1–12 |

---

## 3. Не зроблено (підтверджено FM, 2026-05-19)

| ID / область | Стан | Наступна сесія |
|--------------|------|----------------|
| **FM-003 §4** LAN sign-off | **BLOCKED** (2 хости) | runbook only |
| **FM-004** SIMD TurboQuant | **Deferred** | за запитом |
| **FM-006** Azure/GCP deep | **Deferred** | `CLOUD_SDK_STATUS.md` |
| **FM-009/010** Grid/Solana | **Concept-only** | `docs/concept/` |
| **OpenAPI** `/raid/distributed/*` | **✅ S31** (7 POST, `ProtocolMessage`) | payload schemas backlog |
| **FM-019** axe Playwright | backlog | після стабілізації E2E |
| **Playwright** raid/topology | **✅ S31** | vm/workers backlog |
| **ML ops** pipeline metrics | **✅ S31** | `PIPELINE_MANAGEMENT.md` §Ops verification |
| **P6** Grid/Job/Memory layers | Concept | Architect §7 |

**Закрито S21–S29 (не повторювати):** OpenAPI enterprise+ai-ml+gap v1; pa11y CI; UI_QUALITY P1 (27 tests); Playwright smoke + admin (tenants, monitoring, security, audit).

---

## 4. Рекомендований порядок наступних спринтів

| Порядок | Фокус | Критерій |
|--------|--------|----------|
| 1 | OpenAPI `/raid/distributed/*` | yaml + оновити `OPENAPI_GAP_AUDIT` |
| 2 | ML ops | `PIPELINE_MANAGEMENT.md` + DIGEST §ML |
| 3 | Playwright raid/topology | +1–2 specs, `E2E_PLAYWRIGHT.md` |
| — | FM-003 §4 | лише при 2 хостах |

**Не стартувати без запиту:** FM-004, FM-006, FM-009, FM-010.

---

## 5. Структурні правки (ця сесія FM)

- Банери **Stale/Archived** на файлах без попереднього банера (див. git diff).
- `development/README.md` — актуальний AUTO_RUN і §5.1.
- `STRUCTURE.md` — посилання на цей аудит.
- `INDEX_2026-03-17.md` — legacy taxonomy → цей файл.
- `FUNCTION_MANAGEMENT.md` §5.1/§5.3 — синхрон з S29 і таблицею вище.

**Політика:** нові архівні нотатки → `docs/archive/`; масове перенесення плоских `docs/*.md` — окремий інкремент.
