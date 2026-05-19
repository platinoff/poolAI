# Керування функціоналом PoolAI (індекс, прогалини, тікети)

**Оновлено:** 2026-05-19 (§5.1 backlog S21+; [`AUTO_RUN_SESSION_2026-07-01.md`](../development/AUTO_RUN_SESSION_2026-07-01.md)).

**Зріз комітів (червень 2026):** FM-017/018 ✅; **FM-019 baseline** ✅ (modals, forms, tabs, tables, [`ADMIN_A11Y_RUNBOOK.md`](../development/ADMIN_A11Y_RUNBOOK.md)); pushes `02ea146`…`31266be9` на `main`.

**Не зроблено / backlog:** FM-003 §4 LAN (**BLOCKED**); FM-019 повний WCAG/pa11y CI; FM-004/006 **Deferred**; FM-009/010 **Concept-only**. **P4:** рядок `poolai_health_load` **2026-05-18** у `BENCHMARKS.md` (baseline **2026-04-10** лишається для порівняння).

**Останній `cargo test-ci`:** 2026-05-18 (FM-019 pa11y S6/S7); clippy — CI на `main`.

**Роль документа:** операційна інструкція для людини й агента («менеджер функціоналу»): звірка з **сталевим станом**, пошук **недоробленого**, пріоритизація, **чернетки тікетів** для передачі в розробку.

**Пов’язані кроки канону:** [крок 11 — витяг](./FUNCTIONALITY_DIGEST_2026-04-06.md) · **крок 12 — цей файл** (керування та беклог).

---

## 1. Джерела правди (порядок звірки)

| Порядок | Джерело | Що дає |
|--------|---------|--------|
| 1 | [`docs/status/STABLE_STATE_SUMMARY.md`](../status/STABLE_STATE_SUMMARY.md) | Що задекларовано як стабільне / 100% / CI. |
| 2 | [`docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`](./FUNCTIONALITY_DIGEST_2026-04-06.md) | Структурований перелік підсистем, features, HTTP-шару. |
| 3 | Код | `src/network/api/`, `src/network/mod.rs`, `src/services/`, `Cargo.toml` (`[features]`, `[[test]]`) — фактичні маршрути й опції збірки. |
| 4 | [`docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md`](../development/NEXT_STEPS_ARCHITECT_2026-03-17.md) | Архітектурний беклог P1–P6 (відкриті чекбокси = офіційні наступні кроки). |
| 5 | [`docs/development/HANDOFF_NEW_SESSION.md`](../development/HANDOFF_NEW_SESSION.md) | Короткий операційний фокус сесії. |
| 6 | Концепти | [`GRID_PROTOCOL_CONCEPT_2026-04-06.md`](../development/GRID_PROTOCOL_CONCEPT_2026-04-06.md), [`SOLANA_ADAPTER_CONCEPT_2026-04-06.md`](../development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md), [`concept/poolAI_concept_root.txt`](../concept/poolAI_concept_root.txt) — **наміри без гарантії коду**. |
| 7 | Історія | `docs/archive/`, `git log`, старі плани в `docs/development/` — якщо пункт зник із актуального плану, але згадується в концепті. |

**OpenAPI** (`docs/openapi.yaml`) — довідково; може відставати від роутерів.

---

## 2. Класифікація пунктів

| Тег | Значення |
|-----|----------|
| **Implemented** | Є в коді + згадка в DIGEST / STABLE; покрито тестами або явно позначено як MVP. |
| **Partial** | Є API; частина логіки спрощена або без повного wire (наприклад sync без remote metadata для conflicts). |
| **Planned** | Відкритий пункт у `NEXT_STEPS_ARCHITECT` або README Next Focus. |
| **Concept-only** | Описано в концепті / протоколі; немає цільової реалізації в `src/`. |
| **Deferred** | `cloud-sdk`, SIMD, on-chain — явно optional у планах. |

---

## 3. Процедура для агента / PM (крок за кроком)

1. Прочитати **STABLE_STATE** і **FUNCTIONALITY_DIGEST** — побудувати список «оголошених» можливостей.
2. За потреби вибірково звірити **критичні маршрути** у `src/network/` (не вичитувати весь репозиторій).
3. Пройти **відкриті чекбокси** в `NEXT_STEPS_ARCHITECT` — перенести в таблицю беклогу або оновити існуючі `FM-*`.
4. Для кожної **прогалини** між концептом і кодом — записати: джерело концепту, поточний код (файл/модуль), ризик, пріоритет.
5. Якщо невідомо, чи колись було зроблено — **`git log -S"keyword"`** або пошук по `docs/archive/`.
6. Додати **тікет** (нижче) з посиланнями на доки та (за можливості) шлях у коді.
7. Після реалізації фічі — оновити **FUNCTIONALITY_DIGEST** і за потреби один рядок у **STABLE_STATE** / HANDOFF.

---

## 4. Шаблон тікета (GitHub / внутрішній)

Скопіюй і заповни:

```markdown
## Title
[Area] Коротка дія (наприклад: RAID: wire-реплікація TurboQuant на стенді)

## Type
Planned | Partial | Concept → Code | Tech-debt | Docs

## Priority
P1 / P2 / P2b / P3 / P4 / Deferred

## Sources
- Plan: …
- Concept: …
- Code: `path/to/file.rs`

## Current behavior
…

## Desired behavior
…

## Acceptance criteria
- [ ] …
- [ ] Тести / бенч / док оновлено

## FM-ID
FM-xxx (з таблиці нижче)
```

---

## 5. Чернетка беклогу (FM-*)

Приклади з **актуальних планів і коду**; переносити в Issues при плануванні спринту. Статус періодично звіряти з `NEXT_STEPS_ARCHITECT`.

| ID | Область | Короткий опис | Стан за каноном | Джерело |
|----|---------|---------------|-----------------|---------|
| FM-001 | P1 / тести | Інтеграційні тести: повний nest `/api/v1` + інжектований `AppState` (`attach_*_for_test`), без `raid::`/`vm::` globals | Implemented | `tests/appstate_http_injection_integration.rs`, CI `--features …,test-utils` |
| FM-002 | P2 | Доробити service layer: тонкі handler’и, логіка в `services/*` для решти доменів | Implemented | `src/network/api/*` без `get_global_*`; `discovery.rs` — optional `instance_manager` з `AppState` (коментар); `services/*` — лише закоментовані Raft placeholders |
| FM-003 | P2b / RAID | Dev stand: `run-virtual-node-dev.*`, `verify-dev-stand.*` (health + VN bootstrap). **Реальний LAN** §4 — gated (два хости) | Planned (ops, gated) | `LAN_BENCHMARK_RUNBOOK.md` §5.1 |
| FM-004 | ML | SIMD / прискорений шлях TurboQuant у Rust | Deferred | NEXT_STEPS P2b |
| FM-005 | P3 | Узгоджений JSON-помилок: `HttpAppError` / `AppError::RestError` (без зміни стабільних `error.code`) | Implemented | NEXT_STEPS P3; зроблено: **`ui`**, **`users`**, **`ai_ml`**, **`workers`**, **`instances`**, **`libraries`**, **`vm`**, **`topology`**, **`rewards`**, **`system`**, **`completions`**, **`admin`**, **`raid`**, **`raid_admin`**, **`raid_http`**, **`network/enterprise_api/`**, **`authenticate_user`** / **`refresh_access_token`** / **`SystemService::login`**, **`check_permission`**, **`auth_middleware`**, **`permission_middleware`** |
| FM-006 | Cloud | Реалізація відкладених гілок Azure/GCP під `cloud-sdk` (credential/compute/location тощо) | Partial / Deferred | P5, `src/cloud/providers/azure.rs`, `gcp.rs` |
| FM-007 | Distributed RAID | Sync: порівняння локального каталогу з peer `artifact_ids` за напрямком (Pull/Push/Bidirectional); за наявності `remote_versions` (`artifact_id` -> timestamp) формується `conflicts` | Implemented | `RaidDistributedProtocolService::sync_artifacts`, `diff_sync_catalog`, `build_sync_conflicts`; wire-тести **`tests/distributed_raid_wire_integration.rs`** (15 tests, 2026-05-16) |
| FM-008 | Distributed RAID | LeaveCluster: `graceful` — `replicate_stored_artifact` по всіх локальних артефактах, далі `delete_worker`; якщо немає peer-вузлів і є артефакти — `replication_complete=false`; помилки membership / невалідний `node_id` | Implemented | Membership + graceful/non-graceful leave; wire-тести у **`tests/distributed_raid_wire_integration.rs`** |
| FM-009 | Grid | Єдиний wire envelope для Grid protocol (згадано як залишок P6) | Concept-only | GRID_PROTOCOL_CONCEPT |
| FM-010 | Tokenization | On-chain прототип / crate Solana за адаптер-концептом | Concept-only | SOLANA_ADAPTER_CONCEPT |
| FM-011 | Ops | MSVC: **`[profile.test] debug = 1`** у `Cargo.toml` (менший PDB, обхід LNK1318); alias **`cargo test-ci`** у **`.cargo/config.toml`** = CI-прогін (`--lib` + `--tests`, без doctests) + **`K8S_OPENAPI_ENABLED_VERSION=1.28`**; clippy матриці як у `ci.yml` — на `main` (2026-04-10); локально **`cargo test-ci`** — 2026-05-16; повний `cargo test` з doctests на Windows може дати **os error 1455** | Implemented | `Cargo.toml`, `.cargo/config.toml`, HANDOFF, NEXT_STEPS |
| FM-012 | UI / Auth UX | Апгрейд `/ui` і `/ui/admin/*`: i18n **UA/EN**; **Telegram OAuth** — HMAC/`auth_date`/allowlist/audit; widget HTML UA/EN; нові Telegram-юзери → **Viewer** (без `admin:all`); тести allowlist/expiry/RBAC | Implemented | `src/ui/i18n_core.js`, `src/ui/mod.rs`, `src/ui/admin/`, `src/network/enterprise_api/oauth.rs`, `src/enterprise/security.rs` |
| FM-013 | UI / Admin API | Контрактні тести JSON для admin-сторінок: libraries, topology, VM, workers; узгодження `installed` у libs UI з `metadata.installed_at` | Implemented | `tests/admin_ui_api_contracts.rs`, [`ADMIN_UI_JSON_CONTRACTS.md`](../development/ADMIN_UI_JSON_CONTRACTS.md), `src/ui/admin/libs.rs` |
| FM-014 | UI / Admin API | Фаза 2 контрактів: `GET /config`, `GET /users`, `GET /topology/nodes`; rewards API → `HttpAppError` (FM-005) | Implemented | `tests/admin_ui_api_contracts.rs`, `src/network/api/rewards.rs` |
| FM-015 | UI / Admin API | Фаза 3: `GET /instance`, `GET /raid/artifacts`, `GET /raid/admin/metrics/smallworld` (20 contract tests) | Implemented | `tests/admin_ui_api_contracts.rs`, `src/ui/admin/instances.rs`, `src/ui/admin/raid.rs` |
| FM-016 | Workers / Telegram | **Virtual nodes** + Telegram: bind/webhook/store, `poolai-worker`, **`poolai-telegram-bot`** (`--features tgbot`); pool workload на device | Implemented | `virtual_node_*`, `tgbot/coordinator`, `poolai-telegram-bot`, integration tests |
| FM-017 | P3 / HTTP | **FM-005 залишок:** `discovery` → `HttpAppError` JSON; `virtual_nodes` — status-only (worker); `admin` — `AppError` ✅ | Implemented | `discovery.rs`; `virtual_nodes.rs` worker-safe; `tests/discovery_remote_register_integration.rs` |
| FM-018 | UI / a11y | Admin/login skip links, focus-visible, aria-live, aria-current | Implemented | `admin/mod.rs`, `admin_styles.css`, `admin_common.js`, login `mod.rs`; `admin::a11y_tests` |
| FM-019 | UI / a11y | **Baseline:** admin modals/forms/tabs, dashboard nav, runbook; повний WCAG/pa11y CI — backlog | Baseline Implemented | [`ADMIN_A11Y_RUNBOOK.md`](../development/ADMIN_A11Y_RUNBOOK.md), §5.4 |

### 5.1 Пріоритезовані наступні кроки (зведення FM-* і Architect-плану)

**Канон черги** — таблиця нижче; детальний аудит «що не зроблено» — **§5.3**.

| Порядок | Фокус | FM / план | Дія |
|--------|--------|-----------|-----|
| 1 | Real LAN sign-off | **FM-003 §4** | **BLOCKED** — 2 фізичні хости; dev stand §5.1 + `verify-dev-stand` ✅ |
| 2 | UI a11y | **FM-019** | **Partial ✅** S7–S12: 18 auth, WCAG22, `a11y.yml`, `pa11y-contract`; E2E Playwright — backlog |
| 2b | UI E2E | **UI_QUALITY plan** | Playwright smoke — backlog (див. §5.3 legacy) |
| 3 | ML ops | **DIGEST §ML** | ✅ runbook у `PIPELINE_MANAGEMENT.md` |
| — | Ops / benchmarks | **P4** | ✅ рядок `poolai_health_load` **2026-05-18** (`win10-local-26200`); **2026-04-10** — історичний baseline |
| 4 | Відкладено | **FM-004**, **FM-006** | SIMD TurboQuant; Azure/GCP `cloud-sdk` — поза автопрогоном |
| 5 | Концепт | **FM-009**, **FM-010** | Grid / Solana — поза автопрогоном |

**Закрито (не в черзі):** FM-001–018; FM-019 baseline; FM-005 admin/enterprise/raid/auth JSON.

**Якість збірки:** **`cargo test-ci`** + `cargo fmt` — зріз 2026-05-28; clippy — CI (`ci.yml`).

**Відкриті чекбокси** у [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](../development/NEXT_STEPS_ARCHITECT_2026-03-17.md): LAN-заміри (≈ FM-003 §4, BLOCKED); cloud-sdk (FM-006, Deferred).

### 5.3 Звірка «не зроблено» (менеджер функціоналу, 2026-05-18)

#### Зроблено в автопрогоні (червень 2026) — не повторювати

| Спринт | Результат | Коміти (орієнтир) |
|--------|-----------|-------------------|
| FM-019 modals | focus trap, `aria-modal`, `adminDynamicModal` | `02ea146`, `7d500db0` |
| FM-019 forms | `adminEnhanceFormA11y`, users/instances | `cf431a79` |
| FM-019 tabs/tables | tablist ARIA, `adminObserveDynamicA11y` | `d04088e8` |
| FM-019 baseline docs | `ADMIN_A11Y_RUNBOOK.md`, §5.4 | `31266be9` |

#### Не зроблено (канон backlog)

| Джерело | Пункт | Стан | Примітка |
|---------|--------|------|----------|
| Architect L123 | LAN replication + TQ01 на стенді | **BLOCKED** | FM-003 §4; 2 фізичні хости; wire harness ✅ |
| Architect L183 | Azure/GCP `cloud-sdk` | **Deferred** | FM-006; `TODO` у `azure.rs`/`gcp.rs` |
| FM-003 | §4 acceptance у runbook | **BLOCKED** | §5.1 dev stand ✅; ops **2026-06-01** |
| FM-004 | SIMD TurboQuant | **Deferred** | поза автопрогоном |
| FM-006 | cloud-sdk гілки | **Deferred** | поза автопрогоном |
| FM-009/010 | Grid / Solana | **Concept-only** | поза автопрогоном |
| FM-019 | pa11y/axe у CI | **Partial ✅** | S22: `ci.yml` `pa11y-wcag22` (paths-filter) + `pa11y-contract`; `PA11Y_WCAG22=1`; 18 auth URLs |
| P4 | `poolai_health_load` → `BENCHMARKS.md` | **Implemented (ops)** | рядок **2026-05-18**; baseline **2026-04-10** для порівняння |
| `UI_IMPROVEMENTS_PLAN` | Історичні `[ ]` | **Archived** | S4 2026-05-18; канон §5.4 + runbook §3.1 |
| `UI_BUGFIXES_AND_OAUTH_PLAN` | Модалки 2026-01 | **Archived** | S7 2026-05-18; FM-012/FM-019 канон |
| `CONCEPT_PENDING_FEATURES.md` | «ML не реалізовано» | **Archived** | S11 2026-05-18; канон STABLE + DIGEST |
| `HANDOFF` §5 | Посилання на AUTO_RUN 2026-05-17 | **Fixed 2026-06-07** | → AUTO_RUN 2026-06-08 |
| OpenAPI | Синхронізація при нових маршрутах | **Partial ✅** | S14–S21: enterprise REST + `/ai-ml/optimization*`, `/automl`, `/federated`; Playwright/E2E — backlog |
| `docs/archive/*` | Legacy `.md` | **Archive** | [`STRUCTURE.md`](../STRUCTURE.md) |
| `STATUS_UPDATE_2026-01-16.md` | Cloud SDK `[ ]` | **Stale** | FM-006 Deferred; канон CI |
| `RUST_ARCHITECT_STATUS_2026-01-19.md` | BurstRAID metrics `[ ]` | **Stale** | опційно v0.2+ |
| `STABLE_STATE_UPDATE_2026-01-19.md` | % / cloud tests | **Stale** | → `STABLE_STATE_SUMMARY.md` |
| `PERCENTAGE_PLAN.md` | GlobalState % | **Stale** | не канон |
| `ADMIN_PANEL_STATUS.md` | Admin partial | **Stale** | код + `ADMIN_A11Y_RUNBOOK` |
| `UI_UX_IMPROVEMENTS_PLAN.md` | Monitoring UI | **Stale** | звірити `admin/monitoring.rs` |
| `RUST_ARCHITECT_NEXT_STEPS_2026-01-19.md` | Дубль плану | **Superseded** | → Architect 2026-03-17 |

#### Legacy docs (повна таблиця)

Див. [`AUTO_RUN_SESSION_2026-06-23.md`](../development/AUTO_RUN_SESSION_2026-06-23.md) — аудит січень–квітень 2026 планів.

#### Рекомендований наступний спринт (2026-05-19, після OpenAPI S14–S20)

Повний backlog — [`AUTO_RUN_SESSION_2026-07-01.md`](../development/AUTO_RUN_SESSION_2026-07-01.md).

| Порядок | Спринт | Фокус |
|--------|--------|--------|
| 1 | **S23** | Playwright smoke (опційно) |
| — | **S21** | ✅ OpenAPI `/ai-ml/optimization*`, `automl`, `federated` (2026-05-19) |
| — | **S22** | ✅ FM-019 CI merge gate (`pa11y-wcag22` + contract, 2026-05-19) |
| 2 | **S24** | `DELETE /ui/dashboards` або UI_QUALITY P1 |
| — | **FM-003 §4** | **BLOCKED** (2 хости) |

**Не стартувати без запиту:** FM-004, FM-006, FM-009, FM-010.

### 5.4 FM-019 baseline (вже в коді; runbook 2026-06-07)

**Верифікація:** [`ADMIN_A11Y_RUNBOOK.md`](../development/ADMIN_A11Y_RUNBOOK.md) — `cargo test-ci`, `ui::admin` tests, ручна клавіатура, опційно `pa11y`.

| Можливість | Де |
|------------|-----|
| Skip links (dashboard, admin, login) | `src/ui/mod.rs`, `admin/mod.rs` |
| Ctrl+K global search, Esc modals/drawer | `src/ui/mod.rs` keyboard handlers |
| `aria-live` notifications / errors | dashboard, admin, login |
| `aria-current` nav (admin + dashboard) | `adminMarkCurrentNav`, `dashMarkCurrentNav` |
| Admin modals focus trap + closed `aria-modal` | `admin_common.js`; static modals у `src/ui/admin/*.rs` |
| Admin forms `aria-required`, tablist ARIA | `adminEnhanceFormA11y`, security/config `role="tab*"` |
| Dynamic tables `scope=col` | `adminEnhanceTablesA11y` + `adminObserveDynamicA11y` |
| i18n UA/EN shell | `i18n_core.js` |

### 5.2 Автономний прогін (сесія → git push)

**Завершено:** [`AUTO_RUN_SESSION_2026-05-29.md`](../development/AUTO_RUN_SESSION_2026-05-29.md) (FM-017 discovery HttpAppError, partial).

**Завершено:** [`AUTO_RUN_SESSION_2026-06-08.md`](../development/AUTO_RUN_SESSION_2026-06-08.md) (P4 `poolai_health_load` **2026-05-18**).

**Завершено:** [`AUTO_RUN_SESSION_2026-06-09.md`](../development/AUTO_RUN_SESSION_2026-06-09.md) (FM-019 dashboard modals `08c704fe`).

**Завершено:** [`AUTO_RUN_SESSION_2026-06-10.md`](../development/AUTO_RUN_SESSION_2026-06-10.md) (pa11y CI `8c5dc1df`).

**Поточний:** [`AUTO_RUN_SESSION_2026-06-23.md`](../development/AUTO_RUN_SESSION_2026-06-23.md) (§5.3 legacy docs audit S13).

**Завершено:** [`AUTO_RUN_SESSION_2026-06-22.md`](../development/AUTO_RUN_SESSION_2026-06-22.md) (FM-019 pa11y-contract S12 `e9729152`).

**Завершено:** [`AUTO_RUN_SESSION_2026-06-16.md`](../development/AUTO_RUN_SESSION_2026-06-16.md) (FM-019 pa11y S6 `73c702a9`).

**Завершено:** [`AUTO_RUN_SESSION_2026-06-15.md`](../development/AUTO_RUN_SESSION_2026-06-15.md) (FM-019 pa11y S5 `e368ba11`).

**Завершено:** [`AUTO_RUN_SESSION_2026-06-14.md`](../development/AUTO_RUN_SESSION_2026-06-14.md) (FM-019 docs S4 `c1b2b24e`).

**Завершено:** [`AUTO_RUN_SESSION_2026-06-13.md`](../development/AUTO_RUN_SESSION_2026-06-13.md) (FM-019 ops + test-utils gates `d70c3d33`).

**Завершено:** [`AUTO_RUN_SESSION_2026-06-12.md`](../development/AUTO_RUN_SESSION_2026-06-12.md) (FM-019 pa11y tune `ded58c10`).

**Завершено:** [`AUTO_RUN_SESSION_2026-06-11.md`](../development/AUTO_RUN_SESSION_2026-06-11.md) (FM-019 auth fixture).

**Завершено (partial):** FM-019 pa11y auth — `PA11Y_ADMIN_STRICT`, `run_pa11y_authenticated`.

**Завершено:** [`AUTO_RUN_SESSION_2026-06-07.md`](../development/AUTO_RUN_SESSION_2026-06-07.md) (FM-019 baseline + runbook).

**Завершено:** [`AUTO_RUN_SESSION_2026-06-06.md`](../development/AUTO_RUN_SESSION_2026-06-06.md) (semantic tabs + tables).

**Завершено:** [`AUTO_RUN_SESSION_2026-06-02.md`](../development/AUTO_RUN_SESSION_2026-06-02.md) (§5.3 audit + FM-019 nav).

**Завершено:** [`AUTO_RUN_SESSION_2026-06-01.md`](../development/AUTO_RUN_SESSION_2026-06-01.md) (FM-003 §4 BLOCKED ops).

**Завершено:** [`AUTO_RUN_SESSION_2026-05-31.md`](../development/AUTO_RUN_SESSION_2026-05-31.md) (DIGEST §ML metrics ✅).

**Завершено:** [`AUTO_RUN_SESSION_2026-05-30.md`](../development/AUTO_RUN_SESSION_2026-05-30.md) (FM-018 a11y ✅).

**Завершено:** [`AUTO_RUN_SESSION_2026-05-28.md`](../development/AUTO_RUN_SESSION_2026-05-28.md) (ops hygiene, test-ci).

**Завершено:** [`AUTO_RUN_SESSION_2026-05-27.md`](../development/AUTO_RUN_SESSION_2026-05-27.md) (FM-012 OAuth).

**Попередні:** [`AUTO_RUN_SESSION_2026-05-25.md`](../development/AUTO_RUN_SESSION_2026-05-25.md) (FM-016+++), [`AUTO_RUN_SESSION_2026-05-24.md`](../development/AUTO_RUN_SESSION_2026-05-24.md).

| Спринт | FM | Результат для «100% продукту» |
|--------|-----|-------------------------------|
| S1 | FM-012 | ✅ Telegram/OAuth → **Implemented** (2026-05-16) |
| S2 | FM-007, FM-008 | ✅ 15 wire-тестів → **Implemented** |
| S3 | FM-002 | ✅ `api/` без `get_global_*` → **Implemented** |
| S4 | FM-003 | ✅ baseline 2026-04-10 + [`LAN_BENCHMARK_RUNBOOK.md`](../performance/LAN_BENCHMARK_RUNBOOK.md) — **Planned (ops)** |
| S5 | FM-011 | ✅ `cargo test-ci` 2026-05-16 → **Implemented** |
| S6 | docs | ✅ STABLE_STATE, CHANGELOG, DIGEST, §5.1 |

**Поза автопрогоном:** FM-004, FM-006, FM-009, FM-010.

**Канонічний порядок читання доків** (кроки 1–12) — кореневий [`README.md`](../../README.md); цей файл — **крок 12**.

Нові рядки додавати **вниз таблиці** з наступним вільним `FM-0xx`; дублікати з Issues закривати посиланням на PR.

---

## 6. Швидкий пошук по репозиторію

```bash
# Відкриті архітектурні чекбокси (ручна звірка)
rg "\- \[ \]" docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md

# TODO у виконуваному коді (зведення в NEXT_STEPS P5)
rg "TODO|FIXME" src/

# Історія за ключовим словом
git log --oneline --all --grep="RAID"
git log -S "PutArtifact" --oneline -n 20 -- src/
```

---

## 7. Підтримка цього файлу

- Оновлювати **дату** у шапці, таблицю **FM-*** і **§5.1** після значних змін плану Architect, витягу функціоналу або релізу.
- **§5.1** — коротка агрегація; довгі чекбокси й верифікації лишаються в `NEXT_STEPS`; HANDOFF посилається на **§5.1** як на операційний порядок.
- Правило для Cursor: [`.cursor/rules/functionality-management.mdc`](../../.cursor/rules/functionality-management.mdc).

---

## Див. також

- [FUNCTIONALITY_DIGEST_2026-04-06.md](./FUNCTIONALITY_DIGEST_2026-04-06.md)  
- [STABLE_STATE_SUMMARY.md](../status/STABLE_STATE_SUMMARY.md)  
- [STRUCTURE.md](../STRUCTURE.md) (таксономія `docs/`)
