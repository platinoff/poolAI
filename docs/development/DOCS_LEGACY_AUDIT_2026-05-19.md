# Аудит legacy-документації (менеджер функціоналу)

**Оновлено:** 2026-07-17 (PH-S960 band 31 close) · **Після спринтів:** S21–S29 + band 31 PH-S960…S969

**Призначення:** звірка **старіших** планів/статусів у `docs/` з кодом і FM-*; не читати `[ ]` з архівних файлів для автопрогону.

**Попередній аудит:** [`AUTO_RUN_SESSION_2026-06-23.md`](./AUTO_RUN_SESSION_2026-06-23.md) (S13, січень–квітень 2026).

---

## 1. Канон (єдине джерело пріоритетів)

| Крок | Файл | Роль |
|------|------|------|
| 12 | `catalog/FUNCTION_MANAGEMENT.md` §5.1 | Наступні кроки FM-* |
| 12 | §5.3, §5.5, **§5.11** | «Не зроблено», прогрес %, **наступні 10 PH-S*** |
| 3 | `development/NEXT_STEPS_ARCHITECT_2026-03-17.md` | P1–P6, 2 відкриті чекбокси (LAN BLOCKED, cloud-sdk Deferred) |
| 4 | `development/HANDOFF_NEW_SESSION.md` | Операційний зріз сесії |
| 7 | `development/AUTO_RUN_SESSION_2026-07-01.md` | Черга спринтів S21+ |
| 11 | `catalog/FUNCTIONALITY_DIGEST_2026-04-06.md` | Витяг можливостей |
| — | `status/STABLE_STATE_SUMMARY.md` | Стабільний стан |

---

## 2. Таблиця legacy-файлів (оновлено після S29)

| Документ | Дата | Стан | Канон / дія |
|----------|------|------|-------------|
| `NEXT_STEPS_ARCHITECT_2026-03-17.md` | 03-17 | **Канон P1–P6** | 2× `[ ]`: LAN (BLOCKED), cloud-sdk (Deferred); **PH-S963** FM §5.1 alignment banner |
| `poolAI_concept_root.txt` | 2026 | **Канон concept** | **PH-S962** de-hype zriz; історичний «100% COMPLETE» ≠ операційний зріз |
| `OPENAPI_GAP_AUDIT_2026-05-19.md` | 05-19 | **Канон OpenAPI** | v1 ✅ S28; `/raid/distributed/*` backlog; gap-audit **0** (PH-S841) |
| `UI_QUALITY_AND_E2E_PLAN_2026-04-06.md` | 04-06 | **Канон UX/E2E** | Playwright S23–S29 ✅; axe — backlog |
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
| Плоскі `docs/*.md` (~60) | різні | **Archive / ref** | **PH-S961** stale banners batch → [`INDEX_2026-03-17.md`](../INDEX_2026-03-17.md); [`STRUCTURE.md`](../STRUCTURE.md) §3 |

---

## 3. Autoprogon 100% (S33–S34) + band 31 close (PH-S964) — не повторювати

| Область | Стан |
|---------|------|
| FM-001…019, OpenAPI, pa11y, axe, Playwright admin (усі маршрути) | **✅** |
| `run-poolai`, RUN_LOCAL, Layer A+B docs | **✅** |
| **PH-S960…S969** DOCS_LEGACY audit close | **✅** band 31 (2026-07-17) |
| Flat `docs/*.md` session snapshots | **PH-S961** banners → INDEX / цей файл |
| `poolAI_concept_root.txt` de-hype | **PH-S962** zriz |
| Architect ↔ FM §5.1 | **PH-S963** alignment note |

## 4. Horizon (поза autoprogon)

| ID / область | Стан |
|--------------|------|
| **FM-003 §4** LAN sign-off | **BLOCKED** (2 хости) |
| **FM-004/006** | **Deferred** |
| **FM-009/010**, **P6** | **Concept-only** |
| **Layer C** | **100%** (S40; було 79% до Horizon S35–S40) |

**Наступна сесія:** [`NEXT_SESSION_PROMPT.md`](./NEXT_SESSION_PROMPT.md) — **PH-S970…S979** (band 32 Galaxy concept markers); канон черги **§5.12** / **§5.14**.

---

## 5. Структурні правки (band 31 PH-S960…S969, 2026-07-17)

- **PH-S960:** таблиця §2 — band 31 triage rows (Architect, concept, flat docs, OpenAPI).
- **PH-S961:** банери **Stale/не канон** на плоских `docs/*.md` session snapshots (12+ файлів; див. `FLAT_LEGACY_DOC_SAMPLES` у `docs_legacy_depth.rs`).
- **PH-S962:** `poolAI_concept_root.txt` — de-hype zriz (історичний hype ≠ STABLE/FM).
- **PH-S963:** `NEXT_STEPS_ARCHITECT_2026-03-17.md` — FM §5.1 alignment banner.
- **PH-S964:** §3 batch close (цей файл).
- **PH-S966:** `INDEX_2026-03-17.md` — крок 12 FM §5.12 pointer.

**Політика:** нові архівні нотатки → `docs/archive/`; масове перенесення плоских `docs/*.md` — окремий інкремент.
