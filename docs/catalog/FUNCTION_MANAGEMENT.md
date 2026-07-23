# Керування функціоналом PoolAI (індекс, прогалини, тікети)

**Оновлено:** 2026-07-23 (project completion path · band 66 ✅ · band 67 horizon · Cursor **3.12.30**)

**Зріз комітів (червень 2026):** FM-017/018 ✅; **FM-019 baseline** ✅ (modals, forms, tabs, tables, [`ADMIN_A11Y_RUNBOOK.md`](../development/ADMIN_A11Y_RUNBOOK.md)); pushes `02ea146`…`31266be9` на `main`.

**Horizon (Layer C):** **100%** ✅ S35–S40 — [`HORIZON_TO_100_PLAN.md`](../development/HORIZON_TO_100_PLAN.md). **Поза кодом:** FM-003 §4 LAN (**BLOCKED**, 2 хости).

**Останній `cargo test-ci`:** 2026-05-20 (після `f00bb1d4`); GNU, `K8S_OPENAPI_ENABLED_VERSION=1.28` — **ok** (~16 хв). Clippy — CI на `main`.

**Прогрес продукту (шар A):** **100%** — див. **§5.5** та [`DEVELOPMENT_PROGRESS_2026-05-19.md`](../status/DEVELOPMENT_PROGRESS_2026-05-19.md).

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
| FM-003 | P2b / RAID | Dev stand ✅; wire harness ✅. **LAN §4 sign-off** — ops gated (2 хости) | Implemented (scope A) | `LAN_BENCHMARK_RUNBOOK.md` §5.1 |
| FM-004 | ML | SIMD / прискорений шлях TurboQuant у Rust | **Implemented ✅ S35** | `turboquant-simd` feature, `src/ml/turboquant.rs`, `TURBOQUANT_INTEGRATION.md` §SIMD |
| FM-005 | P3 | Узгоджений JSON-помилок: `HttpAppError` / `AppError::RestError` (без зміни стабільних `error.code`) | Implemented | NEXT_STEPS P3; зроблено: **`ui`**, **`users`**, **`ai_ml`**, **`workers`**, **`instances`**, **`libraries`**, **`vm`**, **`topology`**, **`rewards`**, **`system`**, **`completions`**, **`admin`**, **`raid`**, **`raid_admin`**, **`raid_http`**, **`network/enterprise_api/`**, **`authenticate_user`** / **`refresh_access_token`** / **`SystemService::login`**, **`check_permission`**, **`auth_middleware`**, **`permission_middleware`** |
| FM-006 | Cloud | Azure/GCP під `cloud-sdk`: REST paths, token cache, `AZURE_LOCATION`, mock e2e; native Compute SDK — out of scope | **Implemented ✅ S39** | `azure.rs`, `gcp.rs`, [`CLOUD_SDK_STATUS.md`](../cloud/CLOUD_SDK_STATUS.md) |
| FM-007 | Distributed RAID | Sync: порівняння локального каталогу з peer `artifact_ids` за напрямком (Pull/Push/Bidirectional); за наявності `remote_versions` (`artifact_id` -> timestamp) формується `conflicts` | Implemented | `RaidDistributedProtocolService::sync_artifacts`, `diff_sync_catalog`, `build_sync_conflicts`; wire-тести **`tests/distributed_raid_wire_integration.rs`** (15 tests, 2026-05-16) |
| FM-008 | Distributed RAID | LeaveCluster: `graceful` — `replicate_stored_artifact` по всіх локальних артефактах, далі `delete_worker`; якщо немає peer-вузлів і є артефакти — `replication_complete=false`; помилки membership / невалідний `node_id` | Implemented | Membership + graceful/non-graceful leave; wire-тести у **`tests/distributed_raid_wire_integration.rs`** |
| FM-009 | Grid | Єдиний wire envelope Grid v1 | **Implemented ✅ S36** | `src/grid/`, `GRID_PROTOCOL_CONCEPT` |
| FM-010 | Tokenization | Solana adapter MVP (sidecar, schema v1) | **Implemented ✅ S37** | `crates/poolai-solana-adapter/`, `SOLANA_ADAPTER_CONCEPT` |
| FM-011 | Ops | MSVC: **`[profile.test] debug = 1`** у `Cargo.toml` (менший PDB, обхід LNK1318); alias **`cargo test-ci`** у **`.cargo/config.toml`** = CI-прогін (`--lib` + `--tests`, без doctests) + **`K8S_OPENAPI_ENABLED_VERSION=1.28`**; clippy матриці як у `ci.yml` — на `main` (2026-04-10); локально **`cargo test-ci`** — 2026-05-16; повний `cargo test` з doctests на Windows може дати **os error 1455** | Implemented | `Cargo.toml`, `.cargo/config.toml`, HANDOFF, NEXT_STEPS |
| FM-012 | UI / Auth UX | Апгрейд `/ui` і `/ui/admin/*`: i18n **UA/EN**; **Telegram OAuth** — HMAC/`auth_date`/allowlist/audit; widget HTML UA/EN; нові Telegram-юзери → **Viewer** (без `admin:all`); тести allowlist/expiry/RBAC | Implemented | `src/ui/i18n_core.js`, `src/ui/mod.rs`, `src/ui/admin/`, `src/network/enterprise_api/oauth.rs`, `src/enterprise/security.rs` |
| FM-013 | UI / Admin API | Контрактні тести JSON для admin-сторінок (v1 + enterprise); S25–S26 закрили P1 | Implemented | `tests/admin_ui_api_contracts.rs` (**27 tests**), [`ADMIN_UI_JSON_CONTRACTS.md`](../development/ADMIN_UI_JSON_CONTRACTS.md) |
| FM-014 | UI / Admin API | Фаза 2 контрактів: `GET /config`, `GET /users`, `GET /topology/nodes`; rewards API → `HttpAppError` (FM-005) | Implemented | `tests/admin_ui_api_contracts.rs`, `src/network/api/rewards.rs` |
| FM-015 | UI / Admin API | Фаза 3: `GET /instance`, `GET /raid/artifacts`, `GET /raid/admin/metrics/smallworld` (20 contract tests) | Implemented | `tests/admin_ui_api_contracts.rs`, `src/ui/admin/instances.rs`, `src/ui/admin/raid.rs` |
| FM-016 | Workers / Telegram | **Virtual nodes** + Telegram: bind/webhook/store, `poolai-worker`, **`poolai-telegram-bot`** (`--features tgbot`); pool workload на device | Implemented | `virtual_node_*`, `tgbot/coordinator`, `poolai-telegram-bot`, integration tests |
| FM-017 | P3 / HTTP | **FM-005 залишок:** `discovery` → `HttpAppError` JSON; `virtual_nodes` — status-only (worker); `admin` — `AppError` ✅ | Implemented | `discovery.rs`; `virtual_nodes.rs` worker-safe; `tests/discovery_remote_register_integration.rs` |
| FM-018 | UI / a11y | Admin/login skip links, focus-visible, aria-live, aria-current | Implemented | `admin/mod.rs`, `admin_styles.css`, `admin_common.js`, login `mod.rs`; `admin::a11y_tests` |
| FM-019 | UI / a11y | pa11y CI ✅; axe Playwright ✅ (S33); admin baseline | Implemented (scope A) | [`ADMIN_A11Y_RUNBOOK.md`](../development/ADMIN_A11Y_RUNBOOK.md), `e2e/tests/a11y.spec.ts` |
| FM-020 | Job layer | Scheduler MVP: `Submitted`→`Scheduled`, in-process tick, persist via store | **Implemented ✅** | `src/job/scheduler.rs`, `POST /jobs/schedule`, `AUTO_RUN_POST_HORIZON` |
| FM-021 | Job layer | `PATCH /api/v1/jobs/{id}` (status); OpenAPI `/jobs` schemas | **Implemented ✅** | `lifecycle.rs`, `PATCH /jobs/{id}`, `JobRecord` OpenAPI |
| FM-022 | Memory layer | HTTP stub shard refs / RAID map | **Implemented ✅** | `memory/store.rs`, `GET/POST /memory/shards`, `POOLAI_MEMORY_DATA_DIR` |
| FM-023 | Grid | Wire Job/Result у discovery або distributed path | **Implemented ✅** | `grid/dispatch.rs`, `POST /grid/envelope`, `POST /discovery/grid/envelope` |
| FM-024 | Solana | Sidecar devnet RPC stub (без mainnet) | **Implemented ✅** | `config/devnet.toml`, `rpc/mock.rs`, `SidecarProcessor` |
| FM-025 | OpenAPI | VM template body schemas (gap audit) | **Implemented ✅** | `VmTemplate`, `GpuSchedulingPolicy` in `openapi.yaml` |
| FM-026 | QA | Contract або Playwright для `/api/v1/jobs` | **Implemented ✅** | `tests/jobs_api_contracts.rs` (4 tests) |
| FM-027 | Ops / LAN | 2-host sign-off runbook + checklist | **Prep ✅** (sign-off **BLOCKED**, 2 хости) | `LAN_SIGNOFF_CHECKLIST.md`, `bin/verify-lan-prep.*` |
| FM-028 | P2b / perf | Single-host TQ01+RAID metrics → `BENCHMARKS.md` | **Implemented ✅** | `capture-p2b-single-host-metrics.*`, `poolai-p2b-tq01-snapshot` |
| FM-029 | Job layer | SQLite job store (optional feature) | **Implemented ✅** | `job-store-sqlite`, `POOLAI_JOB_STORE=sqlite`, `src/job/store_sqlite.rs` |
| FM-030 | Enterprise | Monitoring metrics persistence MVP | **Implemented ✅** | `POOLAI_MONITORING_DATA_DIR`, `monitoring.db`, dashboards/alert_rules |
| FM-031 | UI / a11y | Розширення pa11y/axe admin URLs | **Implemented ✅** | `pa11y-ci.sh` 21 URLs; `e2e/tests/a11y.spec.ts` |
| FM-032 | OpenAPI | `VmNetwork` + `NetworkIsolationConfig` body schemas | **Implemented ✅** | `openapi.yaml` components + `/vm/networks*` refs; gap audit exit 0 |
| FM-033 | Solana | On-chain program prototype + real devnet RPC submit | **Implemented ✅** | `program/poolai-events`, `rpc/devnet.rs`, `POOLAI_SOLANA_KEYPAIR_PATH` |
| FM-034 | Job layer | Scheduler → VM/worker binding (beyond in-process tick) | **Implemented ✅** | `src/job/scheduler.rs`, `JobRecord.worker_id`/`vm_id`, `tests/job_scheduler_pool_binding.rs` |
| FM-035 | Runtime / ML | Real model loading (libtorch/onnx path, not metadata-only) | **Implemented ✅** | `src/runtime/model_loader.rs`, `instance.rs` (`LoadedModelHandle`, SHA256 fingerprint) |
| FM-036 | Runtime | Tensor / pipeline parallelism (`sharding`, inter-worker sync) | **Implemented ✅** | `src/runtime/sharding.rs`, `tests/sharding_tests.rs`, `benches/sharding_benchmarks.rs`; `pool/placement.rs` uses `tensor_placement_from_nodes` |
| FM-037 | UI | Cluster topology graph (SVG force layout + latency heatmap) | **Implemented ✅** | `src/ui/topology_graph.js`, `admin/topology.rs`; Playwright topology graph |
| FM-038 | Observability | OpenTelemetry distributed tracing | **Implemented ✅** | `src/observability/`, feature `otel`, [`OPENTELEMETRY_TRACING.md`](../development/OPENTELEMETRY_TRACING.md) (HTTP spans; job lease span attrs PH-S124, instrumentation PH-S126) |
| FM-039 | CI / E2E | Playwright admin suite у `ci.yml` (`workflow_call`) | **Implemented ✅** | `ci.yml` `playwright-admin` → `e2e.yml` (paths-filter `ui`/`e2e`) |
| FM-040 | UI / QA | Admin field audit (усі `admin/*.rs` vs handlers) | **Implemented ✅** | [`ADMIN_UI_FIELD_AUDIT_2026-05-23.md`](../development/ADMIN_UI_FIELD_AUDIT_2026-05-23.md); +5 contract tests (31 total) |
| FM-041 | Cloud | SDK hardening (GCP SA JWT, Azure OAuth refresh, IT suites) | **Deferred** | post FM-006 REST; `CLOUD_SDK_PROGRESS_2026-01-19` |
| FM-042 | P4 / perf | Hot-path profiling + Criterion benchmarks (beyond FM-028 snapshot) | **Implemented ✅** | `benches/http_hotpath_benchmarks.rs`, `benchmarks.yml`, `BENCHMARKS.md` § FM-042 |
| FM-043 | Observability | Prometheus `/metrics` text export (pull model; complements FM-038 OTLP) | **Implemented ✅** | `src/observability/prometheus_export.rs`, feature `prometheus`, [`PROMETHEUS_METRICS.md`](../development/PROMETHEUS_METRICS.md) |
| FM-044 | Security / TLS | TLS 1.3 rustls rollout, HSTS from config, cert reload (`HTTPS_CERT_RELOAD_SECS`) | **Implemented ✅** | `src/network/tls_config.rs`, feature `https`, [`security/TLS.md`](../security/TLS.md) |
| FM-045 | UI / Admin | Design system tokens + unified tables/forms (`admin_common.js`) | **Implemented ✅** | `design_tokens.css`, `admin_styles.css`, [`DESIGN_SYSTEM.md`](../development/DESIGN_SYSTEM.md) |

### 5.1 Пріоритезовані наступні кроки (зведення FM-* і Architect-плану)

**Канон черги розробки** — таблиця нижче (аудит legacy 2026-05-20, **§5.8**). Post-Horizon закрито — **§5.7**. Не повторювати FM-020…033.

| Порядок | Фокус | FM | Дія |
|--------|--------|-----|-----|
| — | **PH-S24** Security ops | **§5.9** | **✅** |
| — | **Ops** LAN §4 sign-off | **FM-003** | **BLOCKED** (2 фізичні хости); prep ✅ FM-027 |
| — | Cloud SDK deep | **FM-041** | **Deferred** — без явного запиту |
| — | **PH-S47** CI green + локально test-ci | **§5.11** | **✅** — `0fe21bf1`; CI #1213 green (ubuntu+windows Test Suite, openapi-gap) |
| — | **PH-S37** Visual baselines workflow | **§5.10** | **✅ infra** — `update-visual-baselines.yml`; PNG refresh on-demand |
| — | **PH-S44** E2E/CI visual + axe gate | **§5.11** | **✅** — `test:ci` incl. visual; paths-filter → Playwright + Pa11y |
| — | **PH-S39** VM Windows resource limits post-spawn | **§5.11** | **✅** — `WindowsJobObjectLimiter`, `apply_limits_post_spawn`, `vm_windows_resource_limits_integration` |
| — | **PH-S42** Admin tables UX | **§5.11** | **✅** — sort/filter/export, `adminEmptyStateHtml`, auto-init |
| — | **PH-S43** ML/monitoring metrics UI | **§5.11** | **✅** — ML step metrics panel, demo btn, sparklines |
| — | **PH-S45** E2E stability (vm modal, axe audit) | **§5.11** | **✅** — VM onclick globals, E2E POST/DELETE wait, audit axe settle, viewport 1920 |
| — | **PH-S46** Solana on-chain program hardening + devnet deploy | **§5.11** | **✅** — `wire/limits.rs`, deploy script, `anchor_mode` |
| — | **PH-S41** macvlan network isolation (Linux) | **§5.11** | **✅** — `NetworkInterfaceMode`, netns move + cleanup |
| 1 | **PH-S40** hardware VM isolation | **§5.10** | **✅** (HardwareVm best-effort isolation plans) |
| — | **PH-S48** Job store RAID-backed persistence | Architect deferred | **✅** |
| — | **PH-S49** Job store RAID ops/docs + §5.11 research | PH-S48 follow-up, DOCS_LEGACY | **✅** — HANDOFF/RUN_LOCAL; черга PH-S50…S59 |
| — | **PH-S50** OpenAPI + DIGEST jobs / `POOLAI_JOB_STORE=raid` | OPENAPI_GAP, DIGEST | **✅** — `JobStoreBackend` schema; Jobs tag; gap-audit 0 |
| 3 | **PH-S51** VM Linux isolation hardening | `vm/isolation/linux.rs` | **✅** — veth host→netns, tracked cleanup, unit + linux apply/remove test |
| 4 | **PH-S52** E2E jobs + RAID persistence smoke | `e2e/`, `job/store.rs` | **✅** — `jobs_raid.spec.ts`; raid `block_on` fix; `test:ci` |
| 5 | **PH-S53** Admin jobs UI + store badge | `src/ui/admin/jobs.rs` | **✅** — `/ui/admin/jobs`; `GET /jobs` → `store_backend` |

**Закрито (не в черзі):** FM-001…040, FM-037…039, FM-042…045; Horizon S35–S40; **PH-S07…S12** ✅ (PH-S11–S12: Playwright visual + theme/i18n — [`VISUAL_REGRESSION_E2E.md`](../development/VISUAL_REGRESSION_E2E.md)).

**Якість збірки:** `cargo test-ci`; `cargo bench --no-run --bench http_hotpath_benchmarks` (FM-042).

**Промпт сесії:** [`NEXT_SESSION_PROMPT.md`](../development/NEXT_SESSION_PROMPT.md).

### 5.9 Post-Horizon PH спринти (maintenance, 2026-05-23)

**Канон черги PH-S01…S14** — одна сесія = один PH-S*; джерела: §5.1, [`UI_UX_IMPROVEMENTS_PLAN.md`](../development/UI_UX_IMPROVEMENTS_PLAN.md), [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](../development/NEXT_STEPS_ARCHITECT_2026-03-17.md), archive Raft/VM plans.

| Sprint | Фокус | FM / джерело | Стан |
|--------|--------|--------------|------|
| **PH-S01** | Cloud SDK deep (GCP SA JWT, Azure OAuth refresh) | **FM-041** | **Deferred** |
| **PH-S02** | LAN §4 sign-off (2-host replication + TQ01) | **FM-003** | **BLOCKED** (2 хости) |
| **PH-S03** | VM admin E2E + write-op contracts (`vm_write_operations`) | UI_QUALITY §P2, `vm_service` | **✅** |
| **PH-S04** | Raft feature wire tests (`--features raft`) | Architect L325, `NEXT_STEPS_2026-01-19` | **✅** |
| **PH-S05** | RAID admin: raft role / cluster status UI | OpenAPI `RaidDistributedRaftRole` | **✅** |
| **PH-S06** | Multi-node Raft harness (single-host simulation) | archive WEEK12, FM-027 prep | **✅** |
| **PH-S07** | Prometheus `GET /metrics` | **FM-043** | **✅** |
| **PH-S08** | TLS 1.3 / cert reload | **FM-044** | **✅** |
| **PH-S09** | Admin design system tokens | **FM-045** | **✅** |
| **PH-S10** | Admin metrics charts (`admin_charts.js`) | DESIGN_SYSTEM | **✅** |
| **PH-S11** | Playwright visual regression baselines | [`VISUAL_REGRESSION_E2E.md`](../development/VISUAL_REGRESSION_E2E.md) | **✅** |
| **PH-S12** | Theme (dark/light) × i18n (EN/UK) visual matrix | `themes.rs`, `i18n_core.js` | **✅** |
| **PH-S13** | Topology graph masked SVG visual baseline | `topology_graph.js`, PH-S11 scope | **✅** |
| **PH-S14** | High-contrast theme + axe contrast CI fixes | UI_UX §102, `a11y.spec.ts` | **✅** |

**Відкрито (2):** PH-S01 (Deferred), PH-S02 (BLOCKED). **Не повторювати:** PH-S03…S14, post-PH a11y HC slice (2026-05-24).

**PH-S01…S14 закрито (2026-05-24).** **Наступна черга (legacy backlog → PH-S15…S24):**

| Sprint | Фокус | Джерело (старіші плани) | Стан |
|--------|--------|-------------------------|------|
| **PH-S15** | Cloud SDK deep (GCP SA JWT, Azure OAuth refresh) | PH-S01, FM-041, `CLOUD_SDK_PROGRESS` | **Deferred** |
| **PH-S16** | LAN §4 sign-off (2-host replication + TQ01) | PH-S02, FM-003, Architect L130 | **BLOCKED** (2 хости) |
| **PH-S17** | ML pipeline ops (step metrics, stand verify) | AUTO_RUN §1.2, DIGEST §ML | **✅** (2026-05-24) |
| **PH-S18** | BurstRAID/SmallWorld admin metrics polish | `RUST_ARCHITECT_STATUS`, raid admin UI | **✅** (2026-05-24) |
| **PH-S19** | OpenAPI gap audit gate у CI | `poolai-openapi-gap-audit`, OPENAPI_GAP | **✅** (2026-05-24) |
| **PH-S20** | VM Windows isolation (AppContainer/firewall) | `vm/isolation/windows.rs` stubs | **✅** (2026-05-24) |
| **PH-S21** | Raft membership з log/snapshot | `raid/raft.rs`, `tests/raft_membership_log.rs` | **✅** (2026-05-24) |
| **PH-S22** | Topology live updates (WebSocket) | `ws_manager`, `topology.rs` admin | **✅** (2026-05-24) |
| **PH-S23** | Playwright admin flows expand | `e2e/tests/admin.spec.ts`, `helpers.ts` | **✅** (2026-05-24) |
| **PH-S24** | Security ops (secret rotation hooks, pen-test checklist) | `src/security/`, `docs/security/PEN_TEST_CHECKLIST.md` | **✅** |

**Одна сесія = один PH-S*** (або FM-003/FM-041 за §5.1). Промпт: [`NEXT_SESSION_PROMPT.md`](../development/NEXT_SESSION_PROMPT.md).

### 5.10 Post-PH backlog PH-S35…S44 (legacy audit 2026-05-25)

**Джерела:** [`DOCS_LEGACY_AUDIT_2026-05-19.md`](../development/DOCS_LEGACY_AUDIT_2026-05-19.md), [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](../development/NEXT_STEPS_ARCHITECT_2026-03-17.md), [`UI_UX_IMPROVEMENTS_PLAN.md`](../development/UI_UX_IMPROVEMENTS_PLAN.md), [`UI_QUALITY_AND_E2E_PLAN_2026-04-06.md`](../development/UI_QUALITY_AND_E2E_PLAN_2026-04-06.md), [`AUTO_RUN_SESSION_2026-07-01.md`](../development/AUTO_RUN_SESSION_2026-07-01.md). **Не плутати** з Horizon **S35–S40** (✅) та закритими **PH-S03…S34**.

| Sprint | Фокус | Джерело (legacy) | Стан |
|--------|--------|------------------|------|
| **PH-S35** | LAN §4 sign-off (2-host + TQ01) | Architect L130, FM-003 | **BLOCKED** (2 хости) |
| **PH-S36** | Cloud SDK deep (GCP SA JWT, Azure OAuth) | FM-041, `CLOUD_SDK_PROGRESS` | **Deferred** |
| **PH-S37** | Playwright visual baselines refresh (**Linux CI**) | `VISUAL_REGRESSION_E2E.md`, `update-visual-baselines.yml` | **✅ infra** — workflow + snapshots in repo; refresh on-demand |
| **PH-S38** | Job scheduler hardening + on-chain submit epics | Architect L236 | **✅** |
| **PH-S39** | VM Windows CPU/memory limits post-spawn | `vm/resources.rs`, AUTO_RUN §1.6 | **✅** |
| **PH-S40** | Hardware VM isolation | `vm/mod.rs`, Architect 2026-01-22 | **✅** |
| **PH-S41** | macvlan network isolation (Linux) | `vm/isolation/linux.rs` | **✅** |
| **PH-S42** | Admin tables UX (sort/filter/export, empty states) | `UI_UX_IMPROVEMENTS_PLAN` §tables | **✅** |
| **PH-S43** | Monitoring / ML step metrics UI | UI_UX §monitoring, DIGEST §ML | **✅** |
| **PH-S44** | E2E gate: visual + axe required on UI PRs | DOCS_LEGACY, `AUTO_RUN` a11y merge | **✅** — `e2e/package.json` `test:ci` + `ci.yml` paths-filter |

**Наступна сесія (код/CI):** **PH-S52** (E2E jobs + RAID smoke). **Не стартувати без інфра:** PH-S35/S16/S02 LAN, PH-S36/S01/S15 Cloud SDK.

### 5.11 Наступні 10 спринтів PH-S55…S64 (Galaxy Grid + ops/docs, 2026-05-26)

**VDT / локальний CI:** одна сесія = один PH-S*; верифікація — `cargo test-ci` (+ scope: raft, openapi-gap, e2e). GitHub Actions — довідково. Правила — [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc). Якщо відкритих <10 — **project scan** всього репо (не лише §5.12) і доповнити §5.12 (**max 10**).

**Dual gate (PH-S1004, band 35):** API scope перед push — **`cargo test-ci`** **і** **`cargo run --bin poolai-openapi-gap-audit`** (0 missing). Playwright лише browser scope; див. [`.cursor/rules/poolai-testing-policy.mdc`](../../.cursor/rules/poolai-testing-policy.mdc) band 35 multi-module.

**Джерела (пріоритет):** локальні failures → §5.10 → [`DOCS_LEGACY_AUDIT_2026-05-19.md`](../development/DOCS_LEGACY_AUDIT_2026-05-19.md) → Architect / UI_UX / [`E2E_PLAYWRIGHT.md`](../development/E2E_PLAYWRIGHT.md).

**Закрито (не в §5.11):** PH-S47…S54 ✅ (S53 — `/ui/admin/jobs` + `store_backend` in list API; S54 — `verify-dev-stand` optional RAID job store step).

| # | Sprint | Фокус | Джерело | Acceptance (скорочено) | Стан |
|---|--------|--------|---------|-------------------------|------|
| 1 | **PH-S55** | `run-poolai` preset для RAID jobs (операторський quick start) | `RUN_LOCAL.md`, `bin/run-poolai.sh`, `bin/run-poolai.ps1` | documented `single`/`lan` one-liner з `POOLAI_JOB_STORE=raid` + `POOLAI_RAID_BASE_PATH`; `single` preset (`--raid-jobs` / `-RaidJobs`) | **✅** |
| 2 | **PH-S56** | Galaxy Grid: job lease + re-migrate policy (концепт) | `concept/POOLAI_GALAXY_GRID.md` | зафіксовано lease/TTL + `lease_epoch`, CAS по result, failover triggers, retry budget, мінімальна state-модель job | **✅** |
| 3 | **PH-S57** | Galaxy Grid: unified worker entity (local/cloud/telegram) (концепт + DTO sketch) | `concept/POOLAI_GALAXY_GRID.md`, FM-016 | зафіксовано DTO sketch (`origin/admin_id/capabilities/network_profile/limits`) + мінімальні UI labels/filter/sort правила | **✅** |
| 4 | **PH-S58** | Fee split: primary 0.1% dev wallet + secondary 1–5% admin (концепт) | `concept/POOLAI_GALAXY_GRID.md`, Solana concept | `src/grid/galaxy_fee_split.rs` + formula/docs + UX hint + unit tests + Criterion bench | **✅** |
| 5 | **PH-S59** | Pricing oracle: -10% від min US providers (концепт + ops notes) | `concept/POOLAI_GALAXY_GRID.md` | unit keys (tokens/GPU-sec/job flat), `floor(min×0.9)`, кеш TTL/SWR, L1–L3 fallback + env ops | **✅** |
| 6 | **PH-S60** | Telegram edge mining: seats + wallet binding (концепт) | FM-016, Galaxy | seats: members vs wallets vs sessions; `seat_limit`; wallet bind flow + FM-016 API ref | **✅** |
| 7 | **PH-S61** | Seeds/locality: placement + prefetch RAM/VRAM policy (концепт) | Memory layer + SmallWorld | locality_score, telemetry table, hot tiers L0–L3, task-driven prefetch + ops env | **✅** |
| 8 | **PH-S62** | Edge verification baseline (концепт) | Galaxy | trust tiers, sampling/replay/replication (K-of-M), trust_score settlement gate (без ZK) | **✅** |
| 9 | **PH-S63** | Open source governance: signed releases + protocol versioning (docs) | Galaxy §9 | signed releases, compat matrix, opt-in update policies, без root super-admin | **✅** |
| 10 | **PH-S64** | Docs sync: додати Galaxy Grid у canonical pointers (коротко) | README/INDEX/STRUCTURE | README/docs/README/INDEX/STRUCTURE узгоджені; short pointer Galaxy Grid | **✅** |

**Черга PH-S55…S64 закрита (2026-05-27).** Наступні спринти — **§5.12** (research).

**Поза чергою:** **PH-S35** / **PH-S16** / **PH-S02** LAN (**BLOCKED**) · **PH-S36** / **PH-S01** / **PH-S15** Cloud SDK (**Deferred**, FM-041).

### 5.12 Research backlog PH-S65+ (Galaxy wire / ops, 2026-05-27)

**VDT:** §5.12 = **журнал PH-S*** (≤10 відкритих). Якщо **< 10** — **`абракадабра`** / project scan **всього проєкту**: concept → FM **§5.1** → roadmaps → architect → DOCS_LEGACY → code → §5.13 fallback. Канон: [`.cursor/rules/poolai-session-iteration.mdc`](../../.cursor/rules/poolai-session-iteration.mdc) § «Project scan». **Rust-first:** API → `tests/`; Playwright лише browser scope.

| # | Sprint | Фокус | Джерело | Acceptance (скорочено) | Стан |
|---|--------|--------|---------|-------------------------|------|
| 1 | **PH-S65** | Worker register: `protocol_version` + compat negotiation | Galaxy §9.3 | OpenAPI + handler; reject/upgrade response | **✅** |
| 2 | **PH-S66** | `poolai verify-release` CLI (signed manifest) | Galaxy §9.2, SECURITY_HARDENING | verify artifact + signature; unit tests | **✅** |
| 3 | **PH-S67** | DIGEST: Galaxy Grid modules zріз | FUNCTIONALITY_DIGEST | `galaxy_fee_split`, grid dispatch, virtual nodes | **✅** |
| 4 | **PH-S68** | Pricing oracle Rust stub (concept S59) | `POOLAI_GALAXY_GRID.md` §4.2 | unit keys + env `POOLAI_GALAXY_PRICE_*`; tests | **✅** |
| 5 | **PH-S69** | SECURITY_HARDENING ↔ Galaxy §9 signed releases | `docs/security/` | checklist items linked; no duplicate prose | **✅** |
| 6 | **PH-S70** | Legacy architect checklist hygiene (canonical pointer) | `NEXT_STEPS_ARCHITECT_2026-01-16.md`, VDT §research | add stale/canonical note, avoid conflicting active queue | **✅** |
| 7 | **PH-S71** | Signed release verification quickstart for operators | `SECURITY_HARDENING.md`, Galaxy §9.2 | short runbook pointer to `poolai-verify-release` usage | **✅** |
| 8 | **PH-S72** | Protocol compatibility ops checklist links | Galaxy §9.3, FM PH-S65 | cross-link compat matrix + rollout guardrails in security docs | **✅** |
| 9 | **PH-S73** | Protocol reject troubleshooting pointer (docs) | `SECURITY_HARDENING.md`, Galaxy §9.3 | short operator checklist for `compat_status`/403/426 triage links | **✅** |
| 10 | **PH-S74** | Security docs link hygiene for release advisories | Galaxy §9.6, `docs/security/` | align cross-links to signed advisory/update policy sections | **✅** |
| 11 | **PH-S75** | Pricing oracle L2 configured fallback (code) | Galaxy §4.2.4, `src/grid/galaxy_pricing_oracle.rs` | add `POOLAI_GALAXY_PRICING_FALLBACK_JSON` parsing + fallback quote path + unit tests | **✅** |
| 12 | **PH-S76** | Release advisory operator actions pointer | Galaxy §9.6, `docs/security/SECURITY_HARDENING.md` | add short action list linked to signed advisory/update policy docs | **✅** |
| 13 | **PH-S77** | Security docs canonical pointer cleanup | `docs/security/*`, FM §5.12 | normalize Galaxy §9.2/§9.3/§9.6 links and remove duplicates | **✅** |
| 14 | **PH-S78** | Grid pricing API snapshot wire (code) | Galaxy §4.2.3, future wire note | read-only `GET /api/v1/grid/pricing` (task/model/unit) wired to oracle cache/fallback path + endpoint tests + OpenAPI sync + gap-audit 0 | **✅** |
| 15 | **PH-S79** | Grid pricing API env fallback wire fix (code) | `src/network/api/grid.rs`, Galaxy §4.2.4 | initialize API oracle via `GalaxyPricingOracle::from_env()` so `POOLAI_GALAXY_PRICING_FALLBACK_JSON` is applied on `/api/v1/grid/pricing`; `cargo test-ci` + openapi-gap 0 | **✅** |
| 16 | **PH-S80** | Pricing oracle L3 hard stop (code) | Galaxy §4.2.4, `galaxy_pricing_oracle.rs` | when L1+L2 unavailable return `503 pricing_unavailable` on new priced jobs/API; unit tests | **✅** |
| 17 | **PH-S81** | Pricing oracle `FORCE_FALLBACK` env wire (code) | Galaxy §4.2.4, `galaxy_pricing_oracle.rs` | `POOLAI_GALAXY_PRICING_FORCE_FALLBACK=1` always L2 path + log metric; unit tests | **✅** |
| 18 | **PH-S82** | Admin UI grid pricing snapshot panel | Galaxy §4.2.3, `src/ui/` | `/ui/admin/*` read-only panel calling `GET /api/v1/grid/pricing`; Playwright smoke | **✅** |
| 19 | **PH-S83** | Galaxy pricing stale-served metric (code) | Galaxy §4.2.4, observability | expose `galaxy_pricing_stale_served` counter on L1 stale path; unit or integration test | **✅** |
| 20 | **PH-S84** | Galaxy §4.2.3 wire note sync (docs) | `POOLAI_GALAXY_GRID.md` | mark `GET /api/v1/grid/pricing` implemented (PH-S78/S79); remove stale «майбутній wire» | **✅** |
| 21 | **PH-S85** | verify-release dev fixtures + RUN_LOCAL pointer (docs) | `SECURITY_HARDENING`, `RUN_LOCAL.md` | sample manifest/sig paths for local verify; no duplicate Galaxy prose | **✅** |
| 22 | **PH-S86** | Grid pricing E2E smoke | `e2e/`, PH-S78 API | Playwright hits `/api/v1/grid/pricing` with env fallback JSON on dev stand | **✅** |
| 23 | **PH-S87** | INDEX security docs cross-link (docs) | `INDEX_2026-03-17.md`, `docs/security/` | step-8 security row links to Galaxy hub in SECURITY_HARDENING | **✅** |
| 24 | **PH-S88** | Release manifest sample JSON (docs) | Galaxy §9.2, `docs/development/` | operator-facing minimal manifest example for `poolai-verify-release` | **✅** |
| 25 | **PH-S89** | Pricing oracle L1 stale TTL metric (code) | Galaxy §4.2.3–4.2.4 | distinguish fresh vs stale cache hits in quote metadata or metrics; tests | **✅** |
| 26 | **PH-S90** | Cursor rules: VDT agent roles + §5.12 sync | `.cursor/rules/`, `.cursor/README.md` | `poolai-agent-roles.mdc`; slim VDT; session-iteration globs; §5.11→§5.12; git-commit-msys | **✅** |
| 27 | **PH-S91** | Pricing oracle fresh-served metric (code) | Galaxy §4.2.3/§4.2.5, `galaxy_pricing_oracle.rs` | `galaxy_pricing_fresh_served` counter + log on L1 fresh path; unit tests | **✅** |
| 28 | **PH-S92** | Pricing providers env catalog stub (code) | Galaxy §4.2.5, `galaxy_pricing_oracle.rs` | parse `POOLAI_GALAXY_PRICING_PROVIDERS` allow-list JSON; no live HTTP fetch; unit tests | **✅** |
| 29 | **PH-S93** | Admin UI updates & compatibility panel (code) | Galaxy §9.8, `src/ui/` | read-only `/ui/admin/updates-compat`: protocol version, verify-release pointer, compat matrix links; i18n EN/UK; Playwright smoke | **✅** |
| 30 | **PH-S94** | Job lease fields wire stub (code) | Galaxy §4.3.1, `src/job/` | optional `lease_owner` / `lease_epoch` / `lease_expires_at` on `JobRecord` + POST/GET jobs API; backward compatible JSON/SQLite; unit + contract tests | **✅** |
| 31 | **PH-S95** | PATCH jobs lease epoch CAS stub (code) | Galaxy §4.3.1, `src/job/`, `src/network/api/jobs.rs` | optional `lease_epoch` on PATCH; `409 lease_epoch_rejected` on mismatch; backward compatible omit; unit + contract tests | **✅** |
| 32 | **PH-S96** | Admin jobs UI lease columns (code) | Galaxy §4.3.1, `src/ui/admin/jobs.rs` | read-only `lease_owner` / `lease_epoch` / `lease_expires_at` in jobs table; i18n EN/UK; Playwright smoke (`admin.spec.ts`) | **✅** |
| 33 | **PH-S97** | Job lease TTL env default stub (code) | Galaxy §4.3.1, `src/job/lease_config.rs` | `POOLAI_JOB_LEASE_TTL_SECS` parse + default 90s; HANDOFF §2a; unit tests (no renew wire) | **✅** |
| 34 | **PH-S98** | Lease acquire at schedule / explicit API (code) | Galaxy §4.3.1, `src/job/lease_acquire.rs` | schedule + `POST /jobs/{id}/lease`; `JobLeaseConfig` TTL; OpenAPI; unit + contract tests | **✅** |
| 35 | **PH-S99** | Lease renew / heartbeat wire (code) | Galaxy §4.3.1, `lease_acquire.rs` | `POST /jobs/{id}/lease/renew`; extends TTL; epoch CAS; OpenAPI; tests | **✅** |
| 36 | **PH-S100** | `JobStatus::Leased` + lifecycle (code) | Galaxy §4.3.2 | `Leased` status + `allows_transition`; backward compatible JSON/SQLite; acquire → `Leased` | **✅** |
| 37 | **PH-S101** | Failover / re-migrate stub (code) | Galaxy §4.3 | detect expired leased jobs; requeue + rebind sketch in scheduler/store; tests | **✅** |
| 38 | **PH-S102** | Live pricing provider HTTP fetch (code) | Galaxy §4.2.5 | L1 refresh from `POOLAI_GALAXY_PRICING_PROVIDERS` endpoints; `POOLAI_GALAXY_PRICING_TIMEOUT_MS`; API integration + tests | **✅** |
| 39 | **PH-S103** | `X-PoolAI-Protocol` middleware (code) | Galaxy §9.8 | middleware on selected wire routes (`/grid/*`, register/heartbeat remote, virtual-nodes); protocol headers + reject unsupported | **✅** |
| 40 | **PH-S104** | `JobStatus::Migrating` + lifecycle (code) | Galaxy §4.3.2 | `Migrating` enum + lifecycle transitions (`Leased/Executing ↔ Migrating`); OpenAPI + contract tests | **✅** |
| 41 | **PH-S105** | Admin jobs lease active/expired badge (code) | Galaxy §4.3.1, `src/ui/admin/jobs.rs` | read-only badge from `lease_expires_at`; i18n EN/UK; Playwright smoke | **✅** |
| 42 | **PH-S106** | `poolai-worker` lease renew client stub (code) | Galaxy §4.3.1, `src/bin/poolai-worker.rs` | POST renew to coordinator when payload has active `job_id` + `lease_epoch`; parser + HTTP renew stub tests; no full failover | **✅** |
| 43 | **PH-S107** | Jobs lease E2E smoke (e2e) | `e2e/`, PH-S98–S99 API | Playwright acquire + renew path; `jobs_lease.spec.ts`; `npm run test:ci` | **✅** |
| 44 | **PH-S108** | Grid ingest sets Leased on acquire (code) | `src/grid/dispatch.rs`, `src/job/` | grid Job ingest + `schedule_with_grid_peer` → `leased` + lease fields when peer binds; unit tests | **✅** |
| 45 | **PH-S109** | Galaxy §4.3 lease wire docs sync (docs) | `POOLAI_GALAXY_GRID.md`, INDEX, DIGEST | §4.3 table PH-S94…S108; смуга 10/10 ✅; roadmap replenish | **✅** |
| 46 | **PH-S110** | Grid result ingest lease_epoch CAS (code) | `src/grid/dispatch.rs`, Galaxy §4.3.1 | `GridResultBody.lease_epoch`; `check_grid_result_lease_epoch`; `409 lease_epoch_rejected`; unit tests | **✅** |
| 47 | **PH-S111** | Job lease renew interval env (code) | `src/job/lease_config.rs`, §4.3.1 | `POOLAI_JOB_LEASE_RENEW_INTERVAL_SECS` optional override; unit tests | **✅** |
| 48 | **PH-S112** | Grid Job envelope E2E smoke (e2e) | `e2e/`, PH-S108 | Playwright grid envelope Job → `leased` + lease fields | **✅** |
| 49 | **PH-S113** | Vision L4/L5 workspace + lib layers (docs) | `docs/vision/`, manifest | `Cargo.toml` / `.cargo/config.toml` L5; `src/lib.rs` + crates L4; manifest nodes + edges | **✅** |
| 50 | **PH-S114** | Vision map pan/zoom navigation (docs) | `docs/vision/vision.js` | wheel zoom, drag pan, +/- reset, dblclick focus node; map transform not browser zoom | **✅** |
| 51 | **PH-S115** | Vision folder-colored edge routing (docs) | `docs/vision/vision.js` | orthogonal paths via folder hub; `edge-docs` / `edge-code` / `edge-toml` / `edge-mixed` | **✅** |
| 52 | **PH-S116** | Worker periodic lease renew loop (code) | `src/bin/poolai-worker.rs`, PH-S111 | ticker from `JobLeaseConfig.lease_renew_interval_secs` while task active | **✅** |
| 53 | **PH-S117** | Grid result lease_epoch E2E (e2e) | `e2e/`, PH-S110 | Playwright stale epoch → `409 lease_epoch_rejected` on grid Result | **✅** |
| 54 | **PH-S118** | Jobs lease negative paths E2E (e2e) | `e2e/`, PH-S107 | renew w/o acquire 400; expired TTL 409; wrong owner 409; `POOLAI_JOB_LEASE_TTL_SECS=2` on e2e stand | **✅** |
| 55 | **PH-S119** | Admin jobs lease column polish (code) | `src/ui/admin/jobs.rs` | `#epoch` display; owner/epoch/col tooltips; i18n EN/UK; Playwright PH-S96 extended | **✅** |
| 56 | **PH-S120** | Solana adapter vision + digest crosslink (docs) | `docs/vision/`, DIGEST | manifest nodes/edges; DIGEST § Solana modules; FM-033 crosslink | **✅** |
| 57 | **PH-S121** | Galaxy §4.3 worker heartbeat wire note (docs) | `POOLAI_GALAXY_GRID.md` | §4.3.1.1 worker lease renew vs discovery heartbeat; env + payload + `LeaseRenewGuard` | **✅** |
| 58 | **PH-S122** | OpenAPI jobs lease schemas audit (docs) | `docs/openapi.yaml`, gap audit | `lease_epoch` on grid result body + examples; gap audit 0 | **✅** |
| 59 | **PH-S123** | Grid pricing E2E negative fallback (e2e) | `e2e/tests/grid_pricing.spec.ts` | force fallback env → snapshot stable quote | **✅** |
| 60 | **PH-S124** | OTel lease span attrs docs (docs) | FM-038, HANDOFF | span attributes for acquire/renew/reject paths | **✅** |
| 61 | **PH-S125** | Vision Galaxy map Eco + click perf (docs) | `docs/vision/` | Eco GPU mode; instant select (no full re-render); Layers/Types fullscreen dock; bottom toolbar layout | **✅** |
| 62 | **PH-S126** | OTel lease span instrumentation (code) | FM-038, `src/job/`, `src/observability/` | spans on acquire/renew/reject (`job.lease.*` attrs); `cargo test --features otel`; cross-link OPENTELEMETRY_TRACING | **✅** |
| 63 | **PH-S127** | Pricing oracle Prometheus export (code) | Galaxy §4.2, `galaxy_pricing_oracle.rs`, FM-043 | export `galaxy_pricing_*_served` + `forced_fallback_total` on `GET /metrics`; unit test | **✅** |
| 64 | **PH-S128** | Locality score scheduler stub (code) | Galaxy §5.1–5.2, `src/grid/` | `locality_score(worker, task)` pure fn + unit tests; no prefetch wire | **✅** |
| 65 | **PH-S129** | Seed inventory + prefetch policy stub (code) | Galaxy §5.5, `src/grid/dispatch.rs` | `SeedInventoryEntry` DTO + noop prefetch hook; unit tests | **✅** |
| 66 | **PH-S130** | Edge trust_score settlement gate stub (code) | Galaxy §6.5, `src/grid/` | `trust_score` 0–100 gate sketch on grid result path; unit tests | **✅** |
| 67 | **PH-S131** | Telegram wallet bind API stub (code) | Galaxy §3.2, `virtual_nodes.rs` | `POST /api/v1/virtual-nodes/telegram/wallet` stub + OpenAPI; contract test | **✅** |
| 68 | **PH-S132** | network_profile contract docs (docs) | Galaxy §8 94.67% #1 | §8.1 schema for `network_profile`; DIGEST row; locality subset cross-link | **✅** |
| 69 | **PH-S133** | Job Migrating lifecycle E2E (e2e) | PH-S104, `e2e/` | Playwright PATCH `migrating` ↔ `executing` roundtrip; `npm run test:ci` | **✅** |
| 70 | **PH-S134** | Protocol middleware E2E smoke (e2e) | PH-S103, `e2e/` | Playwright register-remote with `X-PoolAI-Protocol`; unsupported → 403 | **✅** |
| 71 | **PH-S135** | Telegram wallet GET lookup API (code) | Galaxy §3.2, PH-S131 | `GET /api/v1/virtual-nodes/telegram/wallets/{telegram_user_id}`; OpenAPI; integration test | **✅** |
| 72 | **PH-S136** | Prefetch policy env wire stub (code) | Galaxy §5.6, `dispatch.rs` | `PrefetchPolicyMode` + `POOLAI_GALAXY_*` from_env; unit tests; no enqueue wire | **✅** |
| 73 | **PH-S137** | Trust gate settlement metrics stub (code) | Galaxy §6.5, `galaxy_trust_score.rs` | Prometheus counters `payout_held` / `payout_eligible`; unit test | **✅** |
| 74 | **PH-S138** | Locality rank integration test (tests) | PH-S128, `galaxy_locality.rs` | `tests/` multi-worker `rank_workers_by_locality` fixture; `cargo test-ci` | **✅** |
| 75 | **PH-S139** | Telegram wallet bind E2E (e2e) | PH-S131, `e2e/` | Playwright POST wallet OK + invalid pubkey 400; `npm run test:ci` | **✅** |
| 76 | **PH-S140** | network_profile register-remote stub (code) | Galaxy §8.1, discovery | parse `metadata.network_profile`; **Rust integration test**; no new Playwright | **✅** |
| 77 | **PH-S141** | Admin jobs migrating badge UI (code) | PH-S104, `jobs.rs` | `migrating` status badge + i18n EN/UK; Playwright smoke; `test:ci` | **✅** |
| 78 | **PH-S142** | Verification sample rate env stub (code) | Galaxy §6.1, `src/grid/` | `POOLAI_GALAXY_VERIFY_BASE_SAMPLE_RATE` parser + unit tests; no live sampling wire | **✅** |
| 79 | **PH-S143** | LOC ratio baseline audit (code) | [`RUST_RATIO_STRATEGY_2026-06-13.md`](../development/RUST_RATIO_STRATEGY_2026-06-13.md) | `poolai-loc-audit` bin + `rust_ratio.json` (**91.48%**); FM sync | **✅** |
| 80 | **PH-S144** | Playwright API → Rust migration (tests) | PH-S133…S139 legacy e2e | API specs → `tests/*_integration.rs`; `cargo test-ci`; no new TS API specs | **✅** |
| 81 | **PH-S145** | `poolai-http-stand-smoke` bin (code) | PH-S144, stand env | Rust HTTP stand smoke bin; doc in RUN_LOCAL | **✅** |
| 82 | **PH-S146** | `poolai-ui-core` shared crate (code) | `src/ui/`, ratio strategy | validators/formatters з admin JS → crate + unit tests | **✅** |
| 83 | **PH-S147** | wasm32 admin core POC (code) | PH-S146, portability | one wasm module + portability §2 docs sync | **✅** |
| 84 | **PH-S148** | Slim `e2e/` browser-only (e2e) | PH-S144 | `test:ci` без API TS patterns; ratio ≥90% | **✅** |
| 85 | **PH-S150** | Ratio CI advisory (ops) | PH-S143 audit | CI job `rust-ratio-audit`; `--warn-below 0.88` `--target 0.93` `--stretch 0.96` `--advisory`; `rust_ratio.json` **92.00%** | **✅** |
| 86 | **PH-S151** | wasm grid-pricing panel wiring (code) | PH-S147 | `/ui/admin/grid-pricing` → wasm formatters; `/ui/wasm/*` static; Playwright smoke | **✅** |
| 87 | **PH-S152** | wasm jobs lease display (code) | PH-S151, `jobs.rs` | admin jobs panel: wasm `leaseStateLabel`; shared `POOLAI_UI_WASM_MODULE`; Playwright smoke | **✅** |
| 88 | **PH-S153** | `poolai-ui-core` → admin_common slim (code) | PH-S146 | `api_error`, `format`, table helpers → Rust/wasm; −≥400 LOC JS | **✅** |
| 89 | **PH-S154** | Admin i18n subset in Rust (code) | PH-S153 | grid-pricing + jobs lease EN/UK keys у Rust templates; slim `i18n_core.js` admin block | **✅** |
| 90 | **PH-S155** | ML charts data → Rust/wasm (code) | PH-S146 `ml` | `admin_charts.js` — лише canvas glue; metrics parse у wasm | **✅** |
| 91 | **PH-S156** | `jobs_raid` e2e → Rust stand smoke (tests) | PH-S145 | `poolai-http-stand-smoke --raid-restart`; прибрати `jobs_raid` з `test:ci` | **✅** |
| 92 | **PH-S157** | topology SVG from Rust (code) | `topology.rs` | masked topology data з Rust; slim `topology_graph.js` | **✅** |
| 93 | **PH-S158** | `poolai-e2e-stand` Rust bin (code) | PH-S145 | stand start/restart/env у Rust bin; slim `bin/e2e-playwright.sh` | **✅** |
| 94 | **PH-S159** | Ratio **96%** stretch CI gate (ops) | PH-S150…S158 | `poolai-loc-audit` warn **93%**, stretch **96%**; FM replenish post-S159 band | **✅** |
| 95 | **PH-S160** | Admin theme normalize → Rust (code) | PH-S153 `admin_theme.js` | `poolaiNormalizeTheme` + token map у `poolai-ui-core`; slim `admin_theme.js` | **✅** |
| 96 | **PH-S161** | Admin modal a11y → wasm (code) | PH-S153 `admin_modal_a11y.js` | focus-trap / modal helpers у ui-core/wasm; slim modal JS | **✅** |
| 97 | **PH-S162** | Auth i18n subset Rust (code) | PH-S154 pattern | login/dashboard shell keys у `i18n.rs`; slim `i18n_core.js` auth block | ✅ |
| 98 | **PH-S163** | Galaxy trust metrics wire (code) | PH-S137, Galaxy §6.5 | trust gate Prometheus на grid result path; unit tests | ✅ |
| 99 | **PH-S164** | Verify sampling env apply (code) | PH-S142, Galaxy §6.1 | `galaxy_verify_sampling` у HTTP/grid middleware stub; tests | ✅ |
| 100 | **PH-S165** | Ratio **96%** hold band gate (ops) | PH-S159…S164 | CI `--min-ratio 0.95` advisory hold; target **95%**; spirit **96%**; replenish §5.12 | **✅** |
| 101 | **PH-S166** | Design tokens CSS → Rust (code) | PH-S160, `design_tokens.css` | `design_tokens.rs` + `admin_base_css()`; slim `design_tokens.css` / `admin_styles.css` `:root` | **✅** |
| 102 | **PH-S167** | Galaxy prefetch metrics stub (code) | PH-S129, Galaxy §5.5 | Prometheus counters on `plan_prefetch`; unit + integration tests | **✅** |
| 103 | **PH-S168** | Galaxy pricing cache age /metrics (code) | PH-S89, Galaxy §4.2 | `galaxy_pricing_cache_age_seconds` gauge on GET /metrics; unit tests | **✅** |
| 104 | **PH-S169** | Locality stale profile penalty stub (code) | Galaxy §8.1 | `stale_network_profile_penalty` у `galaxy_locality.rs`; unit tests | **✅** |
| 105 | **PH-S170** | Galaxy settlement pending_verification stub (code) | PH-S165, Galaxy §6.4 | `pending_verification` verdict stub on grid result path; unit tests | **✅** |
| 106 | **PH-S171** | Galaxy replication strict tier stub (code) | Galaxy §6.3 | `replication_strict` tier config stub; unit tests | **✅** |
| 107 | **PH-S172** | Galaxy pricing provider catalog metrics stub (code) | PH-S92, Galaxy §4.2 | Prometheus counters on provider allow-list hits; unit tests | **✅** |
| 108 | **PH-S173** | Galaxy pricing provider errors metrics stub (code) | PH-S92, Galaxy §4.2 | `galaxy_pricing_provider_errors_total` counter on provider fetch fail; unit tests | **✅** |
| 109 | **PH-S174** | Galaxy pricing quote usd_micro metrics stub (code) | PH-S89, Galaxy §4.2 | `galaxy_pricing_quote_usd_micro` gauge on last quote; unit tests | **✅** |
| 110 | **PH-S175** | Galaxy verification mismatch metrics stub (code) | Galaxy §6.2 | `galaxy_verification_mismatch_total` counter stub; unit tests | **✅** |
| 111 | **PH-S176** | Galaxy replay pending metrics stub (code) | Galaxy §6.3 | `galaxy_replay_pending` gauge stub; unit tests | **✅** |
| 112 | **PH-S177** | Galaxy verification sample total metrics stub (code) | Galaxy §6.2 | `galaxy_verification_sample_total` counter stub; unit tests | **✅** |
| 113 | **PH-S178** | Galaxy settlement pending_verification metrics stub (code) | PH-S170, Galaxy §6.4 | `galaxy_settlement_pending_verification_total` counter on grid result path; unit tests | **✅** |
| 114 | **PH-S179** | Galaxy replication strict tier metrics stub (code) | PH-S171, Galaxy §6.3 | `galaxy_replication_strict_total` counter on grid job ingest; unit tests | **✅** |
| 115 | **PH-S180** | Galaxy verification match metrics stub (code) | Galaxy §6.2 | `galaxy_verification_match_total` counter on grid result path; unit tests | **✅** |
| 116 | **PH-S181** | Galaxy pricing market min usd_micro metrics stub (code) | PH-S89, Galaxy §4.2 | `galaxy_pricing_market_min_usd_micro` gauge stub; unit tests | **✅** |
| 117 | **PH-S182** | Galaxy trust score metrics stub (code) | Galaxy §6.2 | `galaxy_trust_score` gauge on grid result path; unit tests | **✅** |
| 118 | **PH-S183** | Galaxy shard local hit ratio metrics stub (code) | Galaxy §5.3 | `galaxy_shard_local_hit_ratio` gauge on locality rank stub; unit tests | **✅** |
| 119 | **PH-S184** | Galaxy prefetch bytes total metrics stub (code) | Galaxy §5.5 PH-S129 | `galaxy_prefetch_bytes_total` counter on `plan_prefetch`; unit tests | **✅** |
| 120 | **PH-S185** | Galaxy cross region egress mb metrics stub (code) | Galaxy §5.3 | `galaxy_cross_region_egress_mb` gauge stub on rank/prefetch path; unit tests | **✅** |
| 121 | **PH-S186** | Galaxy verification sample scheduled /metrics export (code) | PH-S164, Galaxy §6.2 | `galaxy_verification_sample_scheduled_total` on `GET /metrics`; unit tests | **✅** |
| 122 | **PH-S187** | Galaxy settlement cleared total metrics stub (code) | PH-S170, Galaxy §6.4 | `galaxy_settlement_cleared_total` counter on grid result Cleared path; unit tests | **✅** |
| 123 | **PH-S188** | Vision map filters UX (docs/vision) | `docs/vision/vision.js`, PH-S115 filters | незалежні layer/type toggle; **LAYERS**/**TYPES** select-all/none; decouple 3D stack ↔ chips; `vision.js` + README; rev++ | **✅** |
| 124 | **PH-S189** | Vision Eco/FX/Ms hover trace (docs/vision) | PH-S188, `docs/vision/README.md` Eco/FX | tri-mode **Eco→FX→Ms**; hover 1-hop edge/node highlight; `localStorage`; rev++ | **✅** |
| 125 | **PH-S190** | Vision filter dropdowns + panel collapse (docs/vision) | PH-S188 filters, PH-S115 layout | Layers/Types **dropdown** menus; **−** collapse → title strip; grid auto-fill; `localStorage`; rev++ | **✅** |
| 126 | **PH-S191** | Vision sprint queue panel (docs/vision) | FM §5.12, `poolai-vision-sync` | Rust parse FM §5.12 → `sprint_queue` panel; rev++ | **✅** |
| 127 | **PH-S192** | Vision overview LOD + minimap (docs/vision) | PH-S115 map zoom | `map-overview` при low zoom; hub-only labels; viewport inset minimap; rev++ | **✅** |
| 128 | **PH-S193** | Dashboard shell formatters → wasm (code) | PH-S151, `poolai-ui-core` | login/dashboard wasm formatters; slim JS glue; `cargo test-ci` | **✅** |
| 129 | **PH-S194** | Galaxy fee split result counter stub (code) | PH-S58, Galaxy §4.1 | `galaxy_fee_split_applied_total` on grid result path; unit tests | **✅** |
| 130 | **PH-S195** | Galaxy seed_inventory GET stub (code) | PH-S129, Galaxy §5.5 | read-only `GET /api/v1/grid/seed-inventory`; OpenAPI; integration test | **✅** |
| 131 | **PH-S196** | Stand smoke jobs lease renew (tests) | PH-S156, PH-S99 | `poolai-http-stand-smoke --lease-renew`; slim Playwright lease | **✅** |
| 132 | **PH-S197** | Admin updates-compat wasm wiring (code) | PH-S151, PH-S93 | `/ui/admin/updates-compat` → wasm labels; Playwright smoke | **✅** |
| 133 | **PH-S198** | Topology graph Rust labels slim (code) | PH-S157 | hub labels у `topology_graph.rs`; −LOC `topology_graph.js` | **✅** |
| 134 | **PH-S199** | Vision map Ms hit-test + focus nav (docs/vision) | PH-S189 Ms | planes `pointer-events:none`; edge trace; click focus ~14px; zoom back; sidebar scroll | **✅** |
| 135 | **PH-S200** | Vision feed.json RSS ticker (docs/vision) | PH-S191 queue | `feed.json` + RSS ticker panel; rev++ | **✅** |
| 136 | **PH-S201** | Cursor post-push PH-S* hook (ops) | VDT, `.cursor/hooks` | post-push notify after PH-S* close; docs sync pointer | **✅** |
| 137 | **PH-S202** | Vision sprint-queue chip → map focus (docs/vision) | PH-S199 | queue card click centers map node | **✅** |
| 138 | **PH-S203** | Vision keyboard nav linked nodes (docs/vision) | PH-S199 | Arrow keys cycle 1-hop neighbors on map | **✅** |
| 139 | **PH-S204** | Vision edge click neighbor select (docs/vision) | PH-S199 | edge click → trace + select endpoint | **✅** |
| 140 | **PH-S205** | poolai-vision-sync manifest drift gate (ops) | PH-S191 | CI/local check manifest revision vs FM | **✅** |
| 141 | **PH-S206** | Vision minimap selection ring (docs/vision) | PH-S192 | minimap viewport + selected node ring | **✅** |
| 142 | **PH-S207** | Admin i18n slim next panel (code) | PH-S197 | next admin panel strings → `poolai-ui-core` | **✅** |
| 143 | **PH-S208** | Stand smoke vision revision parity (tests) | PH-S196 | `poolai-http-stand-smoke` checks vision rev header | **✅** |
| 144 | **PH-S209** | Vision map a11y focus ring (docs/vision) | PH-S194 | keyboard focus-visible on map controls + nodes | **✅** |
| 145 | **PH-S210** | Stand smoke seed_inventory GET (tests) | PH-S195 | `poolai-http-stand-smoke` case `grid_seed_inventory` | **✅** |
| 146 | **PH-S211** | Admin i18n slim jobs panel (code) | PH-S207 | `admin.jobs.*` → `poolai-ui-core`; slim `i18n_core.js` | **✅** |
| 147 | **PH-S212** | Vision reduced-motion map FX (docs/vision) | PH-S209 | `prefers-reduced-motion` → skip glow/animation; rev++ | **✅** |
| 148 | **PH-S213** | Galaxy prefetch metrics stand smoke (tests) | PH-S184 | stand smoke checks prefetch counters on `/metrics` | **✅** |
| 149 | **PH-S214** | Admin i18n slim raid panel (code) | PH-S207 | `admin.raid.*` → `poolai-ui-core`; slim JS | **✅** |
| 150 | **PH-S215** | Vision panel collapse focus restore (docs/vision) | PH-S209 | collapse/Esc returns focus to panel toggle; rev++ | **✅** |
| 151 | **PH-S216** | Galaxy pricing fallback metrics smoke (tests) | PH-S168 | stand smoke `galaxy_pricing_forced_fallback_total` | **✅** |
| 152 | **PH-S217** | Admin i18n slim grid-pricing panel (code) | PH-S207 | grid-pricing strings → `poolai-ui-core` | **✅** |
| 153 | **PH-S218** | Vision map aria-live selection (docs/vision) | PH-S209 | `aria-live` region for selected node label; rev++ | **✅** |
| 154 | **PH-S219** | Galaxy trust payout metrics smoke (tests) | PH-S182 | stand smoke trust payout counters on `/metrics` | **✅** |
| 155 | **PH-S220** | Admin i18n slim monitoring panel (code) | PH-S207 | `admin.mon.*` → slim `admin_monitoring_patch` | **✅** |
| 156 | **PH-S221** | Admin i18n slim updates-compat panel (code) | PH-S207 | `admin.updatesCompat.*` slim patch | **✅** |
| 157 | **PH-S222** | Admin i18n slim workers panel (code) | PH-S207 | `admin.wrk.*` → `poolai-ui-core` | **✅** |
| 158 | **PH-S223** | Admin i18n slim libs panel (code) | PH-S207 | `admin.lib.*` → `poolai-ui-core` | **✅** |
| 159 | **PH-S224** | Galaxy pricing cache age metrics smoke (tests) | PH-S168 | `galaxy_pricing_cache_age_seconds` on `/metrics` | **✅** |
| 160 | **PH-S225** | Galaxy verification sample metrics smoke (tests) | PH-S177 | verification counters on `/metrics` | **✅** |
| 161 | **PH-S226** | Vision sprint-queue → map focus (docs/vision) | PH-S202 | queue/ticker click → map node; panel expand fix | **✅** |
| 162 | **PH-S227** | Vision VDT rules docs autosync audit (docs/vision) | PH-S205 | manifest ↔ `.mdc` cross-link drift in `--check` | **✅** |
| 163 | **PH-S228** | Admin i18n slim dashboard panel (code) | PH-S207 | `admin.dash.*` → `admin_dashboard_patch` | **✅** |
| 164 | **PH-S229** | Admin i18n slim audit panel (code) | PH-S207 | `admin.audit.*` → `admin_audit_patch` | **✅** |
| 165 | **PH-S230** | Admin i18n slim tenants panel (code) | PH-S207 | `admin.tenants.*` → `admin_tenants_patch` | **✅** |
| 166 | **PH-S231** | Admin i18n slim security panel (code) | PH-S207 | `admin.sec.*` → `admin_security_patch` | **✅** |
| 167 | **PH-S232** | Galaxy replication metrics stand smoke (tests) | PH-S127 | replication counters on `/metrics` | **✅** |
| 168 | **PH-S233** | Vision map sprint chips a11y (docs/vision) | PH-S226 | `aria-label` on map sprint chips | **✅** |
| 169 | **PH-S234** | Admin i18n slim topology panel (code) | PH-S207 | `admin.topo.*` slim patch | **✅** |
| 170 | **PH-S235** | Stand smoke vision rev parity (tests) | PH-S208 | stand checks vision rev vs FM footer + extensions | **✅** |
| 171 | **PH-S236** | Admin i18n slim instances panel (code) | PH-S207 | `admin.inst.*` slim patch | **✅** |
| 172 | **PH-S237** | Admin i18n slim vm panel (code) | PH-S207 | `admin.vmadm.*` slim patch | **✅** |
| 173 | **PH-S238** | Admin i18n slim users panel (code) | PH-S207 | `admin.usr.*` slim patch | **✅** |
| 174 | **PH-S239** | Admin i18n slim config panel (code) | PH-S207 | `admin.cfg.*` slim patch | **✅** |
| 175 | **PH-S240** | Admin i18n slim table toolbar (code) | PH-S207 | `admin.table.*` slim patch | **✅** |
| 176 | **PH-S241** | Galaxy pricing fresh served metrics stand smoke (tests) | PH-S127 | `galaxy_pricing_fresh_served` on live `/metrics` | **✅** |
| 177 | **PH-S242** | Admin i18n nav shell key audit (code) | PH-S162 | verify `admin.nav.*` only in auth_dash patch | **✅** |
| 178 | **PH-S243** | Admin i18n slim admin chrome shell (code) | PH-S242 | `admin.brand` / skip / lang / logout / browserSuffix → auth_dash | **✅** |
| 179 | **PH-S244** | Galaxy pricing stale served metrics stand smoke (tests) | PH-S127 | `galaxy_pricing_stale_served` on live `/metrics` | **✅** |
| 180 | **PH-S245** | Admin shared status keys slim patch (code) | PH-S240 | `admin.status.*` + `admin.na` + `admin.btn.edit` slim patch | **✅** |
| 181 | **PH-S246** | Admin err hint keys slim patch (code) | PH-S245 | `err.hint*` + `err.insufficientAdmin` + `admin.accessRequired` | **✅** |
| 182 | **PH-S247** | Galaxy pricing provider metrics stand smoke (tests) | PH-S127 | provider catalog + errors gauges on `/metrics` | **✅** |
| 183 | **PH-S248** | Admin vm modal i18n slim (code) | PH-S237 | `vm.*` modal keys out of `i18n_core.js` | **✅** |
| 184 | **PH-S249** | Galaxy settlement metrics stand smoke (tests) | PH-S178 | settlement pending + cleared on `/metrics` | **✅** |
| 185 | **PH-S250** | Galaxy shard locality metrics stand smoke (tests) | PH-S183 | `galaxy_shard_local_hit_ratio` on `/metrics` | **✅** |
| 186 | **PH-S251** | Docs roadmap sync band (docs) | PH-S249 | GALAXY_GRID_ROADMAP + README + INDEX sprint zriz | **✅** |
| 187 | **PH-S252** | Admin shared ui.confirm slim patch (code) | PH-S245 | `ui.confirm*` + modal glue keys slim patch | **✅** |
| 188 | **PH-S253** | Galaxy pricing quote + market_min stand smoke (tests) | PH-S174/S181 | `galaxy_pricing_quote_usd_micro` + `galaxy_pricing_market_min_usd_micro` on `/metrics` | **✅** |
| 189 | **PH-S254** | Galaxy fee_split_applied stand smoke (tests) | PH-S194 | `galaxy_fee_split_applied_total` on live `/metrics` | **✅** |
| 190 | **PH-S255** | Galaxy cross_region_egress stand smoke (tests) | PH-S185 | `galaxy_cross_region_egress_mb` on live `/metrics` | **✅** |
| 191 | **PH-S256** | Galaxy replay_pending stand smoke (tests) | PH-S176 | `galaxy_replay_pending` on live `/metrics` | **✅** |
| 192 | **PH-S257** | Admin i18n workers panel slim patch (code) | PH-S222 | `workers.*` keys out of `i18n_core.js` | **✅** |
| 193 | **PH-S258** | Admin i18n home shell slim patch (code) | PH-S162 | `home.*` keys out of `i18n_core.js` | **✅** |
| 194 | **PH-S259** | Admin i18n form + err core slim patch (code) | PH-S246 | `form.*` + residual `err.*` out of `i18n_core.js` | **✅** |
| 195 | **PH-S260** | Admin i18n shared ui toolbar slim patch (code) | PH-S252 | `ui.save`/`ui.search*`/`ui.retry*` glue out of `i18n_core.js` | **✅** |
| 196 | **PH-S261** | Docs canon sync band (docs) | PH-S256 | INDEX/STABLE_STATE/docs README + GALAXY_ROADMAP zriz | **✅** |
| 197 | **PH-S262** | Rust ratio loc-audit refresh + hold gate (ops) | PH-S165 | `poolai-loc-audit` → `rust_ratio.json`; FM §5.13 advisory | **✅** |
| 198 | **PH-S263** | Admin i18n residual ui.* + common.* slim patch (code) | PH-S260 | `common.*` + residual `ui.*` → `admin_ui_common_patch` | **✅** |
| 199 | **PH-S264** | Dashboard libs panel i18n slim (code) | PH-S263 | `libs.*` keys out of `i18n_core.js` | **✅** |
| 200 | **PH-S265** | Dashboard raid panel i18n slim (code) | PH-S264 | `raid.*` keys out of `i18n_core.js` | **✅** |
| 201 | **PH-S266** | i18n_core.js near-empty gate + loc-audit (ops) | PH-S265 | STRINGS core **0** inline keys; `rust_ratio.json` refresh | **✅** |
| 202 | **PH-S267** | Docs canon sync band (docs) | PH-S266 | INDEX/HANDOFF/NEXT/STABLE_STATE sync | **✅** |
| 203 | **PH-S268** | Galaxy prefetch wire horizon doc (docs) | GALAXY §5.5 | roadmap pointer; metrics ✅; live prefetch 94.67% | **✅** |
| 204 | **PH-S269** | Vision feed.json refresh (docs/vision) | PH-S200 | `docs/vision/feed.json` sprint zriz | **✅** |
| 205 | **PH-S270** | poolai-vision-sync drift gate (ops) | PH-S205 | `--check` green after FM/HANDOFF | **✅** |
| 206 | **PH-S271** | Rust ratio hold advisory refresh (ops) | PH-S266 | `--min-ratio 0.95` advisory snapshot **94.34%** | **✅** |
| 207 | **PH-S272** | Docs INDEX sprint zriz (docs) | PH-S267 | INDEX step 8 + §7 ratio pointer | **✅** |
| 208 | **PH-S273** | admin_common api-error path slim (code) | PH-S153 | wasm-first `formatFetchError`; drop `hintFor503` JS dup | **✅** |
| 209 | **PH-S274** | admin_common loading/error DOM wasm glue (code) | PH-S273 | `adminShowLoading` / `adminShowInlineError` → wasm | **✅** |
| 210 | **PH-S275** | admin_charts sparkline wasm-only glue (code) | PH-S155 | slim `admin_charts.js` canvas path | **✅** |
| 211 | **PH-S276** | Galaxy prefetch wire stub (code) | GALAXY §5.5 | `plan_prefetch` ingest stub + unit test | **✅** |
| 212 | **PH-S277** | topology_graph.js paint-only audit (code) | PH-S157 | labels via Rust; JS ≤100 LOC gate | **✅** |
| 213 | **PH-S278** | Rust ratio loc-audit refresh (ops) | PH-S271 | `rust_ratio.json` sprint zriz | **✅** |
| 214 | **PH-S279** | Docs canon sync band (docs) | PH-S278 | INDEX/HANDOFF/NEXT/STABLE_STATE | **✅** |
| 215 | **PH-S280** | poolai-vision-sync drift gate (ops) | PH-S270 | `--check` green | **✅** |
| 216 | **PH-S281** | Ratio hold advisory snapshot (ops) | PH-S278 | `--min-ratio 0.95 --advisory` | **✅** |
| 217 | **PH-S282** | Docs INDEX ratio maintain (docs) | PH-S279 | INDEX §7 + rust_ratio pointer | **✅** |
| 218 | **PH-S283** | Galaxy prefetch enqueue wire stub (code) | PH-S276 | `enqueue_prefetch_hook` + unit test; no live pull | **✅** |
| 219 | **PH-S284** | admin_charts line chart wasm HTML (code) | PH-S275 | `render_line_chart_html` → wasm; slim `poolaiRenderLineChart` | **✅** |
| 220 | **PH-S285** | Galaxy locality rank job ingest stub (code) | PH-S128 | `rank_workers_by_locality` on grid job `required_shard_ids` | **✅** |
| 221 | **PH-S286** | Stand smoke prefetch enqueue path (tests) | PH-S283 | `poolai-http-stand-smoke` ingest + `/metrics` | **✅** |
| 222 | **PH-S287** | admin_charts metric group wasm glue (code) | PH-S155 | `poolaiGroupMetricsByName` → wasm | **✅** |
| 223 | **PH-S288** | Rust ratio loc-audit refresh (ops) | PH-S278 | `rust_ratio.json` sprint zriz | **✅** |
| 224 | **PH-S289** | Docs canon sync band (docs) | PH-S288 | INDEX/HANDOFF/NEXT/STABLE/GALAXY | **✅** |
| 225 | **PH-S290** | poolai-vision-sync drift gate (ops) | PH-S280 | `--check` green | **✅** |
| 226 | **PH-S291** | Ratio hold advisory snapshot (ops) | PH-S288 | `--min-ratio 0.95 --advisory` | **✅** |
| 227 | **PH-S292** | Docs INDEX ratio maintain (docs) | PH-S289 | INDEX §7 + rust_ratio pointer | **✅** |
| 228 | **PH-S293** | Galaxy prefetch wait hook stub (code) | PH-S283 | `wait_prefetch_hook` + wait ms metric | **✅** |
| 229 | **PH-S294** | admin metrics chart grid wasm HTML (code) | PH-S284 | `renderMetricsChartGridHtml` wasm glue | **✅** |
| 230 | **PH-S295** | Galaxy locality rank ingest metric (code) | PH-S285 | `galaxy_locality_rank_ingest_total` | **✅** |
| 231 | **PH-S296** | Stand smoke prefetch wait + locality (tests) | PH-S293 | `/metrics` export shape | **✅** |
| 232 | **PH-S297** | admin_charts sanitizeChartId wasm (code) | PH-S287 | `sanitizeChartId` → wasm | **✅** |
| 233 | **PH-S298** | Rust ratio loc-audit refresh (ops) | PH-S288 | `rust_ratio.json` sprint zriz | **✅** |
| 234 | **PH-S299** | Docs canon sync band (docs) | PH-S298 | INDEX/HANDOFF/NEXT/STABLE/GALAXY | **✅** |
| 235 | **PH-S300** | poolai-vision-sync drift gate (ops) | PH-S290 | `--check` green | **✅** |
| 236 | **PH-S301** | Ratio hold advisory snapshot (ops) | PH-S298 | `--min-ratio 0.95 --advisory` | **✅** |
| 237 | **PH-S302** | Docs INDEX ratio maintain (docs) | PH-S299 | INDEX §7 + rust_ratio pointer | **✅** |
| 238 | **PH-S303** | Galaxy prefetch strict mode metric (code) | PH-S293 | `galaxy_prefetch_strict_mode_total` | **✅** |
| 239 | **PH-S304** | admin line chart empty wasm HTML (code) | PH-S284 | `renderLineChartEmptyHtml` wasm glue | **✅** |
| 240 | **PH-S305** | Galaxy locality rank miss metric (code) | PH-S295 | `galaxy_locality_rank_miss_total` | **✅** |
| 241 | **PH-S306** | Stand smoke strict + rank miss (tests) | PH-S303 | `/metrics` export shape | **✅** |
| 242 | **PH-S307** | Galaxy complete prefetch hook stub (code) | PH-S283 | `complete_prefetch_hook` + complete metric | **✅** |
| 243 | **PH-S308** | Rust ratio loc-audit refresh (ops) | PH-S298 | `rust_ratio.json` sprint zriz | **✅** |
| 244 | **PH-S309** | Docs canon sync band (docs) | PH-S308 | INDEX/HANDOFF/NEXT/STABLE/GALAXY | **✅** |
| 245 | **PH-S310** | poolai-vision-sync drift gate (ops) | PH-S300 | `--check` green | **✅** |
| 246 | **PH-S311** | Ratio hold advisory snapshot (ops) | PH-S308 | `--min-ratio 0.95 --advisory` | **✅** |
| 247 | **PH-S312** | Docs INDEX ratio maintain (docs) | PH-S309 | INDEX §7 + rust_ratio pointer | **✅** |
| 248 | **PH-S313** | Galaxy prefetch ingest metric (code) | PH-S307 | `galaxy_prefetch_ingest_total` | **✅** |
| 249 | **PH-S314** | admin metric history URL wasm (code) | PH-S304 | `buildMetricHistoryUrl` wasm glue | **✅** |
| 250 | **PH-S315** | Galaxy locality empty workers metric (code) | PH-S305 | `galaxy_locality_rank_empty_workers_total` | **✅** |
| 251 | **PH-S316** | Stand smoke prefetch ingest + empty workers (tests) | PH-S313 | `/metrics` export shape | **✅** |
| 252 | **PH-S317** | admin metrics window URL wasm (code) | PH-S314 | `buildMetricsWindowUrl` wasm glue | **✅** |
| 253 | **PH-S318** | Rust ratio loc-audit refresh (ops) | PH-S308 | `rust_ratio.json` sprint zriz | **✅** |
| 254 | **PH-S319** | Docs canon sync band (docs) | PH-S318 | INDEX/HANDOFF/NEXT/STABLE/GALAXY | **✅** |
| 255 | **PH-S320** | poolai-vision-sync drift gate (ops) | PH-S310 | `--check` green | **✅** |
| 256 | **PH-S321** | Ratio hold advisory snapshot (ops) | PH-S318 | `--min-ratio 0.95 --advisory` | **✅** |
| 257 | **PH-S322** | Docs INDEX ratio maintain (docs) | PH-S319 | INDEX §7 + rust_ratio pointer | **✅** |
| 258 | **PH-S323** | Galaxy prefetch skip ingest metric (code) | PH-S313 | `galaxy_prefetch_skip_ingest_total` | **✅** |
| 259 | **PH-S324** | admin ML pipelines URL wasm (code) | PH-S314 | `buildMlPipelinesUrl` wasm glue | **✅** |
| 260 | **PH-S325** | Galaxy locality rank skip metric (code) | PH-S315 | `galaxy_locality_rank_skip_total` | **✅** |
| 261 | **PH-S326** | Stand smoke skip ingest + rank skip (tests) | PH-S323 | `/metrics` export shape | **✅** |
| 262 | **PH-S327** | admin ML pipeline demo URL wasm (code) | PH-S324 | `buildMlPipelineDemoUrl` wasm glue | **✅** |
| 263 | **PH-S328** | Rust ratio loc-audit refresh (ops) | PH-S318 | `rust_ratio.json` sprint zriz | **✅** |
| 264 | **PH-S329** | Docs canon sync band (docs) | PH-S328 | INDEX/HANDOFF/NEXT/STABLE/GALAXY | **✅** |
| 265 | **PH-S330** | poolai-vision-sync drift gate (ops) | PH-S320 | `--check` green | **✅** |
| 266 | **PH-S331** | Ratio hold advisory snapshot (ops) | PH-S328 | `--min-ratio 0.95 --advisory` | **✅** |
| 267 | **PH-S332** | Docs INDEX ratio maintain (docs) | PH-S329 | INDEX §7 + rust_ratio pointer | **✅** |
| 268 | **PH-S333** | Galaxy replay scheduled metric (code) | PH-S176 | `galaxy_replay_pending_scheduled_total` | **✅** |
| 269 | **PH-S334** | admin metric history URL hours wasm (code) | PH-S314 | `buildMetricHistoryUrlWithHours` wasm glue | **✅** |
| 270 | **PH-S335** | Galaxy replay resolved metric (code) | PH-S333 | `galaxy_replay_pending_resolved_total` | **✅** |
| 271 | **PH-S336** | Stand smoke replay scheduled + resolved (tests) | PH-S333 | `/metrics` export shape | **✅** |
| 272 | **PH-S337** | admin metrics window URL hours wasm (code) | PH-S317 | `buildMetricsWindowUrlWithHours` wasm glue | **✅** |
| 273 | **PH-S338** | Rust ratio loc-audit refresh (ops) | PH-S328 | `rust_ratio.json` sprint zriz | **✅** |
| 274 | **PH-S339** | Docs canon sync band (docs) | PH-S338 | INDEX/HANDOFF/NEXT/STABLE/GALAXY | **✅** |
| 275 | **PH-S340** | poolai-vision-sync drift gate (ops) | PH-S330 | `--check` green | **✅** |
| 276 | **PH-S341** | Ratio hold advisory snapshot (ops) | PH-S338 | `--min-ratio 0.95 --advisory` | **✅** |
| 277 | **PH-S342** | Docs INDEX ratio maintain (docs) | PH-S339 | INDEX §7 + rust_ratio pointer | **✅** |
| 278 | **PH-S343** | Galaxy verification sample completed metric (code) | PH-S177 | `galaxy_verification_sample_completed_total` on verdict | **✅** |
| 279 | **PH-S344** | admin monitoring alerts URL wasm (code) | PH-S314 | `buildMonitoringAlertsUrl` wasm glue | **✅** |
| 280 | **PH-S345** | Galaxy verification sample skipped metric (code) | PH-S164 | `galaxy_verification_sample_skipped_total` on NotSelected | **✅** |
| 281 | **PH-S346** | Stand smoke verification completed + skipped (tests) | PH-S343 | `/metrics` export shape | **✅** |
| 282 | **PH-S347** | admin alert-rules URL wasm (code) | PH-S344 | `buildAlertRulesUrl` wasm glue | **✅** |
| 283 | **PH-S348** | Rust ratio loc-audit refresh (ops) | PH-S338 | `rust_ratio.json` sprint zriz | **✅** |
| 284 | **PH-S349** | Docs canon sync band (docs) | PH-S348 | INDEX/HANDOFF/NEXT/STABLE/GALAXY | **✅** |
| 285 | **PH-S350** | poolai-vision-sync drift gate (ops) | PH-S340 | `--check` green | **✅** |
| 286 | **PH-S351** | Ratio hold advisory snapshot (ops) | PH-S348 | `--min-ratio 0.95 --advisory` | **✅** |
| 287 | **PH-S352** | Docs INDEX ratio maintain (docs) | PH-S349 | INDEX §7 + rust_ratio pointer | **✅** |
| 288 | **PH-S353** | monitoring dashboards/ack URL wasm (code) | PH-S347 | `buildMonitoringDashboardsUrl` + `buildMonitoringAlertAcknowledgeUrl`; `build-ui-wasm.sh` Windows cargo PATH | **✅** |
| 289 | **PH-S354** | Galaxy settlement not applicable metric (code) | PH-S178 | `galaxy_settlement_not_applicable_total` on grid result path | **✅** |
| 290 | **PH-S355** | admin active alerts URL wasm (code) | PH-S344 | `buildMonitoringActiveAlertsUrl` wasm glue | **✅** |
| 291 | **PH-S356** | Galaxy verify sampling not applicable metric (code) | PH-S345 | `galaxy_verification_sample_not_applicable_total` on local origin | **✅** |
| 292 | **PH-S357** | Stand smoke settlement + verify not applicable (tests) | PH-S354 | `/metrics` export shape | **✅** |
| 293 | **PH-S358** | admin monitoring wasm glue tests (code) | PH-S353 | `admin_charts_*_wasm_first_ph_s353/355` gates | **✅** |
| 294 | **PH-S359** | Rust ratio loc-audit refresh (ops) | PH-S348 | `rust_ratio.json` sprint zriz **94.33%** | **✅** |
| 295 | **PH-S360** | Docs canon sync band (docs) | PH-S359 | INDEX/HANDOFF/NEXT/STABLE/GALAXY | **✅** |
| 296 | **PH-S361** | poolai-vision-sync drift gate (ops) | PH-S350 | `--check` green | **✅** |
| 297 | **PH-S362** | Ratio hold advisory snapshot (ops) | PH-S359 | `--min-ratio 0.95 --advisory` | **✅** |
| 298 | **PH-S363** | Docs INDEX ratio maintain (docs) | PH-S360 | INDEX §7 + rust_ratio pointer | **✅** |
| 299 | **PH-S364** | Galaxy trust payout not applicable metric (code) | PH-S137 | `galaxy_trust_payout_not_applicable_total` on local origin | **✅** |
| 300 | **PH-S365** | Admin dashboard active alerts wasm (code) | PH-S355 | `buildMonitoringActiveAlertsUrl` on dashboard load | **✅** |
| 301 | **PH-S366** | Admin metric latest URL wasm (code) | PH-S314 | `buildMonitoringMetricLatestUrl` wasm glue | **✅** |
| 302 | **PH-S367** | Stand smoke trust not applicable (tests) | PH-S364 | `/metrics` export shape | **✅** |
| 303 | **PH-S368** | Admin wasm glue tests (code) | PH-S365/S366 | dashboard + metric latest wasm-first gates | **✅** |
| 304 | **PH-S369** | Rust ratio loc-audit refresh (ops) | PH-S359 | `rust_ratio.json` sprint zriz **94.32%** | **✅** |
| 305 | **PH-S370** | Docs canon sync band (docs) | PH-S369 | INDEX/HANDOFF/NEXT/STABLE/GALAXY | **✅** |
| 306 | **PH-S371** | poolai-vision-sync drift gate (ops) | PH-S361 | `--check` green | **✅** |
| 307 | **PH-S372** | Ratio hold advisory snapshot (ops) | PH-S369 | `--min-ratio 0.95 --advisory` | **✅** |
| 308 | **PH-S373** | Docs INDEX ratio maintain (docs) | PH-S370 | INDEX §7 + rust_ratio pointer | **✅** |
| 309 | **PH-S374** | Galaxy trust gate min threshold metric (code) | PH-S364 | `galaxy_trust_gate_min_threshold` on `/metrics` | **✅** |
| 310 | **PH-S375** | Admin dashboard audit events wasm (code) | PH-S366 | `buildAuditEventsUrl` wasm glue | **✅** |
| 311 | **PH-S376** | Admin dashboard overview URL wasm (code) | PH-S375 | `buildAdminOverviewUrl` wasm glue | **✅** |
| 312 | **PH-S377** | Stand smoke trust gate min threshold (tests) | PH-S374 | `/metrics` export shape | **✅** |
| 313 | **PH-S378** | Admin dashboard wasm glue tests (code) | PH-S375/S376 | overview + audit wasm-first gates | **✅** |
| 314 | **PH-S379** | Rust ratio loc-audit refresh (ops) | PH-S369 | `rust_ratio.json` sprint zriz **94.33%** | **✅** |
| 315 | **PH-S380** | Docs canon sync band (docs) | PH-S379 | INDEX/HANDOFF/NEXT/STABLE/GALAXY | **✅** |
| 316 | **PH-S381** | poolai-vision-sync drift gate (ops) | PH-S371 | `--check` green | **✅** |
| 317 | **PH-S382** | Ratio hold advisory snapshot (ops) | PH-S379 | `--min-ratio 0.95 --advisory` | **✅** |
| 318 | **PH-S383** | Docs INDEX ratio maintain (docs) | PH-S380 | INDEX §7 + rust_ratio pointer | **✅** |
| 319 | **PH-S384** | Galaxy trust gate default score metric (code) | PH-S374 | `galaxy_trust_gate_default_score` on `/metrics` | **✅** |
| 320 | **PH-S385** | Admin dashboard formatUptime wasm (code) | PH-S376 | `formatUptime` wasm glue on overview | **✅** |
| 321 | **PH-S386** | Admin dashboard metrics window wasm (code) | PH-S385 | `buildDashboardMetricsWindowUrl` wasm glue | **✅** |
| 322 | **PH-S387** | Stand smoke trust gate default score (tests) | PH-S384 | `/metrics` export shape | **✅** |
| 323 | **PH-S388** | Admin dashboard wasm glue tests (code) | PH-S385/S386 | uptime + metrics window wasm-first gates | **✅** |
| 324 | **PH-S389** | Rust ratio loc-audit refresh (ops) | PH-S379 | `rust_ratio.json` sprint zriz **94.33%** | **✅** |
| 325 | **PH-S390** | Docs canon sync band (docs) | PH-S389 | INDEX/HANDOFF/NEXT/STABLE/GALAXY | **✅** |
| 326 | **PH-S391** | poolai-vision-sync drift gate (ops) | PH-S381 | `--check` green | **✅** |
| 327 | **PH-S392** | Ratio hold advisory snapshot (ops) | PH-S389 | `--min-ratio 0.95 --advisory` | **✅** |
| 328 | **PH-S393** | Docs INDEX ratio maintain (docs) | PH-S390 | INDEX §7 + rust_ratio pointer | **✅** |
| 329 | **PH-S394** | Galaxy trust gate evaluations metric (code) | PH-S384 | `galaxy_trust_gate_evaluations_total` on `/metrics` | **✅** |
| 330 | **PH-S395** | Galaxy default score applied metric (code) | PH-S394 | `galaxy_trust_default_score_applied_total` on grid result path | **✅** |
| 331 | **PH-S396** | Admin dashboard audit timestamp wasm (code) | PH-S385 | `formatIsoDatetime` wasm glue on recent activity | **✅** |
| 332 | **PH-S397** | Stand smoke trust gate counters (tests) | PH-S394/S395 | `/metrics` export shape | **✅** |
| 333 | **PH-S398** | Admin dashboard wasm glue tests (code) | PH-S396 | `formatAuditTimestamp` + `formatIsoDatetime` gates | **✅** |
| 334 | **PH-S399** | Rust ratio loc-audit refresh (ops) | PH-S389 | `rust_ratio.json` sprint zriz **94.34%** | **✅** |
| 335 | **PH-S400** | Docs canon sync band (docs) | PH-S399 | INDEX/HANDOFF/NEXT/STABLE/GALAXY | **✅** |
| 336 | **PH-S401** | poolai-vision-sync drift gate (ops) | PH-S391 | `--check` green | **✅** |
| 337 | **PH-S402** | Ratio hold advisory snapshot (ops) | PH-S399 | `--min-ratio 0.95 --advisory` | **✅** |
| 338 | **PH-S403** | Docs INDEX ratio maintain (docs) | PH-S400 | INDEX §7 + rust_ratio pointer | **✅** |
| 339 | **PH-S404** | Galaxy settlement resolved metric (code) | PH-S178 | `galaxy_settlement_resolved_total` on grid result path | **✅** |
| 340 | **PH-S405** | Galaxy explicit trust score metric (code) | PH-S404 | `galaxy_trust_explicit_score_total` on grid result path | **✅** |
| 341 | **PH-S406** | Admin dashboard alert severity wasm (code) | PH-S396 | `alertSeverityBadgeClass` wasm glue on active alerts | **✅** |
| 342 | **PH-S407** | Stand smoke settlement + explicit score (tests) | PH-S404/S405 | `/metrics` export shape | **✅** |
| 343 | **PH-S408** | Admin dashboard wasm glue tests (code) | PH-S406 | `alertSeverityBadgeClass` gates | **✅** |
| 344 | **PH-S409** | Rust ratio loc-audit refresh (ops) | PH-S399 | `rust_ratio.json` sprint zriz **94.34%** | **✅** |
| 345 | **PH-S410** | Docs canon sync band (docs) | PH-S409 | INDEX/HANDOFF/NEXT/STABLE/GALAXY | **✅** |
| 346 | **PH-S411** | poolai-vision-sync drift gate (ops) | PH-S401 | `--check` green | **✅** |
| 347 | **PH-S412** | Ratio hold advisory snapshot (ops) | PH-S409 | `--min-ratio 0.95 --advisory` | **✅** |
| 348 | **PH-S413** | Docs INDEX ratio maintain (docs) | PH-S410 | INDEX §7 + rust_ratio pointer | **✅** |
| 349 | **PH-S414** | Galaxy verify sampling evaluations metric (code) | PH-S356 | `galaxy_verification_sampling_evaluations_total` on grid result path | **✅** |
| 350 | **PH-S415** | Galaxy replay evaluations metric (code) | PH-S335 | `galaxy_replay_evaluations_total` on grid result path | **✅** |
| 351 | **PH-S416** | Admin dashboard refreshed-at wasm (code/ui) | PH-S406 | `updateDashboardRefreshedAt` + `formatLocaleTimeHms` | **✅** |
| 352 | **PH-S417** | Stand smoke verify + replay evaluations (tests) | PH-S414/S415 | `/metrics` export shape | **✅** |
| 353 | **PH-S418** | Admin dashboard refreshed-at glue tests (code) | PH-S416 | `updateDashboardRefreshedAt` gates | **✅** |
| 354 | **PH-S419** | Rust ratio loc-audit refresh (ops) | PH-S409 | `rust_ratio.json` sprint zriz **94.35%** | **✅** |
| 355 | **PH-S420** | Docs canon sync band (docs) | PH-S410 | INDEX/HANDOFF/NEXT/STABLE/GALAXY | **✅** |
| 356 | **PH-S421** | poolai-vision-sync drift gate (ops) | PH-S411 | `--check` green | **✅** |
| 357 | **PH-S422** | Ratio hold advisory snapshot (ops) | PH-S419 | `--min-ratio 0.95` snapshot **94.35%** | **✅** |
| 358 | **PH-S423** | Docs INDEX ratio maintain (docs) | PH-S420 | INDEX §7 + rust_ratio pointer | **✅** |
| 359 | **PH-S424** | Galaxy prefetch seed-pull metric (code) | Galaxy §5.5 | `galaxy_prefetch_seed_pull_total` on complete hook | **✅** |
| 360 | **PH-S425** | Lease-acquired prefetch trigger (code) | Galaxy §5.5 | `galaxy_prefetch_lease_acquired_total` on lease acquire path | **✅** |
| 361 | **PH-S426** | Replication executor enqueue metric (code) | Galaxy §6.4 | `galaxy_replication_enqueue_total` on grid job ingest | **✅** |
| 362 | **PH-S427** | Settlement payout batch ledger metric (code) | Galaxy §8.2 | `galaxy_settlement_payout_batch_total` on cleared settlement | **✅** |
| 363 | **PH-S428** | Admin dashboard quick-stats wasm (code/ui) | RUST_RATIO §5.13 | `formatPercent` + `formatMegabytes` wasm glue | **✅** |
| 364 | **PH-S429** | Stand smoke horizon metrics band (tests) | PH-S424…S427 | `/metrics` export shape prefetch/replication/settlement | **✅** |
| 365 | **PH-S430** | Admin dashboard quick-stats glue tests (code) | PH-S428 | `formatPercent`/`formatMegabytes` gates | **✅** |
| 366 | **PH-S431** | Rust ratio loc-audit refresh (ops) | PH-S419 | `rust_ratio.json` sprint zriz **94.35%** | **✅** |
| 367 | **PH-S432** | Docs canon sync band (docs) | PH-S420 | INDEX/HANDOFF/NEXT/STABLE/GALAXY | **✅** |
| 368 | **PH-S433** | poolai-vision-sync + INDEX ratio (ops/docs) | PH-S421/S432 | `--check` green + INDEX §7 rust_ratio | **✅** |
| 369 | **PH-S434** | Live prefetch seed-pull resolver stub | Galaxy §5.5 | `resolve_seed_pull_shards` + inventory lookup; unit tests | **✅** |
| 370 | **PH-S435** | Replication executor enqueue hook | Galaxy §6.4 | `replication_executor_hook` + `galaxy_replication_executor_enqueue_total` | **✅** |
| 371 | **PH-S436** | Settlement payout batch ledger entry stub | Galaxy §8.2 | `PayoutBatchLedgerEntry` on cleared path; unit tests | **✅** |
| 372 | **PH-S437** | Verification checker enqueue stub | Galaxy §6.2 | `enqueue_verification_checker` + counter; unit tests | **✅** |
| 373 | **PH-S438** | Replay verification enqueue stub | Galaxy §6.3 | `record_replay_verification_enqueue` on mismatch; unit tests | **✅** |
| 374 | **PH-S439** | Signed capability document parse stub | Galaxy §6.6/§9 | `galaxy_capability_doc.rs` parse/validate; unit tests | **✅** |
| 375 | **PH-S440** | `network_profile` persistence across heartbeat | Galaxy §8.1 | `discovery_network_profile_integration` heartbeat test | **✅** |
| 376 | **PH-S441** | Admin metric history query wasm glue | RUST_RATIO §5.13 | `buildMetricHistoryQuery` wasm; slim `admin_charts.js` | **✅** |
| 377 | **PH-S442** | Stand smoke horizon wire band (S434–S438) | PH-S434…S438 | `/metrics` export shape executor/checker/replay enqueue | **✅** |
| 378 | **PH-S443** | Ops close band: loc-audit + docs + vision-sync | §5.12 fallback | `rust_ratio.json` **94.36%**; FM/HANDOFF/NEXT; `--check` green | **✅** |
| 379 | **PH-S444** | Live prefetch memory fetch stub | Galaxy §5.5, roadmap §4 | `fetch_seed_shards_hook` + `galaxy_prefetch_seed_fetch_*` metrics; unit tests | **✅** |
| 380 | **PH-S445** | Strict-locality grid ingest gate | Galaxy §5.6 | `locality_unsatisfied` + `galaxy_locality_unsatisfied_total`; unit tests | **✅** |
| 381 | **PH-S446** | Co-access graph speculative prefetch | Galaxy §5.5 | `PrefetchTrigger::CoAccessGraph` + `plan_co_access_prefetch`; metric + unit tests | **✅** |
| 382 | **PH-S447** | Verification replay wire DTO | Galaxy §6.3 | `GalaxyVerificationReplayRecord` on mismatch enqueue; unit tests | **✅** |
| 383 | **PH-S448** | Capability document register-remote wire | Galaxy §6.6 | optional `capability_document` on register-remote; integration tests | **✅** |
| 384 | **PH-S449** | Protocol negotiation rejected metric | Galaxy §9.8 | `poolai_protocol_negotiation_rejected_total` middleware + `/metrics` | **✅** |
| 385 | **PH-S450** | ML pipeline metrics panel wasm | RUST_RATIO §5.13 | `renderMlPipelineMetricsPanel` wasm; slim `admin_charts.js` | **✅** |
| 386 | **PH-S451** | Stand smoke horizon wire band (S444–S449) | PH-S444…S449 | `/metrics` export shape seed-fetch/co-access/locality/protocol/replay record | **✅** |
| 387 | **PH-S452** | Ops close band: loc-audit + docs canon | §5.12 fallback | `rust_ratio.json` **94.37%**; FM/HANDOFF/NEXT | **✅** |
| 388 | **PH-S453** | Vision-sync + `--check` | ops | `poolai-vision-sync`; FM rev = manifest | **✅** |
| 389 | **PH-S454** | Re-migrate delta prefetch trigger | Galaxy §5.5 | `re_migrate_prefetch_stub` on Migrating→Leased; `galaxy_prefetch_re_migrate_total`; unit tests | **✅** |
| 390 | **PH-S455** | Elevated verification sample rate | Galaxy §6.2 | `POOLAI_GALAXY_VERIFY_ELEVATED_RATE` + `galaxy_verification_elevated_applied_total`; unit tests | **✅** |
| 391 | **PH-S456** | Trust score delta on verify verdict | Galaxy §6.5 | `+10`/`-100` deltas + `galaxy_trust_score_delta_total`; unit tests | **✅** |
| 392 | **PH-S457** | Replication hourly rate-limit stub | Galaxy §6.6 | `POOLAI_GALAXY_REPLICATION_MAX_PER_HOUR` gate + `galaxy_replication_rate_limited_total` | **✅** |
| 393 | **PH-S458** | Hot tier promote/evict metrics | Galaxy §5.4 | `galaxy_hot_promote_total`/`galaxy_hot_evict_total` on prefetch path | **✅** |
| 394 | **PH-S459** | Locality telemetry counters (§5.3) | Galaxy §5.3 | `galaxy_shard_access_total` + `galaxy_prefetch_queue_depth` gauges | **✅** |
| 395 | **PH-S460** | Verification replay read API | Galaxy §6.3 | `GET /api/v1/grid/verification-replay`; integration test | **✅** |
| 396 | **PH-S461** | Monitoring alerts panel wasm | RUST_RATIO §5.13 | `renderMonitoringAlertsPanel` wasm; slim `monitoring.rs` | **✅** |
| 397 | **PH-S462** | Stand smoke horizon band S454–S460 | PH-S454…S460 | `/metrics` + verification-replay smoke | **✅** |
| 398 | **PH-S463** | Ops close band: loc-audit + vision-sync | §5.12 fallback | `rust_ratio.json`; FM/HANDOFF/NEXT; `--check` green | **✅** |
| 399 | **PH-S464** | Prefetch bandwidth backpressure stub | Galaxy §5.5 | `POOLAI_GALAXY_PREFETCH_MIN_BANDWIDTH_MBPS` + `galaxy_prefetch_backpressure_total`; unit tests | **✅** |
| 400 | **PH-S465** | RAID artifact prefetch fetch stub | Galaxy roadmap §4 | `fetch_seed_shards_from_raid_hook` + `galaxy_prefetch_raid_fetch_*`; unit tests | **✅** |
| 401 | **PH-S466** | Capability document signature verify stub | Galaxy §6.6 | ed25519 dev fixture + `verify_capability_signature_stub`; integration test | **✅** |
| 402 | **PH-S467** | Payout batch read API | Galaxy §8.2 | `GET /api/v1/grid/payout-batch` + integration test; OpenAPI sync | **✅** |
| 403 | **PH-S468** | Protocol negotiation accepted metric | Galaxy §9.3 | `poolai_protocol_negotiation_accepted_total` on register-remote Accepted | **✅** |
| 404 | **PH-S469** | Co-access graph env config | Galaxy §5.5 | `POOLAI_GALAXY_CO_ACCESS_GRAPH_JSON` + `co_access_graph_from_env`; unit tests | **✅** |
| 405 | **PH-S470** | Monitoring dashboards panel wasm | RUST_RATIO §5.13 | `renderMonitoringDashboardsPanel` wasm; slim `monitoring.rs` | **✅** |
| 406 | **PH-S471** | Galaxy horizon wire integration band | PH-S464…S469 | `galaxy_horizon_s464_integration` + `/metrics` shape | **✅** |
| 407 | **PH-S472** | Stand smoke S464–S471 band | PH-S464…S467 | `/metrics` + `grid/payout-batch` smoke | **✅** |
| 408 | **PH-S473** | Ops close band: loc-audit + vision-sync | §5.12 fallback | `rust_ratio.json`; FM/HANDOFF/NEXT; `--check` green | **✅** |
| 409 | **PH-S474** | Prefetch egress guardrail gate | Galaxy §8.1 | `lan_only` cross-region block + `galaxy_prefetch_egress_blocked_total`; unit tests | **✅** |
| 410 | **PH-S475** | Telegram seat cap admission stub | Galaxy §3.1 | `POOLAI_TELEGRAM_SEAT_LIMIT`; register-remote **409** `seat_exhausted`; integration test | **✅** |
| 411 | **PH-S476** | Capability verify env trust root | Galaxy §6.6 | `POOLAI_GALAXY_CAPABILITY_VERIFY_PK_HEX` override; integration test | **✅** |
| 412 | **PH-S477** | Payout batch history read API | Galaxy §8.2 | `GET /api/v1/grid/payout-batch/history`; OpenAPI + integration test | **✅** |
| 413 | **PH-S478** | Verification replay history read API | Galaxy §6.3 | `GET /api/v1/grid/verification-replay/history`; integration test + OpenAPI | **✅** |
| 414 | **PH-S479** | Prefetch peer seed fetch stub | Roadmap §4 | `fetch_seed_shards_from_peer_hook` + `galaxy_prefetch_peer_fetch_*`; unit tests | **✅** |
| 415 | **PH-S480** | Workers admin panel wasm | RUST_RATIO §5.13 | `renderWorkersPanel` wasm; slim `workers.rs` | **✅** |
| 416 | **PH-S481** | Horizon wire integration band S474–S479 | PH-S474…S479 | `galaxy_horizon_s474_integration` + `/metrics` shape | **✅** |
| 417 | **PH-S482** | Stand smoke S474–S479 band | PH-S474…S478 | `/metrics` + history API smoke | **✅** |
| 418 | **PH-S483** | Ops close band: loc-audit + vision-sync | §5.12 fallback | `rust_ratio.json`; FM/HANDOFF/NEXT; `--check` green | **✅** |
| 419 | **PH-S484** | Live prefetch bytes pull wire | Roadmap §4, Galaxy §5.5 | `galaxy_prefetch_pull_bytes_total` on memory fetch; unit tests | **✅** |
| 420 | **PH-S485** | Locality rank → grid schedule bind | Galaxy §5.2 | `ingest_job_locality_rank_stub` → `schedule_with_grid_peer`; integration test | **✅** |
| 421 | **PH-S486** | Telegram `bound_wallet_session` seat policy | Galaxy §3.1 | `POOLAI_TELEGRAM_SEAT_POLICY` + `compute_seat_limit`; unit tests | **✅** |
| 422 | **PH-S487** | Hot-tier promotion threshold env gate | Galaxy §5.4 | `POOLAI_GALAXY_HOT_PROMOTE_THRESHOLD` gates `record_hot_promote`; unit tests | **✅** |
| 423 | **PH-S488** | Verification checker task enqueue wire | Galaxy §6.2 | `enqueue_verification_checker_task` + task record; unit tests | **✅** |
| 424 | **PH-S489** | `network_profile` disk persistence | Galaxy §8.1 | `galaxy_network_profile_store` + register-remote persist; unit tests | **✅** |
| 425 | **PH-S490** | Instances admin panel wasm | RUST_RATIO §5.13 | `renderInstancesPanel` wasm; slim `instances.rs` | **✅** |
| 426 | **PH-S491** | Horizon wire integration band S484–S489 | PH-S484…S489 | `galaxy_horizon_s484_integration` + `/metrics` shape | **✅** |
| 427 | **PH-S492** | Stand smoke S484–S489 band | PH-S484…S488 | `/metrics` pull-bytes smoke | **✅** |
| 428 | **PH-S493** | Ops close band: loc-audit + vision-sync | §5.12 fallback | `rust_ratio.json` **94.41%**; FM/HANDOFF/NEXT; `--check` green | **✅** |
| 429 | **PH-S494** | Verification checker tasks read API | Galaxy §6.2 | `GET /api/v1/grid/verification-checker/tasks`; OpenAPI + unit tests | **✅** |
| 430 | **PH-S495** | Checker task drain on verdict | Galaxy §6.2 | `drain_verification_checker_task` on grid result match/mismatch | **✅** |
| 431 | **PH-S496** | Checker pending Prometheus gauge | Galaxy §6.2 | `galaxy_verification_checker_pending_total` on `/metrics` | **✅** |
| 432 | **PH-S497** | Network profile read API | Galaxy §8.1 | `GET /api/v1/grid/network-profiles/{peer_id}`; OpenAPI + tests | **✅** |
| 433 | **PH-S498** | Register-remote profile hydrate | PH-S489 | load persisted `network_profile` when metadata absent | **✅** |
| 434 | **PH-S499** | VM admin panel wasm | RUST_RATIO §5.13 | `renderVmPanel` wasm; slim `vm.rs` | **✅** |
| 435 | **PH-S500** | Horizon wire integration band S494–S499 | PH-S494…S499 | `galaxy_horizon_s494_integration` + `/metrics` shape | **✅** |
| 436 | **PH-S501** | Stand smoke S494–S499 band | PH-S494…S498 | `/metrics` + checker tasks + network profile API smoke | **✅** |
| 437 | **PH-S502** | Ops close band: loc-audit | §5.12 fallback | `rust_ratio.json` **94.41%**; FM/HANDOFF/NEXT | **✅** |
| 438 | **PH-S503** | Ops close band: vision-sync | §5.12 fallback | `poolai-vision-sync` + `--check` green | **✅** |
| 439 | **PH-S504** | Mandatory signed capability for `telegram_edge` | Galaxy §6.6/§9 | reject unsigned register-remote; dev fixture passes; integration test | **✅** |
| 440 | **PH-S505** | Telegram seat coordinator read API | Galaxy §3.1 | `GET /api/v1/grid/telegram-seats`; seat snapshot fields | **✅** |
| 441 | **PH-S506** | Network profile upsert API | Galaxy §8.1 | `PUT /api/v1/grid/network-profiles/{peer_id}`; round-trip GET | **✅** |
| 442 | **PH-S507** | Unified Galaxy worker DTO on virtual-nodes | Galaxy §2.3 | `galaxy` field on virtual-nodes list; origin + network_profile | **✅** |
| 443 | **PH-S508** | Admin workers: origin badges + locality sort | PH-S507 | virtual-nodes panel on `/ui/admin/workers`; wasm renderer | **✅** |
| 444 | **PH-S509** | tgbot `/wallet` command | Galaxy §3.2 | `Wallet` command POSTs wallet bind API; coordinator unit test | **✅** |
| 445 | **PH-S510** | Payout wallet rebind cooldown | Galaxy §3.2 | 409 `wallet_rebind_cooldown`; env cooldown secs | **✅** |
| 446 | **PH-S511** | Non-deterministic `semantic_hash` stub | Galaxy §6.2 | semantic_hash match/mismatch on grid result path | **✅** |
| 447 | **PH-S512** | Admin grid verification-checker panel | PH-S494 | `/ui/admin/grid-verification`; wasm table renderer | **✅** |
| 448 | **PH-S513** | Horizon close band S504–S512 | §5.12 fallback | `galaxy_horizon_s504_integration` + stand smoke + loc-audit + vision-sync | **✅** |
| 449 | **PH-S514** | tgbot `/status` → telegram-seats snapshot | Galaxy §3.1 | `fetch_telegram_seats`; `tgbot_coordinator_bridge_integration` | **✅** |
| 450 | **PH-S515** | tgbot `/stop` → unbind edge worker | Galaxy §3.2 | `DELETE .../telegram/bindings/{id}` client; integration test | **✅** |
| 451 | **PH-S516** | Galaxy DTO `capabilities` + `seed_inventory` | Galaxy §2.3 | virtual-nodes list round-trip; `galaxy_worker_dto_integration` | **✅** |
| 452 | **PH-S517** | Admin Telegram seats panel | PH-S505 | `/ui/admin/telegram-seats`; wasm renderer | **✅** |
| 453 | **PH-S518** | Job failover retry budget + fail_reason | Galaxy §4.3.3 | `lease_failover.rs`; `jobs_failover_budget_integration` | **✅** |
| 454 | **PH-S519** | Heartbeat refreshes `network_profile` freshness | Galaxy §8.1 | `last_measured_at` on heartbeat-remote; integration test | **✅** |
| 455 | **PH-S520** | `build_id` allowlist on register-remote | Galaxy §9.3 | `POOLAI_ALLOWED_BUILD_IDS`; 403 `build_id_rejected` | **✅** |
| 456 | **PH-S521** | Payout batch fee-split ledger fields | Galaxy §8.2 | `PayoutBatchLedgerEntry` split fields; payout-batch GET test | **✅** |
| 457 | **PH-S522** | Heartbeat miss → worker unhealthy metric | Galaxy §4.3.3 | `galaxy_worker_health`; `/metrics` gauge | **✅** |
| 458 | **PH-S523** | Horizon close band S514–S522 | §5.12 fallback | `galaxy_horizon_s514_integration` + stand smoke + loc-audit + vision-sync | **✅** |
| 459 | **PH-S524** | Worker-unhealthy → lease failover requeue | Galaxy §4.3.3 | `fail_reason=worker-unhealthy`; `jobs_worker_unhealthy_failover_integration` | **✅** |
| 460 | **PH-S525** | Scheduler bind skips unhealthy peers | Galaxy §4.3.3 | `pick_worker` + grid peer bind; `jobs_scheduler_unhealthy_integration` | **✅** |
| 461 | **PH-S526** | `max_total_runtime` job lifecycle cap | Galaxy §4.3.3 | `POOLAI_JOB_MAX_TOTAL_RUNTIME_SECS`; unit tests in `lease_failover.rs` | **✅** |
| 462 | **PH-S527** | Signed capability `expires_at` enforcement | Galaxy §6.6/§9 | `validate_capability_document_at`; telegram_edge requires expiry | **✅** |
| 463 | **PH-S528** | Governance ops Prometheus gauges | Galaxy §9.8 | `poolai_release_verify_*` + `poolai_update_notify_pending`; stand smoke | **✅** |
| 464 | **PH-S529** | Startup hydrate persisted `network_profile` | Galaxy §8.1 | `hydrate_persisted_network_profiles`; `network_profile_hydrate_integration` | **✅** |
| 465 | **PH-S530** | `queue_starvation` failover stub | Galaxy §4.3.3 | `POOLAI_JOB_QUEUE_STARVATION_SECS` + `leased_at`; unit tests | **✅** |
| 466 | **PH-S531** | Offline settlement mode on payout-batch wire | Galaxy §8.2 | `GET /api/v1/grid/payout-batch` → `settlement_mode: offline_batch` | **✅** |
| 467 | **PH-S532** | `admin_charts.js` wasm slim band | RUST_RATIO §5.13 | line/sparkline JS fallbacks removed; wasm-first chart glue | **✅** |
| 468 | **PH-S533** | Horizon close band S524–S532 | §5.12 fallback | `galaxy_horizon_s524_integration` + stand smoke + loc-audit + vision-sync | **✅** |
| 469 | **PH-S534** | Verification checker shadow job submit | Galaxy §6.2 | `submit_shadow_verification_checker_job` → JobStore; `galaxy_verification_checker_job_submit_total` | **✅** |
| 470 | **PH-S535** | Replay verification job enqueue | Galaxy §6.3 | `submit_replay_verification_job` on mismatch; JobStore `Verifying` row | **✅** |
| 471 | **PH-S536** | Replication parallel executor fan-out | Galaxy §6.4 | `enqueue_replication_executor_jobs` M parallel strict-tier jobs | **✅** |
| 472 | **PH-S537** | Peer HTTP seed-pull prefetch wire | Galaxy §5.5 | `fetch_seed_shards_from_peer_http` + `POOLAI_GALAXY_PREFETCH_PEER_HTTP_URL` | **✅** |
| 473 | **PH-S538** | Payout ledger `payout_pubkey` resolution | Galaxy §8.2 | `resolve_payout_pubkey`; `PayoutBatchLedgerEntry.payout_pubkey` on Cleared | **✅** |
| 474 | **PH-S539** | Cleared settlement → Solana sidecar stub | FM-010 · Galaxy §7 | `emit_settlement_job_rewarded` NDJSON `JobCompleted` + `payout_lamports` | **✅** |
| 475 | **PH-S540** | Task-probe capability admission gate | Galaxy §6.1 | `check_telegram_edge_capability_admission`; `capability_probe_required` | **✅** |
| 476 | **PH-S541** | Telegram cold-mining limits DTO | Galaxy §2.3 | `GalaxyWorkerLimits` max_cpu/ram/disk on virtual-nodes DTO | **✅** |
| 477 | **PH-S542** | Checker timeout inconclusive policy | Galaxy §6.2 | `evaluate_checker_timeout_policy`; retry → `VerificationInconclusive` | **✅** |
| 478 | **PH-S543** | Horizon close band S534–S542 | §5.12 fallback | `galaxy_horizon_s534_integration` + stand smoke + loc-audit + vision-sync | **✅** |
| 479 | **PH-S544** | Vision feed ticker header marquee | docs/vision UX | FEED у header; однорядкова бігуча строка; тонкий custom scroll rail; rev++ | **✅** |
| 480 | **PH-S545** | Replication quorum result gate | Galaxy §6.4 | `replication_quorum_allows_cleared` on strict-tier digests before Cleared | **✅** |
| 481 | **PH-S546** | Prefetch timeout fail trigger | Galaxy §5.6 | `evaluate_strict_prefetch_timeout` → `prefetch-timeout` under strict_locality | **✅** |
| 482 | **PH-S547** | Capacity-preemption lease failover | Galaxy §4.3.3 | `LeaseFailReason::CapacityPreemption` + `apply_capacity_preemption_failover` | **✅** |
| 483 | **PH-S548** | Scheduler pricing + queue_depth rank | Galaxy §5.2 | `LocalityWorker` tie-break queue_depth + pricing_usd_micro after locality score | **✅** |
| 484 | **PH-S549** | Update policy notify env stub | Galaxy §9.5/§9.8 | `POOLAI_UPDATE_POLICY` + `tick_update_notify_from_env` on verify-release | **✅** |
| 485 | **PH-S550** | On-chain settlement mode toggle | Galaxy §8.2 | `POOLAI_SETTLEMENT_ON_CHAIN=1` → payout-batch `on_chain` + pending | **✅** |
| 486 | **PH-S551** | Telegram cold-mining + GPU horizon docs | Galaxy §8.2 94.67% #2 | MVP CPU/RAM/Disk probros scope + GPU migration path in concept + RUN_LOCAL | **✅** |
| 487 | **PH-S552** | Edge trust_score disk persistence | Galaxy §6.5 | `galaxy_trust_score_store` persist/hydrate on register-remote | **✅** |
| 488 | **PH-S553** | Admin payout-batch read panel | Galaxy §8.2 | `/ui/admin/payout-batch` read-only GET payout-batch + history | **✅** |
| 489 | **PH-S554** | Horizon close band S545–S553 | §5.12 fallback | `galaxy_horizon_s545_integration` + loc-audit + vision-sync | **✅** |
| 490 | **PH-S555** | Vision map 3D orbit WASD + touch pad | docs/vision UX | `map-scene-3d` perspective orbit; WASD + center pad; layer stack sync; rev++ | **✅** |
| 491 | **PH-S556** | Vision map true 3D layer projection | docs/vision UX | `applyMap3DProjection` layer Z; WASD W↑S↓A←D→; pad bottom-center; rev++ | **✅** |
| 492 | **PH-S557** | Vision gravity solar-system layout | docs/vision UX | folder mass hubs + multi-ring orbits; orphans at rim; orbit 2× slower; planes 50% transparent; stack sync | **✅** |
| 493 | **PH-S558** | Fee settlement payout routing wire | Galaxy §8.2 94.67% #1 | `GET /api/v1/grid/payout-batch` routing snapshot; integration test; openapi-gap 0 | **✅** |
| 494 | **PH-S559** | Telegram wallet devnet verify opt-in | Galaxy §3.2 | `POOLAI_WALLET_VERIFY_DEVNET=1` → verified on bind; integration test | **✅** |
| 495 | **PH-S560** | Human-review settlement hold | Galaxy §6.2 | non-deterministic semantic_hash → PendingVerification; `galaxy_settlement_human_review_total` | **✅** |
| 496 | **PH-S561** | Signed capability production key verify | Galaxy §6.6 | `POOLAI_CAPABILITY_VERIFY_KEY` → 403 invalid sig on register-remote; integration test | **✅** |
| 497 | **PH-S562** | GPU passthrough admission gate | Galaxy §6.6 / §8.2 | `gpu_passthrough` in capability_document for inference:gpu; unit tests | **✅** |
| 498 | **PH-S563** | Network profile stale downgrade metric | Galaxy §8.1 | `galaxy_network_profile_stale_total` on locality rank; stand smoke | **✅** |
| 499 | **PH-S564** | Admin payout-batch wasm renderer | PH-S553, ratio §5.13 | `poolai-ui-core/payout_batch.rs` + wasm; slim admin JS | **✅** |
| 500 | **PH-S565** | Vision solar Playwright + pa11y payout-batch | PH-S557, FM-019 | `e2e/tests/vision.spec.ts`; `/ui/admin/payout-batch` in a11y matrix | **✅** |
| 501 | **PH-S566** | Topology label helpers → ui-core/wasm | PH-S157 | `poolai-ui-core/topology.rs`; wasm exports; slim JS glue | **✅** |
| 502 | **PH-S567** | Horizon close band S558–S566 | §5.12 fallback | `galaxy_horizon_s558_integration` + loc-audit + vision-sync | **✅** |
| 503 | **PH-S568** | On-chain Cleared → mock RPC submit | Galaxy §7 · FM-010 | `POOLAI_SETTLEMENT_ON_CHAIN=1` → mock RPC ack on Cleared NDJSON; unit test | **✅** |
| 504 | **PH-S569** | Checker-timeout `/metrics` export | Galaxy §6.2 PH-S542 | `galaxy_verification_checker_timeout_*` in prometheus + stand smoke | **✅** |
| 505 | **PH-S570** | `GET /api/v1/grid/network-profiles` list | Galaxy §8.1 | list persisted peer ids; integration round-trip | **✅** |
| 506 | **PH-S571** | Fraud-proof horizon stub | Galaxy §6.6 | `POOLAI_GALAXY_FRAUD_PROOF=1` → hold + `galaxy_fraud_proof_pending_total` | **✅** |
| 507 | **PH-S572** | TEE attestation on capability doc | Galaxy §6.6 | `tee_attestation` field + `POOLAI_TEE_ATTEST_REQUIRED` gate | **✅** |
| 508 | **PH-S573** | Security advisory acknowledge wire | Galaxy §9.6 | `POST /admin/security-advisories/{id}/acknowledge` + metric | **✅** |
| 509 | **PH-S574** | Peer HTTP prefetch integration | Roadmap §5.5 | wiremock seed-inventory HTTP → `galaxy_prefetch_peer_fetch_total` | **✅** |
| 510 | **PH-S575** | Admin table toolbar → wasm | RUST_RATIO §5.13 | `table_export_buttons_html` + `exportFilenameFromAria` wasm; slim JS | **✅** |
| 511 | **PH-S576** | Protocol sunset env gate | Galaxy §9.6 | `POOLAI_PROTOCOL_SUNSET_MIN` → HTTP 426 on register-remote | **✅** |
| 512 | **PH-S577** | Horizon close band S568–S576 | §5.12 fallback | `galaxy_horizon_s568_integration` + loc-audit + vision-sync | **✅** |
| 513 | **PH-S578** | Vision fullscreen PiP above header | docs/vision UX | `.panel-fullscreen` `inset:0` + workspace z-index above header; Explorer overlay `top:0`; rev++ | **✅** |
| 514 | **PH-S579** | Galaxy map fit-all zoom + auto-orbit + WebP bg | docs/vision UX | default/⌂ fit-all; ▶/⏸ orbit 90% WASD + auto zoom; `vision2.webp`; FX tune; rev **257** | **✅** |
| 515 | **PH-S580** | `hot_tier_hit_ratio` Prometheus gauge | Galaxy §5.2 | `galaxy_hot_tier_hit_ratio` on rank path; unit test; `/metrics` export | **✅** |
| 516 | **PH-S581** | Hot-tier metric stand smoke | stand smoke | `poolai-http-stand-smoke` case `galaxy_hot_tier_hit_ratio_metrics` | **✅** |
| 517 | **PH-S582** | Admin network-profiles panel | Galaxy §8.1 | `/ui/admin/network-profiles` + list/per-peer GET; admin smoke | **✅** |
| 518 | **PH-S583** | heartbeat-remote `network_profile` persist | Galaxy §8.1 | optional `metadata.network_profile` on heartbeat → persist; integration test | **✅** |
| 519 | **PH-S584** | Admin seed-inventory panel | Galaxy §5.5 | `/ui/admin/seed-inventory` read-only; admin smoke | **✅** |
| 520 | **PH-S585** | Vision auto-orbit / fit-all Playwright | docs/vision UX | `vision.spec.ts` ▶/⏸ + ⌂ fit-all smoke (PH-S579) | **✅** |
| 521 | **PH-S586** | Security advisories list + admin UI | Galaxy §9.6 | `GET /admin/security-advisories` stub + `/ui/admin/security-advisories` | **✅** |
| 522 | **PH-S587** | Updates-compat update policy readout | Galaxy §9.5 | `POOLAI_UPDATE_POLICY` + `POOLAI_RELEASE_MANIFEST_URL` on updates-compat page | **✅** |
| 523 | **PH-S588** | Co-access prefetch HTTP integration | Roadmap §5.5 | grid ingest + `POOLAI_GALAXY_CO_ACCESS_GRAPH_JSON` → `galaxy_prefetch_co_access_total` | **✅** |
| 524 | **PH-S589** | Horizon close band S580–S588 | §5.12 fallback | `galaxy_horizon_s580_integration` + loc-audit + vision-sync `--check` | **✅** |
| 525 | **PH-S590** | Vision orbit UX carry-over (pause/layers/speed) | docs/vision UX | pause RAF fix; orbit ~30% WASD; `galaxy-bg` `pointer-events:none`; controls z-index; `vision.spec.ts` rotY play/pause | **✅** |
| 526 | **PH-S591** | Prefetch backpressure from profile `bandwidth_mbps` | Galaxy §8.1 | `with_prefetch_peer` + persisted profile gate; unit + integration | **✅** |
| 527 | **PH-S592** | Prefetch egress guardrail from profile `egress_policy` | Galaxy §8.1 | profile-driven `lan_only` cross-region block + metric | **✅** |
| 528 | **PH-S593** | GPU passthrough grid envelope HTTP integration | Galaxy §6.6 | `POST /grid/envelope` inference:gpu → 403 `gpu_passthrough_required` | **✅** |
| 529 | **PH-S594** | TEE attestation register-remote HTTP integration | Galaxy §6.6 | `POOLAI_TEE_ATTEST_REQUIRED=1` → 400 without attestation | **✅** |
| 530 | **PH-S595** | Wallet rebind admin override API | Galaxy §3.2 | `POST …/wallet/rebind-override` + admin bearer; override metric | **✅** |
| 531 | **PH-S596** | Admin network-profiles upsert UI | Galaxy §8.1 | PUT form round-trip; `admin.spec.ts` smoke | **✅** |
| 532 | **PH-S597** | On-chain Cleared grid complete HTTP integration | Galaxy §7 | `SETTLEMENT_ON_CHAIN=1` result → mock RPC ack via HTTP | **✅** |
| 533 | **PH-S598** | Galaxy admin a11y matrix expand | FM-019 / DOCS_LEGACY | network-profiles, seed-inventory, security-advisories in pa11y + axe | **✅** |
| 534 | **PH-S599** | Horizon close band S591–S598 | §5.12 fallback | `galaxy_horizon_s591_integration` + loc-audit + vision-sync `--check` | **✅** |
| 535 | **PH-S600** | Strict-locality grid job ingest HTTP | Galaxy §5.6 | `POST /grid/envelope` + `strict_locality` → 409 `locality_unsatisfied` / `prefetch-timeout`; integration test | **✅** |
| 536 | **PH-S601** | Human-review settlement hold HTTP | Galaxy §6.2 | grid result `non_deterministic` semantic_hash mismatch → `galaxy_settlement_human_review_total` | **✅** |
| 537 | **PH-S602** | Wallet rebind cooldown HTTP | Galaxy §3.2 | second `POST …/telegram/wallet` → 409 `wallet_rebind_cooldown` integration test | **✅** |
| 538 | **PH-S603** | `latency_ms_p95` tail-latency locality penalty | Galaxy §8.1 | p95≫p50 penalty in `locality_score` + `galaxy_tail_latency_penalty_total` | **✅** |
| 539 | **PH-S604** | Topology ring / white-IP prefetch admission | Galaxy §8.1 | `prefetch_topology_admission_blocked_skip` + `galaxy_prefetch_topology_blocked_total` | **✅** |
| 540 | **PH-S605** | RAID prefetch fetch grid job HTTP | Roadmap §5.5 | job ingest → `galaxy_prefetch_raid_fetch_total` via HTTP | **✅** |
| 541 | **PH-S606** | Re-migrate prefetch Migrating→Leased HTTP | Galaxy §5.5 | `PATCH /jobs/{id}` → `galaxy_prefetch_re_migrate_total` integration test | **✅** |
| 542 | **PH-S607** | Fraud-proof hold grid envelope HTTP | Galaxy §6.6 | `POOLAI_GALAXY_FRAUD_PROOF=1` result mismatch → `galaxy_fraud_proof_pending_total` | **✅** |
| 543 | **PH-S608** | Admin dashboard wasm-first formatters slim | RUST_RATIO §5.13 | remove JS `formatUptime` dup; wasm `formatPercent`/`formatMegabytes` glue | **✅** |
| 544 | **PH-S609** | Horizon close band S600–S608 | §5.12 fallback | `galaxy_horizon_s600_integration` + loc-audit + vision-sync `--check` | **✅** |
| 545 | **PH-S610** | Trust delta on stale-epoch grid result (−50) | Galaxy §6.5 | `lease_epoch_rejected` result path → `galaxy_trust_score_delta_total`; integration test | **✅** |
| 546 | **PH-S611** | Trust delta on worker-unhealthy streak (−30) | Galaxy §6.5 + §4.3.3 | heartbeat miss threshold → trust store delta + metric | **✅** |
| 547 | **PH-S612** | Hot-tier hit-ratio scheduling gate HTTP | Galaxy §5.4 | prefer `hot_tier_hit_ratio > 0.8` over zero-hit peer; `galaxy_hot_tier_gate_applied_total` | **✅** |
| 548 | **PH-S613** | Re-migrate delta-fetch missing shards | Galaxy §5.5 | Migrating→Leased PATCH plans prefetch only for shards absent from memory | **✅** |
| 549 | **PH-S614** | Prefetch order by shard access weight | Galaxy §5.5 + §5.3 | `POOLAI_GALAXY_SHARD_ACCESS_WEIGHTS` orders `plan_prefetch`; integration test | **✅** |
| 550 | **PH-S615** | Replication hourly cap HTTP integration | Galaxy §6.6 | `replication_strict` over cap → `galaxy_replication_rate_limited_total` via HTTP | **✅** |
| 551 | **PH-S616** | Primary/secondary/worker lamports payout-batch wire | Galaxy §8.2 | `GET /api/v1/grid/payout-batch` routing includes `worker_lamports` split | **✅** |
| 552 | **PH-S617** | Checker-timeout grid result HTTP integration | Galaxy §6.2 PH-S542 | grid result `checker_timeout` retry→inconclusive; `/metrics` counters | **✅** |
| 553 | **PH-S618** | Admin raid `formatBytes` wasm-first slim | RUST_RATIO §5.13 | `poolai-ui-core`/`poolai-ui-wasm` `formatBytes`; slim `raid.rs` JS dup | **✅** |
| 554 | **PH-S619** | Horizon close band S610–S618 | §5.12 fallback | `galaxy_horizon_s610_integration` + loc-audit + vision-sync `--check` | **✅** |
| 555 | **PH-S620** | Verification verdict trust delta → persist store | Galaxy §6.5 | grid result `verification_verdict` adjusts stored `trust_score` + `galaxy_trust_score_delta_total`; integration test | **✅** |
| 556 | **PH-S621** | Trust payout-held gate HTTP for `telegram_edge` | Galaxy §6.5 | low `trust_score` tg peer → `galaxy_trust_payout_held_total` on `/metrics`; integration test | **✅** |
| 557 | **PH-S622** | Post-mismatch elevated sampling HTTP | Galaxy §6.2 PH-S455 | grid result `verification_verdict:mismatch` → `galaxy_verification_elevated_applied_total` via HTTP | **✅** |
| 558 | **PH-S623** | Prefetch lease-acquired HTTP wire | Galaxy §5.5 PH-S425 | `POST /api/v1/jobs/{id}/lease` → `galaxy_prefetch_lease_acquired_total`; integration test | **✅** |
| 559 | **PH-S624** | Hot-tier promote/evict HTTP integration | Galaxy §5.4 PH-S458 | grid job ingest → `galaxy_hot_promote_total` / `galaxy_hot_evict_total` on `/metrics` | **✅** |
| 560 | **PH-S625** | Prefetch ingest/wait/complete HTTP metric band | Galaxy §5.5 | grid job ingest drives ingest/wait/complete counters on `/metrics`; integration test | **✅** |
| 561 | **PH-S626** | `shard_fetch_latency_ms_p50` telemetry gauge | Galaxy §5.3 | gauge on prefetch fetch path + unit test + `/metrics` export | **✅** |
| 562 | **PH-S627** | Admin raid drop JS `formatBytes` dup (wasm-only) | RUST_RATIO §5.13 PH-S618 | remove JS fallback in `raid.rs`; wasm `formatBytes` only | **✅** |
| 563 | **PH-S628** | Admin security datetime helpers → ui-core/wasm | RUST_RATIO §5.13 | `formatUnixTimestamp` / `formatRotationKind` in `poolai-ui-core` + wasm; slim `security.rs` | **✅** |
| 564 | **PH-S629** | Horizon close band S620–S628 | §5.12 fallback | `galaxy_horizon_s620_integration` + loc-audit + vision-sync `--check` | **✅** |
| 565 | **PH-S630** | Verification mismatch trust delta → persist store | Galaxy §6.5 / §6.2 | grid result `verification_verdict:mismatch` adjusts stored `trust_score` (−100) + `galaxy_trust_score_delta_total`; integration test | **✅** |
| 566 | **PH-S631** | Cleared settlement payout-batch HTTP wire | Galaxy §8.2 / PH-S427 | grid Cleared result → `galaxy_settlement_payout_batch_total` on `/metrics`; integration test | **✅** |
| 567 | **PH-S632** | Prefetch seed-pull complete hook HTTP | Galaxy §5.5 / roadmap §4 | grid job ingest + hot-tier skip fallback drives `galaxy_prefetch_seed_pull_total`; integration test | **✅** |
| 568 | **PH-S633** | Replication executor enqueue HTTP | Galaxy §6.4 / PH-S435 | grid job ingest → `galaxy_replication_executor_enqueue_total` on `/metrics`; integration test | **✅** |
| 569 | **PH-S634** | Replay verification enqueue on mismatch HTTP | Galaxy §6.3 / PH-S438 | grid result `verification_verdict:mismatch` → `galaxy_replay_verification_enqueue_total`; integration test | **✅** |
| 570 | **PH-S635** | Worker-unhealthy heartbeat-remote HTTP | Galaxy §4.3.3 / PH-S522 | consecutive `POST /discovery/heartbeat-remote` misses → `galaxy_worker_unhealthy_total`; integration test | **✅** |
| 571 | **PH-S636** | Admin topology formatters → ui-core/wasm | RUST_RATIO §5.13 | `formatTopologyTimestamp` / `formatLoadFraction` / `formatLatencyMs` wasm-only; slim `topology.rs` | **✅** |
| 572 | **PH-S637** | Admin security datetime wasm-only slim | RUST_RATIO §5.13 / PH-S628 | remove JS fallback paths in `security.rs`; wasm-only glue | **✅** |
| 573 | **PH-S638** | Admin grid-pricing formatters wasm-only | RUST_RATIO §5.13 / PH-S151 | drop `formatUsdMicroFallback` in `grid_pricing.rs`; wasm-only USD/time formatters | **✅** |
| 574 | **PH-S639** | Horizon close band S630–S638 | §5.12 fallback / roadmap §4 | `galaxy_horizon_s630_integration` + loc-audit + vision-sync `--check` | **✅** |
| 575 | **PH-S640** | Replay pending resolved HTTP wire | Galaxy §6.3 | grid result `replay_verdict:accepted` → `galaxy_replay_pending_resolved_total`; integration test | **✅** |
| 576 | **PH-S641** | Verification replay record HTTP + history API | Galaxy §6.3 PH-S460/S478 | mismatch result → `galaxy_verification_replay_record_total` + `GET /grid/verification-replay/history`; integration test | **✅** |
| 577 | **PH-S642** | Verification checker enqueue HTTP | Galaxy §6.2 PH-S437 | sampled `telegram_edge` result → `galaxy_verification_checker_enqueue_total`; integration test | **✅** |
| 578 | **PH-S643** | Trust payout-eligible HTTP wire | Galaxy §6.5 | high-trust `telegram_edge` cleared result → `galaxy_trust_payout_eligible_total`; integration test | **✅** |
| 579 | **PH-S644** | Settlement resolved counter HTTP | Galaxy §6.4 PH-S404 | grid result path → `galaxy_settlement_resolved_total` on `/metrics`; integration test | **✅** |
| 580 | **PH-S645** | Prefetch strict-mode HTTP wire | Galaxy §5.5 PH-S303 | `strict_locality` job ingest → `galaxy_prefetch_strict_mode_total`; integration test | **✅** |
| 581 | **PH-S646** | Admin dashboard datetime wasm-only | RUST_RATIO §5.13 | `formatAuditTimestamp` wasm-only; drop `toLocaleString` fallback in `dashboard.rs` | **✅** |
| 582 | **PH-S647** | Admin updates-compat labels wasm-only | RUST_RATIO §5.13 PH-S197 | drop compat/protocol JS fallbacks in `updates_compat.rs` | **✅** |
| 583 | **PH-S648** | Admin jobs lease badge wasm-only | RUST_RATIO §5.13 PH-S152 | remove `leaseStateFallback`; wasm-only `leaseStateLabel` | **✅** |
| 584 | **PH-S649** | Horizon close band S640–S648 | §5.12 fallback | `galaxy_horizon_s640_integration` + loc-audit + vision-sync `--check` | **✅** |
| 585 | **PH-S650** | ui-core warning cleanup (`table.rs`) | RUST_RATIO §5.13 maintain | remove unused `json` import in `poolai-ui-core` table module | **✅** |
| 586 | **PH-S651** | Galaxy roadmap snapshot sync to S640…S649 | Roadmap §4 / FM §5.12 | refresh `GALAXY_GRID_ROADMAP_2026-05-27.md` current band + ratio | **✅** |
| 587 | **PH-S652** | Cursor sandbox temp cache triage | VDT S0 / shell hygiene | audit + cleanup `%TEMP%/cursor-sandbox-cache` disk pressure blocker | **✅** |
| 588 | **PH-S653** | Vision gate recovery after temp cleanup | `poolai-session-iteration` Vision close | restore sandbox path and run `poolai-vision-sync --check` (`ok`, rev 264) | **✅** |
| 589 | **PH-S654** | Workspace format gate | VDT test gate | `cargo fmt --all` (workspace) | **✅** |
| 590 | **PH-S655** | ui-core test gate rerun + blocker note | Rust test policy | `cargo test -p poolai-ui-core` rerun; 3 pre-existing failing tests recorded | **✅** |
| 591 | **PH-S656** | FM §5.12 maintenance close-band sync | FM journal canonical | append PH-S650…S659 maintenance rows + close-band marker | **✅** |
| 592 | **PH-S657** | HANDOFF maintenance snapshot sync | HANDOFF canonical | add PH-S650…S659 summary + gate notes (vision/test/temp) | **✅** |
| 593 | **PH-S658** | NEXT session prompt sync | NEXT canonical | update top snapshot to latest closed band and next trigger | **✅** |
| 594 | **PH-S659** | STABLE_STATE header refresh | status canon | update stable-state header/date to current §5.12/ratio snapshot | **✅** |
| 595 | **PH-S660** | ui-core format timestamp UTC fix | PH-S655 blocker / `format.rs` | `format_unix_timestamp_display_ph_s628` green (UTC, not local TZ) | **✅** |
| 596 | **PH-S661** | ui-core ML metric URL encode fix | PH-S655 blocker / `ml.rs` | `build_metric_history_url_ph_s314/s334` green (`cpu%2Eusage`) | **✅** |
| 597 | **PH-S662** | ui-core full test gate | Rust test policy | `cargo test -p poolai-ui-core` — 0 failed | **✅** |
| 598 | **PH-S663** | Shared layout datetime wasm-only | RUST_RATIO §5.13 / `src/ui/mod.rs` | drop `toLocaleString` fallback in shared layout helper | **✅** |
| 599 | **PH-S664** | network_profile persist stub | Galaxy §8 L916 | heartbeat metadata + in-memory persist stub; unit test | **✅** |
| 600 | **PH-S665** | Rust ratio loc-audit refresh | §5.13 fallback | `poolai-loc-audit` → `rust_ratio.json` sprint zriz | **✅** |
| 601 | **PH-S666** | Docs INDEX canon sync | docs canon | INDEX §7 + rust_ratio **94.70%** + vision rev from manifest | **✅** |
| 602 | **PH-S667** | poolai-vision-sync drift gate | ops / PH-S350 pattern | `poolai-vision-sync --check` green | **✅** |
| 603 | **PH-S668** | Ratio hold advisory snapshot | PH-S351 pattern | `poolai-loc-audit --min-ratio 0.95 --advisory` snapshot **94.70%** | **✅** |
| 604 | **PH-S669** | Horizon close band S660–S668 | §5.12 fallback | `galaxy_horizon_s660_integration` + FM/HANDOFF/NEXT/STABLE/GALAXY sync | **✅** |
| 605 | **PH-S670** | Galaxy verification metric HTTP wire | Galaxy verification/replay band 2 | `GET /api/v1/grid/verification-metrics`; integration test | **✅** |
| 606 | **PH-S671** | Galaxy replay metric HTTP wire | Galaxy verification/replay band 2 | `GET /api/v1/grid/replay-metrics`; integration test | **✅** |
| 607 | **PH-S672** | Admin panel wasm glue | poolai-ui-core + grid_verification | `parsePrometheusGauge` wasm; admin test | **✅** |
| 608 | **PH-S673** | Stand smoke /metrics export shape | poolai-http-stand-smoke | verification/replay metrics API smoke + export shape | **✅** |
| 609 | **PH-S674** | Galaxy concept helper stub | Galaxy §6.3 | `verification_replay_depth_stub` + unit test | **✅** |
| 610 | **PH-S675** | Rust ratio loc-audit refresh | §5.13 fallback | `poolai-loc-audit` → `rust_ratio.json` **94.72%** | **✅** |
| 611 | **PH-S676** | Docs INDEX canon sync | docs canon | INDEX + HANDOFF + NEXT + STABLE + GALAXY sync | **✅** |
| 612 | **PH-S677** | poolai-vision-sync drift gate | ops | `poolai-vision-sync --check` green | **✅** |
| 613 | **PH-S678** | Ratio hold advisory snapshot | PH-S351 pattern | `--min-ratio 0.95 --advisory` **94.72%** | **✅** |
| 614 | **PH-S679** | Horizon close band S670–S678 | §5.12 fallback | `galaxy_horizon_s670_integration` + docs sync | **✅** |
| 615 | **PH-S680** | Galaxy settlement metric HTTP wire | Galaxy settlement/trust band 3 | `GET /api/v1/grid/settlement-metrics`; integration test | **✅** |
| 616 | **PH-S681** | Galaxy trust metric HTTP wire | Galaxy settlement/trust band 3 | `GET /api/v1/grid/trust-metrics`; integration test | **✅** |
| 617 | **PH-S682** | Admin panel wasm glue | poolai-ui-core + payout_batch | `parsePrometheusGauge` + settlement/trust JSON; admin test | **✅** |
| 618 | **PH-S683** | Stand smoke /metrics export shape | poolai-http-stand-smoke | settlement/trust metrics API smoke + export shape | **✅** |
| 619 | **PH-S684** | Galaxy concept helper stub | Galaxy §6.4–§6.5 | `settlement_gate_depth_stub` + unit test | **✅** |
| 620 | **PH-S685** | Rust ratio loc-audit refresh | §5.13 fallback | `poolai-loc-audit` → `rust_ratio.json` **94.73%** | **✅** |
| 621 | **PH-S686** | Docs INDEX canon sync | docs canon | INDEX + HANDOFF + NEXT + STABLE + GALAXY sync | **✅** |
| 622 | **PH-S687** | poolai-vision-sync drift gate | ops | `poolai-vision-sync --check` green | **✅** |
| 623 | **PH-S688** | Ratio hold advisory snapshot | PH-S351 pattern | `--min-ratio 0.95 --advisory` **94.73%** | **✅** |
| 624 | **PH-S689** | Horizon close band S680–S688 | §5.12 fallback | `galaxy_horizon_s680_integration` + docs sync | **✅** |
| 625 | **PH-S690** | Galaxy replication metric HTTP wire | Galaxy §6.4 replication | `GET /api/v1/grid/replication-metrics`; integration test | **✅** |
| 626 | **PH-S691** | Galaxy pricing metric HTTP wire | Galaxy §4.2 oracle snapshot | `GET /api/v1/grid/pricing-metrics`; integration test | **✅** |
| 627 | **PH-S692** | Admin panel wasm glue | replication/pricing admin panels | `parsePrometheusGauge` + JSON metrics fetch; admin test | **✅** |
| 628 | **PH-S693** | Stand smoke replication/pricing API | poolai-http-stand-smoke | replication-metrics + pricing-metrics API shape | **✅** |
| 629 | **PH-S694** | Galaxy concept helper stub | Galaxy §4.2 / §6.4 | `replication_pricing_depth_stub` + unit test | **✅** |
| 630 | **PH-S695** | Rust ratio loc-audit refresh | §5.13 fallback | `poolai-loc-audit` → `rust_ratio.json` **94.67%** | **✅** |
| 631 | **PH-S696** | Docs INDEX canon sync | docs canon | INDEX + HANDOFF + NEXT + STABLE + GALAXY sync | **✅** |
| 632 | **PH-S697** | poolai-vision-sync drift gate | ops | `poolai-vision-sync --check` green | **✅** |
| 633 | **PH-S698** | Ratio hold advisory snapshot | PH-S351 pattern | `--min-ratio 0.95 --advisory` **94.67%** | **✅** |
| 634 | **PH-S699** | Horizon close band S690–S698 | §5.12 fallback | `galaxy_horizon_s690_integration` + docs sync | **✅** |
| 635 | **PH-S700** | Admin wasm slim panel #1 | ui-core → poolai-ui-wasm | `render_grid_replication_pricing_panel_html` + wasm export | **✅** |
| 636 | **PH-S701** | Admin wasm slim panel #2 | admin_charts canvas glue → wasm | ML/chart JS fallbacks removed; `poolaiRenderGridReplicationPricingPanel` | **✅** |
| 637 | **PH-S702** | Admin wasm glue regression | admin/mod.rs wasm render | parsePrometheusGauge + wasm slim regression tests | **✅** |
| 638 | **PH-S703** | Stand smoke /metrics export shape | poolai-http-stand-smoke | replication/pricing wasm panel export shape tests | **✅** |
| 639 | **PH-S704** | Galaxy concept helper stub | Galaxy §4–§8 horizon | `admin_wasm_slim_depth_stub` + unit test | **✅** |
| 640 | **PH-S705** | Rust ratio loc-audit refresh | §5.13 fallback | `poolai-loc-audit` → `rust_ratio.json` **94.75%** | **✅** |
| 641 | **PH-S706** | Docs INDEX canon sync | docs canon | INDEX + HANDOFF + NEXT + STABLE + GALAXY sync | **✅** |
| 642 | **PH-S707** | poolai-vision-sync drift gate | ops | `poolai-vision-sync --check` green | **✅** |
| 643 | **PH-S708** | Ratio hold advisory snapshot | PH-S351 pattern | `--min-ratio 0.95 --advisory` **94.75%** | **✅** |
| 644 | **PH-S709** | Horizon close band S700–S708 | §5.12 fallback | `galaxy_horizon_s700_integration` + docs sync | **✅** |
| 645 | **PH-S710** | Stand smoke JSON metric API #1 | band 6 stand smoke | verification/replay JSON export shape parity tests | **✅** |
| 646 | **PH-S711** | Stand smoke JSON metric API #2 | band 6 stand smoke | settlement/trust/replication/pricing JSON export shape | **✅** |
| 647 | **PH-S712** | Admin panel wasm glue | poolai-ui-core | verification-metrics JSON + `renderGridVerificationMetricsStrip` | **✅** |
| 648 | **PH-S713** | Stand smoke runner extend | poolai-http-stand-smoke | `grid_metrics_json_prometheus_parity_band6` live + unit tests | **✅** |
| 649 | **PH-S714** | Galaxy concept helper stub | Galaxy §4–§8 | `stand_smoke_metrics_parity_depth_stub` + unit test | **✅** |
| 650 | **PH-S715** | Rust ratio loc-audit refresh | §5.13 fallback | `poolai-loc-audit` → `rust_ratio.json` **94.76%** | **✅** |
| 651 | **PH-S716** | Docs INDEX canon sync | docs canon | INDEX + HANDOFF + NEXT + STABLE + GALAXY sync | **✅** |
| 652 | **PH-S717** | poolai-vision-sync drift gate | ops | `poolai-vision-sync --check` green | **✅** |
| 653 | **PH-S718** | Ratio hold advisory snapshot | PH-S351 pattern | `--min-ratio 0.95 --advisory` **94.76%** | **✅** |
| 654 | **PH-S719** | Horizon close band S710–S718 | §5.12 fallback | `galaxy_horizon_s710_integration` + docs sync | **✅** |
| 655 | **PH-S720** | `re_migrate_policy_depth_stub` | Galaxy **§4.3** | unit test; dispatch/scheduler hook | **✅** |
| 656 | **PH-S721** | `routing_policy_locality_gate` | Galaxy **§4.1** | strict routing helper + unit test | **✅** |
| 657 | **PH-S722** | Admin settlement/trust metrics wasm strip | poolai-ui-core | fetch JSON metrics + wasm render | **✅** |
| 658 | **PH-S723** | Stand smoke settlement/trust JSON↔Prom parity | poolai-http-stand-smoke | unit tests in stand smoke bin | **✅** |
| 659 | **PH-S724** | Concept stub extend (§4–§8) | band 7 depth | unit test | **✅** |
| 660 | **PH-S725** | Rust ratio loc-audit refresh | §5.13 fallback | `poolai-loc-audit` → `rust_ratio.json` **94.55%** | **✅** |
| 661 | **PH-S726** | Docs INDEX canon sync | docs canon | INDEX + HANDOFF + NEXT + STABLE + GALAXY + completion roadmap | **✅** |
| 662 | **PH-S727** | poolai-vision-sync drift gate | ops | `poolai-vision-sync --check` green | **✅** |
| 663 | **PH-S728** | Ratio hold advisory snapshot | PH-S351 pattern | `--min-ratio 0.95 --advisory` **94.55%** | **✅** |
| 664 | **PH-S729** | Horizon close band S720–S728 | Galaxy **§4** routing | `galaxy_horizon_s720_integration` + docs sync | **✅** |
| 665 | **PH-S730** | `network_profile_store` persist read | Galaxy **§8.1** | GET profile survives restart stub + integration test | **✅** |
| 666 | **PH-S731** | `network_profile_store` persist write | Galaxy **§8.1** | PUT + heartbeat merge persist + test | **✅** |
| 667 | **PH-S732** | Admin network-profile panel wasm glue | poolai-ui-core | fetch `/grid/network-profiles` + wasm render | **✅** |
| 668 | **PH-S733** | Stand smoke network-profiles list/put | poolai-http-stand-smoke | live runner cases green | **✅** |
| 669 | **PH-S734** | `network_profile_depth_stub` | Galaxy **§8.1** | egress/locality classification + unit test | **✅** |
| 670 | **PH-S735** | Rust ratio loc-audit refresh | §5.13 fallback | `poolai-loc-audit` → `rust_ratio.json` **94.57%** | **✅** |
| 671 | **PH-S736** | Docs INDEX canon sync | docs canon | INDEX + HANDOFF + NEXT + STABLE + GALAXY + completion roadmap | **✅** |
| 672 | **PH-S737** | poolai-vision-sync drift gate | ops | `poolai-vision-sync --check` green | **✅** |
| 673 | **PH-S738** | Ratio hold advisory snapshot | PH-S351 pattern | `--min-ratio 0.95 --advisory` **94.57%** | **✅** |
| 674 | **PH-S739** | Horizon close band S730–S738 | Galaxy **§8.1** profile | `galaxy_horizon_s730_integration` + docs sync | **✅** |
| 675 | **PH-S740** | signed capability strict gate | Galaxy **§6.6** | unsigned edge → 403 + metric | **✅** |
| 676 | **PH-S741** | signed capability dev fixture pass | Galaxy **§6.6** | integration test register-remote OK | **✅** |
| 677 | **PH-S742** | Admin capability doc panel extend | poolai-ui-core | updates-compat capability section | **✅** |
| 678 | **PH-S743** | Stand smoke signed-cap reject shape | poolai-http-stand-smoke | export shape unit test | **✅** |
| 679 | **PH-S744** | `capability_admission_depth_stub` | Galaxy **§6.6** | unit test | **✅** |
| 680 | **PH-S745** | Rust ratio loc-audit refresh | §5.13 fallback | `poolai-loc-audit` → `rust_ratio.json` PH-S745 zriz | **✅** |
| 681 | **PH-S746** | SECURITY_HARDENING ↔ §6.6 cross-link | docs canon | docs canon | **✅** |
| 682 | **PH-S747** | poolai-vision-sync drift gate | ops | `poolai-vision-sync --check` green | **✅** |
| 683 | **PH-S748** | Ratio hold advisory snapshot | PH-S351 pattern | `--min-ratio 0.95 --advisory` **94.59%** | **✅** |
| 684 | **PH-S749** | Horizon close band S740–S748 | Galaxy **§6.6** capability | `galaxy_horizon_s740_integration` + docs sync | **✅** |
| 685 | **PH-S750** | prefetch live bytes metric parity | Galaxy **§5.5** | JSON/Prometheus parity test | **✅** |
| 686 | **PH-S751** | prefetch backpressure bandwidth gate | Galaxy **§5.5** | unit + integration test | **✅** |
| 687 | **PH-S752** | Admin prefetch metrics wasm glue | poolai-ui-core | ui-core metrics strip | **✅** |
| 688 | **PH-S753** | Stand smoke prefetch-metrics API | poolai-http-stand-smoke | runner + unit test | **✅** |
| 689 | **PH-S754** | `prefetch_depth_stub` | Galaxy **§5.5** | unit test | **✅** |
| 690 | **PH-S755** | Rust ratio loc-audit refresh | §5.13 fallback | `poolai-loc-audit` → `rust_ratio.json` PH-S755 zriz | **✅** |
| 691 | **PH-S756** | GALAXY §5.5 implemented table | docs canon | docs canon | **✅** |
| 692 | **PH-S757** | poolai-vision-sync drift gate | ops | `poolai-vision-sync --check` green | **✅** |
| 693 | **PH-S758** | Ratio hold advisory snapshot | PH-S351 pattern | `--min-ratio 0.95 --advisory` **94.62%** | **✅** |
| 694 | **PH-S759** | Horizon close band S750–S758 | Galaxy **§5.5** prefetch | `galaxy_horizon_s750_integration` + docs sync | **✅** |
| 695 | **PH-S760** | locality-metrics HTTP wire depth | Galaxy **§5.2–5.4** | integration test | **✅** |
| 696 | **PH-S761** | hot-tier promote/evict metrics parity | Galaxy **§5.2–5.4** | JSON/Prom parity | **✅** |
| 697 | **PH-S762** | Admin locality wasm glue | poolai-ui-core | ui-core metrics strip | **✅** |
| 698 | **PH-S763** | Stand smoke locality/prefetch band | poolai-http-stand-smoke | runner extend | **✅** |
| 699 | **PH-S764** | `locality_hot_tier_depth_stub` | Galaxy **§5.2–5.4** | unit test | **✅** |
| 700 | **PH-S765** | Rust ratio loc-audit refresh | §5.13 fallback | `poolai-loc-audit` → `rust_ratio.json` PH-S765 zriz | **✅** |
| 701 | **PH-S766** | docs canon INDEX §7 | docs canon | sync | **✅** |
| 702 | **PH-S767** | poolai-vision-sync drift gate | ops | `poolai-vision-sync --check` green | **✅** |
| 703 | **PH-S768** | Ratio hold advisory snapshot | PH-S351 pattern | `--min-ratio 0.95 --advisory` **94.63%** | **✅** |
| 704 | **PH-S769** | Horizon close band S760–S768 | Galaxy **§5.2–5.4** locality | `galaxy_horizon_s760_integration` + docs sync | **✅** |
| 705 | **PH-S770** | offline payout batch settlement wire | Galaxy **§8.2** | cleared → batch queue stub + metric | **✅** |
| 706 | **PH-S771** | payout-batch history admin wasm panel | poolai-ui-core | ui-core render + fetch | **✅** |
| 707 | **PH-S772** | Stand smoke payout-batch/history | poolai-http-stand-smoke | runner green | **✅** |
| 708 | **PH-S773** | `settlement_payout_depth_stub` | Galaxy **§8.2** | unit test | **✅** |
| 709 | **PH-S774** | settlement mode on-chain vs offline gate | galaxy_settlement_mode | test extend | **✅** |
| 710 | **PH-S775** | Rust ratio loc-audit refresh | §5.13 fallback | `poolai-loc-audit` → `rust_ratio.json` PH-S775 zriz | **✅** |
| 711 | **PH-S776** | Galaxy §8.2 payout row ✅ | docs canon | docs canon | **✅** |
| 712 | **PH-S777** | poolai-vision-sync drift gate | ops | `poolai-vision-sync --check` green | **✅** |
| 713 | **PH-S778** | Ratio hold advisory snapshot | PH-S351 pattern | `--min-ratio 0.95 --advisory` **94.65%** | **✅** |
| 714 | **PH-S779** | Horizon close band S770–S778 | Galaxy **§8.2** payout | `galaxy_horizon_s770_integration` + docs sync | **✅** |
| 715 | **PH-S780** | fee split applied metric parity | Galaxy **§1.2** | JSON/Prom parity | **✅** |
| 716 | **PH-S781** | fee hint admin read-only strip | poolai-ui-core | ui-core or grid-pricing extend | **✅** |
| 717 | **PH-S782** | Stand smoke fee-split metrics | poolai-http-stand-smoke | unit test | **✅** |
| 718 | **PH-S783** | `galaxy_fee_split_depth_stub` | Galaxy **§1.2** | unit test | **✅** |
| 719 | **PH-S784** | BENCHMARKS fee-split bench pointer | docs canon | docs/BENCHMARKS pointer | **✅** |
| 720 | **PH-S785** | Rust ratio loc-audit refresh | §5.13 fallback | `poolai-loc-audit` → `rust_ratio.json` PH-S785 zriz | **✅** |
| 721 | **PH-S786** | concept §1.2 implemented | docs canon | docs canon | **✅** |
| 722 | **PH-S787** | poolai-vision-sync drift gate | ops | `poolai-vision-sync --check` green | **✅** |
| 723 | **PH-S788** | Ratio hold advisory snapshot | PH-S351 pattern | `--min-ratio 0.95 --advisory` | **✅** |
| 724 | **PH-S789** | Horizon close band S780–S788 | Galaxy **§1.2** fee | `galaxy_horizon_s780_integration` + docs sync | **✅** |
| 725 | **PH-S790** | update policy env stub wire | Galaxy **§9.5** | `galaxy_update_policy` HTTP read + test | **✅** |
| 726 | **PH-S791** | security advisory metric/export shape | Galaxy **§9.6** | stand smoke or unit test | **✅** |
| 727 | **PH-S792** | admin updates-compat governance extend | poolai-ui-core | wasm panel | **✅** |
| 728 | **PH-S793** | Stand smoke governance metrics | poolai-http-stand-smoke | runner | **✅** |
| 729 | **PH-S794** | `governance_depth_stub` | Galaxy governance | unit test | **✅** |
| 730 | **PH-S795** | Rust ratio loc-audit refresh | §5.13 fallback | `poolai-loc-audit` → `rust_ratio.json` PH-S795 zriz | **✅** |
| 731 | **PH-S796** | SECURITY_HARDENING hub sync | docs canon | docs canon | **✅** |
| 732 | **PH-S797** | poolai-vision-sync drift gate | ops | `poolai-vision-sync --check` green | **✅** |
| 733 | **PH-S798** | Ratio hold advisory snapshot | PH-S351 pattern | `--min-ratio 0.95 --advisory` | **✅** |
| 734 | **PH-S799** | Horizon close band S790–S798 | Galaxy governance | `galaxy_horizon_s790_integration` + docs sync | **✅** |
| 735 | **PH-S800** | wasm slim monitoring ML panel | poolai-ui-wasm | `poolaiRenderMlPipelineMetricsPanel` wasm-only | **✅** |
| 736 | **PH-S801** | wasm slim payout-batch panel | poolai-ui-core | ui-core → wasm export | **✅** |
| 737 | **PH-S802** | admin/mod.rs regression PH-S800/S801 | admin tests | `parsePrometheusGauge` tests | **✅** |
| 738 | **PH-S803** | stand smoke monitoring/payout APIs | poolai-http-stand-smoke | runner shape tests | **✅** |
| 739 | **PH-S804** | admin wasm slim depth stub extend | wasm slim | unit test | **✅** |
| 740 | **PH-S805** | Rust ratio loc-audit refresh | §5.13 fallback | `poolai-loc-audit` → `rust_ratio.json` PH-S805 zriz | **✅** |
| 741 | **PH-S806** | docs canon sync | HANDOFF/NEXT/STABLE | docs canon | **✅** |
| 742 | **PH-S807** | poolai-vision-sync drift gate | ops | `poolai-vision-sync --check` green | **✅** |
| 743 | **PH-S808** | Ratio hold advisory snapshot | PH-S351 pattern | `--min-ratio 0.95 --advisory` | **✅** |
| 744 | **PH-S809** | Horizon close band S800–S808 | wasm monitoring | `galaxy_horizon_s800_integration` + docs sync | **✅** |
| 745 | **PH-S810** | wasm slim security panel glue | poolai-ui-wasm | secret rotation strip wasm | **✅** |
| 746 | **PH-S811** | wasm slim topology panel glue | poolai-ui-core | topology timestamp wasm | **✅** |
| 747 | **PH-S812** | admin/mod.rs regression PH-S810/S811 | admin tests | wasm glue tests | **✅** |
| 748 | **PH-S813** | stand smoke security/topology APIs | poolai-http-stand-smoke | export shape if applicable | **✅** |
| 749 | **PH-S814** | concept stub security/topology | wasm slim | unit test | **✅** |
| 750 | **PH-S815** | Rust ratio loc-audit refresh | §5.13 fallback | `poolai-loc-audit` → `rust_ratio.json` PH-S815 zriz | **✅** |
| 751 | **PH-S816** | docs canon sync | HANDOFF/NEXT/STABLE | docs canon | **✅** |
| 752 | **PH-S817** | poolai-vision-sync drift gate | ops | `poolai-vision-sync --check` green | **✅** |
| 753 | **PH-S818** | Ratio hold advisory snapshot | PH-S351 pattern | `--min-ratio 0.95 --advisory` | **✅** |
| 754 | **PH-S819** | Horizon close band S810–S818 | wasm security/topology | `galaxy_horizon_s810_integration` + docs sync | **✅** |
| 755 | **PH-S820** | wasm slim vm panel glue | poolai-ui-wasm | vm admin wasm render | **✅** |
| 756 | **PH-S821** | wasm slim workers/libs panels | poolai-ui-core | ui-core → wasm | **✅** |
| 757 | **PH-S822** | admin/mod.rs regression PH-S820/S821 | admin tests | wasm glue tests | **✅** |
| 758 | **PH-S823** | stand smoke vm/workers API shape | poolai-http-stand-smoke | runner tests | **✅** |
| 759 | **PH-S824** | concept stub vm/workers DTO | Galaxy §2.3 | unit test | **✅** |
| 760 | **PH-S825** | Rust ratio loc-audit refresh | §5.13 fallback | `poolai-loc-audit` → `rust_ratio.json` PH-S825 zriz | **✅** |
| 761 | **PH-S826** | docs canon sync | HANDOFF/NEXT/STABLE | docs canon | **✅** |
| 762 | **PH-S827** | poolai-vision-sync drift gate | ops | `poolai-vision-sync --check` green | **✅** |
| 763 | **PH-S828** | Ratio hold advisory snapshot | PH-S351 pattern | `--min-ratio 0.95 --advisory` | **✅** |
| 764 | **PH-S829** | Horizon close band S820–S828 | wasm vm/workers | `galaxy_horizon_s820_integration` + docs sync | **✅** |
| 765 | **PH-S830** | stand_smoke_metrics_parity all 6 APIs | stand smoke v2 | validate_band6 extend v2 | **✅** |
| 766 | **PH-S831** | stand smoke prefetch/locality parity | JSON↔Prom | unit tests | **✅** |
| 767 | **PH-S832** | stand smoke governance/fee parity | grid metrics | unit tests | **✅** |
| 768 | **PH-S833** | live runner grid_metrics_json_prometheus_parity | stand smoke | stand smoke case green | **✅** |
| 769 | **PH-S834** | stand smoke export shape regression suite | poolai-http-stand-smoke | bin unit tests | **✅** |
| 770 | **PH-S835** | Rust ratio loc-audit refresh | §5.13 fallback | `poolai-loc-audit` → `rust_ratio.json` PH-S835 zriz | **✅** |
| 771 | **PH-S836** | PROMETHEUS_METRICS.md stand smoke sync | docs | docs | **✅** |
| 772 | **PH-S837** | poolai-vision-sync drift gate | ops | `poolai-vision-sync --check` green | **✅** |
| 773 | **PH-S838** | Ratio hold advisory snapshot | PH-S351 pattern | `--min-ratio 0.95 --advisory` | **✅** |
| 774 | **PH-S839** | Horizon close band S830–S838 | stand smoke v2 | `galaxy_horizon_s830_integration` + docs sync | **✅** |
| 775 | **PH-S840** | openapi.yaml sync band APIs | OpenAPI gap | routes match grid.rs | **✅** |
| 776 | **PH-S841** | poolai-openapi-gap-audit 0 | CI gate | gap-audit 0 | **✅** |
| 777 | **PH-S842** | contract test band top routes | tests/*_contracts.rs | extend | **✅** |
| 778 | **PH-S843** | stand smoke OpenAPI path smoke | poolai-http-stand-smoke | key paths 200 shape | **✅** |
| 779 | **PH-S844** | OpenAPI examples for grid metrics | docs/openapi.yaml | yaml examples | **✅** |
| 780 | **PH-S845** | Rust ratio loc-audit refresh | §5.13 fallback | `poolai-loc-audit` → `rust_ratio.json` PH-S845 zriz | **✅** |
| 781 | **PH-S846** | OPENAPI_GAP_AUDIT doc sync | docs | docs canon | **✅** |
| 782 | **PH-S847** | poolai-vision-sync drift gate | ops | `poolai-vision-sync --check` green | **✅** |
| 783 | **PH-S848** | Ratio hold advisory snapshot | PH-S351 pattern | `--min-ratio 0.95 --advisory` | **✅** |
| 784 | **PH-S849** | Horizon close band S840–S848 | OpenAPI gap | `galaxy_horizon_s840_integration` + docs sync | **✅** |
| 785 | **PH-S850** | job store RAID restart persistence | Job store RAID | integration test like PH-S52 | **✅** |
| 786 | **PH-S851** | verify-dev-stand RAID jobs path | ops | bin script green | **✅** |
| 787 | **PH-S852** | admin jobs store_backend badge wire | UI wasm glue | admin jobs panel | **✅** |
| 788 | **PH-S853** | stand smoke jobs store_backend | tests | runner case | **✅** |
| 789 | **PH-S854** | job store depth stub | code | unit test | **✅** |
| 790 | **PH-S855** | poolai-loc-audit PH-S855 | §5.13 | rust_ratio.json zriz | **✅** |
| 791 | **PH-S856** | RUN_LOCAL.md RAID jobs preset | docs | docs | **✅** |
| 792 | **PH-S857** | poolai-vision-sync drift gate | ops | `--check` green | **✅** |
| 793 | **PH-S858** | Ratio hold advisory | ops | `--min-ratio 0.95 --advisory` | **✅** |
| 794 | **PH-S859** | Horizon close band S850–S858 | Job RAID | `galaxy_horizon_s850_integration` | **✅** |
| 795 | **PH-S860** | memory shard persist stub | Memory shard persist | MemoryShardStore persist + test | **✅** |
| 796 | **PH-S861** | seed-inventory HTTP depth | Galaxy §5.5 | GET /grid/seed-inventory extend | **✅** |
| 797 | **PH-S862** | admin memory/seed wasm glue | UI wasm glue | ui-core helper | **✅** |
| 798 | **PH-S863** | stand smoke seed-inventory API | tests | runner | **✅** |
| 799 | **PH-S864** | memory layer depth stub | code | POOLAI_MEMORY_LAYER unit test | **✅** |
| 800 | **PH-S865** | poolai-loc-audit PH-S865 | §5.13 | rust_ratio.json zriz | **✅** |
| 801 | **PH-S866** | POOLAI_MEMORY_LAYER.md sync | docs | docs ✅ | **✅** |
| 802 | **PH-S867** | poolai-vision-sync drift gate | ops | `--check` green | **✅** |
| 803 | **PH-S868** | Ratio hold advisory | ops | `--min-ratio 0.95 --advisory` | **✅** |
| 804 | **PH-S869** | Horizon close band S860–S868 | Memory shard | `galaxy_horizon_s860_integration` | **✅** |
| 805 | **PH-S870** | on-chain cleared mock RPC depth | Solana §8 | POOLAI_SETTLEMENT_ON_CHAIN test | **✅** |
| 806 | **PH-S871** | solana-adapter event schema v1 | crate | crate test | **✅** |
| 807 | **PH-S872** | job onchain events NDJSON persist | domain_events | domain_events test | **✅** |
| 808 | **PH-S873** | stand smoke on-chain metrics if exposed | tests | runner | **✅** |
| 809 | **PH-S874** | solana depth stub | concept | unit test | **✅** |
| 810 | **PH-S875** | poolai-loc-audit PH-S875 | §5.13 | rust_ratio.json zriz | **✅** |
| 811 | **PH-S876** | SOLANA_ADAPTER_CONCEPT sync | docs | docs ✅ | **✅** |
| 812 | **PH-S877** | poolai-vision-sync --check | ops | `--check` green | **✅** |
| 813 | **PH-S878** | Ratio hold advisory | ops | `--min-ratio 0.95 --advisory` | **✅** |
| 814 | **PH-S879** | Horizon close band S870–S878 | Solana | `galaxy_horizon_s870_integration` | **✅** |
| 815 | **PH-S880** | checker task drain lifecycle | Galaxy §6.2 | PH-S495 extend integration | **✅** |
| 816 | **PH-S881** | checker shadow job submit depth | Galaxy §6.2 | integration test | **✅** |
| 817 | **PH-S882** | admin grid-verification wasm complete | ui | metrics+tasks strip | **✅** |
| 818 | **PH-S883** | stand smoke verification-checker/tasks | tests | runner | **✅** |
| 819 | **PH-S884** | verification lifecycle depth stub | concept | unit test | **✅** |
| 820 | **PH-S885** | poolai-loc-audit PH-S885 | §5.13 | rust_ratio.json zriz | **✅** |
| 821 | **PH-S886** | Galaxy §6.2 implemented table | docs | docs ✅ | **✅** |
| 822 | **PH-S887** | poolai-vision-sync --check | ops | `--check` green | **✅** |
| 823 | **PH-S888** | Ratio hold advisory | ops | `--min-ratio 0.95 --advisory` | **✅** |
| 824 | **PH-S889** | Horizon close band S880–S888 | Verification | `galaxy_horizon_s880_integration` | **✅** |
| 825 | **PH-S890** | replication quorum gate production | Galaxy §6.4 | strict tier integration | **✅** |
| 826 | **PH-S891** | replication rate cap HTTP wire | Galaxy §6.4 | integration test | **✅** |
| 827 | **PH-S892** | admin replication-pricing wasm polish | ui-core | ui-core regression | **✅** |
| 828 | **PH-S893** | stand smoke replication metrics parity | tests | JSON↔Prom | **✅** |
| 829 | **PH-S894** | replication depth stub | concept | unit test | **✅** |
| 830 | **PH-S895** | poolai-loc-audit PH-S895 | §5.13 | rust_ratio.json zriz | **✅** |
| 831 | **PH-S896** | Galaxy §6.4 implemented | docs | docs ✅ | **✅** |
| 832 | **PH-S897** | poolai-vision-sync --check | ops | `--check` green | **✅** |
| 833 | **PH-S898** | Ratio hold advisory | ops | `--min-ratio 0.95 --advisory` | **✅** |
| 834 | **PH-S899** | Horizon close band S890–S898 | Replication | `galaxy_horizon_s890_integration` | **✅** |
| 835 | **PH-S900** | pricing live provider timeout hardening | Galaxy §4.2 | oracle unit + integration | **✅** |
| 836 | **PH-S901** | pricing forced-fallback stand smoke | PH-S123 pattern | stand smoke | **✅** |
| 837 | **PH-S902** | admin grid-pricing wasm polish | ui-core | freshness metadata display | **✅** |
| 838 | **PH-S903** | stand smoke pricing-metrics parity | tests | JSON↔Prom | **✅** |
| 839 | **PH-S904** | pricing depth stub | concept | unit test | **✅** |
| 840 | **PH-S905** | poolai-loc-audit PH-S905 | §5.13 | rust_ratio.json zriz | **✅** |
| 841 | **PH-S906** | Galaxy §4.2 live fetch ✅ docs | docs | docs canon | **✅** |
| 842 | **PH-S907** | poolai-vision-sync --check | ops | `--check` green | **✅** |
| 843 | **PH-S908** | Ratio hold advisory | ops | `--min-ratio 0.95 --advisory` | **✅** |
| 844 | **PH-S909** | Horizon close band S900–S908 | Pricing | `galaxy_horizon_s900_integration` | **✅** |
| 845 | **PH-S910** | trust score SQLite persist | Galaxy §6.5 | galaxy_trust_score_store wire | **✅** |
| 846 | **PH-S911** | trust payout gate integration | Galaxy §6.5 | low trust → held metric | **✅** |
| 847 | **PH-S912** | admin trust metrics wasm strip | ui-core | trust metrics strip | **✅** |
| 848 | **PH-S913** | stand smoke trust-metrics parity | tests | JSON↔Prom | **✅** |
| 849 | **PH-S914** | trust persist depth stub | concept | unit test | **✅** |
| 850 | **PH-S915** | poolai-loc-audit PH-S915 | §5.13 | rust_ratio.json zriz | **✅** |
| 851 | **PH-S916** | Galaxy §6.5 trust persist ✅ docs | docs | docs canon | **✅** |
| 852 | **PH-S917** | poolai-vision-sync --check | ops | `--check` green | **✅** |
| 853 | **PH-S918** | Ratio hold advisory | ops | `--min-ratio 0.95 --advisory` | **✅** |
| 854 | **PH-S919** | Horizon close band S910–S918 | Trust | `galaxy_horizon_s910_integration` | **✅** |
| 855 | **PH-S920** | admin_charts ML sparkline → wasm | ui-core | `render_sparkline_html` wasm-only | **✅** |
| 856 | **PH-S921** | admin_charts line chart → wasm | ui-core | `render_line_chart_html` wasm-only | **✅** |
| 857 | **PH-S922** | admin_charts regression tests | admin | mod.rs PH-S920/S921 | **✅** |
| 858 | **PH-S923** | build-ui-wasm.sh gate in drain doc | ops | bin verify | **✅** |
| 859 | **PH-S924** | charts depth stub | concept | unit test | **✅** |
| 860 | **PH-S925** | poolai-loc-audit PH-S925 | §5.13 | rust_ratio.json zriz | **✅** |
| 861 | **PH-S926** | RUST_RATIO §5.13 charts row | docs | docs canon | **✅** |
| 862 | **PH-S927** | poolai-vision-sync --check | ops | `--check` green | **✅** |
| 863 | **PH-S928** | Ratio hold advisory | ops | `--min-ratio 0.95 --advisory` | **✅** |
| 864 | **PH-S929** | Horizon close band S920–S928 | Charts | `galaxy_horizon_s920_integration` | **✅** |
| 865 | **PH-S930** | admin_common.js table init slim | ui-core | delegate to ui-core where possible | **✅** |
| 866 | **PH-S931** | admin_common.js empty state slim | ui-core | wasm/html from ui-core | **✅** |
| 867 | **PH-S932** | i18n_core.js audit — no duplicate logic | ui-core | rg audit + fix | **✅** |
| 868 | **PH-S933** | ratio 95% gate test | §5.13 | rust_ratio ≥ 0.95 or advisory documented | **✅** |
| 869 | **PH-S934** | ui JS loc reduction stub metric | ops | loc-audit by_category ui_js down | **✅** |
| 870 | **PH-S935** | poolai-loc-audit PH-S935 | §5.13 | rust_ratio.json zriz | **✅** |
| 871 | **PH-S936** | RUST_RATIO_STRATEGY band 28 note | docs | docs canon | **✅** |
| 872 | **PH-S937** | poolai-vision-sync --check | ops | `--check` green | **✅** |
| 873 | **PH-S938** | Ratio hold advisory | ops | `--min-ratio 0.95` meets or hold | **✅** |
| 874 | **PH-S939** | galaxy_horizon_s930_integration | Ratio | ratio 95% band close | **✅** |
| 875 | **PH-S940** | e2e scope audit — API-only removed | e2e | no duplicate Rust tests | **✅** |
| 876 | **PH-S941** | e2e TS loc reduction plan executed | e2e | shrink legacy API specs | **✅** |
| 877 | **PH-S942** | ratio 96% stretch spirit check | §5.13 | loc-audit stretch flag | **✅** |
| 878 | **PH-S943** | ops shell audit — no product logic | ops | bin/ vs scripts/ canon | **✅** |
| 879 | **PH-S944** | stretch depth stub | concept | unit test | **✅** |
| 880 | **PH-S945** | poolai-loc-audit PH-S945 | §5.13 | rust_ratio.json zriz | **✅** |
| 881 | **PH-S946** | RUST_RATIO 96% spirit docs | docs | docs canon | **✅** |
| 882 | **PH-S947** | poolai-vision-sync --check | ops | `--check` green | **✅** |
| 883 | **PH-S948** | Ratio hold advisory | ops | stretch note | **✅** |
| 884 | **PH-S949** | galaxy_horizon_s940_integration | Ratio | ratio stretch close | **✅** |
| 885 | **PH-S950** | FUNCTIONALITY_DIGEST grid section sync | docs | all src/grid modules listed | **✅** |
| 886 | **PH-S951** | FUNCTIONALITY_DIGEST job/lease sync | docs | src/job rows | **✅** |
| 887 | **PH-S952** | FUNCTIONALITY_DIGEST ui/wasm sync | docs | crates rows | **✅** |
| 888 | **PH-S953** | FUNCTIONALITY_DIGEST bins table | docs | src/bin/ all listed | **✅** |
| 889 | **PH-S954** | DIGEST OpenAPI pointer refresh | docs | gap audit note | **✅** |
| 890 | **PH-S955** | poolai-loc-audit PH-S955 | §5.13 | rust_ratio.json zriz | **✅** |
| 891 | **PH-S956** | file_list.csv catalog sync | docs | key paths | **✅** |
| 892 | **PH-S957** | poolai-vision-sync --check | ops | `--check` green | **✅** |
| 893 | **PH-S958** | Ratio hold advisory | ops | hold | **✅** |
| 894 | **PH-S959** | galaxy_horizon_s950_integration | docs | digest band close | **✅** |
| 895 | **PH-S960** | DOCS_LEGACY_AUDIT remaining rows triage | docs | table update | **✅** |
| 896 | **PH-S961** | stale banners on flat docs/*.md | docs | pointer to INDEX/archive | **✅** |
| 897 | **PH-S962** | concept root de-hype pass | docs | poolAI_concept_root.txt zriz | **✅** |
| 898 | **PH-S963** | ARCHITECT vs FM §5.1 alignment | docs | NEXT_STEPS_ARCHITECT sync | **✅** |
| 899 | **PH-S964** | docs archive pointer batch | docs | DOCS_LEGACY §5.3 | **✅** |
| 900 | **PH-S965** | poolai-loc-audit PH-S965 | §5.13 | rust_ratio.json zriz | **✅** |
| 901 | **PH-S966** | INDEX step 12 FM pointer | docs | docs | **✅** |
| 902 | **PH-S967** | poolai-vision-sync --check | ops | `--check` green | **✅** |
| 903 | **PH-S968** | Ratio hold advisory | ops | hold | **✅** |
| 904 | **PH-S969** | galaxy_horizon_s960_integration | docs | DOCS_LEGACY close | **✅** |
| 905 | **PH-S970** | Galaxy §1–3 implemented markers | docs | POOLAI_GALAXY_GRID.md | **✅** |
| 906 | **PH-S971** | Galaxy §4–6 implemented markers | docs | same | **✅** |
| 907 | **PH-S972** | Galaxy §7–9 implemented markers | docs | same | **✅** |
| 908 | **PH-S973** | §8 TBD closed or BLOCKED noted | docs | §8.2 payout ✅; LAN blocked | **✅** |
| 909 | **PH-S974** | GALAXY_GRID_ROADMAP horizon table final | docs | all rows ✅ or BLOCKED | **✅** |
| 910 | **PH-S975** | poolai-loc-audit PH-S975 | §5.13 | rust_ratio.json zriz | **✅** |
| 911 | **PH-S976** | concept cross-links INDEX | docs | docs | **✅** |
| 912 | **PH-S977** | poolai-vision-sync --check | ops | `--check` green | **✅** |
| 913 | **PH-S978** | Ratio hold advisory | ops | hold | **✅** |
| 914 | **PH-S979** | galaxy_horizon_s970_integration | docs | concept markers close | **✅** |
| 915 | **PH-S980** | STABLE_STATE product-complete draft | docs | development complete section | **✅** |
| 916 | **PH-S981** | INDEX product-complete zriz | docs | step 1–12 final | **✅** |
| 917 | **PH-S982** | README Next Focus → maintenance | docs | root README | **✅** |
| 918 | **PH-S983** | HANDOFF maintenance mode template | docs | post-S1010 prep | **✅** |
| 919 | **PH-S984** | DEVELOPMENT_PROGRESS 100% code scope | docs | honest scope note | **✅** |
| 920 | **PH-S985** | poolai-loc-audit PH-S985 | §5.13 | rust_ratio.json zriz | **✅** |
| 921 | **PH-S986** | FM §5.15 draft product-complete | docs | FM catalog | **✅** |
| 922 | **PH-S987** | poolai-vision-sync --check | ops | `--check` green | **✅** |
| 923 | **PH-S988** | Ratio hold advisory | ops | final hold | **✅** |
| 924 | **PH-S989** | galaxy_horizon_s980_integration | docs | STABLE band close | **✅** |
| 925 | **PH-S990** | integration gap: telegram wallet | tests | tests/* if missing | **✅** |
| 926 | **PH-S991** | integration gap: grid job lease | tests | extend if gap | **✅** |
| 927 | **PH-S992** | integration gap: protocol middleware | tests | extend if gap | **✅** |
| 928 | **PH-S993** | integration gap: jobs raid restart | tests | extend if gap | **✅** |
| 929 | **PH-S994** | integration gap: vm write lifecycle | tests | extend if gap | **✅** |
| 930 | **PH-S995** | poolai-loc-audit PH-S995 | §5.13 | rust_ratio.json zriz | **✅** |
| 931 | **PH-S996** | poolai-testing-policy gap note | docs | docs | **✅** |
| 932 | **PH-S997** | poolai-vision-sync --check | ops | `--check` green | **✅** |
| 933 | **PH-S998** | Ratio hold advisory | ops | hold | **✅** |
| 934 | **PH-S999** | galaxy_horizon_s990_integration | tests | integration gap close | **✅** |
| 935 | **PH-S1000** | multi-module wire smoke harness | tests | top 5 grid APIs one test | **✅** |
| 936 | **PH-S1001** | multi-module admin wasm regression | tests | ui-core full test gate | **✅** |
| 937 | **PH-S1002** | multi-module stand smoke full suite | ops | bin --json all green | **✅** |
| 938 | **PH-S1003** | cargo test-ci scope note final | docs | HANDOFF | **✅** |
| 939 | **PH-S1004** | openapi-gap + test-ci dual gate doc | docs | FM | **✅** |
| 940 | **PH-S1005** | poolai-loc-audit PH-S1005 | §5.13 | rust_ratio.json zriz | **✅** |
| 941 | **PH-S1006** | vision manifest final sprint_queue | ops | poolai-vision-sync | **✅** |
| 942 | **PH-S1007** | poolai-vision-sync --check | ops | `--check` green | **✅** |
| 943 | **PH-S1008** | Ratio hold advisory | ops | final pre-S1010 | **✅** |
| 944 | **PH-S1009** | galaxy_horizon_s1000_integration | tests | final code band close | **✅** |
| 945 | **PH-S1010** | FM §5.15 product-complete closure | docs | STABLE maintenance mode | **✅** |
| 946 | **PH-S1011** | Light compile profile | ops `RUN_LOCAL.md` | `--light` minimal-features build | **✅** |
| 947 | **PH-S1012** | Light full-stack launch preset | `bin/run-poolai.*` | preset `quick` + health wait | **✅** |
| 948 | **PH-S1013** | Vision easy launch | README `open-docs-vision` | PS1 + MSYS2 shim localhost URL | **✅** |
| 949 | **PH-S1014** | Runtime state snapshot persist | `data/dev/last_run.json` | save on stop; restore on quick; unit test | **✅** |
| 950 | **PH-S1015** | PoolAI admin power panel UI | `src/ui/admin` | toolbar modal Виключити/Перезавантажити | **✅** |
| 951 | **PH-S1016** | PoolAI power ops wire | `POST /api/v1/ops/power` | shutdown/reboot dev-safe; integration test | **✅** |
| 952 | **PH-S1017** | Vision poweroff/reset controls | `docs/vision/index.html` | power menu; localStorage; soft/hard reload | **✅** |
| 953 | **PH-S1018** | Ops power band close | tests/docs band 37 | `galaxy_horizon_s1011_integration`; RUN_LOCAL sync | **✅** |

**Відкритих у §5.12:** **10** (band 73 ✅ · band 74 open). **Master horizon:** PH-S1379…S1388 (band 74). **Completion pending:** PH-S1379…S2278 = **900** · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](../development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md). Vision rev **375**. **Наступна сесія:** **`абракадабра`** — drain band 74.

### 5.55 Audit admin/ops glue queue — band 74 (PH-S1379…S1388, 2026-07-23) · **ACTIVE**

**Джерело:** project completion / enterprise phase C — Audit admin UI + ops glue (store strip / query refresh / verify). Mirror band 64 [`SSO_ADMIN_OPS.md`](../development/SSO_ADMIN_OPS.md). Plan: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](../development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md).

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1314 | **PH-S1379** | `audit_admin_ops_depth` ui-core module | `audit_admin_ops_depth.rs` | depth enum + admin/ops criteria registry | **[ ]** |
| 1315 | **PH-S1380** | Admin audit store-wire status strip | `src/ui/admin` audit | `#audit-store-badge` ← `GET /audit/store` | **[ ]** |
| 1316 | **PH-S1381** | Admin audit query ops glue | same | refresh events from HTTP query contracts | **[ ]** |
| 1317 | **PH-S1382** | Admin audit ops HTML contracts | `audit_admin_ops_integration.rs` | store/query markers | **[ ]** |
| 1318 | **PH-S1383** | i18n Audit admin ops keys | `i18n.rs` ADMIN_AUDIT_* | EN/UK patch keys | **[ ]** |
| 1319 | **PH-S1384** | `VERIFY_AUDIT_ADMIN_OPS` + quick `--audit-admin-ops` | `bin/verify-dev-stand.sh` | gate + RUN_LOCAL | **[ ]** |
| 1320 | **PH-S1385** | Stand smoke + `poolai-loc-audit --audit-admin-ops` | stand smoke / loc-audit | export shape + `rust_ratio.json` fields | **[ ]** |
| 1321 | **PH-S1386** | Docs `AUDIT_ADMIN_OPS.md` + canon sync | RUN_LOCAL/INDEX/HANDOFF/NEXT | ops matrix + backlog override | **[ ]** |
| 1322 | **PH-S1387** | Ratio hold advisory | loc-audit | `--min-ratio 0.95 --advisory` | **[ ]** |
| 1323 | **PH-S1388** | Audit admin/ops band close | tests/docs | `galaxy_horizon_s1379_integration`; HANDOFF/NEXT | **[ ]** |

### 5.54 Audit API contracts queue — band 73 (PH-S1369…S1378, 2026-07-23) · **✅**

**Джерело:** project completion / enterprise phase C — Audit API contracts (mirror band 63 [`SSO_API.md`](../development/SSO_API.md)). Plan: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](../development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md).

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1304 | **PH-S1369** | `audit_api_contracts_depth` ui-core module | `audit_api_contracts_depth.rs` | depth enum + HTTP API criteria registry | **✅** |
| 1305 | **PH-S1370** | Audit query HTTP lifecycle | `audit_api_contracts_integration` | GET query filters + pagination stub | **✅** |
| 1306 | **PH-S1371** | Store-wire status HTTP read | `GET /audit/store` | `{mode,durable_path,configured}` file/sqlite | **✅** |
| 1307 | **PH-S1372** | OpenAPI `AuditStoreWire` + errors | `docs/openapi.yaml` | store schema + path; gap-audit 0 | **✅** |
| 1308 | **PH-S1373** | Event field validation fixtures | same suite | missing action/resource → 4xx | **✅** |
| 1309 | **PH-S1374** | `VERIFY_AUDIT_API` + quick `--audit-api` | verify-dev-stand | API gate | **✅** |
| 1310 | **PH-S1375** | Stand smoke + `poolai-loc-audit --audit-api` | stand smoke / loc-audit | export shape + rust_ratio fields | **✅** |
| 1311 | **PH-S1376** | Docs `AUDIT_API.md` + canon | RUN_LOCAL/INDEX/HANDOFF/NEXT | HTTP contract matrix | **✅** |
| 1312 | **PH-S1377** | vision-sync --check + ratio hold | vision / loc-audit | drift + `--min-ratio 0.95 --advisory` | **✅** |
| 1313 | **PH-S1378** | Audit API band close | tests/docs | `galaxy_horizon_s1369_integration`; HANDOFF/NEXT | **✅** |

### 5.53 Audit store wire queue — band 72 (PH-S1359…S1368, 2026-07-23) · **✅**

**Джерело:** project completion / enterprise phase C — Audit store wire (mirror band 62 [`SSO_STORE.md`](../development/SSO_STORE.md)). Plan: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](../development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md).

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1294 | **PH-S1359** | `audit_store_depth` ui-core module | `audit_store_depth.rs` | depth enum + store criteria registry | **✅** |
| 1295 | **PH-S1360** | Audit store wire durable path | `audit.rs` | `POOLAI_AUDIT_DATA_DIR` + `audit_store_wire()` + unit tests | **✅** |
| 1296 | **PH-S1361** | Audit store wire contracts | `audit_store_integration` | wire labels + ui-core depth stub | **✅** |
| 1297 | **PH-S1362** | `VERIFY_AUDIT_STORE` + quick `--audit-store` | verify-dev-stand | store gate | **✅** |
| 1298 | **PH-S1363** | Stand smoke export shape band 72 | stand smoke | `audit_store_band72_export_shape` | **✅** |
| 1299 | **PH-S1364** | `poolai-loc-audit --audit-store` | loc-audit | `rust_ratio.json` fields | **✅** |
| 1300 | **PH-S1365** | Docs `AUDIT_STORE.md` + canon | RUN_LOCAL/INDEX/HANDOFF/NEXT | docs matrix | **✅** |
| 1301 | **PH-S1366** | vision-sync --check | vision | drift gate green | **✅** |
| 1302 | **PH-S1367** | Ratio hold advisory | loc-audit | `--min-ratio 0.95 --advisory` | **✅** |
| 1303 | **PH-S1368** | Audit store band close | tests/docs | `galaxy_horizon_s1359_integration`; HANDOFF/NEXT | **✅** |

### 5.52 Audit depth scaffold queue — band 71 (PH-S1349…S1358, 2026-07-23) · **✅**

**Джерело:** project completion / enterprise phase C — Audit depth scaffold (`POOLAI_AUDIT_STORE` + event field stub; mirror band 61 [`SSO_DEPTH.md`](../development/SSO_DEPTH.md)). Plan: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](../development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md).

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1284 | **PH-S1349** | `audit_depth` ui-core module | `audit_depth.rs` | depth enum + criteria registry | **✅** |
| 1285 | **PH-S1350** | Audit store/wire slice stub | `audit.rs` | `POOLAI_AUDIT_STORE` + `validate_audit_event_fields` unit tests | **✅** |
| 1286 | **PH-S1351** | Criteria contracts | `audit_depth_integration` | markers + registry | **✅** |
| 1287 | **PH-S1352** | `VERIFY_AUDIT` + quick `--audit` | verify-dev-stand | depth gate | **✅** |
| 1288 | **PH-S1353** | Stand smoke export shape band 71 | stand smoke | `audit_band71_export_shape` | **✅** |
| 1289 | **PH-S1354** | `poolai-loc-audit --audit` | loc-audit | `rust_ratio.json` fields | **✅** |
| 1290 | **PH-S1355** | Docs `AUDIT_DEPTH.md` + canon | RUN_LOCAL/INDEX/HANDOFF/NEXT | docs matrix | **✅** |
| 1291 | **PH-S1356** | vision-sync --check | vision | drift gate green | **✅** |
| 1292 | **PH-S1357** | Ratio hold advisory | loc-audit | `--min-ratio 0.95 --advisory` | **✅** |
| 1293 | **PH-S1358** | Audit depth band close | tests/docs | `galaxy_horizon_s1349_integration`; HANDOFF/NEXT | **✅** |

### 5.51 SSO horizon close queue — band 70 (PH-S1339…S1348, 2026-07-23) · **✅**

**Джерело:** project completion / enterprise phase B — SSO horizon close (mirror band 60 [`TENANT_HORIZON.md`](../development/TENANT_HORIZON.md)). Plan: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](../development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md).

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1274 | **PH-S1339** | `sso_horizon_depth` ui-core module | `sso_horizon_depth.rs` | depth enum + criteria registry | **✅** |
| 1275 | **PH-S1340** | Horizon slice aggregate stub | `SSO_HORIZON_SLICES` | prior `--sso*` + ratio-advisory slices; unit test | **✅** |
| 1276 | **PH-S1341** | Criteria contracts | `sso_horizon_integration` | markers + registry | **✅** |
| 1277 | **PH-S1342** | `VERIFY_SSO_HORIZON` + quick `--sso-horizon` | verify-dev-stand | horizon gate | **✅** |
| 1278 | **PH-S1343** | Stand smoke export shape band 70 | stand smoke | `sso_horizon_band70_export_shape` | **✅** |
| 1279 | **PH-S1344** | `poolai-loc-audit --sso-horizon` | loc-audit | `rust_ratio.json` fields | **✅** |
| 1280 | **PH-S1345** | Docs `SSO_HORIZON.md` + canon | RUN_LOCAL/INDEX/HANDOFF/NEXT | docs matrix | **✅** |
| 1281 | **PH-S1346** | vision-sync --check | vision | drift gate green | **✅** |
| 1282 | **PH-S1347** | Ratio hold advisory | loc-audit | `--min-ratio 0.95 --advisory` | **✅** |
| 1283 | **PH-S1348** | SSO horizon band close | tests/docs | `galaxy_horizon_s1339_integration`; HANDOFF/NEXT | **✅** |

### 5.50 SSO ratio advisory queue — band 69 (PH-S1329…S1338, 2026-07-23) · **✅**

**Джерело:** project completion / enterprise phase B — SSO ratio-advisory (mirror band 59 [`TENANT_RATIO_ADVISORY.md`](../development/TENANT_RATIO_ADVISORY.md)). Plan: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](../development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md).

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1264 | **PH-S1329** | `sso_ratio_advisory_depth` ui-core module | `sso_ratio_advisory_depth.rs` | depth enum + criteria registry | **✅** |
| 1265 | **PH-S1330** | Ratio-advisory slice aggregate stub | `SSO_RATIO_ADVISORY_SLICES` | prior `--sso*` + vision-sync slices; unit test | **✅** |
| 1266 | **PH-S1331** | Criteria contracts | `sso_ratio_advisory_integration` | markers + registry | **✅** |
| 1267 | **PH-S1332** | `VERIFY_SSO_RATIO_ADVISORY` + quick `--sso-ratio-advisory` | verify-dev-stand | ratio-advisory gate | **✅** |
| 1268 | **PH-S1333** | Stand smoke export shape band 69 | stand smoke | `sso_ratio_advisory_band69_export_shape` | **✅** |
| 1269 | **PH-S1334** | `poolai-loc-audit --sso-ratio-advisory` | loc-audit | `rust_ratio.json` fields | **✅** |
| 1270 | **PH-S1335** | Docs `SSO_RATIO_ADVISORY.md` + canon | RUN_LOCAL/INDEX/HANDOFF/NEXT | docs matrix | **✅** |
| 1271 | **PH-S1336** | vision-sync --check | vision | drift gate green | **✅** |
| 1272 | **PH-S1337** | Ratio hold advisory | loc-audit | `--min-ratio 0.95 --advisory` | **✅** |
| 1273 | **PH-S1338** | SSO ratio-advisory band close | tests/docs | `galaxy_horizon_s1329_integration`; HANDOFF/NEXT | **✅** |

### 5.49 SSO vision sync queue — band 68 (PH-S1319…S1328, 2026-07-23) · **✅**

**Джерело:** project completion / enterprise phase B — SSO vision-sync (mirror band 58 [`TENANT_VISION_SYNC.md`](../development/TENANT_VISION_SYNC.md)). Plan: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](../development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md).

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1254 | **PH-S1319** | `sso_vision_sync_depth` ui-core module | `sso_vision_sync_depth.rs` | depth enum + vision/criteria registry | **✅** |
| 1255 | **PH-S1320** | SSO vision-sync slice aggregate stub | `SSO_VISION_SYNC_SLICES` | six vision slices; unit test | **✅** |
| 1256 | **PH-S1321** | Criteria contracts | `sso_vision_sync_integration` | markers + registry | **✅** |
| 1257 | **PH-S1322** | `VERIFY_SSO_VISION_SYNC` + quick `--sso-vision-sync` | verify-dev-stand | vision-sync gate | **✅** |
| 1258 | **PH-S1323** | Stand smoke export shape band 68 | stand smoke | `sso_vision_sync_band68_export_shape` | **✅** |
| 1259 | **PH-S1324** | `poolai-loc-audit --sso-vision-sync` | loc-audit | `rust_ratio.json` fields | **✅** |
| 1260 | **PH-S1325** | Docs `SSO_VISION_SYNC.md` + canon | RUN_LOCAL/INDEX/HANDOFF/NEXT | docs matrix | **✅** |
| 1261 | **PH-S1326** | vision-sync --check | vision | drift gate green | **✅** |
| 1262 | **PH-S1327** | Ratio hold advisory | loc-audit | `--min-ratio 0.95 --advisory` | **✅** |
| 1263 | **PH-S1328** | SSO vision-sync band close | tests/docs | `galaxy_horizon_s1319_integration`; HANDOFF/NEXT | **✅** |

### 5.48 SSO docs canon queue — band 67 (PH-S1309…S1318, 2026-07-23) · **✅**

**Джерело:** project completion / enterprise phase B — SSO docs-canon aggregate (bands 61–66 `SSO_*.md`). Mirror band 57 [`TENANT_DOCS_CANON.md`](../development/TENANT_DOCS_CANON.md). Plan: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](../development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md).

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1244 | **PH-S1309** | `sso_docs_canon_depth` ui-core module | `sso_docs_canon_depth.rs` | depth enum + docs-canon criteria registry | **✅** |
| 1245 | **PH-S1310** | Docs slice aggregate stub | `SSO_DOCS_CANON_SLICES` | six `SSO_*.md` present | **✅** |
| 1246 | **PH-S1311** | Criteria contracts | `sso_docs_canon_integration` | markers + registry | **✅** |
| 1247 | **PH-S1312** | `VERIFY_SSO_DOCS_CANON` + quick `--sso-docs-canon` | verify-dev-stand | docs-canon gate | **✅** |
| 1248 | **PH-S1313** | Stand smoke export shape band 67 | stand smoke | unit export shape | **✅** |
| 1249 | **PH-S1314** | `poolai-loc-audit --sso-docs-canon` | loc-audit | `rust_ratio.json` fields | **✅** |
| 1250 | **PH-S1315** | Docs `SSO_DOCS_CANON.md` + canon | RUN_LOCAL/INDEX/HANDOFF/NEXT | docs matrix | **✅** |
| 1251 | **PH-S1316** | vision-sync --check | vision | drift gate green | **✅** |
| 1252 | **PH-S1317** | Ratio hold advisory | loc-audit | `--min-ratio 0.95 --advisory` | **✅** |
| 1253 | **PH-S1318** | SSO docs-canon band close | tests/docs | `galaxy_horizon_s1309_integration`; HANDOFF/NEXT | **✅** |

### 5.47 SSO loc-audit aggregate queue — band 66 (PH-S1299…S1308, 2026-07-23) · **✅**

**Джерело:** project completion / enterprise phase B — SSO loc-audit aggregate (bands 61–65 slices). Mirror band 56 [`TENANT_LOC_AUDIT.md`](../development/TENANT_LOC_AUDIT.md). Plan: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](../development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md).

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1234 | **PH-S1299** | `sso_loc_audit_depth` ui-core module | `sso_loc_audit_depth.rs` | depth enum + aggregate criteria registry | **✅** |
| 1235 | **PH-S1300** | Slice aggregate stub | `SSO_LOC_AUDIT_SLICES` | `--sso`…`--sso-stand-smoke` | **✅** |
| 1236 | **PH-S1301** | Criteria contracts | `sso_loc_audit_integration` | markers + registry | **✅** |
| 1237 | **PH-S1302** | `VERIFY_SSO_LOC_AUDIT` + quick `--sso-loc-audit` | verify-dev-stand | aggregate gate | **✅** |
| 1238 | **PH-S1303** | Stand smoke export shape band 66 | stand smoke | unit export shape | **✅** |
| 1239 | **PH-S1304** | `poolai-loc-audit --sso-loc-audit` | loc-audit | `rust_ratio.json` fields | **✅** |
| 1240 | **PH-S1305** | Docs `SSO_LOC_AUDIT.md` + canon | RUN_LOCAL/INDEX/HANDOFF/NEXT | aggregate matrix | **✅** |
| 1241 | **PH-S1306** | vision-sync --check | vision | drift gate green | **✅** |
| 1242 | **PH-S1307** | Ratio hold advisory | loc-audit | `--min-ratio 0.95 --advisory` | **✅** |
| 1243 | **PH-S1308** | SSO loc-audit band close | tests/docs | `galaxy_horizon_s1299_integration`; HANDOFF/NEXT | **✅** |

### 5.46 SSO stand smoke queue — band 65 (PH-S1289…S1298, 2026-07-22) · **✅**

**Джерело:** project completion / enterprise phase B — SSO live stand smoke (store + CRUD + callbacks). Mirror band 55 [`TENANT_STAND_SMOKE.md`](../development/TENANT_STAND_SMOKE.md). Plan: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](../development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md).

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1224 | **PH-S1289** | `sso_stand_smoke_depth` ui-core module | `sso_stand_smoke_depth.rs` | depth enum + stand-smoke criteria registry | **✅** |
| 1225 | **PH-S1290** | Live store wire smoke | `GET /security/sso/store` | shape + integration | **✅** |
| 1226 | **PH-S1291** | Live OAuth2/SAML CRUD smoke | list→create→get→delete | providers lifecycle | **✅** |
| 1227 | **PH-S1292** | Live callback fixture smoke | OAuth/SAML fixtures | no live IdP | **✅** |
| 1228 | **PH-S1293** | CLI `--sso-stand-smoke` | stand smoke bin | live suite + export shape | **✅** |
| 1229 | **PH-S1294** | `poolai-loc-audit --sso-stand-smoke` | loc-audit | `rust_ratio.json` fields | **✅** |
| 1230 | **PH-S1295** | `VERIFY_SSO_STAND_SMOKE` | verify-dev-stand | live + loc-audit verify | **✅** |
| 1231 | **PH-S1296** | Docs `SSO_STAND_SMOKE.md` + canon | RUN_LOCAL/INDEX/HANDOFF/NEXT | stand matrix | **✅** |
| 1232 | **PH-S1297** | Ratio hold advisory | loc-audit | `--min-ratio 0.95 --advisory` | **✅** |
| 1233 | **PH-S1298** | SSO stand smoke band close | tests/docs | `galaxy_horizon_s1289_integration`; HANDOFF/NEXT | **✅** |

### 5.45 SSO admin/ops glue queue — band 64 (PH-S1279…S1288, 2026-07-22) · **✅**

**Джерело:** project completion 1000 / enterprise phase B — SSO admin UI + ops glue (store strip / providers / verify). Mirror band 54 [`TENANT_ADMIN_OPS.md`](../development/TENANT_ADMIN_OPS.md). Plan: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](../development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md).

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1214 | **PH-S1279** | `sso_admin_ops_depth` ui-core module | `sso_admin_ops_depth.rs` | depth enum + admin/ops criteria registry | **✅** |
| 1215 | **PH-S1280** | Admin SSO store-wire status strip | `src/ui/admin` security/SSO | `#sso-store-badge` ← `GET /security/sso/store` | **✅** |
| 1216 | **PH-S1281** | Admin OAuth2/SAML ops glue | same | list/refresh providers from HTTP contracts | **✅** |
| 1217 | **PH-S1282** | Admin SSO ops HTML contracts | `sso_admin_ops_integration.rs` | store/providers markers | **✅** |
| 1218 | **PH-S1283** | i18n SSO admin ops keys | `i18n.rs` ADMIN_SSO_* | EN/UK patch keys | **✅** |
| 1219 | **PH-S1284** | `VERIFY_SSO_ADMIN_OPS` + quick `--sso-admin-ops` | `bin/verify-dev-stand.sh` | gate + RUN_LOCAL | **✅** |
| 1220 | **PH-S1285** | Stand smoke + `poolai-loc-audit --sso-admin-ops` | stand smoke / loc-audit | export shape + `rust_ratio.json` fields | **✅** |
| 1221 | **PH-S1286** | Docs `SSO_ADMIN_OPS.md` + canon sync | RUN_LOCAL/INDEX/HANDOFF/NEXT | ops matrix + backlog override | **✅** |
| 1222 | **PH-S1287** | Ratio hold advisory | loc-audit | `--min-ratio 0.95 --advisory` | **✅** |
| 1223 | **PH-S1288** | SSO admin/ops band close | tests/docs | `galaxy_horizon_s1279_integration`; HANDOFF/NEXT | **✅** |

### 5.44 SSO API contracts queue — band 63 (PH-S1269…S1278, 2026-07-22)

**Джерело:** FM-horizon v2 / enterprise phase B — SSO HTTP API contracts (OAuth2/SAML CRUD / store-wire read / callback fixtures). Overrides master-backlog template (`sso_depth scaffold`) — mirror band 53 [`TENANT_API.md`](../development/TENANT_API.md).

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1204 | **PH-S1269** | `sso_api_contracts_depth` ui-core module | `sso_api_contracts_depth.rs` | depth enum + HTTP API criteria registry | **✅** |
| 1205 | **PH-S1270** | OAuth2 HTTP CRUD lifecycle | `sso_api_contracts_integration.rs` | POST→GET→PUT→DELETE `/security/oauth2/providers` | **✅** |
| 1206 | **PH-S1271** | SAML HTTP CRUD lifecycle | same suite | POST→GET→PUT→DELETE `/security/saml/providers` | **✅** |
| 1207 | **PH-S1272** | Store-wire status HTTP read | `GET /security/sso/store` | `{mode,durable_path,configured}` memory/sqlite | **✅** |
| 1208 | **PH-S1273** | OpenAPI `SsoStoreWire` + errors | `docs/openapi.yaml` | store schema + path; gap-audit 0 | **✅** |
| 1209 | **PH-S1274** | Callback fixtures (no live IdP) | same suite | OAuth missing code + SAML invalid + audience stub | **✅** |
| 1210 | **PH-S1275** | `verify-dev-stand` / quick `--sso-api` | `bin/verify-dev-stand.sh` | `VERIFY_SSO_API=1` + `--sso-api` | **✅** |
| 1211 | **PH-S1276** | Stand smoke + `poolai-loc-audit --sso-api` | stand smoke / loc-audit | export shape + `rust_ratio.json` sso_api fields | **✅** |
| 1212 | **PH-S1277** | Docs `SSO_API.md` + canon sync | RUN_LOCAL/INDEX/HANDOFF/NEXT | HTTP contract matrix + master backlog override | **✅** |
| 1213 | **PH-S1278** | SSO API band close | tests/docs | `galaxy_horizon_s1269_integration`; HANDOFF/NEXT | **✅** |

### 5.43 SSO store wire queue — band 62 (PH-S1259…S1268, 2026-07-22)

**Джерело:** FM-horizon v2 / enterprise phase B — SSO store wire (`sso_store_wire` + `POOLAI_SSO_DATA_DIR`).

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1194 | **PH-S1259** | `sso_store_depth` ui-core module | `sso_store_depth.rs` | depth enum + SSO store criteria registry | **✅** |
| 1195 | **PH-S1260** | SSO store wire durable path | `security.rs` | `POOLAI_SSO_DATA_DIR` + `sso_store_wire()` + unit tests | **✅** |
| 1196 | **PH-S1261** | SSO store wire contracts | `sso_store_wire_integration.rs` | wire labels + ui-core depth stub | **✅** |
| 1197 | **PH-S1262** | `VERIFY_SSO_STORE` + quick `--sso-store` | RUN_LOCAL / verify-dev-stand | SSO store gate | **✅** |
| 1198 | **PH-S1263** | Stand smoke SSO store export shape | `poolai_http_stand_smoke` | `sso_store_band62_export_shape` | **✅** |
| 1199 | **PH-S1264** | `poolai-loc-audit --sso-store` | RUST_RATIO / §5.13 | `sso_store_*` fields in `rust_ratio.json` | **✅** |
| 1200 | **PH-S1265** | Docs `SSO_STORE.md` + canon sync | DOCS_LEGACY maintain | matrix + RUN_LOCAL/INDEX/HANDOFF/NEXT | **✅** |
| 1201 | **PH-S1266** | `poolai-vision-sync --check` | docs-vision | drift gate green after band 62 | **✅** |
| 1202 | **PH-S1267** | Ratio hold advisory + roadmap zriz | RUST_RATIO stretch | `--min-ratio 0.95 --advisory`; roadmap → band 63 | **✅** |
| 1203 | **PH-S1268** | SSO store band close | Enterprise band 62 | `galaxy_horizon_s1259_integration`; HANDOFF/NEXT → band 63 | **✅** |

### 5.42 SSO depth scaffold queue — band 61 (PH-S1249…S1258, 2026-07-21)

**Джерело:** FM-horizon v2 / enterprise phase B — SSO depth scaffold (`POOLAI_SSO_STORE` + SAML audience/time stub).

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1184 | **PH-S1249** | `sso_depth` ui-core module | `sso_depth.rs` | depth enum + SSO criteria registry | **✅** |
| 1185 | **PH-S1250** | SSO store/verify stub | `security.rs` | `POOLAI_SSO_STORE` + audience/NotOnOrAfter unit tests | **✅** |
| 1186 | **PH-S1251** | SSO depth gate audit | `sso_depth_audit.rs` | criteria registry + FM markers | **✅** |
| 1187 | **PH-S1252** | `VERIFY_SSO` + quick `--sso` | RUN_LOCAL / verify-dev-stand | SSO gate | **✅** |
| 1188 | **PH-S1253** | Stand smoke SSO export shape | `poolai_http_stand_smoke` | `sso_band61_export_shape` | **✅** |
| 1189 | **PH-S1254** | `poolai-loc-audit --sso` | RUST_RATIO / §5.13 | `sso_*` fields in `rust_ratio.json` | **✅** |
| 1190 | **PH-S1255** | Docs `SSO_DEPTH.md` + canon sync | DOCS_LEGACY maintain | matrix + RUN_LOCAL/INDEX/HANDOFF/NEXT | **✅** |
| 1191 | **PH-S1256** | `poolai-vision-sync --check` | docs-vision | drift gate green after band 61 | **✅** |
| 1192 | **PH-S1257** | Ratio hold advisory + roadmap zriz | RUST_RATIO stretch | `--min-ratio 0.95 --advisory`; roadmap → band 62 | **✅** |
| 1193 | **PH-S1258** | SSO depth band close | Enterprise band 61 | `galaxy_horizon_s1249_integration`; HANDOFF/NEXT → band 62 | **✅** |

### 5.41 Tenant horizon close queue — band 60 (PH-S1239…S1248, 2026-07-21)

**Джерело:** FM-horizon v2 / enterprise phase A close — aggregate bands 51–59 `--tenant-*` under `--tenant-horizon`.

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1174 | **PH-S1239** | `tenant_horizon_depth` ui-core module | `tenant_horizon_depth.rs` | depth enum + criteria registry | **✅** |
| 1175 | **PH-S1240** | Phase-A slice aggregate | FM §5.17 / `TENANT_HORIZON_SLICES` | all `--tenant-*` + `tenants.sqlite`; unit test | **✅** |
| 1176 | **PH-S1241** | Tenant horizon contracts | FM §5.17 | `tenant_horizon_integration` criteria totals | **✅** |
| 1177 | **PH-S1242** | `VERIFY_TENANT_HORIZON` + quick flag | RUN_LOCAL / verify-dev-stand | horizon gate | **✅** |
| 1178 | **PH-S1243** | Stand smoke export shape | `poolai_http_stand_smoke` | `tenant_horizon_band60_export_shape` | **✅** |
| 1179 | **PH-S1244** | `poolai-loc-audit --tenant-horizon` | RUST_RATIO / §5.13 | `tenant_horizon_*` fields in `rust_ratio.json` | **✅** |
| 1180 | **PH-S1245** | Docs `TENANT_HORIZON.md` + canon sync | DOCS_LEGACY maintain | matrix + RUN_LOCAL/INDEX/HANDOFF/NEXT | **✅** |
| 1181 | **PH-S1246** | `poolai-vision-sync --check` | docs-vision | drift gate green after band 60 | **✅** |
| 1182 | **PH-S1247** | Ratio hold advisory + roadmap zriz | RUST_RATIO stretch | `--min-ratio 0.95 --advisory`; roadmap → band 61 | **✅** |
| 1183 | **PH-S1248** | Tenant horizon band close | Enterprise band 60 | `galaxy_horizon_s1239_integration`; HANDOFF/NEXT → band 61 | **✅** |

### 5.40 Tenant ratio advisory queue — band 59 (PH-S1229…S1238, 2026-07-21)

**Джерело:** FM-horizon v2 / enterprise phase A — restart-safe SQLite CRUD + aggregate band 51–58 `--tenant-*` ratio-advisory gate.

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1164 | **PH-S1229** | `tenant_ratio_advisory_depth` ui-core module | `tenant_ratio_advisory_depth.rs` | depth enum + criteria registry | **✅** |
| 1165 | **PH-S1230** | Tenant SQLite restart-safe CRUD + slice aggregate | FM §5.17 / `TENANT_RATIO_ADVISORY_SLICES` | `persist_tenant_to_sqlite`; create/get across recreate | **✅** |
| 1166 | **PH-S1231** | Tenant ratio-advisory + sqlite durable contracts | FM §5.17 | `tenant_ratio_advisory_integration` + `tenant_sqlite_durable_integration` | **✅** |
| 1167 | **PH-S1232** | `VERIFY_TENANT_RATIO_ADVISORY` + quick flag | RUN_LOCAL / verify-dev-stand | ratio-advisory gate | **✅** |
| 1168 | **PH-S1233** | Stand smoke export shape | `poolai_http_stand_smoke` | `tenant_ratio_advisory_band59_export_shape` | **✅** |
| 1169 | **PH-S1234** | `poolai-loc-audit --tenant-ratio-advisory` | RUST_RATIO / §5.13 | `tenant_ratio_advisory_*` fields in `rust_ratio.json` | **✅** |
| 1170 | **PH-S1235** | Docs `TENANT_RATIO_ADVISORY.md` + canon sync | DOCS_LEGACY maintain | matrix + RUN_LOCAL/INDEX/HANDOFF/NEXT | **✅** |
| 1171 | **PH-S1236** | `poolai-vision-sync --check` | docs-vision | drift gate green after band 59 | **✅** |
| 1172 | **PH-S1237** | Ratio hold advisory + roadmap zriz | RUST_RATIO stretch | `--min-ratio 0.95 --advisory`; roadmap pointer | **✅** |
| 1173 | **PH-S1238** | Tenant ratio-advisory band close | Enterprise band 59 | `galaxy_horizon_s1229_integration`; HANDOFF/NEXT → band 60 | **✅** |

### 5.39 Tenant vision sync queue — band 58 (PH-S1219…S1228, 2026-07-21)

**Джерело:** FM-horizon v2 / enterprise phase A — aggregate `docs/vision/*` + `TENANT_DOCS_CANON.md` + verify/loc-audit.

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1154 | **PH-S1219** | `tenant_vision_sync_depth` ui-core module | `tenant_vision_sync_depth.rs` | depth enum + vision/criteria registry | **✅** |
| 1155 | **PH-S1220** | Tenant vision-sync slice aggregate stub | RUST_RATIO / `TENANT_VISION_SYNC_SLICES` | six vision slices present; unit test | **✅** |
| 1156 | **PH-S1221** | Tenant vision-sync criteria contracts | FM §5.17 | `tenant_vision_sync_integration` totals consistent | **✅** |
| 1157 | **PH-S1222** | `VERIFY_TENANT_VISION_SYNC` + quick flag | RUN_LOCAL / verify-dev-stand | vision-sync gate | **✅** |
| 1158 | **PH-S1223** | Stand smoke export shape | `poolai_http_stand_smoke` | `tenant_vision_sync_band58_export_shape` | **✅** |
| 1159 | **PH-S1224** | `poolai-loc-audit --tenant-vision-sync` | RUST_RATIO / §5.13 | `tenant_vision_sync_*` fields in `rust_ratio.json` | **✅** |
| 1160 | **PH-S1225** | Docs `TENANT_VISION_SYNC.md` + canon sync | DOCS_LEGACY maintain | matrix + RUN_LOCAL/INDEX/HANDOFF/NEXT | **✅** |
| 1161 | **PH-S1226** | `poolai-vision-sync --check` | docs-vision | drift gate green after band 58 | **✅** |
| 1162 | **PH-S1227** | Ratio hold advisory + roadmap zriz | RUST_RATIO stretch | `--min-ratio 0.95 --advisory`; roadmap pointer | **✅** |
| 1163 | **PH-S1228** | Tenant vision-sync band close | Enterprise band 58 | `galaxy_horizon_s1219_integration`; HANDOFF/NEXT → band 59 | **✅** |

### 5.38 Tenant docs canon queue — band 57 (PH-S1209…S1218, 2026-07-21)

**Джерело:** FM-horizon v2 / enterprise phase A — aggregate band 51–56 `TENANT_*.md` canon docs + verify/loc-audit.

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1144 | **PH-S1209** | `tenant_docs_canon_depth` ui-core module | `tenant_docs_canon_depth.rs` | depth enum + docs/criteria registry | **✅** |
| 1145 | **PH-S1210** | Tenant docs-canon slice aggregate stub | RUST_RATIO / `TENANT_DOCS_CANON_SLICES` | six `TENANT_*.md` present; unit test | **✅** |
| 1146 | **PH-S1211** | Tenant docs-canon criteria contracts | FM §5.17 | `tenant_docs_canon_integration` totals consistent | **✅** |
| 1147 | **PH-S1212** | `VERIFY_TENANT_DOCS_CANON` + quick flag | RUN_LOCAL / verify-dev-stand | docs-canon gate | **✅** |
| 1148 | **PH-S1213** | Stand smoke export shape | `poolai_http_stand_smoke` | `tenant_docs_canon_band57_export_shape` | **✅** |
| 1149 | **PH-S1214** | `poolai-loc-audit --tenant-docs-canon` | RUST_RATIO / §5.13 | `tenant_docs_canon_*` fields in `rust_ratio.json` | **✅** |
| 1150 | **PH-S1215** | Docs `TENANT_DOCS_CANON.md` + canon sync | DOCS_LEGACY maintain | matrix + RUN_LOCAL/INDEX/HANDOFF/NEXT | **✅** |
| 1151 | **PH-S1216** | `poolai-vision-sync --check` | docs-vision | drift gate green after band 57 | **✅** |
| 1152 | **PH-S1217** | Ratio hold advisory + roadmap zriz | RUST_RATIO stretch | `--min-ratio 0.95 --advisory`; roadmap pointer | **✅** |
| 1153 | **PH-S1218** | Tenant docs-canon band close | Enterprise band 57 | `galaxy_horizon_s1209_integration`; HANDOFF/NEXT → band 58 | **✅** |

### 5.37 Tenant loc-audit aggregate queue — band 56 (PH-S1199…S1208, 2026-07-21)

**Джерело:** FM-horizon v2 / enterprise phase A — aggregate band 51–55 `--tenant-*` loc-audit slices + verify/docs.

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1134 | **PH-S1199** | `tenant_loc_audit_depth` ui-core module | `tenant_loc_audit_depth.rs` | depth enum + slice/aggregate criteria registry | **✅** |
| 1135 | **PH-S1200** | Tenant loc-audit slice aggregate stub | RUST_RATIO / `TENANT_LOC_AUDIT_SLICES` | five `--tenant-*` flags present; unit test | **✅** |
| 1136 | **PH-S1201** | Tenant loc-audit criteria contracts | FM §5.17 | `tenant_loc_audit_integration` totals consistent | **✅** |
| 1137 | **PH-S1202** | `VERIFY_TENANT_LOC_AUDIT` + quick flag | RUN_LOCAL / verify-dev-stand | loc-audit aggregate gate | **✅** |
| 1138 | **PH-S1203** | Stand smoke export shape | `poolai_http_stand_smoke` | `tenant_loc_audit_band56_export_shape` | **✅** |
| 1139 | **PH-S1204** | `poolai-loc-audit --tenant-loc-audit` | RUST_RATIO / §5.13 | `tenant_loc_audit_*` fields in `rust_ratio.json` | **✅** |
| 1140 | **PH-S1205** | Docs `TENANT_LOC_AUDIT.md` + canon sync | DOCS_LEGACY maintain | matrix + RUN_LOCAL/INDEX/HANDOFF/NEXT | **✅** |
| 1141 | **PH-S1206** | `poolai-vision-sync --check` | docs-vision | drift gate green after band 56 | **✅** |
| 1142 | **PH-S1207** | Ratio hold advisory + roadmap zriz | RUST_RATIO stretch | `--min-ratio 0.95 --advisory`; roadmap pointer | **✅** |
| 1143 | **PH-S1208** | Tenant loc-audit band close | Enterprise band 56 | `galaxy_horizon_s1199_integration`; HANDOFF/NEXT → band 57 | **✅** |

### 5.36 Tenant stand smoke queue — band 55 (PH-S1189…S1198, 2026-07-21)

**Джерело:** FM-horizon v2 / enterprise phase A — live tenant stand smoke (store / CRUD / usage+quota) + verify/loc-audit.

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1124 | **PH-S1189** | `tenant_stand_smoke_depth` ui-core module | `tenant_stand_smoke_depth.rs` | depth enum + live/CLI/verify criteria registry | **✅** |
| 1125 | **PH-S1190** | Live stand smoke `GET /tenants/store` | TENANT_API §store · stand-smoke | `smoke_tenants_store_wire` + `tenant_stand_smoke_integration` | **✅** |
| 1126 | **PH-S1191** | Live stand smoke tenant CRUD | EnterpriseTenants HTTP | list→create→get→delete; Rust integration gate | **✅** |
| 1127 | **PH-S1192** | Live stand smoke usage + quota + isolation | TENANT_API §usage/quota | usage/quota allow/deny + foreign UUID → 404 | **✅** |
| 1128 | **PH-S1193** | CLI `--tenant-stand-smoke` + export suite | `poolai_http_stand_smoke` | flag/`POOLAI_STAND_SMOKE_TENANT=1` + band55 export shape | **✅** |
| 1129 | **PH-S1194** | `poolai-loc-audit --tenant-stand-smoke` | RUST_RATIO / §5.13 | `tenant_stand_smoke_*` fields in `rust_ratio.json` | **✅** |
| 1130 | **PH-S1195** | `VERIFY_TENANT_STAND_SMOKE` + quick flag | RUN_LOCAL / verify-dev-stand | live smoke + loc-audit gate | **✅** |
| 1131 | **PH-S1196** | Docs `TENANT_STAND_SMOKE.md` + canon sync | DOCS_LEGACY maintain | matrix + RUN_LOCAL/INDEX/HANDOFF/NEXT | **✅** |
| 1132 | **PH-S1197** | Ratio hold advisory + roadmap zriz | RUST_RATIO stretch | `--min-ratio 0.95 --advisory`; roadmap pointer | **✅** |
| 1133 | **PH-S1198** | Tenant stand-smoke band close | Enterprise band 55 | `galaxy_horizon_s1189_integration`; HANDOFF/NEXT → band 56 | **✅** |

### 5.35 Tenant admin/ops glue queue — band 54 (PH-S1179…S1188, 2026-07-20)

**Джерело:** FM-horizon v2 / enterprise phase A — tenant admin UI + ops glue (store strip / usage+quota / verify).

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1114 | **PH-S1179** | `tenant_admin_ops_depth` ui-core module | `tenant_admin_ops_depth.rs` | depth enum + admin/ops criteria registry | **✅** |
| 1115 | **PH-S1180** | Admin store-wire status strip | `src/ui/admin/tenants.rs` | `#tenant-store-badge` ← `GET /tenants/store` | **✅** |
| 1116 | **PH-S1181** | Admin usage + quota ops glue | same | `refreshTenantUsage` + `probeTenantQuota` | **✅** |
| 1117 | **PH-S1182** | Admin ops HTML contracts | `tenant_admin_ops_integration.rs` | store/usage/quota markers | **✅** |
| 1118 | **PH-S1183** | i18n store/usage/quota keys | `i18n.rs` ADMIN_TENANTS_* | EN/UK patch keys | **✅** |
| 1119 | **PH-S1184** | `verify-dev-stand` / quick `--tenant-admin-ops` | `bin/verify-dev-stand.sh` | `VERIFY_TENANT_ADMIN_OPS=1` + quick flag | **✅** |
| 1120 | **PH-S1185** | Stand smoke + `poolai-loc-audit --tenant-admin-ops` | stand smoke / loc-audit | export shape + `rust_ratio.json` fields | **✅** |
| 1121 | **PH-S1186** | Docs `TENANT_ADMIN_OPS.md` + canon sync | RUN_LOCAL/INDEX/HANDOFF/NEXT | ops matrix + master backlog override | **✅** |
| 1122 | **PH-S1187** | Ratio hold advisory | loc-audit | `--min-ratio 0.95 --advisory` | **✅** |
| 1123 | **PH-S1188** | Tenant admin/ops band close | tests/docs | `galaxy_horizon_s1179_integration`; HANDOFF/NEXT | **✅** |

### 5.34 Tenant API contracts queue — band 53 (PH-S1169…S1178, 2026-07-20)

**Джерело:** FM-horizon v2 / enterprise phase A — tenant HTTP API contracts (CRUD / quota / isolation / store-wire read).

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1104 | **PH-S1169** | `tenant_api_contracts_depth` ui-core module | `tenant_api_contracts_depth.rs` | depth enum + HTTP API criteria registry | **✅** |
| 1105 | **PH-S1170** | HTTP CRUD lifecycle contracts | `tenant_api_contracts_integration.rs` | POST→GET→update→DELETE via AppState router | **✅** |
| 1106 | **PH-S1171** | Quota + usage HTTP contracts | same suite | `GET …/usage` + `POST …/quota` allow/deny shapes | **✅** |
| 1107 | **PH-S1172** | Cross-tenant isolation API | same suite | mutate A; B unchanged; foreign UUID → 404 | **✅** |
| 1108 | **PH-S1173** | Store-wire status HTTP read | `GET /tenants/store` | `{mode,durable_path,configured}` memory/sqlite | **✅** |
| 1109 | **PH-S1174** | OpenAPI `TenantStoreWire` + errors | `docs/openapi.yaml` | store schema + 400/404/503; gap-audit 0 | **✅** |
| 1110 | **PH-S1175** | `verify-dev-stand` / quick `--tenant-api` | `bin/verify-dev-stand.sh` | `VERIFY_TENANT_API=1` + `--tenant-api` | **✅** |
| 1111 | **PH-S1176** | Stand smoke + `poolai-loc-audit --tenant-api` | stand smoke / loc-audit | export shape + `rust_ratio.json` tenant_api fields | **✅** |
| 1112 | **PH-S1177** | Docs `TENANT_API.md` + canon sync | RUN_LOCAL/INDEX/HANDOFF/NEXT | HTTP contract matrix + master backlog override | **✅** |
| 1113 | **PH-S1178** | Tenant API band close | tests/docs | `galaxy_horizon_s1169_integration`; HANDOFF/NEXT | **✅** |

### 5.33 Tenant store wire queue — band 52 (PH-S1159…S1168, 2026-07-20)

**Джерело:** FM-horizon v2 / enterprise phase A — tenant store wire (`POOLAI_TENANT_DATA_DIR` + `tenant_store_wire`).

| # | Sprint | Фокус | Джерело | Acceptance | Status |
|---|--------|--------|---------|------------|--------|
| 1094 | **PH-S1159** | `tenant_depth` ui-core module | `tenant_depth.rs` | depth enum + tenant store criteria registry | **✅** |
| 1095 | **PH-S1160** | `tenant_store_wire` durable path stub | `multi_tenancy.rs` | `POOLAI_TENANT_DATA_DIR` + wire snapshot + unit tests | **✅** |
| 1096 | **PH-S1161** | Tenant store API contracts | `tenant_store_wire_integration.rs` | wire contract + lifecycle isolation | **✅** |
| 1097 | **PH-S1162** | `verify-dev-stand` / quick tenant store | `bin/verify-dev-stand.sh` | `VERIFY_TENANT_STORE=1` + `--tenant-store` | **✅** |
| 1098 | **PH-S1163** | Stand smoke tenant store export shape | `poolai_http_stand_smoke.rs` | `tenant_store_band52_export_shape` | **✅** |
| 1099 | **PH-S1164** | `poolai-loc-audit --tenant-store` | `poolai_loc_audit.rs` | tenant_store fields in `rust_ratio.json` | **✅** |
| 1100 | **PH-S1165** | Docs canon sync | RUN_LOCAL/INDEX/HANDOFF/NEXT | band 52 ops + `TENANT_STORE.md` | **✅** |
| 1101 | **PH-S1166** | poolai-vision-sync --check | docs/vision | drift gate green | **✅** |
| 1102 | **PH-S1167** | Ratio hold advisory | loc-audit | `--min-ratio 0.95 --advisory` | **✅** |
| 1103 | **PH-S1168** | Tenant store band close | tests/docs | `galaxy_horizon_s1159_integration`; HANDOFF/NEXT | **✅** |

### 5.32 Tenant persistence queue — band 51 (PH-S1149…S1158, 2026-07-19)

**Джерело:** FM-horizon v2 / enterprise phase A — tenant persist depth scaffold (`POOLAI_TENANT_STORE`).

| 1084 | **PH-S1149** | `tenant_persistence_depth` ui-core module | `tenant_persistence_depth.rs` | depth enum + tenant persist criteria registry | **✅** |
| 1085 | **PH-S1150** | `poolai-loc-audit --tenant-persist` | `poolai_loc_audit.rs` | tenant_persist fields in `rust_ratio.json` | **✅** |
| 1086 | **PH-S1151** | Tenant persist gate audit | `tenant_persistence_audit.rs` | criteria registry + FM markers | **✅** |
| 1087 | **PH-S1152** | `multi_tenancy` store env hint | `multi_tenancy.rs` | `POOLAI_TENANT_STORE` + `tenant_store_mode()` | **✅** |
| 1088 | **PH-S1153** | `verify-dev-stand` tenant persist hook | `bin/verify-dev-stand.sh` | `VERIFY_TENANT_PERSIST=1` | **✅** |
| 1089 | **PH-S1154** | `run-poolai quick --tenant-persist` | `bin/run-poolai.*` | post-health `--tenant-persist` | **✅** |
| 1090 | **PH-S1155** | Stand smoke tenant persist export shape | `poolai_http_stand_smoke.rs` | export shape unit test | **✅** |
| 1091 | **PH-S1156** | RUN_LOCAL + RUST_RATIO + GALAXY sync | docs | band 51 ops + enterprise roadmap pointer | **✅** |
| 1092 | **PH-S1157** | `TENANT_PERSIST.md` + master backlog 1000 | docs/scripts | durable tenant workflow + PH-S1149…S2148 registry | **✅** |
| 1093 | **PH-S1158** | Tenant persist band close | tests/docs | `galaxy_horizon_s1149_integration`; HANDOFF/NEXT | **✅** |

### 5.31 CI canon gate queue — band 50 (PH-S1139…S1148, 2026-07-19)

**Джерело:** project scan band 50 — local dual-gate (test-ci + openapi-gap + rust-ratio advisory) mirroring GitHub CI jobs.

| 1074 | **PH-S1139** | `ci_canon_depth` ui-core module | `ci_canon_depth.rs` | depth enum + CI canon criteria registry constants | **✅** |
| 1075 | **PH-S1140** | `poolai-loc-audit --ci-canon` | `poolai_loc_audit.rs` | ci_canon fields in `rust_ratio.json` | **✅** |
| 1076 | **PH-S1141** | CI canon gate audit | `ci_canon_audit.rs` | criteria registry + maintenance markers | **✅** |
| 1077 | **PH-S1142** | `verify-dev-stand` CI canon hook | `bin/verify-dev-stand.sh` | `VERIFY_CI_CANON=1`; openapi-gap + loc-audit `--ci-canon` + rust-ratio advisory | **✅** |
| 1078 | **PH-S1143** | `run-poolai quick --ci-canon` | `bin/run-poolai.sh`, `bin/run-poolai.ps1` | post-health `--ci-canon` + openapi-gap-audit | **✅** |
| 1079 | **PH-S1144** | Stand smoke CI canon export shape | `poolai_http_stand_smoke.rs` | `ci_canon_depth` JSON field on export shape test | **✅** |
| 1080 | **PH-S1145** | RUN_LOCAL.md band 50 ops sync | docs | `--ci-canon`, `VERIFY_CI_CANON` | **✅** |
| 1081 | **PH-S1146** | RUST_RATIO + GALAXY_GRID_ROADMAP band 50 sync | docs | `--ci-canon` pointer + loc-audit flag | **✅** |
| 1082 | **PH-S1147** | `CI_CANON.md` canon gate docs | docs | local dual-gate workflow doc | **✅** |
| 1083 | **PH-S1148** | CI canon gate band close | tests/docs | `galaxy_horizon_s1139_integration`; HANDOFF/NEXT | **✅** |

### 5.30 Pre-push vision canon gate queue — band 49 (PH-S1129…S1138, 2026-07-19)

**Джерело:** project scan band 49 — git pre-push hook + `poolai-vision-sync` canon doc validation + `cargo fmt` gate.

| 1064 | **PH-S1129** | `pre_push_hook_depth` ui-core module | `pre_push_hook_depth.rs` | depth enum + pre-push canon criteria registry constants | **✅** |
| 1065 | **PH-S1130** | `poolai-loc-audit --pre-push-canon` | `poolai_loc_audit.rs` | pre_push fields in `rust_ratio.json` | **✅** |
| 1066 | **PH-S1131** | Pre-push canon gate audit | `pre_push_hook_audit.rs` | criteria registry + maintenance markers | **✅** |
| 1067 | **PH-S1132** | `poolai-vision-sync` canon doc validation | `poolai_vision_sync.rs` | README/INDEX/NEXT/vision.svg canon sync + `--check` drift | **✅** |
| 1068 | **PH-S1133** | `bin/pre-push-hook.sh` + install script | `bin/pre-push-hook.sh` | vision sync + fmt gate before push | **✅** |
| 1069 | **PH-S1134** | `verify-dev-stand` + `quick` pre-push hooks | `bin/verify-dev-stand.sh`, `bin/run-poolai.*` | `VERIFY_PRE_PUSH_CANON=1`; `quick --pre-push-canon` | **✅** |
| 1070 | **PH-S1135** | RUN_LOCAL.md band 49 ops sync | docs | `--pre-push-canon`, `VERIFY_PRE_PUSH_CANON` | **✅** |
| 1071 | **PH-S1136** | RUST_RATIO + GALAXY_GRID_ROADMAP band 49 sync | docs | pre-push canon pointer + loc-audit flag | **✅** |
| 1072 | **PH-S1137** | PRE_PUSH_HOOK.md canon gate docs | docs | install hook + vision sync + fmt workflow | **✅** |
| 1073 | **PH-S1138** | Pre-push canon gate band close | tests/docs | `galaxy_horizon_s1129_integration`; HANDOFF/NEXT | **✅** |

### 5.29 Galaxy edge verification horizon queue — band 48 (PH-S1119…S1128, 2026-07-18)

**Джерело:** project scan band 48 — Galaxy §6.6 edge verification metrics HTTP wire + maintenance ops scaffold.

| 1054 | **PH-S1119** | `galaxy_edge_verification_depth` ui-core module | `galaxy_edge_verification_depth.rs` | depth enum + edge verification criteria registry constants | **✅** |
| 1055 | **PH-S1120** | `poolai-loc-audit --edge-verification-advisory` | `poolai_loc_audit.rs` | edge_verification fields in `rust_ratio.json` | **✅** |
| 1056 | **PH-S1121** | Edge verification advisory audit | `galaxy_edge_verification_audit.rs` | criteria registry + maintenance markers | **✅** |
| 1057 | **PH-S1122** | `GET /api/v1/grid/edge-verification-metrics` | Galaxy §6.6 | JSON snapshot fraud-proof/capability/network/TEE + OpenAPI | **✅** |
| 1058 | **PH-S1123** | `galaxy_edge_verification_depth` grid stub + parity v4 | `stand_smoke_metrics_parity.rs` | `validate_band6_metrics_parity_v4` + horizon depth wire | **✅** |
| 1059 | **PH-S1124** | Admin updates-compat edge-verification wasm strip | `updates_compat.rs` | `renderGridEdgeVerificationMetricsStrip` on updates-compat | **✅** |
| 1060 | **PH-S1125** | `verify-dev-stand` + `quick` edge-verification hooks | `bin/verify-dev-stand.sh`, `bin/run-poolai.*` | `VERIFY_EDGE_VERIFICATION=1`; `quick --edge-verification` | **✅** |
| 1061 | **PH-S1126** | RUN_LOCAL.md band 48 ops sync | docs | `--edge-verification`, `VERIFY_EDGE_VERIFICATION` | **✅** |
| 1062 | **PH-S1127** | RUST_RATIO + GALAXY_GRID_ROADMAP band 48 sync | docs | edge-verification pointer + loc-audit flag | **✅** |
| 1063 | **PH-S1128** | Edge verification horizon band close | tests/docs | `galaxy_horizon_s1119_integration`; HANDOFF/NEXT | **✅** |

### 5.28 STABLE touch-up queue — band 47 (PH-S1109…S1118, 2026-07-18)

**Джерело:** project scan band 47 — maintenance-mode STABLE criteria registry touch-up after band 46 migration advisory.

| 1044 | **PH-S1109** | `stable_state_touchup_depth` ui-core module | `stable_state_touchup_depth.rs` | depth enum + STABLE criteria registry constants | **✅** |
| 1045 | **PH-S1110** | `poolai-loc-audit --stable-touchup` | `poolai_loc_audit.rs` | stable_touchup fields in `rust_ratio.json` | **✅** |
| 1046 | **PH-S1111** | STABLE touch-up audit | `stable_state_touchup_audit.rs` | criteria registry + maintenance markers | **✅** |
| 1047 | **PH-S1112** | STABLE criteria registry | `stable_state_touchup_depth.rs` | 7 maintenance-mode criteria ids | **✅** |
| 1048 | **PH-S1113** | `verify-dev-stand` STABLE hook | `bin/verify-dev-stand.sh` | `VERIFY_STABLE_TOUCHUP=1` → loc-audit | **✅** |
| 1049 | **PH-S1114** | `run-poolai quick --stable-touchup` | `bin/run-poolai.*` | post-health `--stable-touchup` + stand smoke export shape | **✅** |
| 1050 | **PH-S1115** | RUN_LOCAL.md band 47 ops sync | docs | `--stable-touchup`, `VERIFY_STABLE_TOUCHUP` | **✅** |
| 1051 | **PH-S1116** | RUST_RATIO_STRATEGY band 47 sync | docs | STABLE touch-up pointer + loc-audit flag | **✅** |
| 1052 | **PH-S1117** | GALAXY_GRID_ROADMAP + STABLE touch-up | docs + loc-audit | band 47 closed row; maintenance criteria | **✅** |
| 1053 | **PH-S1118** | STABLE touch-up band close | tests/docs | `galaxy_horizon_s1109_integration`; HANDOFF/NEXT | **✅** |

| 1034 | **PH-S1099** | `rust_migration_advisory_depth` ui-core module | `rust_migration_advisory_depth.rs` | depth enum + ui_js/e2e registry constants | **✅** |
| 1035 | **PH-S1100** | `poolai-loc-audit --migration-advisory` | `poolai_loc_audit.rs` | migration fields in `rust_ratio.json` | **✅** |
| 1036 | **PH-S1101** | Rust migration advisory audit | `rust_migration_advisory_audit.rs` | ui_js candidates + archived e2e canon paths | **✅** |
| 1037 | **PH-S1102** | Admin JS migration candidate registry | `rust_migration_advisory_depth.rs` | 6 ui_js → wasm targets | **✅** |
| 1038 | **PH-S1103** | `verify-dev-stand` migration hook | `bin/verify-dev-stand.sh` | `VERIFY_MIGRATION_ADVISORY=1` → loc-audit | **✅** |
| 1039 | **PH-S1104** | `run-poolai quick --migration-advisory` | `bin/run-poolai.*` | post-health `--migration-advisory` + stand smoke export shape | **✅** |
| 1040 | **PH-S1105** | RUN_LOCAL.md band 46 ops sync | docs | `--migration-advisory`, `VERIFY_MIGRATION_ADVISORY` | **✅** |
| 1041 | **PH-S1106** | RUST_RATIO_STRATEGY band 46 sync | docs | migration advisory pointer + loc-audit flag | **✅** |
| 1042 | **PH-S1107** | GALAXY_GRID_ROADMAP + rust ratio | docs + loc-audit | band 46 closed row; stretch 96% hold | **✅** |
| 1043 | **PH-S1108** | Ratio migration advisory band close | tests/docs | `galaxy_horizon_s1099_integration`; HANDOFF/NEXT | **✅** |

| 1024 | **PH-S1089** | RUN_LOCAL health export shape | `stand_smoke_run_local_depth.rs` | `RUN_LOCAL_HEALTH_KEYS` + enhanced `smoke_health` | **✅** |
| 1025 | **PH-S1090** | Monitoring alerts stand smoke | `poolai_http_stand_smoke.rs` | GET `/api/enterprise/monitoring/alerts` array | **✅** |
| 1026 | **PH-S1091** | Monitoring dashboards stand smoke | `poolai_http_stand_smoke.rs` | GET `/api/enterprise/monitoring/dashboards` array | **✅** |
| 1027 | **PH-S1092** | VM instances stand smoke | `poolai_http_stand_smoke.rs` | GET `/api/v1/vm/instances` list shape | **✅** |
| 1028 | **PH-S1093** | `--run-local-smoke` CLI subset | `poolai_http_stand_smoke.rs` | 6-case RUN_LOCAL gate; env `POOLAI_STAND_SMOKE_RUN_LOCAL` | **✅** |
| 1029 | **PH-S1094** | `verify-dev-stand` stand smoke hook | `bin/verify-dev-stand.sh` | `VERIFY_STAND_SMOKE=1` → `--run-local-smoke` | **✅** |
| 1030 | **PH-S1095** | `run-poolai quick --stand-smoke` | `bin/run-poolai.*` | post-health `--run-local-smoke` | **✅** |
| 1031 | **PH-S1096** | RUN_LOCAL.md band 45 ops sync | docs | `--run-local-smoke`, `VERIFY_STAND_SMOKE`, `--stand-smoke` | **✅** |
| 1032 | **PH-S1097** | GALAXY_GRID_ROADMAP + rust ratio advisory | docs | band 45 closed row; ratio hold → band 46 | **✅** |
| 1033 | **PH-S1098** | Stand smoke RUN_LOCAL band close | tests/docs | `galaxy_horizon_s1089_integration`; HANDOFF/NEXT | **✅** |

| 1014 | **PH-S1079** | Monitoring alerts wasm slim depth | `admin_wasm_slim_depth.rs` | `MonitoringAlertsPanel` flag + renderer smoke | **✅** |
| 1015 | **PH-S1080** | Monitoring dashboards wasm slim depth | `admin_wasm_slim_depth.rs` | `MonitoringDashboardsPanel` flag + renderer smoke | **✅** |
| 1016 | **PH-S1081** | Instances + Telegram seats depth | `admin_wasm_slim_depth.rs` | `InstancesPanel` + `TelegramSeatsPanel` flags | **✅** |
| 1017 | **PH-S1082** | Virtual nodes + network profiles depth | `admin_wasm_slim_depth.rs` | `GalaxyVirtualNodesPanel` + `NetworkProfilesPanel` | **✅** |
| 1018 | **PH-S1083** | Grid prefetch/locality strips depth | `stand_smoke_metrics.rs` | `GridPrefetchMetricsStrip` + `GridLocalityMetricsStrip` | **✅** |
| 1019 | **PH-S1084** | Grid governance/fee-split strips depth | `stand_smoke_metrics.rs` | stand smoke band44 export-shape + strip flags | **✅** |
| 1020 | **PH-S1085** | `admin/mod.rs` wasm glue regression | `src/ui/admin/mod.rs` | band-44 `poolaiRender*` / `render*` asserts | **✅** |
| 1021 | **PH-S1086** | `admin_wasm_slim_depth` ui-core module | `admin_wasm_slim_depth.rs` | extract from `grid_replication_pricing` + FM band 44 | **✅** |
| 1022 | **PH-S1087** | GALAXY_GRID_ROADMAP + rust ratio | docs + loc-audit | band 44 closed row; `rust_ratio.json` zriz | **✅** |
| 1023 | **PH-S1088** | Admin wasm slim band close | tests/docs | `galaxy_horizon_s1079_integration`; HANDOFF/NEXT | **✅** |

| 1004 | **PH-S1069** | Verification extended parity | `stand_smoke_metrics_parity.rs` | `VERIFICATION_EXTENDED_PARITY` mismatch/match JSON↔Prom | **✅** |
| 1005 | **PH-S1070** | Replication extended parity | `stand_smoke_metrics_parity.rs` | `REPLICATION_EXTENDED_PARITY` executor/rate-cap | **✅** |
| 1006 | **PH-S1071** | Pricing extended parity | `stand_smoke_metrics_parity.rs` | `PRICING_EXTENDED_PARITY` forced_fallback + provider hits/errors | **✅** |
| 1007 | **PH-S1072** | Prefetch/settlement/trust extended parity | `stand_smoke_metrics_parity.rs` | `PREFETCH_EXTENDED_PARITY` + settlement/trust extended pairs | **✅** |
| 1008 | **PH-S1073** | `validate_band6_metrics_parity_v3` + stand smoke v3 | `poolai_http_stand_smoke.rs` | `grid_metrics_json_prometheus_parity_band6_v3` live runner | **✅** |
| 1009 | **PH-S1074** | Grid metrics parity contracts | `grid_metrics_parity_contracts.rs` | all 11 `*-metrics` APIs shape + v3 synthetic gate | **✅** |
| 1010 | **PH-S1075** | PROMETHEUS_METRICS band 43 sync | `PROMETHEUS_METRICS.md` | v3 extended parity pointer | **✅** |
| 1011 | **PH-S1076** | GALAXY_GRID_ROADMAP + concept markers | roadmap §4 | band 43 closed row | **✅** |
| 1012 | **PH-S1077** | `grid_metrics_parity_depth` ui-core stub | `grid_metrics_parity_depth.rs` | depth enum + FM band 43 markers | **✅** |
| 1013 | **PH-S1078** | Grid metrics parity band close | tests/docs | `galaxy_horizon_s1069_integration`; HANDOFF/NEXT | **✅** |

| 994 | **PH-S1059** | OpenAPI gap audit regression gate | `poolai-openapi-gap-audit` | exit 0; no missing Axum routes | **✅** |
| 995 | **PH-S1060** | Grid OpenAPI contract extend | `grid_openapi_contracts.rs` | seed-inventory, verification-replay, checker tasks shape | **✅** |
| 996 | **PH-S1061** | Memory shards OpenAPI contract | `memory_api_contracts.rs` | list/register/get shard keys per OpenAPI | **✅** |
| 997 | **PH-S1062** | Ops power stand smoke OpenAPI | `poolai_http_stand_smoke.rs` | POST `/ops/power` → 202 + structured body | **✅** |
| 998 | **PH-S1063** | OpenAPI examples depth tier-2 | `docs/openapi.yaml` | ops power, memory shards, seed-inventory examples | **✅** |
| 999 | **PH-S1064** | POOLAI_GALAXY_GRID maintenance markers | concept | power ops PH-S1016 + E2E visual PH-S1049…S1058 ✅ | **✅** |
| 1000 | **PH-S1065** | GALAXY_GRID_ROADMAP maintenance rows | roadmap §4 | bands 38–41 closed; band 42 in progress row | **✅** |
| 1001 | **PH-S1066** | DIGEST + DOCS_LEGACY + INDEX refresh | docs canon | bands 38–42 zriz; openapi audit + ops power | **✅** |
| 1002 | **PH-S1067** | `openapi_wire_depth` stub | `openapi_wire_depth.rs` | depth enum + FM band 42 markers | **✅** |
| 1003 | **PH-S1068** | OpenAPI wire band close | tests/docs | `galaxy_horizon_s1059_integration`; HANDOFF/NEXT | **✅** |

| 984 | **PH-S1049** | Visual parity tier-1 | `e2e/tests/visual.spec.ts` | config + jobs snapshots | **✅** |
| 985 | **PH-S1050** | Visual parity tier-2 grid panels | `visual.spec.ts` | updates-compat, seed, advisories | **✅** |
| 986 | **PH-S1051** | Vision axe smoke | `e2e/tests/a11y.spec.ts` | axe on vision map; skip when server down | **✅** |
| 987 | **PH-S1052** | Vision map visual snapshot | `e2e/tests/vision.spec.ts` | masked starfield/orbit shell baseline | **✅** |
| 988 | **PH-S1053** | High-contrast axe extend | `a11y.spec.ts` | config, jobs, tenants HC color-contrast | **✅** |
| 989 | **PH-S1054** | Visual snapshot ready helper | `e2e/tests/helpers.ts` | `waitForVisualSnapshotReady` fonts+rAF | **✅** |
| 990 | **PH-S1055** | e2e scope visual/axe parity | `tests/e2e_scope_audit.rs` | tier1/tier2 routes + baseline PNG gate | **✅** |
| 991 | **PH-S1056** | rust_ratio loc-audit | `rust_ratio.json` | poolai-loc-audit band 41 zriz | **✅** |
| 992 | **PH-S1057** | ui-core depth stub | `e2e_visual_axe_depth.rs` | depth enum + FM band 41 markers | **✅** |
| 993 | **PH-S1058** | E2E visual/axe band close | tests/docs | `galaxy_horizon_s1049_integration`; HANDOFF/NEXT | **✅** |

| 974 | **PH-S1039** | Skip links + landmarks | `docs/vision/index.html` | skip to map/queue/preview; `role="main"` | **✅** |
| 975 | **PH-S1040** | Icon control aria-label parity | `docs/vision/` | header/map/panel icon buttons labeled; `aria-pressed` toggles | **✅** |
| 976 | **PH-S1041** | Explorer tree keyboard | `vision.js` file-tree | `role="tree"` / `treeitem`; Arrow/Home/End nav | **✅** |
| 977 | **PH-S1042** | Link graph neighbour a11y | `#link-graph` | focusable neighbours; Enter select; focus restore | **✅** |
| 978 | **PH-S1043** | Map sprint-dim incremental | `vision.js` | `updateMapSprintDim` without full `renderMap` | **✅** |
| 979 | **PH-S1044** | Dense-map LOD hardening | `isMapOverviewMode` | adaptive threshold when layer >120 nodes | **✅** |
| 980 | **PH-S1045** | Background tab perf | `vision.js` | pause starfield + orbit on `visibilitychange` | **✅** |
| 981 | **PH-S1046** | ui-core depth stub | `vision_map_depth.rs` | depth enum + FM band 40 markers | **✅** |
| 982 | **PH-S1047** | Vision Playwright smoke | `e2e/tests/vision.spec.ts` | skip-link focus; `#map-scene-3d`; tree role | **✅** |
| 983 | **PH-S1048** | Vision map band close | tests/docs | `galaxy_horizon_s1039_integration`; HANDOFF/NEXT | **✅** |

### 5.27 Ratio/rust migration advisory queue — band 46 (PH-S1099…S1108, 2026-07-18)

**Джерело:** project scan band 46 — stretch 96% spirit hold; ui_js glue + archived e2e migration registry without new Playwright API specs.

| 1034 | **PH-S1099** | `rust_migration_advisory_depth` ui-core module | `rust_migration_advisory_depth.rs` | depth enum + ui_js/e2e registry constants | **✅** |
| 1035 | **PH-S1100** | `poolai-loc-audit --migration-advisory` | `poolai_loc_audit.rs` | migration fields in `rust_ratio.json` | **✅** |
| 1036 | **PH-S1101** | Rust migration advisory audit | `rust_migration_advisory_audit.rs` | ui_js candidates + archived e2e canon paths | **✅** |
| 1037 | **PH-S1102** | Admin JS migration candidate registry | `rust_migration_advisory_depth.rs` | 6 ui_js → wasm targets | **✅** |
| 1038 | **PH-S1103** | `verify-dev-stand` migration hook | `bin/verify-dev-stand.sh` | `VERIFY_MIGRATION_ADVISORY=1` → loc-audit | **✅** |
| 1039 | **PH-S1104** | `run-poolai quick --migration-advisory` | `bin/run-poolai.*` | post-health `--migration-advisory` + stand smoke export shape | **✅** |
| 1040 | **PH-S1105** | RUN_LOCAL.md band 46 ops sync | docs | `--migration-advisory`, `VERIFY_MIGRATION_ADVISORY` | **✅** |
| 1041 | **PH-S1106** | RUST_RATIO_STRATEGY band 46 sync | docs | migration advisory pointer + loc-audit flag | **✅** |
| 1042 | **PH-S1107** | GALAXY_GRID_ROADMAP + rust ratio | docs + loc-audit | band 46 closed row; stretch 96% hold | **✅** |
| 1043 | **PH-S1108** | Ratio migration advisory band close | tests/docs | `galaxy_horizon_s1099_integration`; HANDOFF/NEXT | **✅** |

### 5.26 Stand smoke + RUN_LOCAL ops queue — band 45 (PH-S1089…S1098, 2026-07-18)

**Джерело:** project scan band 45 — RUN_LOCAL quick/verify hooks without live stand smoke subset; monitoring/vm API gaps.

| 1024 | **PH-S1089** | RUN_LOCAL health export shape | `stand_smoke_run_local_depth.rs` | `RUN_LOCAL_HEALTH_KEYS` + enhanced `smoke_health` | **✅** |
| 1025 | **PH-S1090** | Monitoring alerts stand smoke | `poolai_http_stand_smoke.rs` | GET `/api/enterprise/monitoring/alerts` array | **✅** |
| 1026 | **PH-S1091** | Monitoring dashboards stand smoke | `poolai_http_stand_smoke.rs` | GET `/api/enterprise/monitoring/dashboards` array | **✅** |
| 1027 | **PH-S1092** | VM instances stand smoke | `poolai_http_stand_smoke.rs` | GET `/api/v1/vm/instances` list shape | **✅** |
| 1028 | **PH-S1093** | `--run-local-smoke` CLI subset | `poolai_http_stand_smoke.rs` | 6-case RUN_LOCAL gate; env `POOLAI_STAND_SMOKE_RUN_LOCAL` | **✅** |
| 1029 | **PH-S1094** | `verify-dev-stand` stand smoke hook | `bin/verify-dev-stand.sh` | `VERIFY_STAND_SMOKE=1` → `--run-local-smoke` | **✅** |
| 1030 | **PH-S1095** | `run-poolai quick --stand-smoke` | `bin/run-poolai.*` | post-health `--run-local-smoke` | **✅** |
| 1031 | **PH-S1096** | RUN_LOCAL.md band 45 ops sync | docs | `--run-local-smoke`, `VERIFY_STAND_SMOKE`, `--stand-smoke` | **✅** |
| 1032 | **PH-S1097** | GALAXY_GRID_ROADMAP + rust ratio advisory | docs | band 45 closed row; ratio hold → band 46 | **✅** |
| 1033 | **PH-S1098** | Stand smoke RUN_LOCAL band close | tests/docs | `galaxy_horizon_s1089_integration`; HANDOFF/NEXT | **✅** |

### 5.25 Admin wasm slim panels queue — band 44 (PH-S1079…S1088, 2026-07-18)

**Джерело:** project scan band 44 — wasm-first admin panels without `admin_wasm_slim_depth` classification.

| 1014 | **PH-S1079** | Monitoring alerts wasm slim depth | `admin_wasm_slim_depth.rs` | `MonitoringAlertsPanel` flag + renderer smoke | **✅** |
| 1015 | **PH-S1080** | Monitoring dashboards wasm slim depth | `admin_wasm_slim_depth.rs` | `MonitoringDashboardsPanel` flag + renderer smoke | **✅** |
| 1016 | **PH-S1081** | Instances + Telegram seats depth | `admin_wasm_slim_depth.rs` | `InstancesPanel` + `TelegramSeatsPanel` flags | **✅** |
| 1017 | **PH-S1082** | Virtual nodes + network profiles depth | `admin_wasm_slim_depth.rs` | `GalaxyVirtualNodesPanel` + `NetworkProfilesPanel` | **✅** |
| 1018 | **PH-S1083** | Grid prefetch/locality strips depth | `stand_smoke_metrics.rs` | `GridPrefetchMetricsStrip` + `GridLocalityMetricsStrip` | **✅** |
| 1019 | **PH-S1084** | Grid governance/fee-split strips depth | `stand_smoke_metrics.rs` | stand smoke band44 export-shape + strip flags | **✅** |
| 1020 | **PH-S1085** | `admin/mod.rs` wasm glue regression | `src/ui/admin/mod.rs` | band-44 `poolaiRender*` / `render*` asserts | **✅** |
| 1021 | **PH-S1086** | `admin_wasm_slim_depth` ui-core module | `admin_wasm_slim_depth.rs` | extract from `grid_replication_pricing` + FM band 44 | **✅** |
| 1022 | **PH-S1087** | GALAXY_GRID_ROADMAP + rust ratio | docs + loc-audit | band 44 closed row; `rust_ratio.json` zriz | **✅** |
| 1023 | **PH-S1088** | Admin wasm slim band close | tests/docs | `galaxy_horizon_s1079_integration`; HANDOFF/NEXT | **✅** |

### 5.24 Grid metrics parity hardening queue — band 43 (PH-S1069…S1078, 2026-07-18)

**Джерело:** project scan band 43 — extended JSON↔Prom pairs beyond band-6 v2; stand smoke v3 runner.

### 5.23 OpenAPI/docs wire sync queue — band 42 (PH-S1059…S1068, 2026-07-18)

**Джерело:** project scan band 42 — OpenAPI depth + contract tests + stand smoke ops/power + docs canon drift after bands 38–41.

### 5.22 E2E visual/axe regression queue — band 41 (PH-S1049…S1058, 2026-07-18)

**Джерело:** project scan band 41 — visual snapshots missing grid panels; vision axe gap; HC axe thin.

### 5.21 Vision map/a11y/perf queue — band 40 (PH-S1039…S1048, 2026-07-18)

**Джерело:** project scan band 40 — Vision map UX gaps (skip links, tree keyboard, link graph, perf).

| 964 | **PH-S1029** | Empty-state parity tier 1 | `src/ui/admin` | tenants + security OAuth/SAML/policies → `adminEmptyStateHtml` | **✅** |
| 965 | **PH-S1030** | Security tables polish | `src/ui/admin/security.rs` | aria-label, container, `adminInitTablesIn`, actions `data-no-sort` | **✅** |
| 966 | **PH-S1031** | Tenants + jobs tables | tenants/jobs | container, aria-label, striped, explicit init | **✅** |
| 967 | **PH-S1032** | Instances + topology tables | instances/topology | aria-label, container, `adminInitTablesIn` | **✅** |
| 968 | **PH-S1033** | Grid panel tables | network_profiles/seed/security_advisories | empty → `adminEmptyStateHtml`; table init | **✅** |
| 969 | **PH-S1034** | Raid artifacts table | `src/ui/admin/raid.rs` | container, aria-label, empty state | **✅** |
| 970 | **PH-S1035** | Modal form a11y | tenants/workers/vm/libs/security | `aria-required` + `aria-hidden` on `*` | **✅** |
| 971 | **PH-S1036** | Config + dashboard forms/tables | config/dashboard | `aria-required` on required fields; dashboard empty states | **✅** |
| 972 | **PH-S1037** | ui-core depth stub + docs | `admin_tables_forms_depth.rs` | depth enum + FM band 39 markers | **✅** |
| 973 | **PH-S1038** | Tables/forms band close | tests/docs | `galaxy_horizon_s1029_integration`; HANDOFF/NEXT | **✅** |

### 5.20 Admin tables/forms polish queue — band 39 (PH-S1029…S1038, 2026-07-18)

**Джерело:** project scan band 39 — FM-019 baseline adoption gaps (empty states, table aria-label, form `aria-required`).

| 954 | **PH-S1019** | Vision power menu polish | `docs/vision/` | dropdown shutdown/reboot; localStorage; a11y keyboard | **✅** |
| 955 | **PH-S1020** | Admin power modal polish | `src/ui/admin` | showModal parity; labeled Power btn; i18n UA/EN | **✅** |
| 956 | **PH-S1021** | Home UI power shortcut | `src/ui/` | `/ui` shell power entry → same ops API | **✅** |
| 957 | **PH-S1022** | Clippy unused imports batch | `src/grid` `src/network` | dispatch/grid/admin unused imports; `cargo clippy -D warnings` scope | **✅** |
| 958 | **PH-S1023** | chrono deprecations ui-core | `poolai-ui-core/format.rs` | `from_timestamp_opt` → `DateTime::from_timestamp` | **✅** |
| 959 | **PH-S1024** | admin mod.rs duplicate test attr | `src/ui/admin/mod.rs` | fix `duplicate_macro_attributes` on PH-S920 tests | **✅** |
| 960 | **PH-S1025** | Design tokens audit stub | UI_UX plan | `design_tokens` parity note + unit gate | **✅** |
| 961 | **PH-S1026** | Ops power feedback UX | admin+vision | toast/announce on power action; save last_run | **✅** |
| 962 | **PH-S1027** | poolai-msys.ps1 hardening | `bin/` | `-lc` + approved verbs; RUN_LOCAL note | **✅** |
| 963 | **PH-S1028** | UI debug band close | tests/docs | `galaxy_horizon_s1019_integration`; HANDOFF/NEXT | **✅** |

### 5.18 UI/debug polish queue — band 38 (PH-S1019…S1028, 2026-07-18)

**Джерело:** project scan після band 37 — power UX сирий, clippy warnings, Vision/Admin parity.

### 5.19 Maintenance horizon bands 38–47 (PH-S1019…S1118, 100 спринтів)

| Band | PH-S range | Фокус |
|------|------------|--------|
| **38** | S1019…S1028 | Power UX + clippy/ui-core hygiene ✅ |
| **39** | S1029…S1038 | Admin tables/forms polish (FM-019) ✅ |
| **40** | S1039…S1048 | Vision map/a11y/perf ✅ |
| **41** | S1049…S1058 | E2E visual/axe regression band ✅ |
| **42** | S1059…S1068 | OpenAPI/docs wire sync ✅ |
| 43 | S1069…S1078 | Grid metrics parity hardening ✅ |
| 44 | S1079…S1088 | wasm admin slim panels ✅ |
| 45 | S1089…S1098 | stand smoke + RUN_LOCAL ops ✅ |
| 46 | S1099…S1108 | ratio/rust migration advisory ✅ |
| 47 | S1109…S1118 | horizon close + STABLE touch-up ✅ |

**Не в scope:** FM-003 LAN · FM-041 Cloud SDK.

### 5.17 Post-maintenance owner queue — band 37 (PH-S1011…S1018, 2026-07-18)

**Джерело:** owner ops UX v2 — light launch, power controls, vision/admin power. **Band 37 ✅** drained.

| Тема | Спринти |
|------|---------|
| Легка збірка + запуск повного проєкту | PH-S1011, PH-S1012 |
| Легкий запуск Vision (README) | PH-S1013 |
| Збереження останніх параметрів | PH-S1014 |
| Кнопки poweroff/reset PoolAI | PH-S1015, PH-S1016 |
| Кнопки poweroff/reset Vision | PH-S1017 |
| Закриття смуги | PH-S1018 |

**Не в scope:** FM-003 LAN · FM-041 Cloud SDK · production host reboot без dev guard.

### 5.14 Master backlog PH-S720…S1010 (291 pending → product-complete, 2026-06-20)

**Призначення:** **completion roadmap v2** — конкретний шлях до **product-complete** (code + docs + ratio **95–96%**). **Не** дублювати в §5.12 — там max **10** `[ ]` активних.

| Поле | Значення |
|------|----------|
| **Pending** | **0** |
| **Drained bands 1–36** | PH-S660…S1010 ✅ |
| **Активна §5.12** | — (maintenance mode) |
| **Наступна promote** | — (owner scan only) |
| **Сесій `абракадабра`** | band 44 drained (PH-S1079…S1088) |
| **План фаз** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](../development/PH_S_COMPLETION_ROADMAP_2026-06-20.md) |
| **Реєстр sprint×acceptance** | [`PH_S_MASTER_BACKLOG_351.md`](../development/PH_S_MASTER_BACKLOG_351.md) |
| **Regen** | `bash scripts/generate-ph-s-master-backlog-351.sh` |

**Фази до S1010:**

| Фаза | Bands | Sprints | Фокус |
|------|-------|---------|--------|
| **A — Galaxy depth** | 7–14 | S720–S789 | §4 routing, §8 profile, §6.6 caps, §5 prefetch/locality, §8.2 payout, §1 fees, §9 governance |
| **B — wasm + wire** | 15–19 | S800–S849 | admin wasm slim panels, stand smoke v2, OpenAPI gap 0 |
| **C — Job/Memory/Solana** | 20–22 | S850–S879 | RAID job store, memory persist, on-chain cleared |
| **D — Production gates** | 23–26 | S880–S919 | verification lifecycle, replication quorum, pricing live, trust persist |
| **E — Ratio 95–96%** | 27–29 | S920–S949 | admin_charts → wasm, JS glue removal, ratio gates |
| **F — Docs complete** | 30–33 | S950–S989 | DIGEST, DOCS_LEGACY, Galaxy ✅ markers, STABLE |
| **G — Final verify** | 34–35 | S990–S1009 | integration gap fill, multi-module horizon close |
| **H — Closure** | 36 | S1010 | FM **§5.15** product-complete |

**Workflow promote (після drain + push):**

1. Закрити band у §5.12 → ✅
2. Взяти **наступні 10** з master backlog → §5.12 `[ ]` з **конкретним** Focus + Acceptance
3. Оновити HANDOFF + NEXT + GALAXY + completion roadmap zriz
4. **BLOCKED/Deferred** (FM-003 LAN, FM-041 Cloud SDK, ZK/TEE) — поза backlog

**Після PH-S1010 ✅:** FM **§5.15**; maintenance mode; новий scan лише для BLOCKED/Deferred або явного FM-horizon v2.

### 5.14b Enterprise master backlog PH-S1149…S2148 + completion extension → S2278 (2026-07-22)

**Призначення:** durable single-host enterprise 100% (**§5.17** @ S2148) + project-close extension (**§5.18** @ S2278). **Активний шлях від зараз:** **PH-S1339…S2278 = 940** спринтів. **Не** дублювати всі 940 у §5.12 — там max **10** `[ ]` активних.

| Поле | Значення |
|------|----------|
| **Pending (completion path)** | **900** (S1379…S2278) |
| **Enterprise subset pending** | **770** (S1379…S2148 → §5.17) |
| **Extension pending** | **130** (S2149…S2278 → §5.18) |
| **Drained** | band 51–73 PH-S1149…S1378 ✅ |
| **Активна §5.12** | band 74 **PH-S1379…S1388** `[ ]` (§5.55) |
| **План** | [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](../development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) · [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](../development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md) |
| **Реєстр** | [`PH_S_MASTER_BACKLOG_1000.md`](../development/PH_S_MASTER_BACKLOG_1000.md) |
| **Regen** | `bash scripts/generate-ph-s-master-backlog-1000.sh` · `bash scripts/generate-ph-s-completion-extension.sh` |
| **Closures** | FM **§5.17** @ PH-S2148 · FM **§5.18** @ PH-S2278 |

**Поза backlog:** FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE.

### 5.15 Product-complete closure (PH-S1010)

**Статус:** **✅ product-complete (PH-S1010)** — bands 1–36 drained; FM **§5.15** closed.

**Draft checklist (band 33 PH-S986):**

| Критерій | Зріз |
|----------|------|
| Galaxy concept gaps (§4–§9 wire) | ✅ roadmap + concept markers bands 7–32 |
| OpenAPI gap | ✅ `poolai-openapi-gap-audit` → **0** |
| Integration coverage | ✅ band 34 (PH-S990…S999) |
| Rust ratio | **≥95%** formal gate ✅ (`ratio_95_formal_gate_met`; stretch 96% advisory) |
| Docs canon | ✅ STABLE draft + INDEX zriz (band 33) |
| Vision | ✅ `poolai-vision-sync --check` after band close |
| Ops | `cargo test-ci` green; HANDOFF maintenance template ready |

**Acceptance (PH-S1010 — final):**

| Критерій | Перевірка |
|----------|-----------|
| Galaxy concept gaps (§4–§9 wire) | roadmap + concept ✅ markers |
| OpenAPI gap | `poolai-openapi-gap-audit` → **0** |
| Integration coverage | top FM gaps closed (band 34) |
| Rust ratio | **≥95%** formal; **96%** stretch advisory logged |
| Docs canon | STABLE «development complete»; INDEX + DIGEST synced |
| Vision | `poolai-vision-sync --check` green |
| Ops | `cargo test-ci` green; HANDOFF → maintenance mode |

**Поза product-complete (не блокує S1010):** FM-003 LAN 2-host · FM-041 Cloud SDK prod · ZK/TEE attestation roadmap.

### 5.17 Enterprise-complete closure (PH-S2148 target)

**Статус:** **in progress** — band 65 **active** (SSO stand smoke) · pending PH-S1289…S2148 · roadmap [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](../development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md) · completion wrapper [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](../development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md).

| Критерій | Ціль |
|----------|------|
| Tenants + quotas durable | restart-safe + isolation tests |
| SSO production path | SAML verify + OAuth fixtures |
| Durable audit | queryable + retention |
| Policies / secrets | persist + rotation wire |
| Monitoring durable | rules + dashboards when env set |
| OpenAPI enterprise | gap-audit **0** for `/api/enterprise/*` |
| Rust ratio | ≥95% hold; 96% met or advisory |
| Galaxy single-host | capability + network_profile + offline settlement |
| Governance | signed-release verify |
| Gates | `cargo test-ci` + vision `--check`; §5.17 ✅ at **PH-S2148** |

**Поза enterprise-complete:** FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE.

### 5.18 Project development complete (PH-S2278 target)

**Статус:** **planned** — extension bands 151–163 (Memory · Job depth · Solana · Wasm/UI · project close) after §5.17. Acceptance: STABLE/DIGEST truth, openapi-gap 0, `cargo test-ci` green, vision `--check`, ratio hold; HANDOFF → owner-scan only. **Поза:** FM-003 · FM-041 · ZK/TEE.

### 5.16 Service band (Cursor / toolchain / docs hygiene)

**Призначення:** **першочерговий** research + sync поза product PH-S* drain, коли власник просить service-сесію. **Не** замінює §5.12 product backlog; **не** блокує **`абракадабра`**.

| Sprint | Focus | Acceptance | Статус |
|--------|--------|------------|--------|
| **PH-SVC01** | Cursor 3.12.17 + toolchain research | `CURSOR_UPDATE_RESEARCH_2026-07-17.md` | **✅** |
| **PH-SVC02** | `cursor-environment-baseline.mdc` | 3.12.17 + git 2.50.0 | **✅** |
| **PH-SVC03** | `.cursor/CHANGELOG` + `poolai-agent-roles.mdc` | side chats + §5.16 pointer | **✅** |
| **PH-SVC04** | HANDOFF + NEXT_SESSION | cursor + service zriz | **✅** |
| **PH-SVC05** | README release/Next Focus | rev 297, §5.12 active 10 | **✅** |
| **PH-SVC06** | ENVIRONMENT_AND_CURSOR_UPDATES | pointer → Jul research | **✅** |
| **PH-SVC07** | `file_list.csv` | CURSOR_UPDATE_2026-07-17 row | **✅** |
| **PH-SVC08** | `poolai-vision-sync --check` | drift gate green | **✅** |
| **PH-SVC09** | INDEX + docs cross-links | INDEX zriz Jul 17 | **✅** |
| **PH-SVC10** | git push + самарі | service commit `main` | **✅** |
| **PH-SVC11** | Cursor 3.12.29 + changelog re-check | `CURSOR_UPDATE_RESEARCH_2026-07-21.md` | **✅** |
| **PH-SVC12** | `cursor-environment-baseline.mdc` | 3.12.29 + changelog 3.11 note | **✅** |
| **PH-SVC13** | `.cursor/CHANGELOG` + `poolai-agent-roles.mdc` | local-only side chats; mode picker | **✅** |
| **PH-SVC14** | HANDOFF + NEXT_SESSION | service zriz; next `абракадабра` | **✅** |
| **PH-SVC15** | README Next Focus | Cursor 3.12.29 service note | **✅** |
| **PH-SVC16** | ENVIRONMENT_AND_CURSOR_UPDATES | pointer → Jul 21 research | **✅** |
| **PH-SVC17** | `file_list.csv` | CURSOR_UPDATE_2026-07-21 row | **✅** |
| **PH-SVC18** | `poolai-vision-sync --check` | drift gate green | **✅** |
| **PH-SVC19** | INDEX + docs/README cross-links | zriz Jul 21 | **✅** |
| **PH-SVC20** | git push + самарі | service commit `main` | **✅** |
| **PH-SVC21** | Cursor 3.12.30 + changelog re-check | `CURSOR_UPDATE_RESEARCH_2026-07-22.md` | **✅** |
| **PH-SVC22** | `cursor-environment-baseline.mdc` | 3.12.30 + High Contrast note | **✅** |
| **PH-SVC23** | `.cursor/CHANGELOG` + `poolai-agent-roles.mdc` | pointer Jul 22 | **✅** |
| **PH-SVC24** | HANDOFF + NEXT_SESSION | service zriz; next `абракадабра` | **✅** |
| **PH-SVC25** | README Next Focus | Cursor 3.12.30 service note | **✅** |
| **PH-SVC26** | ENVIRONMENT_AND_CURSOR_UPDATES | pointer → Jul 22 research | **✅** |
| **PH-SVC27** | `file_list.csv` | CURSOR_UPDATE_2026-07-22 row | **✅** |
| **PH-SVC28** | Vision queue/feed enterprise bands | `poolai-vision-sync` merge `queue — band` | **✅** |
| **PH-SVC29** | INDEX + docs/README cross-links | zriz Jul 22 | **✅** |
| **PH-SVC30** | git push + самарі | service commit `main` | **✅** |

**Наступний service trigger:** після major Cursor/OS update або за запитом власника (повторити scan → §5.16 band).

### 5.13 Rust ratio band (дзеркало §5.12 PH-S150…S262)

Рядки **PH-S150…S262** у таблиці §5.12 вище — **єдина черга** (max 10 відкритих). §5.13 — тематичний індекс ratio/portability/wasm stretch + post-stretch maintain.

**Активна смуга (2026-07-23):** band 74 **PH-S1379…S1388** `[ ]` · §5.12 **10** · наступна **`абракадабра`** → drain band 74 (C Audit · admin/ops glue) · completion pending **900** → S2278.

**Ціль:** формально **90–95%** Rust у product code; **spirit 96%** — орієнтир replenish (більше Rust — краще).

### 5.3 Звірка «не зроблено» (менеджер функціоналу, 2026-05-18)

#### Зроблено в автопрогоні (червень 2026) — не повторювати

| Спринт | Результат | Коміти (орієнтир) |
|--------|-----------|-------------------|
| FM-019 modals | focus trap, `aria-modal`, `adminDynamicModal` | `02ea146`, `7d500db0` |
| FM-019 forms | `adminEnhanceFormA11y`, users/instances | `cf431a79` |
| FM-019 tabs/tables | tablist ARIA, `adminObserveDynamicA11y` | `d04088e8` |
| FM-019 baseline docs | `ADMIN_A11Y_RUNBOOK.md`, §5.4 | `31266be9` |
| S21–S24 | OpenAPI ai-ml; pa11y CI; Playwright smoke; dashboard DELETE | `fa96a6b4`…`56edbce9` |
| **S25** | UI_QUALITY P1 — tenants, OAuth2, dashboards contracts (+3 tests) | `2720c3d3` |
| **S26** | UI_QUALITY P1 close — metrics, alert-rules, SAML, policies (+4 tests) | `285b898d` |
| **S27** | Playwright admin E2E — tenants, monitoring (`admin.spec.ts`, `helpers.ts`) | `862cd016` |
| **S28** | OpenAPI gap audit — Users, workers, RAID, VM templates/networks | `46a299c5` |
| **S29** | Playwright — security + audit admin routes | `73a5e965` |
| **S30** | FM legacy docs audit + stale banners | (ця сесія) |
| **S31** | OpenAPI `/raid/distributed/*`; ML ops runbook; Playwright raid/topology | `c20a10f2` |
| **S32** | `bin/run-poolai.*` + `RUN_LOCAL.md` — єдиний локальний запуск | `ef4a0aa5` |
| **S33** | OpenAPI distributed DTO; axe Playwright; E2E vm/workers; шар A **100%** | `69873f7e` |
| **S34** | Docs harmonization A+B **100%**; Playwright libs; `data/dev/` gitignore | (ця сесія) |
| **S35** | FM-004 `turboquant-simd` feature + tests | Horizon |
| **S36** | FM-009 `GridEnvelope` v1 + map discovery/RAID | Horizon |
| **S37** | FM-010 `poolai-solana-adapter` crate + event schema v1 | Horizon |
| **S38** | P6 `src/job` + `src/memory` + `/api/v1/jobs` stub | Horizon |

#### Не зроблено (канон backlog) — оновлено 2026-05-20 (legacy audit)

| Джерело | Пункт | FM | Стан |
|---------|--------|-----|------|
| `OPENAPI_GAP_AUDIT` §залишок | VM network body schemas | **FM-032** | **✅** |
| `SOLANA_ADAPTER_CONCEPT` §8–9 | On-chain program + devnet RPC | **FM-033** | **✅** (`poolai-events`, `rpc/devnet.rs`) |
| `ARCHITECT_PLAN_EXO` §Real Model Loading | libtorch/onnx load | **FM-035** | **✅** (`model_loader.rs`) |
| `ARCHITECT_PLAN_EXO` §3.1–3.2 | Tensor sharding runtime | **FM-036** | **✅** |
| `ARCHITECT_PLAN_EXO` §4.1 | Topology graph viz | **FM-037** | **✅** |
| `NEXT_STEPS_2026-01-16` | OpenTelemetry tracing | **FM-038** | **✅** |
| `E2E_PLAYWRIGHT.md` | Playwright у main CI | **FM-039** | **✅** |
| `UI_QUALITY_AND_E2E_PLAN` §P1 | Admin field audit | **FM-040** | **✅** |
| `CLOUD_SDK_PROGRESS_2026-01-19` | GCP/Azure auth deep | **FM-041** | **Deferred** |
| `PERCENTAGE_PLAN` / P4 | Hot-path profiling | **FM-042** | **✅** |
| `NEXT_STEPS_ARCHITECT` L234 | Job scheduler → VM bind | **FM-034** | **✅** |
| Architect L123 | LAN replication + TQ01 на стенді | **FM-003** | **BLOCKED** |
| Architect L183 | Azure/GCP `cloud-sdk` | **✅ S39** | REST + mock tests; `CLOUD_SDK_STATUS.md` |
| FM-003 | §4 acceptance у runbook | **BLOCKED** | §5.1 dev stand ✅; ops **2026-06-01** |
| FM-004 | SIMD TurboQuant | **✅ S35** | `turboquant-simd` feature |
| FM-006 | cloud-sdk гілки | **✅ S39** | REST scope; `AZURE_LOCATION` |
| FM-009 | Grid envelope v1 | **✅ S36** | `src/grid/` |
| FM-010 | Solana adapter MVP | **✅ S37** | `crates/poolai-solana-adapter/` |
| FM-019 | pa11y/axe у CI | **Partial ✅** | S22: `ci.yml` `pa11y-wcag22` (paths-filter) + `pa11y-contract`; `PA11Y_WCAG22=1`; 18 auth URLs |
| P4 | `poolai_health_load` → `BENCHMARKS.md` | **Implemented (ops)** | рядок **2026-05-18**; baseline **2026-04-10** для порівняння |
| `UI_IMPROVEMENTS_PLAN` | Історичні `[ ]` | **Archived** | S4 2026-05-18; канон §5.4 + runbook §3.1 |
| `UI_BUGFIXES_AND_OAUTH_PLAN` | Модалки 2026-01 | **Archived** | S7 2026-05-18; FM-012/FM-019 канон |
| `CONCEPT_PENDING_FEATURES.md` | «ML не реалізовано» | **Archived** | S11 2026-05-18; канон STABLE + DIGEST |
| `HANDOFF` §5 | Посилання на AUTO_RUN 2026-05-17 | **Fixed 2026-06-07** | → AUTO_RUN 2026-06-08 |
| OpenAPI | Синхронізація при нових маршрутах | **✅** | S14–S33: distributed DTO schemas |
| UI E2E | Playwright | **✅** | S23–S33: admin routes + axe `a11y.spec.ts` |
| FM-019 | axe Playwright | **✅ S33** | `e2e/tests/a11y.spec.ts` |
| `docs/archive/*` | Legacy `.md` | **Archive** | [`STRUCTURE.md`](../STRUCTURE.md) |
| `STATUS_UPDATE_2026-01-16.md` | Cloud SDK `[ ]` | **Stale** | FM-006 Deferred; канон CI |
| `RUST_ARCHITECT_STATUS_2026-01-19.md` | BurstRAID metrics `[ ]` | **Stale** | опційно v0.2+ |
| `STABLE_STATE_UPDATE_2026-01-19.md` | % / cloud tests | **Stale** | → `STABLE_STATE_SUMMARY.md` |
| `PERCENTAGE_PLAN.md` | GlobalState % | **Stale** | не канон |
| `ADMIN_PANEL_STATUS.md` | Admin partial | **Stale** | код + `ADMIN_A11Y_RUNBOOK` |
| `UI_UX_IMPROVEMENTS_PLAN.md` | Monitoring UI | **Stale** | звірити `admin/monitoring.rs` |
| `RUST_ARCHITECT_NEXT_STEPS_2026-01-19.md` | Дубль плану | **Superseded** | → Architect 2026-03-17 |

#### Legacy docs (повна таблиця)

**Канон:** [`DOCS_LEGACY_AUDIT_2026-05-19.md`](../development/DOCS_LEGACY_AUDIT_2026-05-19.md). Історичний S13: [`AUTO_RUN_SESSION_2026-06-23.md`](../development/AUTO_RUN_SESSION_2026-06-23.md).

#### Рекомендований наступний спринт (2026-05-19, після S29)

Повний backlog — [`AUTO_RUN_SESSION_2026-07-01.md`](../development/AUTO_RUN_SESSION_2026-07-01.md), §4 у [`DOCS_LEGACY_AUDIT_2026-05-19.md`](../development/DOCS_LEGACY_AUDIT_2026-05-19.md).

| Порядок | Спринт | Фокус |
|--------|--------|--------|
| — | **S33** | ✅ шар A 100% |
| 1 | FM-003 §4 LAN | **BLOCKED** (2 хости) |
| — | Horizon S35–S40 | **✅** Layer C + project **100%** |
| — | **S21–S30** | ✅ (див. §5.3) |
| — | **FM-003 §4** | **BLOCKED** (2 хости) |

**Не стартувати без інфраструктури:** FM-003 §4 (2 хости).

### 5.6 Horizon — Layer C → 100% (2026-05-19)

**Канон:** [`HORIZON_TO_100_PLAN.md`](../development/HORIZON_TO_100_PLAN.md) · **черга:** [`AUTO_RUN_SESSION_2026_HORIZON.md`](../development/AUTO_RUN_SESSION_2026_HORIZON.md) · **промпт:** [`NEXT_SESSION_PROMPT.md`](../development/NEXT_SESSION_PROMPT.md).

| Спринт | FM / P6 | Статус |
|--------|---------|--------|
| S35 | FM-004 SIMD | [x] ✅ |
| S36 | FM-009 Grid envelope | [x] ✅ |
| S37 | FM-010 Solana adapter MVP | [x] ✅ |
| S38 | Job/Memory wire | [x] ✅ |
| S39 | FM-006 cloud-sdk | [x] ✅ |
| S40 | Layer C + project 100% docs | [x] ✅ |

### 5.5 Прогрес розробки (аудит менеджера функціоналу, 2026-05-19)

**Канонічний звіт:** [`DEVELOPMENT_PROGRESS_2026-05-19.md`](../status/DEVELOPMENT_PROGRESS_2026-05-19.md).

| Шар | % | Коментар |
|-----|---|----------|
| **A. Продукт (autoprogon)** | **100%** | FM-001…019 (S33) |
| **B. Architect P1–P5 (autoprogon)** | **100%** | S34; LAN §4 / cloud-sdk deep — ops/Deferred |
| **A+B autoprogon** | **100%** | офіційний зріз HANDOFF |
| **C. Horizon** | **100%** | S35–S40 ✅ |
| **Проєкт (A+B+C)/3** | **100%** | офіційний зріз S40 |

**Поза autoprogon:** FM-003 §4 LAN (**BLOCKED**).

**Baseline:** rust_ratio **94.36%** → **hold 95%** advisory · **stretch spirit 96%** ([`rust_ratio.json`](../development/rust_ratio.json), [`RUST_RATIO_STRATEGY_2026-06-13.md`](../development/RUST_RATIO_STRATEGY_2026-06-13.md)).

**Наступна сесія:** **`абракадабра`** (project scan) · [`NEXT_SESSION_PROMPT.md`](../development/NEXT_SESSION_PROMPT.md).

### 5.7 Post-Horizon backlog (2026-05-20)

**Канон:** [`AUTO_RUN_SESSION_2026_POST_HORIZON.md`](../development/AUTO_RUN_SESSION_2026_POST_HORIZON.md) · **Architect P6 залишок:** on-chain program (post FM-024 stub), VM network DTO (опційно).

| FM | Статус | Коментар |
|----|--------|----------|
| FM-020…023 | **✅** | jobs + memory + grid ingress |
| FM-024 | **✅** | devnet config + mock RPC (crate only) |
| FM-025 | **✅** | `VmTemplate` / `GpuSchedulingPolicy` OpenAPI |
| FM-026 | **✅** | `tests/jobs_api_contracts.rs` (4 tests) |
| FM-027 | **Prep ✅** | checklist + verify-lan-prep; §4 sign-off **BLOCKED** |
| FM-028 | **✅** | dual-port stand metrics in `BENCHMARKS.md` |
| FM-029 | **✅** | `job-store-sqlite`, `jobs.db`, migrate `jobs.json` |
| FM-030 | **✅** | `POOLAI_MONITORING_DATA_DIR`, metrics + config SQLite |
| FM-031 | **✅** | 21 pa11y auth URLs; axe Playwright admin matrix |

**Одна FM за автономну сесію** — commit + push MSYS2 з Summary.

### 5.8 Legacy backlog audit (2026-05-20)

**Метод:** менеджер функціоналу — звірка старіших планів (`RUST_ARCHITECT_*`, `PERCENTAGE_PLAN`, `UI_*_PLAN`, `CONCEPT_PENDING_FEATURES`, `OPENAPI_GAP_AUDIT`, `ARCHITECT_PLAN_EXO_INTEGRATION`, `PIPELINE_MANAGEMENT` future) з кодом і FM-001…031. **Не** брати `[ ]` з `docs/archive/` для автопрогону.

| Результат | Кількість |
|-----------|-----------|
| Нові тікети **FM-032…042** | 11 (розробка + Deferred FM-041) |
| **Stale / Already done** | GlobalState, ML modules, BurstRAID, distributed OpenAPI, axe Playwright — див. git log / §5.3 |
| **BLOCKED** | FM-003 §4 LAN (2 хости) |

**Канон пріоритетів розробки:** **§5.1** (порядок 1–11). **Джерела:** [`DOCS_LEGACY_AUDIT_2026-05-19.md`](../development/DOCS_LEGACY_AUDIT_2026-05-19.md), [`OPENAPI_GAP_AUDIT_2026-05-19.md`](../development/OPENAPI_GAP_AUDIT_2026-05-19.md).

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
