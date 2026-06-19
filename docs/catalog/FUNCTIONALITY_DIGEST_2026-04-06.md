# PoolAI — витяг функціоналу (зведення за доками та кодом)

**Оновлено:** 2026-06-19 (PH-S524…S533: lease failover health, governance metrics, settlement wire, wasm charts slim).

Цей документ — **не автогенерація з коду**, а структурований **витяг можливостей** системи, узгоджений з кореневим [`README.md`](../../README.md), [`docs/status/STABLE_STATE_SUMMARY.md`](../status/STABLE_STATE_SUMMARY.md), [`docs/development/HANDOFF_NEW_SESSION.md`](../development/HANDOFF_NEW_SESSION.md), модулями `src/` та (частково) [`docs/openapi.yaml`](../openapi.yaml). Для повного переліку HTTP-шляхів див. роутери в `src/network/` — OpenAPI може відставати від фактичного API.

---

## Канонічний порядок читання документації

Узгоджено з **кроками 1–12** у [`README.md`](../../README.md), [`docs/README.md`](../README.md), [`docs/INDEX_2026-03-17.md`](../INDEX_2026-03-17.md). **Цей файл — крок 11** (каталог / витяг функціоналу). **Крок 12** — [`FUNCTION_MANAGEMENT.md`](./FUNCTION_MANAGEMENT.md) (беклог, прогалини, тікети). **Таксономія папок `docs/` і правила агента:** [`docs/STRUCTURE.md`](../STRUCTURE.md), [`.cursor/rules/documentation.md`](../../.cursor/rules/documentation.md).

---

## Підсистеми та feature-прапорці (`Cargo.toml`)

| Feature | Призначення |
|---------|-------------|
| `jwt` | JWT для API (потрібен нативний toolchain на Windows GNU). |
| `https` | TLS (axum-server). |
| `raft` | Розподілений RAID / Raft (`async-raft`): `RaidRaftNode`, HTTP transport (`raft_transport.rs`), inbound RPC (`network/api/raft_rpc.rs` — `/raft/*`); `AppState::raft_node` + `GET /api/v1/raid/status` → `raft_status`; тести — `cargo test-raft-ci`. |
| `enterprise` | Мультитенантність, audit, monitoring, security (OAuth2/SAML), enterprise REST. |
| `ml` | ML-модулі, пайплайн, TurboQuant-гілка в квантизації тощо. |
| `cloud` | Хмарний модуль (автомасштабування, LB, K8s-обгортки) без повного SDK. |
| `cloud-sdk` | Важкі залежності (K8s OpenAPI, Azure, AWS SDK, GCP-частина). |
| `vm-isolation-linux` / `vm-isolation-windows` | Ізоляція VM на платформі. |
| `test-utils` | `AppState::attach_*_for_test` для тестів; приклад повного `/api/v1` без module globals — `tests/appstate_http_injection_integration.rs`. |
| `prometheus` | Pull-model `GET /metrics` (text exposition); див. [`PROMETHEUS_METRICS.md`](../development/PROMETHEUS_METRICS.md). |
| `otel` | OpenTelemetry OTLP export + W3C trace context; див. [`OPENTELEMETRY_TRACING.md`](../development/OPENTELEMETRY_TRACING.md). |

**Типова CI-матриця (див. `.github/workflows/ci.yml`):** `ml`, `enterprise`, `cloud`, `prometheus`, `job-store-sqlite` + `K8S_OPENAPI_ENABLED_VERSION=1.28` для збірки з cloud; локально — **`cargo test-ci`** (див. `.cargo/config.toml`); Raft wire/harness — **`cargo test-raft-ci`** (`raft`, `test-utils`).

---

## Точки входу та процеси

| Компонент | Опис |
|-----------|------|
| **`poolai` (default-run)** | Основний сервер: HTTP(S), UI, REST, WebSocket, інтеграція модулів. |
| **`poolai-worker`** (`src/bin/poolai-worker.rs`) | **FM-016 ✅:** віртуальна нода на device — `POOLAI_COORDINATOR_URL`, реєстрація/heartbeat на coordinator, poll/complete tasks, bootstrap `ping` + `raid_health_check`, локальний `GET /health`. **PH-S65:** шле `protocol_version` / `build_id` на register-remote (`POOLAI_PROTOCOL_VERSION`, `POOLAI_BUILD_ID`). |
| **`poolai-verify-release`** (`src/bin/poolai_verify_release.rs`) | **PH-S66 ✅:** перевірка підписаного release manifest (ed25519) + опційна SHA-256 artifact; `src/release/`; Galaxy §9.2 — [`SECURITY_HARDENING.md`](../security/SECURITY_HARDENING.md). |
| **`poolai_health_load`** (`src/bin/poolai_health_load.rs`) | Дев-утиліта: навантажувальний **`GET /api/v1/health`** (Tokio + `reqwest`); опційно **`--json`** на stdout для baseline; див. `docs/performance/BENCHMARKS.md`. |

---

## Модулі ядра (`src/` — функціональні області)

| Область | Файли / пакет | Функціонал (за README та архітектурою) |
|---------|----------------|--------------------------------------|
| **Core** | `core/` | Конфіг, `AppState` / `ApiContext` (у т.ч. `rewards_engine` → `rewards::RewardSystem`), помилки (`AppError`, `ErrorContext`), користувачі, discovery-типи, WS-менеджер, інтерфейс моделі. |
| **Pool** | `pool/` | Пул воркерів, топологія, discovery-інтеграція, розміщення. |
| **Network** | `network/` | Axum: `/api/v1/*`, RAID REST (`api/raid.rs` + **`api/raid_http.rs`** + **`api/raid_rpc.rs`** [`feature raft`] + **`raid_admin.rs`**), enterprise API, auth, rate limit, WebSocket, distributed RAID handlers (`LeaveCluster`: при непорожньому membership залишати кластер може лише зареєстрований `node_id`). **`api/system.rs`**: **`POST /login`**, **`POST /refresh`** — у відповіді (JWT) опційно **`bootstrap_default_admin`** для UI першого входу. Узгоджені JSON-помилки: **`json_errors.rs`** — **`HttpAppError`**, **`AppError::RestError`**. FM-005 ✅: **`users`**, **`ui`**, **`ai_ml`**, **`workers`**, **`instances`**, **`libraries`**, **`vm`**, **`topology`**, **`rewards`**, **`system`**, **`completions`**, **`admin`**, **`raid*`** (**`raid_api_err`**), **`enterprise_api`**, **`authenticate_user`** / **`refresh_access_token`**, **`check_permission`**, **`auth_middleware`**. |
| **Platform** | `platform/` | GPU / апаратний рівень. |
| **Monitoring** | `monitoring/` | Метрики, context memory (ML-контекст). |
| **Runtime** | `runtime/` | Інстанси, планувальник, кеш, черги, процеси, сховище, оркестратор. |
| **Libs** | `libs/` | Реєстр бібліотек моделей, версіонування, залежності. |
| **VM** | `vm/` | Менеджер VM, ресурси, ізоляція (Linux/Windows за фічами). **PH-S03:** write lifecycle + RBAC — `tests/vm_api_contracts.rs`; admin modal create/delete — `e2e/tests/admin.spec.ts`. |
| **RAID** | `raid/` | Локальний і розподілений RAID, протокол, реплікація, BurstRAID, SmallWorld, події, snapshot, адмін-стратегії. **PH-S04…S06 (`feature raft`):** `RaidRaftNode`, `HttpRaftTransport`, cluster status (`RaidService::cluster_status` → `raft_status`), multi-node harness (`tests/raft_multi_node_harness.rs`). |
| **Enterprise** | `enterprise/` | Тенанти, audit, monitoring, security (OAuth2, SAML, політики). |
| **Cloud** | `cloud/` | Провайдери (AWS/Azure/GCP), Kubernetes manager, operator, autoscaling, load balancing (повна поведінка з `cloud-sdk`). |
| **ML** | `ml/` | Оптимізація, AutoML, federated, pruning, pipeline, versioning, experiments, TurboQuant (`turboquant.rs`, формат TQ01). |
| **Rewards** | `rewards/` | Система нагород / прогресу; процесовий `shared_reward_engine()` (`OnceLock<Arc<RewardSystem>>`), узгоджений із `AppState`. |
| **UI** | `ui/` | Вбудована веб-адмінка (дашборди, теми, доступність). **FM-012 ✅:** i18n **UA/EN**, `/ui/auth`, enterprise **admin**, Telegram OAuth. **FM-045 ✅:** `design_tokens.css` + уніфіковані `admin-table` / `admin-form` / `adminRenderTable` / `adminFormFieldHtml` — [`DESIGN_SYSTEM.md`](../development/DESIGN_SYSTEM.md). **PH-S42 ✅:** admin table sort/filter/export toolbar, `adminEmptyStateHtml`, auto-init (`adminInitTablesIn`). **PH-S43 ✅:** `/ui/admin/monitoring` ML step metrics panel + Run ML Demo (`admin_charts.js`, `admin/monitoring.rs`). **PH-S05:** `/ui/admin/raid` — `#raid-cluster-status`. **PH-S24/S37b:** `/ui/admin/security` — вкладка **Secret rotation** (`#security-tab-rotation`); API `GET /api/v1/admin/secrets/rotation`, `POST /api/v1/admin/secrets/rotate`. **PH-S11…S13:** Playwright visual regression — [`VISUAL_REGRESSION_E2E.md`](../development/VISUAL_REGRESSION_E2E.md), workflow [`update-visual-baselines.yml`](../../.github/workflows/update-visual-baselines.yml). |
| **Observability** | `observability/` | HTTP trace spans; **FM-038** OTLP (`otel`); **FM-043** Prometheus scrape at **`GET /metrics`**; **`poolai_secret_rotations_total{kind,success}`** (PH-S24/S29, `record_secret_rotation`). |
| **Services** | `services/` | `RaidService`, `RaidDistributedProtocolService`, `VmService`, `LibraryService`, `InstanceService`, `ChatCompletionService`, `SystemService`, `UiService` (каталог UI + делегування enterprise-дашбордів), `DiscoveryService`, `TopologyService`, `WorkerPoolService`, `RewardsService`, `EnterpriseService`, `CloudService`, `AdminService`, **`VirtualNodeTaskService`** (FM-016) — оркестрація для HTTP. |
| **TGBot** | `tgbot/` | **FM-016++:** `coordinator` bridge + `poolai-telegram-bot` (`--features tgbot`); OAuth login — FM-012. |

---

## HTTP / API (узагальнено)

- **REST під `/api/v1/`** — модульні роутери в `src/network/api/` (`system`, `workers`, `vm`, `raid`, `raid_admin`, `libraries`, `users`, `rewards`, `instances`, `completions`, `topology`, `discovery`, **`virtual_nodes`**, `ui`, `admin`, …). Див. `create_api_routes()` у `api/mod.rs`.
- **FM-016 virtual nodes** — `POST /api/v1/discovery/register-remote`, `heartbeat-remote`, `GET /discovery/virtual-nodes`; `GET/POST /api/v1/virtual-nodes/{id}/tasks/*`, probe health; тести `discovery_remote_register_integration`, `virtual_node_tasks_integration`.
- **PH-S65 Galaxy protocol** — `register-remote` з `protocol_version` / `build_id` → `src/grid/protocol_compat.rs` (`accepted` \| `upgrade_required` \| `unsupported`; HTTP 403/426); env `POOLAI_COORDINATOR_PROTOCOL_VERSION`, worker `POOLAI_PROTOCOL_VERSION`.
- **FM-016+ Telegram** — `POST/GET/DELETE /api/v1/virtual-nodes/telegram/bind*`, `POST .../telegram/webhook` → task на bound `peer_id`; `POST .../telegram/wallet` payout pubkey stub (PH-S131, Galaxy §3.2); env: `POOLAI_VIRTUAL_NODE_DATA_DIR`, `POOLAI_TELEGRAM_WEBHOOK_SECRET`, worker `POOLAI_TELEGRAM_ID`.
- **FM-016+++** — `POST /virtual-nodes/{id}/pool/join`; bootstrap tasks + `raid_artifact_probe`; worker `POOLAI_WORKER_CACHE_DIR`, health `cached_artifacts`; `bin/verify-dev-stand.*` e2e.
- **RAID** — додаткові шляхи під `/raid/…` (артефакти, воркери, події, snapshot, GC, strategies, metrics, rebalance, health) через `raid.rs`; **`GET /api/v1/raid/status`** — `RaidStatusResponse` з опційним **`raft_status`** (role, term, leader_id).
- **Raft RPC (`feature raft`)** — `POST /raft/append-entries`, `/raft/vote`, `/raft/install-snapshot` (`raft_rpc.rs`; використовується `HttpRaftTransport` і PH-S06 harness).
- **Enterprise** — при `feature enterprise`: маршрути в **`src/network/enterprise_api/`** (`mod.rs` + tenants, audit, monitoring, security, oauth, saml).
- **ML enterprise** — при `enterprise` + `ml`: `/api/enterprise/ai-ml/…` (пайплайн), див. `ai_ml.rs`.
- **WebSocket** — наприклад `/ws/metrics` (JWT/безпека залежно від конфігурації).
- **Prometheus (FM-043)** — `GET /metrics` (root, `feature = prometheus`); enterprise gauges on scrape; JSON metrics лишаються на `/api/v1/metrics`.
- **HTTPS (FM-044)** — `feature = https`: rustls TLS 1.3 (опційно 1.2), HSTS з config, `HTTPS_CERT_RELOAD_SECS` — [`security/TLS.md`](../security/TLS.md).
- **UI/Admin UX** — FM-012 закрито (2026-05-16): i18n UA/EN + Telegram OAuth; LAN perf — FM-003 ops ([`LAN_BENCHMARK_RUNBOOK.md`](../performance/LAN_BENCHMARK_RUNBOOK.md)).
- **OpenAPI** — [`docs/openapi.yaml`](../openapi.yaml) описує **частину** публічних шляхів; повний перелік — з коду роутерів і `src/network/mod.rs`.

---

## ML (Stage 4.4 — за доками)

| Елемент | Стан (за README / HANDOFF) |
|---------|----------------------------|
| ML.1–ML.6 каркас у `src/ml` | Є (оптимізація, AutoML, federated, context memory, versioning, experiments, pipeline). |
| TurboQuant | Фаза 1 у коді (`turboquant.rs`, крок pipeline); див. [`docs/ml/TURBOQUANT_INTEGRATION.md`](../ml/TURBOQUANT_INTEGRATION.md). |
| Hardening / ops | ✅ Runbook метрик + dev stand (`verify-dev-stand` ML demo) — [`docs/ml/PIPELINE_MANAGEMENT.md`](../ml/PIPELINE_MANAGEMENT.md) §Ops (PH-S17, 2026-05-24). |

---

## Cloud (Stage 4.3 — за доками)

| Елемент | Стан (за README) |
|---------|------------------|
| Модульна структура, K8s, провайдери, autoscaling, LB | Інфраструктура та тести — є. |
| Повний SDK / operator у продакшн-глибину | Планова доробка (`cloud-sdk`). |

---

## Концепція (вісі продукту)

| Документ | Зміст |
|----------|--------|
| [`concept/poolAI_concept_root.txt`](../concept/poolAI_concept_root.txt) | Головна концепція PoolAI. |
| [`concept/POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md) | **Galaxy Grid** (федерація srvN): fees, lease, Telegram seats, locality, governance §9, compat matrix — канон продукту. |
| [`concept/POOLAI_GRID_NODE.md`](../concept/POOLAI_GRID_NODE.md) | Вузол грида, ролі, модулі. |
| [`concept/POOLAI_MEMORY_LAYER.md`](../concept/POOLAI_MEMORY_LAYER.md) | Memory layer, зв’язок RAID/ML. |
| [`development/JOB_LAYER_CONCEPT_2026-03-17.md`](../development/JOB_LAYER_CONCEPT_2026-03-17.md) | Job / mining layer, життєвий цикл job. |
| [`development/GRID_PROTOCOL_CONCEPT_2026-04-06.md`](../development/GRID_PROTOCOL_CONCEPT_2026-04-06.md) | Grid protocol: типи повідомлень, Discovery/RAID/тести. |
| [`development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md`](../development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md) | Solana adapter: core vs on-chain, події Job/Memory. |
| [`vision/manifest.json`](../vision/manifest.json) | **Galaxy map** — вузли `crate_solana`, `job_onchain`, sidecar/program (PH-S120); **Eco/perf UI** PH-S125; `bin/open-docs-vision.ps1`. |

### Solana adapter — модулі (FM-010 / FM-024 / FM-033, PH-S120)

Канон концепту: [`SOLANA_ADAPTER_CONCEPT_2026-04-06.md`](../development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md) · FM **FM-033** ✅ · карта: [`docs/vision/`](../vision/) (`solana_concept` → `crate_solana` → sidecar/program).

| Модуль | Шлях | Функція | Wire |
|--------|------|---------|------|
| **Domain events (core)** | `src/job/domain_events.rs`, `src/job/onchain.rs` | NDJSON epics (`JobCompleted`, …); `POOLAI_ONCHAIN_EVENTS_DIR`; **без** `solana-sdk` у `poolai` | stdout/file → sidecar stdin |
| **Adapter crate** | `crates/poolai-solana-adapter/` | Events v1, `SidecarProcessor`, mock RPC (FM-024), devnet submit (FM-033) | TOML config + env |
| **Sidecar binary** | `poolai-solana-adapter` | NDJSON line in → RPC ack JSON out | `tail -f data/onchain/events.ndjson \| poolai-solana-adapter` |
| **On-chain program** | `program/poolai-events/` | BPF `PoolAiInstruction` (Borsh); deploy via Solana CLI | devnet only |
| **Wire limits** | `wire/limits.rs`, `src/wire_limits.rs` | Shared limits adapter + BPF (PH-S46) | — |

**Перевірка:** `cargo test -p poolai-solana-adapter -p poolai-events -j 1` (не повний `test-ci`, якщо main `src/` не змінювався).

### Horizon wire-шар (код, S35–S38)

| Модуль / crate | Призначення | HTTP / wire |
|----------------|-------------|-------------|
| `src/grid/` | `GridEnvelope` v1 — Job, Result, MemoryShard, PeerStatus (див. **Galaxy Grid modules** нижче) | JSON; map ↔ discovery/RAID |
| `src/job/` | `JobStore`, scheduler, lifecycle; `lease_config` / `lease_acquire` (PH-S97–S99); persistence JSON / SQLite (`FM-029`) / RAID (`PH-S48`); optional lease fields (PH-S94); PATCH CAS (PH-S95) | `GET/POST /api/v1/jobs`, `GET/PATCH /jobs/{id}`, `POST /jobs/schedule`, `POST /jobs/{id}/lease`, `POST /jobs/{id}/lease/renew` |
| `src/memory/` | `MemoryShardRef` — shards поверх RAID | Grid `memory_shard` |
| `src/ml/turboquant.rs` | TurboQuant + optional `turboquant-simd` | ML pipeline Quantization |
| `crates/poolai-solana-adapter/` | Events v1, sidecar, mock RPC (FM-024), `poolai-events` + devnet submit (FM-033) | Solana deps лише в sidecar crate |

### Galaxy Grid — модулі в коді (PH-S67, `src/grid/` + wire)

Концепт: [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md). Протокол envelope: [`GRID_PROTOCOL_CONCEPT_2026-04-06.md`](../development/GRID_PROTOCOL_CONCEPT_2026-04-06.md).

| Модуль | Шлях | Функція | Тести / утиліти |
|--------|------|---------|-----------------|
| **Grid envelope** | `src/grid/envelope.rs` | `GridEnvelope` v1, `GridMessage` (Job/Result/MemoryShard/PeerStatus), `validate()` | `envelope` unit tests |
| **Grid map** | `src/grid/map.rs` | map ↔ `PeerInfo`, RAID `put_artifact`, memory shard bodies | `map` unit tests |
| **Grid dispatch** | `src/grid/dispatch.rs` | `ingest_envelope` → `JobStore` / `MemoryShardStore`; epics `emit_memory_updated`, `emit_seed_provided`; schedule via `schedule_with_grid_peer` | `dispatch` unit tests |
| **Galaxy fee split** | `src/grid/galaxy_fee_split.rs` | primary **0.1%** (10 bps) + secondary **1–5%** admin (floor bps); `GalaxyFeeSplit` lamports | unit tests; `cargo bench --bench galaxy_fee_split_benchmarks` |
| **Pricing oracle (stub + L2 fallback)** | `src/grid/galaxy_pricing_oracle.rs` | unit keys; TTL/SWR cache (L1 fresh/stale); L2 fallback + FORCE_FALLBACK; `POOLAI_GALAXY_PRICING_PROVIDERS` catalog (PH-S92); in-process metrics fresh/stale/forced_fallback (PH-S81/S83/S91); Prometheus gauges on `GET /metrics` (PH-S127) | unit tests (`galaxy_pricing_oracle`) |
| **Protocol compat** | `src/grid/protocol_compat.rs` | matrix coordinator↔worker `1.x`; `negotiate()` на register-remote; `CompatStatus` + docs URL | unit tests; `tests/discovery_remote_register_integration.rs` |
| **Network profile** | `src/grid/galaxy_network_profile.rs`, `galaxy_network_profile_store.rs`, `discovery.rs` | parse `metadata.network_profile` on register-remote (§8.1); startup hydrate persisted profiles (PH-S529); canonical JSON in peer metadata | unit tests; `tests/discovery_network_profile_integration.rs`, `tests/network_profile_hydrate_integration.rs` |
| **Signed capability doc** | `src/grid/galaxy_capability_doc.rs` | ed25519 capability documents for telegram_edge; `expires_at` enforcement (PH-S527) | `tests/discovery_telegram_edge_capability_integration.rs` |
| **Governance metrics** | `src/grid/galaxy_governance_metrics.rs`, `src/release/verify.rs` | Prometheus `poolai_release_verify_*`, `poolai_update_notify_pending` (Galaxy §9.2, PH-S528) | unit + `/metrics` scrape |
| **Worker health** | `src/grid/galaxy_worker_health.rs` | peer health signals for scheduler bind / failover (PH-S524…S525) | `tests/jobs_scheduler_unhealthy_integration.rs`, `tests/jobs_worker_unhealthy_failover_integration.rs` |
| **Verify sampling** | `src/grid/galaxy_verify_sampling.rs`, `verify_sampling_middleware.rs` | `POOLAI_GALAXY_VERIFY_BASE_SAMPLE_RATE` (§6.2, default `0.05`); middleware header + result ingest stub (PH-S142/S164) | unit + integration tests |
| **Virtual nodes API** | `src/network/api/virtual_nodes.rs`, `discovery.rs` | register-remote/heartbeat, tasks, Telegram bind/webhook, pool join | `virtual_node_*_integration` |
| **Virtual node services** | `src/services/virtual_node_task_service.rs`, `virtual_node_telegram_binding_service.rs` | task queue, Telegram seat bind (FM-016+) | integration tests |
| **Signed release** | `src/release/`, `poolai-verify-release` | ed25519 manifest verify + artifact SHA-256 (PH-S66) | `release::verify` unit tests |
| **Grid pricing API** | `src/network/api/grid.rs` | `GET /api/v1/grid/pricing` (task/model/unit); oracle from `galaxy_pricing_oracle` (PH-S78…S83) | `grid.rs` + `galaxy_pricing_oracle` tests |
| **Job lease wire** | `src/job/types.rs`, `lease_config.rs`, `lease_acquire.rs`, `lease_failover.rs`, `src/network/api/jobs.rs` | TTL env; acquire/renew; worker-unhealthy + queue-starvation failover (PH-S524/S530); max runtime cap `POOLAI_JOB_MAX_TOTAL_RUNTIME_SECS` (PH-S526); PATCH CAS → `409 lease_epoch_rejected` | `lease_tests`, `jobs_api_contracts`, `jobs_failover_budget_integration` |
| **Worker lease ticker** | `src/bin/poolai-worker.rs` | `LeaseRenewGuard` → periodic `POST /jobs/{id}/lease/renew`; payload `job_id` + `lease_epoch`; env `POOLAI_JOB_LEASE_RENEW_INTERVAL_SECS` (PH-S116, Galaxy §4.3.1.1) | `cargo test --bin poolai-worker` |
| **OTel lease spans** | `src/observability/lease_trace.rs` | `job.lease.acquire` / `renew` / `reject` + `job.lease.*` attrs; wired store/jobs/grid/dispatch (PH-S126); contract PH-S124 | `observability_otel` (`--features otel`) |
| **Prometheus export** | `src/observability/prometheus_export.rs` | `GET /metrics` pull model (FM-043); galaxy pricing + governance gauges (PH-S127/S528) | `observability_prometheus` tests |

**Admin UI (Galaxy ops, read-only):**

| Сторінка | Шлях | Спринт |
|----------|------|--------|
| Grid pricing | `/ui/admin/grid-pricing` | PH-S82 |
| Updates & compatibility | `/ui/admin/updates-compat` | PH-S93 |
| Jobs + lease columns | `/ui/admin/jobs` | PH-S53, PH-S96, PH-S105, PH-S141 |

**Env (Galaxy wire, орієнтир):**

| Змінна | Де | Призначення |
|--------|-----|-------------|
| `POOLAI_PROTOCOL_VERSION` | worker | Wire protocol на register-remote (default `1.2`) |
| `POOLAI_BUILD_ID` | worker | Build id у register payload |
| `POOLAI_COORDINATOR_PROTOCOL_VERSION` | coordinator | Рядок matrix для compat (default `1.2`) |
| `POOLAI_VIRTUAL_NODE_DATA_DIR` | coordinator | Персистентні tasks/bindings |
| `POOLAI_RELEASE_TRUST_ROOT` | ops | `maintainer_keys.json` для `poolai-verify-release` (концепт §9.2) |
| `POOLAI_GALAXY_PRICE_CACHE_TTL_SECS` | coordinator | Pricing oracle fresh TTL (default `300`, §4.2) |
| `POOLAI_GALAXY_PRICE_MAX_STALE_SECS` | coordinator | Pricing oracle stale-while-revalidate (default `3600`) |
| `POOLAI_GALAXY_PRICING_FALLBACK_JSON` | coordinator | L2 fixed fallback quote map by unit key (usd_micro JSON) for provider outage |
| `POOLAI_GALAXY_PRICING_FORCE_FALLBACK` | coordinator | `1` = L2-only emergency mode (`pricing_forced_fallback` log; PH-S81) |
| `POOLAI_GALAXY_PRICING_PROVIDERS` | coordinator | JSON allow-list provider catalog (PH-S92) |
| `POOLAI_JOB_LEASE_TTL_SECS` | coordinator | Default lease TTL seconds (default `90`; `JobLeaseConfig::from_env()`, PH-S97) |
| `POOLAI_JOB_LEASE_RENEW_INTERVAL_SECS` | coordinator | Optional renew interval override (default `ttl/3`, capped at TTL; PH-S111) |
| `POOLAI_JOB_MAX_TOTAL_RUNTIME_SECS` | coordinator | Wall-clock cap on total job runtime before forced fail (PH-S526) |
| `POOLAI_JOB_QUEUE_STARVATION_SECS` | coordinator | Queue starvation failover threshold on `leased_at` (PH-S530) |
| `POOLAI_GALAXY_LOCALITY_MODE` | coordinator | Prefetch/locality strictness: `best_effort` (default) or `strict_locality` (PH-S136) |
| `POOLAI_GALAXY_PREFETCH_DEADLINE_MS` | coordinator | Max prefetch wait before Running (default `15000`; PH-S136) |
| `POOLAI_GALAXY_MIN_TRUST_PAYOUT` | coordinator | Min `trust_score` 0–100 for `telegram_edge` auto payout gate (default `40`; PH-S130) |
| `POOLAI_GALAXY_VERIFY_BASE_SAMPLE_RATE` | coordinator | Base `telegram_edge` verification sample rate 0.0..=1.0 (default `0.05`; PH-S142) |

**Pricing/lease wire (PH-S94…S127):** lease MVP + grid CAS + worker ticker + E2E negatives (PH-S107…S118); renew-interval env (PH-S111); OTel lease span contract + instrumentation (PH-S124/S126); pricing oracle Prometheus gauges (PH-S127). **Migrating lifecycle E2E (PH-S133):** `e2e/tests/jobs_migrating.spec.ts` — Playwright PATCH `leased → migrating → executing` + `executing ↔ migrating` roundtrip (PH-S104 wire); `npm run test:ci` includes `jobs_migrating`. **Protocol middleware E2E (PH-S134):** `e2e/tests/protocol_middleware.spec.ts` — Playwright `POST /discovery/register-remote` with `X-PoolAI-Protocol` (accepted compat headers; unsupported → 403 `protocol_unsupported`); `npm run test:ci` includes `protocol_middleware`. **Locality (PH-S128/S138):** `src/grid/galaxy_locality.rs` — `locality_score` + `rank_workers_by_locality` scheduler stub; integration fixture `tests/galaxy_locality_rank_integration.rs` (PH-S138); no prefetch wire. **Prefetch (PH-S129/S136):** `src/grid/dispatch.rs` — `SeedInventoryEntry` DTO + `plan_prefetch` / `noop_prefetch_hook`; `PrefetchPolicyConfig::from_env()` (`POOLAI_GALAXY_LOCALITY_MODE`, `POOLAI_GALAXY_PREFETCH_DEADLINE_MS`); unit tests; no live enqueue wire. **Trust gate (PH-S130/S137):** `src/grid/galaxy_trust_score.rs` + `dispatch.rs` result path — `trust_score` 0–100 settlement gate stub (`PayoutEligible` / `PayoutHeld` / `NotApplicable`); optional `metrics.trust_score` on grid result; Prometheus gauges `galaxy_trust_payout_eligible_total` / `galaxy_trust_payout_held_total` (PH-S137); unit tests; no payout wire. **Wallet bind (PH-S131):** `virtual_node_telegram_wallet_service.rs` + `POST /api/v1/virtual-nodes/telegram/wallet` — `telegram_user_id` + `chat_id` + `payout_pubkey` (`chain=solana`); stub `verified=true`; integration tests; no on-chain wire. **Wallet GET (PH-S135):** `GET /api/v1/virtual-nodes/telegram/wallets/{telegram_user_id}` — read-only lookup; OpenAPI `getTelegramWallet`; 404 when unbound; `poolai-openapi-gap-audit` 0. **Wallet bind E2E (PH-S139):** `e2e/tests/telegram_wallet.spec.ts` — Playwright POST wallet verified bind + invalid pubkey → 400; `npm run test:ci` includes `telegram_wallet`. **Network profile (PH-S132/S140):** `POOLAI_GALAXY_GRID.md` §8.1 — `network_profile` wire schema (`region`, `latency_ms_p50`, `bandwidth_mbps`, `egress_policy`, SmallWorld hints); cross-link §5.2 locality subset → `src/grid/galaxy_locality.rs` (`LocalityNetworkProfile`). **Register-remote parse (PH-S140):** `src/grid/galaxy_network_profile.rs` — parse `metadata.network_profile` on `POST /api/v1/discovery/register-remote` (object or JSON string); canonical JSON in peer metadata; `400` on invalid region; `tests/discovery_network_profile_integration.rs`. **Verification metrics (PH-S175…S177):** `galaxy_verification_metrics.rs` + `galaxy_replay_metrics.rs` — mismatch / sample total / replay pending on grid result → `/metrics` via `refresh_galaxy_verification_gauges`; integration tests `galaxy_verification_*_integration.rs`, `galaxy_replay_pending_integration.rs`. **Черга §5.12 (10 відкритих PH-S178…S187):** settlement/replication/verification match/pricing market min/trust score/locality-prefetch metrics stubs — FM §5.12 · [`NEXT_SESSION_PROMPT.md`](../development/NEXT_SESSION_PROMPT.md). [`RUST_RATIO_STRATEGY_2026-06-13.md`](../development/RUST_RATIO_STRATEGY_2026-06-13.md). Роадмеп: [`GALAXY_GRID_ROADMAP_2026-05-27.md`](../development/GALAXY_GRID_ROADMAP_2026-05-27.md).

---

## Безпека та спостережуваність (за доками)

- JWT, HTTPS (**FM-044** TLS 1.3 + cert reload), RBAC, rate limiting, security headers — [`security/TLS.md`](../security/TLS.md), кореневий README.
- **FM-043** Prometheus pull — [`PROMETHEUS_METRICS.md`](../development/PROMETHEUS_METRICS.md) (galaxy pricing gauges PH-S127); **FM-038** OTLP — [`OPENTELEMETRY_TRACING.md`](../development/OPENTELEMETRY_TRACING.md) (job lease spans PH-S124/S126).
- Audit, алерти, метрики enterprise — `enterprise/` + `POOLAI_MONITORING_DATA_DIR` (FM-030).

---

## Як підтримувати цей витяг

1. Після значних змін у **модулях** або **публічному API** — оновити відповідні рядки таблиць і розділ HTTP.
2. Після нових **кроків у канонічному порядку** доків — оновити посилання вгорі та [`README.md`](../../README.md) (крок 11).
3. **Інвентар:** додати шлях до цього файлу в кореневий [`file_list.csv`](../../file_list.csv).

---

## Див. також

- [`FUNCTION_MANAGEMENT.md`](./FUNCTION_MANAGEMENT.md) — керування функціоналом, індекс vs сталевий стан, чернетки тікетів `FM-*`, **§5.1 — пріоритезовані наступні кроки** (крок 12).
- [`docs/INDEX_2026-03-17.md`](../INDEX_2026-03-17.md) — повна карта `docs/`.
- [`docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md`](../development/NEXT_STEPS_ARCHITECT_2026-03-17.md) — архітектурний беклог P1–P6.
