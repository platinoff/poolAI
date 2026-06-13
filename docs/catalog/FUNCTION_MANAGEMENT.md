# Керування функціоналом PoolAI (індекс, прогалини, тікети)

**Оновлено:** 2026-06-13 (PH-S146 ✅ · §5.12 **7** відкритих PH-S147…S150 · vision rev 77 · **rust_ratio 91.97%**) · FM-041 Deferred).

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

**VDT / локальний CI:** одна сесія = один PH-S*; верифікація — `cargo test-ci` (+ scope: raft, openapi-gap, e2e). GitHub Actions — довідково. Правила — [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc). Якщо відкритих <3 — research у `docs/` і доповнити чергу (**max 10**).

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

**VDT:** якщо відкритих <3 — `rg "\- \[ \]" docs/development/NEXT_STEPS_ARCHITECT_*.md`, DOCS_LEGACY §5.3, `rg "TODO|FIXME" src/` → доповнити до **≤10** відкритих. **Rust-first:** API acceptance → `tests/`; Playwright лише browser scope; після S142 — replenish з **§5.13** ([`RUST_RATIO_STRATEGY_2026-06-13.md`](../development/RUST_RATIO_STRATEGY_2026-06-13.md)).

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
| 68 | **PH-S132** | network_profile contract docs (docs) | Galaxy §8 TBD #1 | §8.1 schema for `network_profile`; DIGEST row; locality subset cross-link | **✅** |
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
| 83 | **PH-S147** | wasm32 admin core POC (code) | PH-S146, portability | one wasm module + portability §2 docs sync | відкрито |
| 84 | **PH-S148** | Slim `e2e/` browser-only (e2e) | PH-S144 | `test:ci` без API TS patterns; ratio ≥90% | відкрито |
| 85 | **PH-S150** | Ratio CI advisory (ops) | PH-S143 audit | CI step warn if Rust share <88%; target 90% | відкрито |

**Відкритих у §5.12:** **7** (PH-S147…S150 Rust ratio band). **Закрито смуга:** PH-S128…S146 ✅. Vision rev 77. **Baseline:** rust_ratio **91.97%** ([`rust_ratio.json`](../development/rust_ratio.json)).

**Rust ratio 90–95%:** канон [`RUST_RATIO_STRATEGY_2026-06-13.md`](../development/RUST_RATIO_STRATEGY_2026-06-13.md) · **одна сесія = один PH-S*** · replenish після S150.

### 5.13 Rust ratio band (дзеркало §5.12 PH-S143…S150)

Рядки **PH-S143…S150** у таблиці §5.12 вище — **єдина черга** (max 10 відкритих). §5.13 лишається як тематичний індекс ratio/portability; не дублювати нові PH-S* поза §5.12.

**Ціль:** **90–95%** Rust у product code (`src/`, `tests/`, `crates/`, `src/bin/`).

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

**Наступна сесія:** **PH-S147** · черга **7** відкритих PH-S147…S150 · [`NEXT_SESSION_PROMPT.md`](../development/NEXT_SESSION_PROMPT.md).

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
