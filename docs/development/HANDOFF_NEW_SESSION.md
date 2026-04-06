# Передача контексту новій сесії (PoolAI)

**Оновлено:** 2026-04-06 (кроки 1–12; таксономія `docs/STRUCTURE.md`; ML integration tests під `required-features`; P2b wire harness)  
**Гілка роботи:** `main` (`git push origin main` → `origin/main`).

## 1. Канонічний порядок документації та планів

Той самий список, що в кореневому [`README.md`](../../README.md) (*Documentation map*) і [`docs/README.md`](../README.md) (*Canonical reading order*), кроки **1–12**.

| Крок | Що читати |
|------|-----------|
| 1 | Кореневий [`README.md`](../../README.md) — швидкий старт, збірка, CI, карта доків. |
| 2 | [`INDEX_2026-03-17.md`](../INDEX_2026-03-17.md) — навігація по всьому `docs/`. |
| 3 | [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](./NEXT_STEPS_ARCHITECT_2026-03-17.md) — **головний** план Rust Architect (P1–P6, TurboQuant). |
| 4 | **Цей файл** — [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md): гілка, git-push, зріз P2/P3, next steps. |
| 5 | Концепція: [`concept/poolAI_concept_root.txt`](../concept/poolAI_concept_root.txt), Grid/Memory/Job у `docs/concept/` та [`JOB_LAYER_CONCEPT_2026-03-17.md`](./JOB_LAYER_CONCEPT_2026-03-17.md). |
| 6 | Архітектура: [`ARCHITECTURE_REVIEW.md`](../ARCHITECTURE_REVIEW.md), [`ARCHITECTURE_BEST_PRACTICES.md`](../ARCHITECTURE_BEST_PRACTICES.md). |
| 7 | Продуктивність: [`performance/BENCHMARKS.md`](../performance/BENCHMARKS.md), [`performance/PROFILING.md`](../performance/PROFILING.md); опційно [`benchmarks.yml`](../../.github/workflows/benchmarks.yml). |
| 8 | CI: [`ci.yml`](../../.github/workflows/ci.yml). |
| 9 | Інвентар: [`file_list.csv`](../../file_list.csv) (оновлюй також `docs/catalog/` при зміні витягу); повний список: `git ls-files`. |
| 10 | Git push (Windows): [`.cursor/commands/git-push.md`](../../.cursor/commands/git-push.md). |
| 11 | Витяг функціоналу: [`catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`](../catalog/FUNCTIONALITY_DIGEST_2026-04-06.md). |
| 12 | Керування функціоналом: [`catalog/FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md); правило [`.cursor/rules/functionality-management.mdc`](../../.cursor/rules/functionality-management.mdc). |

Індекс планів у `docs/development/`: [`README.md`](./README.md). **Таксономія каталогу `docs/`:** [`../STRUCTURE.md`](../STRUCTURE.md). OpenAPI: [`docs/openapi.yaml`](../openapi.yaml). UI↔API: [`UI_QUALITY_AND_E2E_PLAN_2026-04-06.md`](./UI_QUALITY_AND_E2E_PLAN_2026-04-06.md). **Крок 11 / витяг функціоналу:** [`catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`](../catalog/FUNCTIONALITY_DIGEST_2026-04-06.md). **Крок 12 / беклог:** [`catalog/FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md). **Project skill (Cursor):** [`.cursor/skills/poolai-documentation/SKILL.md`](../../.cursor/skills/poolai-documentation/SKILL.md).

## 2. Git push (Windows / Cursor)

- **Канонічна інструкція:** [`.cursor/commands/git-push.md`](../../.cursor/commands/git-push.md) — MSYS2 UCRT64 **зовнішній** термінал, `PATH` з `~/.cargo/bin`, `K8S_OPENAPI_ENABLED_VERSION=1.28` за потреби cloud-sdk, формат коміта з Summary.
- Не робити `git add -A` без потреби; не стаджити `data/audit/*.log.gz`.
- Старі одноразові нотатки `PUSH_*.md` перенесені в [`docs/archive/`](../archive/); актуальні проблеми — [`docs/troubleshooting/`](../troubleshooting/).

## 3. Що вже зроблено (орієнтир для нової сесії)

- **`src/services/`**: `raid_service`, **`raid_distributed_protocol_service`** (distributed RAID JSON protocol; тонкий `raid_distributed_handlers.rs`), `vm_service`, `library_service`, **`instance_service`** (`/api/v1/instance/*`, `/state`), **`chat_completion_service`** (`/v1/chat/completions` — тонкий `completions.rs`), **`system_service`** (status/health/metrics/models/GPU, login, config get/update), **`ui_service`** (теми/компоненти + enterprise-дашборди через `EnterpriseService`), **`discovery_service`**, **`topology_service`**, **`worker_pool_service`**, **`rewards_service`** (`/api/v1/rewards/*`), `enterprise_service`, `cloud_service`, `admin_service` + `GET /api/v1/admin/overview` (`src/network/api/admin.rs`). HTML **`GET /api/v1/status`** — модуль **`network/api/system_status_html.rs`** (не в `SystemService`).
- **RaidService (P2)**: крім list — `put_artifact`, `delete_artifact`, `quota`, `cluster_status`; DTO квоти/статусу в `raid_service.rs`; тонкі handlers у `src/network/api/raid.rs`.
- **ML pipeline (Stage 4.4)**: детерміновані Rust-бекенди для `Preprocessing`, `Training`, `Evaluation`, `Deployment` (`src/ml/pipeline.rs`).
- **TurboQuant (P2b, фаза 1)**: `src/ml/turboquant.rs` (формат `TQ01`), інтеграція в крок `Quantization` за конфігом; див. `docs/ml/TURBOQUANT_INTEGRATION.md`.
- **Priority 3 (основний HTTP-шар)**: `src/network/api/common.rs` — `api_error_response`, **`api_json_error`**, `http_status_for_app_error`; `src/core/error.rs` — **`AppError::Forbidden`**, `ErrorContext` (+ `hint`). Узгоджені відповіді: **`raid_http.rs`** + **`raid.rs`** (мапінг `RaidServiceError` та спільні RAID JSON-помилки), **повний** **`enterprise_api.rs`** (хелпер **`enterprise_err`**), **`users`**, **`ui`**, **`system`**, **`completions`**, **`raid_admin`**, раніше — **`ai_ml`**, **instances/libraries/vm/workers/topology/rewards**, tenant CRUD, RAID `Operation` через `api_error_response`.
- **P3 (auth / WS / rate limit)**: **`auth.rs`**, **`ws.rs`** (upgrade + payload помилок), **`rate_limit.rs`** — узгоджено з **`api_json_error`** / **`ErrorContext`** (`src/network/json_errors.rs`); UI читає `error.message`.
- **Перевірка тестів (як CI)**: `K8S_OPENAPI_ENABLED_VERSION=1.28` + `cargo test --lib --tests --features ml,enterprise,cloud,test-utils` (інжектований `AppState`: **`tests/appstate_http_injection_integration.rs`** поряд з **`distributed_raid_wire_integration`**). На Windows при OOM лінкера: `cargo test ... -j 1 -- --test-threads=1`.

## 4. Наступні кроки за тим самим планом

1. **P4 (наступний горизонт)** — на **референс-машині**: повні прогони Criterion (усі чотири bench) та **`wrk`** на `/api/v1/health`; оновити таблицю baseline у `BENCHMARKS.md` під мітку хоста. **CI** вже є: `.github/workflows/benchmarks.yml` (`workflow_dispatch` + неділя 06:00 UTC, артефакт `criterion-report`). `service_layer_benchmarks`: `AppState::new` під **`rt.enter()`** (реалізовано).
2. **P2** — REST `/raid/*` через **`RaidService`**; RAID JSON-помилки — **`raid_http.rs`**. **`network/api/ui.rs`** тонкий: **`UiService`** + **`EnterpriseService`** для дашбордів. Distributed wire — **`RaidDistributedProtocolService`** + **`raid_distributed_handlers`**. **`POST /v1/chat/completions`** — **`ChatCompletionService`**. **`/api/v1/status`** (HTML) — розмітка в **`network/api/system_status_html.rs`**; JSON та інші системні точки — **`SystemService`** у **`system.rs`**. **`/api/v1/discovery/*`** — **`DiscoveryService`**; topology / workers / rewards — **`TopologyService`**, **`WorkerPoolService`**, **`RewardsService`** + **`AppState::rewards_engine`**. Далі за P2: довести критерії (усі handlers мінімальні; сервіси лише з `ApiContext`).
3. **P2b** — Criterion **`raid_replication_engine`** у `runtime_benchmarks`; in-tree harness: **`tests/distributed_raid_wire_integration.rs`** (`--features test-utils`, з `ml` — перевірка TQ01 vs raw на розмірі JSON); далі — повні заміри по мережі на стенді; також `tests/replication_benchmarks.rs` (інтеграційні таймінги control-plane).
4. За потреби — `cargo test --all-features` на Windows (`-j 1` при OOM лінкера).
5. **P5** — канонічні та архівні доки + **інвентар TODO у `src/`** (2026‑04-06) зафіксовані в [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](./NEXT_STEPS_ARCHITECT_2026-03-17.md); опційно — доробка Azure/GCP під `cloud-sdk`.

6. **P6** — концепти **Grid Protocol** [`GRID_PROTOCOL_CONCEPT_2026-04-06.md`](./GRID_PROTOCOL_CONCEPT_2026-04-06.md) та **Solana adapter** [`SOLANA_ADAPTER_CONCEPT_2026-04-06.md`](./SOLANA_ADAPTER_CONCEPT_2026-04-06.md); далі — прототип on-chain і Grid wire envelope.

Деталі й чекбокси — [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](./NEXT_STEPS_ARCHITECT_2026-03-17.md).
