# PoolAI — витяг функціоналу (зведення за доками та кодом)

**Оновлено:** 2026-08-02 (GSV реалізовано — band 102 `PH-S1659…S1668` ✅).

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
| **`poolai-worker`** (`src/bin/poolai-worker.rs`) | **FM-016 ✅:** віртуальна нода — register-remote/heartbeat, tasks, RAID health; lease renew ticker (PH-S116). |
| **`poolai-verify-release`** (`src/bin/poolai_verify_release.rs`) | **PH-S66 ✅:** ed25519 release manifest + SHA-256 artifact; Galaxy §9.2. |
| **`poolai_health_load`** (`src/bin/poolai_health_load.rs`) | Load generator `GET /api/v1/health`; `--json` baseline — [`BENCHMARKS.md`](../performance/BENCHMARKS.md). |
| **`poolai-openapi-gap-audit`** (`src/bin/poolai_openapi_gap_audit.rs`) | **PH-S841 ✅:** Axum routes vs `docs/openapi.yaml`; exit 0 = 0 missing (PH-S954). |
| **`poolai-http-stand-smoke`** (`src/bin/poolai_http_stand_smoke.rs`) | Live stand HTTP smoke + metrics parity; replaces legacy Playwright API specs (PH-S145+). |
| **`poolai-e2e-stand`** (`src/bin/poolai_e2e_stand.rs`) | E2E stand start/restart/stop lifecycle (PH-S158). |
| **`poolai-loc-audit`** (`src/bin/poolai_loc_audit.rs`) | Rust ratio LOC audit → `docs/development/rust_ratio.json` (PH-S143+; band zriz PH-S955). |
| **`poolai-vision-sync`** (`src/bin/poolai_vision_sync.rs`) | `docs/vision/manifest.json` merge + FM §5.12 sprint queue; `--check` drift gate (PH-S957). |
| **`poolai-p2b-tq01-snapshot`** (`src/bin/poolai_p2b_tq01_snapshot.rs`) | Deterministic TQ01 wire size snapshot (`--features ml`; FM-028). |
| **`poolai-telegram-bot`** (`src/bin/poolai-telegram-bot.rs`) | Telegram bot sidecar (`--features tgbot`); coordinator bridge (FM-016++). |
| **`poolai-solana-adapter`** (`crates/poolai-solana-adapter/src/bin/`) | NDJSON domain events → mock/devnet RPC ack (FM-024/033). |

**GSV (Galaxy StarWalker Vision)** — окремий Rust-first проєкт у `GSV/` (vision migration): bin-сервер `gsv-server` (axum 0.8, SSE `/events`, single-page UI) з боксами **Tracker · SLI console · Toolchain · IDE · Update/offline · Box preview · SLI terminal · Tests/bench hooks**. Rust 95–100% / wasm 0–5%. **52 tests green, clippy 0.** Канон: [`GSV/README.md`](../../GSV/README.md) · docs [`docs/gsv/`](../gsv/README.md) · **TechPreroadMap** [`docs/gsv/GSV_TECH_ROADMAP.md`](../gsv/GSV_TECH_ROADMAP.md) · band 102 `PH-S1659…S1668` **✅** (FM §5.12 §5.83).

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
- **OpenAPI** — [`docs/openapi.yaml`](../openapi.yaml) синхронізовано з Axum routes (PH-S840/S841); **`cargo run --bin poolai-openapi-gap-audit`** → **0 missing**; контракти — `tests/grid_openapi_contracts.rs` (PH-S954); band 42 — `memory_api_contracts.rs` (PH-S1061), stand smoke `ops_power_openapi` (PH-S1062).

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

### Job layer — модулі в коді (PH-S951, `src/job/`)

| Модуль | Шлях | Функція |
|--------|------|---------|
| **Types** | `src/job/types.rs` | `JobRecord`, `JobSpec`, `JobStatus`, lease epoch CAS |
| **Store** | `src/job/store.rs` | JSON `JobStore`, `POOLAI_JOB_DATA_DIR` |
| **Store SQLite** | `src/job/store_sqlite.rs` | `feature job-store-sqlite` → `jobs.db` |
| **Store depth** | `src/job/store_depth.rs` | RAID persist depth stub + metrics (PH-S854) |
| **Scheduler** | `src/job/scheduler.rs` | `schedule_with_grid_peer`, worker/vm bind |
| **Lifecycle** | `src/job/lifecycle.rs` | state transition guards |
| **Lease config** | `src/job/lease_config.rs` | TTL / renew interval from env (PH-S97/S111) |
| **Lease acquire** | `src/job/lease_acquire.rs` | acquire/renew lease on record |
| **Lease failover** | `src/job/lease_failover.rs` | worker-unhealthy, queue starvation, max runtime (PH-S524+) |
| **Map** | `src/job/map.rs` | Grid envelope ↔ job spec/result |
| **Domain events** | `src/job/domain_events.rs` | NDJSON epics (`JobCompleted`, …) |
| **On-chain** | `src/job/onchain.rs` | emit hooks → `POOLAI_ONCHAIN_EVENTS_DIR` |

### UI / WASM crates (PH-S952)

| Crate / модуль | Шлях | Функція |
|----------------|------|---------|
| **poolai-ui-core** | `crates/poolai-ui-core/` | Shared admin validators, formatters, panel HTML builders |
| **admin_common_depth** | `crates/poolai-ui-core/src/admin_common_depth.rs` | table/empty wasm depth stub (PH-S930) |
| **admin_wasm_slim_depth** | `crates/poolai-ui-core/src/admin_wasm_slim_depth.rs` | wasm panel depth classification band 44 (PH-S1086) |
| **charts_depth** | `crates/poolai-ui-core/src/charts_depth.rs` | sparkline/line wasm depth (PH-S924) |
| **stretch_depth** | `crates/poolai-ui-core/src/stretch_depth.rs` | ratio 96% stretch band stub (PH-S944) |
| **digest_depth** | `crates/poolai-ui-core/src/digest_depth.rs` | FUNCTIONALITY_DIGEST band inventory (PH-S950) |
| **grid panels** | `grid_verification.rs`, `grid_replication_pricing.rs`, `updates_compat.rs`, … | wasm-first admin Galaxy strips |
| **poolai-ui-wasm** | `crates/poolai-ui-wasm/` | `wasm32` exports wrapping ui-core for browser admin glue (PH-S147+) |
| **build** | `bin/build-ui-wasm.sh` | wasm build gate for admin panels |

### Galaxy Grid — модулі в коді (PH-S950, `src/grid/` + wire)

Концепт: [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md). Протокол envelope: [`GRID_PROTOCOL_CONCEPT_2026-04-06.md`](../development/GRID_PROTOCOL_CONCEPT_2026-04-06.md). Повний інвентар — `digest_depth::GRID_MODULE_STEMS` (57 stems).

| Модуль | Шлях | Функція |
|--------|------|---------|
| **Grid envelope** | `src/grid/envelope.rs` | `GridEnvelope` v1, `GridMessage`, `validate()` |
| **Grid map** | `src/grid/map.rs` | map ↔ `PeerInfo`, RAID artifacts, memory shard bodies |
| **Grid dispatch** | `src/grid/dispatch.rs` | `ingest_envelope`, prefetch hooks, strict locality gate |
| **Protocol compat** | `src/grid/protocol_compat.rs` | coordinator↔worker matrix `negotiate()` |
| **Fee split** | `src/grid/galaxy_fee_split.rs` | primary 0.1% + secondary admin bps |
| **Fee split depth** | `src/grid/galaxy_fee_split_depth.rs` | fee-split depth stub + wire |
| **Fee split metrics** | `src/grid/galaxy_fee_split_metrics.rs` | JSON↔Prom `galaxy_fee_split_applied_total` |
| **Pricing oracle** | `src/grid/galaxy_pricing_oracle.rs` | L1 cache + L2 fallback + provider catalog |
| **Pricing depth** | `src/grid/galaxy_pricing_depth.rs` | pricing depth stub |
| **Pricing metrics** | `src/grid/galaxy_pricing_metrics.rs` | oracle served/fresh/stale counters |
| **Pricing provider metrics** | `src/grid/galaxy_pricing_provider_metrics.rs` | provider timeout / forced-fallback metrics |
| **Settlement** | `src/grid/galaxy_settlement.rs` | Cleared / payout gate core |
| **Settlement mode** | `src/grid/galaxy_settlement_mode.rs` | offline vs on-chain mode gate |
| **Settlement metrics** | `src/grid/galaxy_settlement_metrics.rs` | settlement counters on grid result |
| **Settlement on-chain** | `src/grid/galaxy_settlement_onchain.rs` | mock RPC submit on Cleared (PH-S568+) |
| **Settlement on-chain depth** | `src/grid/galaxy_settlement_onchain_depth.rs` | on-chain depth stub |
| **Payout batch queue** | `src/grid/galaxy_settlement_payout_batch_queue.rs` | offline payout batch queue |
| **Payout depth** | `src/grid/galaxy_settlement_payout_depth.rs` | payout depth stub |
| **Payout metrics** | `src/grid/galaxy_settlement_payout_metrics.rs` | payout-batch JSON↔Prom parity |
| **Replication** | `src/grid/galaxy_replication.rs` | replication quorum helpers |
| **Replication depth** | `src/grid/galaxy_replication_depth.rs` | replication depth stub |
| **Replication metrics** | `src/grid/galaxy_replication_metrics.rs` | replication rate / quorum metrics |
| **Replication quorum gate** | `src/grid/galaxy_replication_quorum_gate.rs` | strict-tier digest quorum before Cleared |
| **Locality** | `src/grid/galaxy_locality.rs` | `locality_score`, scheduler rank stub |
| **Locality metrics** | `src/grid/galaxy_locality_metrics.rs` | hot-tier promote/evict metrics |
| **Locality hot-tier depth** | `src/grid/galaxy_locality_hot_tier_depth.rs` | hot-tier depth stub |
| **Prefetch depth** | `src/grid/galaxy_prefetch_depth.rs` | prefetch policy depth stub |
| **Prefetch metrics** | `src/grid/galaxy_prefetch_metrics.rs` | pull bytes / backpressure counters |
| **Prefetch peer pull** | `src/grid/galaxy_prefetch_peer_pull.rs` | live seed shard pull wire |
| **Trust score** | `src/grid/galaxy_trust_score.rs` | payout eligibility gate stub |
| **Trust persist depth** | `src/grid/galaxy_trust_persist_depth.rs` | SQLite trust persist depth |
| **Trust score store** | `src/grid/galaxy_trust_score_store.rs` | in-memory / JSON trust store |
| **Trust score store SQLite** | `src/grid/galaxy_trust_score_store_sqlite.rs` | persisted trust scores (PH-S910) |
| **Verification metrics** | `src/grid/galaxy_verification_metrics.rs` | mismatch / sample totals |
| **Verification lifecycle depth** | `src/grid/galaxy_verification_lifecycle_depth.rs` | checker lifecycle depth |
| **Verification checker jobs** | `src/grid/galaxy_verification_checker_jobs.rs` | shadow checker job submit |
| **Verification replay** | `src/grid/galaxy_verification_replay.rs` | replay record + history wire |
| **Verify sampling** | `src/grid/galaxy_verify_sampling.rs` | base sample rate middleware |
| **Replay jobs** | `src/grid/galaxy_replay_jobs.rs` | replay job enqueue |
| **Replay metrics** | `src/grid/galaxy_replay_metrics.rs` | replay pending resolved counters |
| **Capability doc** | `src/grid/galaxy_capability_doc.rs` | ed25519 telegram_edge capability |
| **Capability admission** | `src/grid/galaxy_capability_admission.rs` | signed capability gate |
| **Capability admission depth** | `src/grid/galaxy_capability_admission_depth.rs` | admission depth stub |
| **Capability admission metrics** | `src/grid/galaxy_capability_admission_metrics.rs` | unsigned rejected counter |
| **Governance metrics** | `src/grid/galaxy_governance_metrics.rs` | release verify + update notify gauges |
| **Governance depth** | `src/grid/galaxy_governance_depth.rs` | governance depth stub |
| **Update policy** | `src/grid/galaxy_update_policy.rs` | `GET /grid/update-policy` env snapshot |
| **Network profile** | `src/grid/galaxy_network_profile.rs` | parse `metadata.network_profile` |
| **Network profile store** | `src/grid/galaxy_network_profile_store.rs` | persisted profiles hydrate |
| **Network profile depth** | `src/grid/galaxy_network_profile_depth.rs` | profile depth stub |
| **Worker health** | `src/grid/galaxy_worker_health.rs` | unhealthy peer signals for scheduler |
| **Worker DTO** | `src/grid/galaxy_worker_dto.rs` | worker health DTO helpers |
| **Routing policy** | `src/grid/galaxy_routing_policy.rs` | locality routing policy gate |
| **Re-migrate policy** | `src/grid/galaxy_re_migrate_policy.rs` | re-migrate prefetch trigger |
| **Security advisory** | `src/grid/galaxy_security_advisory.rs` | security advisory helpers |
| **Fraud proof** | `src/grid/galaxy_fraud_proof.rs` | fraud proof stub |
| **Protocol negotiation metrics** | `src/grid/galaxy_protocol_negotiation_metrics.rs` | compat negotiation counters |
| **Solana depth** | `src/grid/solana_depth.rs` | settlement on-chain depth wire |
| **Stand smoke parity** | `src/grid/stand_smoke_metrics_parity.rs` | JSON↔Prom parity band helpers; **v3** extended pairs (band 43 PH-S1069…S1078) |

**Wire / API (Galaxy cross-ref):**

| Область | Шлях | Примітка |
|---------|------|----------|
| Virtual nodes API | `src/network/api/virtual_nodes.rs`, `discovery.rs` | register-remote, tasks, Telegram bind |
| Grid pricing API | `src/network/api/grid.rs` | `GET /api/v1/grid/pricing` (PH-S78+) |
| Job lease wire | `src/network/api/jobs.rs` | acquire/renew/PATCH CAS |
| Signed release | `src/release/`, `poolai-verify-release` | ed25519 manifest (PH-S66) |
| OTel lease spans | `src/observability/lease_trace.rs` | PH-S124/S126 |
| Prometheus export | `src/observability/prometheus_export.rs` | `GET /metrics` (FM-043) |

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

**Band 30 digest sync (PH-S950…S959):** повний інвентар `src/grid/` (57 stems), `src/job/`, `crates/poolai-ui-{core,wasm}`, `src/bin/` ops bins; OpenAPI gap audit **0** (PH-S954); rust_ratio zriz PH-S955; master backlog band 31 → PH-S960…S969 — [`PH_S_MASTER_BACKLOG_351.md`](../development/PH_S_MASTER_BACKLOG_351.md) · [`RUST_RATIO_STRATEGY_2026-06-13.md`](../development/RUST_RATIO_STRATEGY_2026-06-13.md) · [`GALAXY_GRID_ROADMAP_2026-05-27.md`](../development/GALAXY_GRID_ROADMAP_2026-05-27.md).

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
