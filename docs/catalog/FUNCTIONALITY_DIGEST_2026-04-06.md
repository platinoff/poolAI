# PoolAI — витяг функціоналу (зведення за доками та кодом)

**Версія репозиторію:** 0.2.2 (`Cargo.toml`). **Оновлено:** 2026-05-18 (FM-016 ✅ virtual nodes + `poolai-worker`; FM-012/015 — див. **`FUNCTION_MANAGEMENT`** / **HANDOFF**).

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
| `raft` | Розподілений RAID / Raft (опційно). |
| `enterprise` | Мультитенантність, audit, monitoring, security (OAuth2/SAML), enterprise REST. |
| `ml` | ML-модулі, пайплайн, TurboQuant-гілка в квантизації тощо. |
| `cloud` | Хмарний модуль (автомасштабування, LB, K8s-обгортки) без повного SDK. |
| `cloud-sdk` | Важкі залежності (K8s OpenAPI, Azure, AWS SDK, GCP-частина). |
| `vm-isolation-linux` / `vm-isolation-windows` | Ізоляція VM на платформі. |
| `test-utils` | `AppState::attach_*_for_test` для тестів; приклад повного `/api/v1` без module globals — `tests/appstate_http_injection_integration.rs`. |

**Типова CI-матриця (див. `.github/workflows/ci.yml`):** `ml`, `enterprise`, `cloud` + `K8S_OPENAPI_ENABLED_VERSION=1.28` для збірки з cloud.

---

## Точки входу та процеси

| Компонент | Опис |
|-----------|------|
| **`poolai` (default-run)** | Основний сервер: HTTP(S), UI, REST, WebSocket, інтеграція модулів. |
| **`poolai-worker`** (`src/bin/poolai-worker.rs`) | **FM-016 ✅:** віртуальна нода на device — `POOLAI_COORDINATOR_URL`, реєстрація/heartbeat на coordinator, poll/complete tasks, bootstrap `ping` + `raid_health_check`, локальний `GET /health`. |
| **`poolai_health_load`** (`src/bin/poolai_health_load.rs`) | Дев-утиліта: навантажувальний **`GET /api/v1/health`** (Tokio + `reqwest`); опційно **`--json`** на stdout для baseline; див. `docs/performance/BENCHMARKS.md`. |

---

## Модулі ядра (`src/` — функціональні області)

| Область | Файли / пакет | Функціонал (за README та архітектурою) |
|---------|----------------|--------------------------------------|
| **Core** | `core/` | Конфіг, `AppState` / `ApiContext` (у т.ч. `rewards_engine` → `rewards::RewardSystem`), помилки (`AppError`, `ErrorContext`), користувачі, discovery-типи, WS-менеджер, інтерфейс моделі. |
| **Pool** | `pool/` | Пул воркерів, топологія, discovery-інтеграція, розміщення. |
| **Network** | `network/` | Axum: `/api/v1/*`, RAID REST (`api/raid.rs` + **`api/raid_http.rs`** + **`raid_admin.rs`**), enterprise API, auth, rate limit, WebSocket, distributed RAID handlers (`LeaveCluster`: при непорожньому membership залишати кластер може лише зареєстрований `node_id`). **`api/system.rs`**: **`POST /login`**, **`POST /refresh`** — у відповіді (JWT) опційно **`bootstrap_default_admin`** для UI першого входу. Узгоджені JSON-помилки: **`json_errors.rs`** — **`HttpAppError`**, **`AppError::RestError`**. FM-005 ✅: **`users`**, **`ui`**, **`ai_ml`**, **`workers`**, **`instances`**, **`libraries`**, **`vm`**, **`topology`**, **`rewards`**, **`system`**, **`completions`**, **`admin`**, **`raid*`** (**`raid_api_err`**), **`enterprise_api`**, **`authenticate_user`** / **`refresh_access_token`**, **`check_permission`**, **`auth_middleware`**. |
| **Platform** | `platform/` | GPU / апаратний рівень. |
| **Monitoring** | `monitoring/` | Метрики, context memory (ML-контекст). |
| **Runtime** | `runtime/` | Інстанси, планувальник, кеш, черги, процеси, сховище, оркестратор. |
| **Libs** | `libs/` | Реєстр бібліотек моделей, версіонування, залежності. |
| **VM** | `vm/` | Менеджер VM, ресурси, ізоляція (Linux/Windows за фічами). |
| **RAID** | `raid/` | Локальний і розподілений RAID, протокол, реплікація, BurstRAID, SmallWorld, події, snapshot, адмін-стратегії. |
| **Enterprise** | `enterprise/` | Тенанти, audit, monitoring, security (OAuth2, SAML, політики). |
| **Cloud** | `cloud/` | Провайдери (AWS/Azure/GCP), Kubernetes manager, operator, autoscaling, load balancing (повна поведінка з `cloud-sdk`). |
| **ML** | `ml/` | Оптимізація, AutoML, federated, pruning, pipeline, versioning, experiments, TurboQuant (`turboquant.rs`, формат TQ01). |
| **Rewards** | `rewards/` | Система нагород / прогресу; процесовий `shared_reward_engine()` (`OnceLock<Arc<RewardSystem>>`), узгоджений із `AppState`. |
| **UI** | `ui/` | Вбудована веб-адмінка (дашборди, теми, доступність). **FM-012 ✅:** i18n **UA/EN**, `/ui/auth`, enterprise **admin**, Telegram OAuth (HMAC/`auth_date`/allowlist/audit, widget UA/EN). Мапінг JSON адмінки → екран: `docs/development/ADMIN_UI_JSON_CONTRACTS.md`. |
| **Services** | `services/` | `RaidService`, `RaidDistributedProtocolService`, `VmService`, `LibraryService`, `InstanceService`, `ChatCompletionService`, `SystemService`, `UiService` (каталог UI + делегування enterprise-дашбордів), `DiscoveryService`, `TopologyService`, `WorkerPoolService`, `RewardsService`, `EnterpriseService`, `CloudService`, `AdminService`, **`VirtualNodeTaskService`** (FM-016) — оркестрація для HTTP. |
| **TGBot** | `tgbot/` | **FM-016++:** `coordinator` bridge + `poolai-telegram-bot` (`--features tgbot`); OAuth login — FM-012. |

---

## HTTP / API (узагальнено)

- **REST під `/api/v1/`** — модульні роутери в `src/network/api/` (`system`, `workers`, `vm`, `raid`, `raid_admin`, `libraries`, `users`, `rewards`, `instances`, `completions`, `topology`, `discovery`, **`virtual_nodes`**, `ui`, `admin`, …). Див. `create_api_routes()` у `api/mod.rs`.
- **FM-016 virtual nodes** — `POST /api/v1/discovery/register-remote`, `heartbeat-remote`, `GET /discovery/virtual-nodes`; `GET/POST /api/v1/virtual-nodes/{id}/tasks/*`, probe health; тести `discovery_remote_register_integration`, `virtual_node_tasks_integration`.
- **FM-016+ Telegram** — `POST/GET/DELETE /api/v1/virtual-nodes/telegram/bind*`, `POST .../telegram/webhook` → task на bound `peer_id`; env: `POOLAI_VIRTUAL_NODE_DATA_DIR`, `POOLAI_TELEGRAM_WEBHOOK_SECRET`, worker `POOLAI_TELEGRAM_ID`.
- **FM-016+++** — `POST /virtual-nodes/{id}/pool/join`; bootstrap tasks + `raid_artifact_probe`; worker `POOLAI_WORKER_CACHE_DIR`, health `cached_artifacts`; `bin/verify-dev-stand.*` e2e.
- **RAID** — додаткові шляхи під `/raid/…` (артефакти, воркери, події, snapshot, GC, strategies, metrics, rebalance, health) через `raid.rs`.
- **Enterprise** — при `feature enterprise`: маршрути в **`src/network/enterprise_api/`** (`mod.rs` + tenants, audit, monitoring, security, oauth, saml).
- **ML enterprise** — при `enterprise` + `ml`: `/api/enterprise/ai-ml/…` (пайплайн), див. `ai_ml.rs`.
- **WebSocket** — наприклад `/ws/metrics` (JWT/безпека залежно від конфігурації).
- **UI/Admin UX** — FM-012 закрито (2026-05-16): i18n UA/EN + Telegram OAuth; LAN perf — FM-003 ops ([`LAN_BENCHMARK_RUNBOOK.md`](../performance/LAN_BENCHMARK_RUNBOOK.md)).
- **OpenAPI** — [`docs/openapi.yaml`](../openapi.yaml) описує **частину** публічних шляхів; повний перелік — з коду роутерів і `src/network/mod.rs`.

---

## ML (Stage 4.4 — за доками)

| Елемент | Стан (за README / HANDOFF) |
|---------|----------------------------|
| ML.1–ML.6 каркас у `src/ml` | Є (оптимізація, AutoML, federated, context memory, versioning, experiments, pipeline). |
| TurboQuant | Фаза 1 у коді (`turboquant.rs`, крок pipeline); див. [`docs/ml/TURBOQUANT_INTEGRATION.md`](../ml/TURBOQUANT_INTEGRATION.md). |
| Hardening / ops | ✅ Runbook метрик + `cargo test-ci` — [`docs/ml/PIPELINE_MANAGEMENT.md`](../ml/PIPELINE_MANAGEMENT.md) §Ops verification (2026-05-19). |

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
| [`concept/POOLAI_GRID_NODE.md`](../concept/POOLAI_GRID_NODE.md) | Вузол грида, ролі, модулі. |
| [`concept/POOLAI_MEMORY_LAYER.md`](../concept/POOLAI_MEMORY_LAYER.md) | Memory layer, зв’язок RAID/ML. |
| [`development/JOB_LAYER_CONCEPT_2026-03-17.md`](../development/JOB_LAYER_CONCEPT_2026-03-17.md) | Job / mining layer, життєвий цикл job. |
| [`development/GRID_PROTOCOL_CONCEPT_2026-04-06.md`](../development/GRID_PROTOCOL_CONCEPT_2026-04-06.md) | Grid protocol: типи повідомлень, Discovery/RAID/тести. |
| [`development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md`](../development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md) | Solana adapter: core vs on-chain, події Job/Memory. |

### Horizon wire-шар (код, S35–S38)

| Модуль / crate | Призначення | HTTP / wire |
|----------------|-------------|-------------|
| `src/grid/` | `GridEnvelope` v1 — Job, Result, MemoryShard, PeerStatus | JSON; map ↔ discovery/RAID |
| `src/job/` | `JobSpec`, `JobStatus`, lifecycle types | `POST/GET /api/v1/jobs` (stub) |
| `src/memory/` | `MemoryShardRef` — shards поверх RAID | Grid `memory_shard` |
| `src/ml/turboquant.rs` | TurboQuant + optional `turboquant-simd` | ML pipeline Quantization |
| `crates/poolai-solana-adapter/` | Domain events v1, NDJSON sidecar | без `solana-sdk` у `poolai` |

---

## Безпека та спостережуваність (за доками)

- JWT, HTTPS, RBAC, rate limiting, security headers — див. кореневий README та `docs/security/*`.
- Audit, алерти, метрики enterprise — `enterprise/` + доки `monitoring/`.

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
