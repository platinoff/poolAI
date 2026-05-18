# Патерни автономної розробки PoolAI

**Призначення:** реєстр **конкретних** повторюваних рішень для наступних сесій авторозробки. Оркестратор доповнює цей файл після P0 (збір) і S6 (закриття).

**Оновлено:** 2026-06-02 (§5.3 audit + FM-019 dashboard nav).

---

## Як додавати запис

```markdown
### [AREA] Коротка назва
- **Де:** `path/to/file.rs:line`
- **Сигнал:** rg / grep / умова
- **Патерн:** 1–3 речення
- **Перевірка:** команда cargo / тест
- **FM:** FM-xxx (якщо є)
```

---

## HTTP / помилки

### [HTTP] Узгоджений JSON помилок
- **Де:** `src/network/json_errors.rs:47-109`
- **Сигнал:** `rg "HttpAppError|AppError::RestError" src/network` → **216** збігів у 17 файлах (2026-05-17)
- **Патерн:** `api_error_response` → `{ "error": { "code", "message" }, "context"?: ... }`; UI читає `error.message`
- **Перевірка:** `rg "HttpAppError" src/network/api`
- **FM:** FM-005 ✅

### [HTTP] Мапінг AppError → HTTP status
- **Де:** `src/network/json_errors.rs:47-68`
- **Сигнал:** `pub fn http_status_for_app_error`
- **Патерн:** кожен варіант `AppError` → консервативний 4xx/5xx; не змінювати стабільні `error.code` без потреби
- **Перевірка:** unit-тести в `json_errors.rs`

### [HTTP] HttpAppError + IntoResponse
- **Де:** `src/network/json_errors.rs:157-178`
- **Сигнал:** `pub struct HttpAppError`
- **Патерн:** обгортка `AppError` + опційний `ErrorContext` і override статусу для Axum `IntoResponse`
- **Перевірка:** `rg -n "pub struct HttpAppError" src/network/json_errors.rs`

### [HTTP] Re-export через api/common
- **Де:** `src/network/api/common.rs:3-5`
- **Сигнал:** `pub use crate::network::json_errors::HttpAppError`
- **Патерн:** handlers імпортують `HttpAppError` з `api::common`, щоб розірвати цикл `auth` ↔ `common`
- **Перевірка:** `rg -n "pub use crate::network::json_errors" src/network/api/common.rs`

### [HTTP] RBAC у check_permission
- **Де:** `src/network/api/common.rs:16-29`
- **Сигнал:** `pub fn check_permission`
- **Патерн:** deny → `HttpAppError` + `AppError::Forbidden` + структурований `ErrorContext`
- **Перевірка:** enterprise route + Viewer RBAC (FM-012)

### [HTTP] RAID RestError helper
- **Де:** `src/network/api/raid_http.rs:9-24`
- **Сигнал:** `fn raid_api_err`
- **Патерн:** `AppError::RestError { code, message }` + status override для RAID REST
- **Перевірка:** `rg -n "raid_api_err" src/network/api/raid_http.rs`

### [HTTP] Rate limit JSON
- **Де:** `src/network/rate_limit.rs:214-220`
- **Сигнал:** `create_rate_limit_response`
- **Патерн:** `api_json_error("RATE_LIMIT_EXCEEDED", …)` + `retry_after` у тілі відповіді
- **Перевірка:** `rg -n "RATE_LIMIT_EXCEEDED" src/network/rate_limit.rs`

### [HTTP] Rewards → HttpAppError
- **Де:** `src/network/api/rewards.rs:32-71`
- **Сигнал:** усі handlers → `Result<_, HttpAppError>`; `user_progress` → `ApiNotFound` + `ErrorContext`
- **Патерн:** узгоджено з FM-005; `From<AppError>` для `HttpAppError` у `json_errors.rs`
- **Перевірка:** `rg "Result<.*AppError>" src/network/api/rewards.rs` → 0
- **FM:** FM-005 ✅, FM-014 ✅

### [UI] Admin JSON contract tests
- **Де:** `tests/admin_ui_api_contracts.rs`
- **Сигнал:** `cargo test --test admin_ui_api_contracts --features test-utils,enterprise`
- **Патерн:** `oneshot` на `/api/v1/*`; 503 → `error` object; `attach_*_for_test` для 200 shapes
- **Перевірка:** 15+ tests (config, users, topology/nodes, …)
- **FM:** FM-013, FM-014 ✅

---

## Service layer / AppState

### [P2] Thin handler → service
- **Де:** `src/network/api/raid.rs:153-157` (приклад), `src/services/raid_service.rs`
- **Сигнал:** `rg "get_global_" src/network/api` → **0**
- **Патерн:** handler `match RaidService::…`; логіка в `*_service.rs`; map `RaidServiceError` → HTTP
- **Перевірка:** `rg "get_global_" src/network/api`
- **FM:** FM-002 ✅

### [P2] RaidService без globals
- **Де:** `src/services/raid_service.rs:35-40`
- **Сигнал:** `fn require_raid_manager`
- **Патерн:** `ctx.raid_manager.get().cloned()` з `OnceLock` на `AppState`, не `get_global_manager()`
- **Перевірка:** `rg -n "require_raid_manager" src/services/raid_service.rs`

### [STATE] ApiContext = Arc<AppState>
- **Де:** `src/core/state.rs:90`
- **Сигнал:** `pub type ApiContext`
- **Патерн:** єдиний тип стану роутера `/api/v1`; підсистеми через `OnceLock` на `AppState`
- **Перевірка:** `rg -n "pub type ApiContext" src/core/state.rs`

### [STATE] attach_*_for_test
- **Де:** `src/core/state.rs:454-469`
- **Сигнал:** `attach_raid_manager_for_test`
- **Патерн:** інжекція менеджерів у тестах без `main` / `get_global_*`
- **Перевірка:** `cargo test --test appstate_http_injection_integration --features test-utils`

### [STATE] Discovery injection
- **Де:** `src/network/discovery.rs:276-296`
- **Сигнал:** коментар `avoids .get_global_instance_manager`
- **Патерн:** `DiscoveryService` отримує `instance_manager` з `AppState`; виняток від правила «без globals у HTTP» — лише в `discovery.rs`, не в `api/*`
- **Перевірка:** `rg "get_global_" src/network/api` (0)

### [P2] Topology / Instance mapping
- **Де:** `src/network/api/topology.rs:80-97`, `src/network/api/instances.rs:74-87`
- **Сигнал:** `TopologyService::get_snapshot`, `InstanceService::placement_previews`
- **Патерн:** `TopologyNotReady` / `InstanceServiceError` → типізований `HttpAppError` / `SubsystemUnavailable`
- **Перевірка:** `cargo test --test appstate_http_injection_integration --features test-utils`

### [API] Feature-gated merge routes
- **Де:** `src/network/api/mod.rs:47-59`
- **Сигнал:** `pub fn create_api_routes`
- **Патерн:** один `Router<ApiContext>` з `.merge(...)` для під-роутерів за features
- **Перевірка:** `rg -n "create_api_routes" src/network/api/mod.rs`

---

## Distributed RAID / wire

### [RAID] Wire handlers → protocol service
- **Де:** `src/network/raid_distributed_handlers.rs:12-17`
- **Сигнал:** `RaidDistributedProtocolService::put_artifact`
- **Патерн:** Axum handler лише `State<ApiContext>` + делегування в `RaidDistributedProtocolService`
- **Перевірка:** `cargo test -j 1 --test distributed_raid_wire_integration --features test-utils`

### [RAID] Wire integration tests
- **Де:** `tests/distributed_raid_wire_integration.rs:1-6`
- **Патерн:** sync conflicts, graceful leave, membership — HTTP wire, не module globals
- **Перевірка:** `cargo test -j 1 --test distributed_raid_wire_integration --features test-utils`
- **FM:** FM-007, FM-008 ✅

---

## Auth / enterprise

### [AUTH] JWT middleware
- **Де:** `src/network/auth.rs:435-465`
- **Сигнал:** `pub async fn auth_middleware`
- **Патерн:** `Bearer` JWT → validate → `Claims` у request extensions
- **Перевірка:** `rg -n "auth_middleware" src/network/enterprise_api/mod.rs`

### [AUTH] Permission middleware
- **Де:** `src/network/auth.rs:469-491`
- **Сигнал:** `pub async fn permission_middleware`
- **Патерн:** відсутні `Claims` або permission → `AUTH_*` `RestError` коди
- **Перевірка:** FM-012 unit-тести RBAC

### [ENTERPRISE] Thin enterprise_api
- **Де:** `src/network/enterprise_api/security.rs:26`, `src/services/enterprise_service.rs:1-3`
- **Патерн:** parse → `EnterpriseService::…` → map errors; mutating routes + `.layer(from_fn(auth_middleware))`
- **Перевірка:** `rg -n "from_fn\(auth_middleware\)" src/network/enterprise_api/mod.rs`

### [OAuth] Pending state + callback
- **Де:** `src/core/state.rs:288-290`, `src/network/enterprise_api/oauth.rs:4-14`
- **Сигнал:** `oauth2_pending_states`, `verify_oauth2_pending`
- **Патерн:** CSRF/state у `AppState`; Telegram HMAC/`auth_date`/allowlist; audit success/fail/deny
- **Перевірка:** `cargo test -p poolai --lib enterprise::security` (або відповідні unit у `security.rs`)
- **FM:** FM-012 ✅

---

## UI / i18n

### [UI] poolaiT / t(key)
- **Де:** `src/ui/i18n_core.js:1648-1653`
- **Сигнал:** `function t(key)`
- **Патерн:** `STRINGS[lang]` → fallback EN → повертає key якщо відсутній
- **Перевірка:** `rg -n "function t\(key\)" src/ui/i18n_core.js`
- **FM:** FM-012 ✅

### [UI] apply data-i18n
- **Де:** `src/ui/i18n_core.js:1656-1676`
- **Сигнал:** `function apply(root)`
- **Патерн:** `[data-i18n]` / `[data-i18n-html]` → `t(key)` для text/placeholder
- **Перевірка:** ручний smoke `/ui/auth` UA/EN

### [UI] OAuth рядки en/uk
- **Де:** `src/ui/i18n_core.js:19-23`
- **Сигнал:** `'auth.oauthStartFail'`, `'auth.oauthOr'`
- **Патерн:** паралельні ключі `en`/`uk` для login/admin OAuth UI
- **Перевірка:** `rg -n "'auth.oauth" src/ui/i18n_core.js`

---

## ML / TurboQuant

### [ML] Pipeline quantization step
- **Де:** `src/ml/pipeline.rs:739-768`
- **Сигнал:** `execute_turboquant_quantization_step`
- **Патерн:** `turboquant=true` → `turboquant::pack_uniform_rows`, тег `step_kind=turboquant`
- **Перевірка:** `cargo test --test ml_pipeline_integration --features ml`
- **FM:** FM-004 (SIMD deferred) — логіка без SIMD у автопрогоні

### [ML] Demo handler AppState
- **Де:** `src/network/api/ai_ml.rs:170-180`
- **Сигнал:** `ml_pipeline_manager`
- **Патерн:** ML demo читає менеджер з `AppState`, не module global
- **Перевірка:** `rg -n "ml_pipeline_manager" src/network/api/ai_ml.rs`

---

## Тести та CI

### [CI] test-ci alias
- **Де:** `.cargo/config.toml:25-26`
- **Патерн:** `-j 1 --lib --tests --features ml,enterprise,cloud,test-utils --test-threads=1` (без doctests)
- **Перевірка:** `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
- **FM:** FM-011 ✅

### [CI] HTTP injection integration
- **Де:** `tests/appstate_http_injection_integration.rs:45-51`
- **Патерн:** `attach_*_for_test` → `create_api_routes().with_state(ctx)` → `oneshot`
- **Перевірка:** `cargo test --test appstate_http_injection_integration --features test-utils`

### [CI] Router smoke oneshot
- **Де:** `tests/network_api_integration.rs:21-34`
- **Патерн:** `tower::ServiceExt::oneshot` на `/api/v1/*` з `ApiContext::default()`
- **Перевірка:** `cargo test --test network_api_integration`

### [CI] Integration targets у Cargo.toml
- **Де:** `Cargo.toml:186-194`
- **Патерн:** `distributed_raid_wire_integration`, `appstate_http_injection_integration` потребують `test-utils`
- **Перевірка:** `rg -n "appstate_http_injection_integration" Cargo.toml`

### [Workers] Virtual node task API
- **Де:** `src/network/api/virtual_nodes.rs:1-50`
- **Сигнал:** `VirtualNodeTaskService`, `poll` / `complete` routes
- **Патерн:** thin Axum handlers → `VirtualNodeTaskService`; bootstrap tasks (`ping`, `raid_health_check`) у `virtual_node_task_service.rs`
- **Перевірка:** `cargo test --test virtual_node_tasks_integration --features test-utils`
- **FM:** FM-016 ✅

### [Workers] poolai-worker coordinator loop
- **Де:** `src/bin/poolai-worker.rs:392-419`
- **Сигнал:** `register_remote`, `heartbeat_remote`, `poll_and_run_tasks`
- **Патерн:** env `POOLAI_COORDINATOR_URL` + periodic re-register on heartbeat failure
- **Перевірка:** `cargo build --bin poolai-worker`; manual: coordinator + worker health `GET /health`

### [FM-003] Dev stand HTTP port
- **Де:** `src/core/dev_stand.rs`, `src/main.rs`
- **Патерн:** `POOLAI_HTTP_PORT` → `resolve_http_port()` (default 8080)
- **Перевірка:** `cargo test --lib dev_stand`

### [FM-003] Virtual-node dev stand scripts
- **Де:** `bin/run-virtual-node-dev.sh`, `bin/verify-dev-stand.sh`
- **Патерн:** coordinator :8080 + worker :9090; `POOLAI_VIRTUAL_NODE_DATA_DIR` under `data/lan-stand/virtual-node`
- **Перевірка:** `bash bin/run-virtual-node-dev.sh`; `bash bin/verify-dev-stand.sh` (warmup 50s, checks discovery/pool/tasks>=4)

### [Workers] Telegram bot → coordinator webhook
- **Де:** `src/tgbot/coordinator.rs`, `src/bin/poolai-telegram-bot.rs`
- **Патерн:** teloxide handler → `forward_message` → `POST /virtual-nodes/telegram/webhook`
- **Збірка:** `cargo build --bin poolai-telegram-bot --features tgbot`
- **Перевірка:** `cargo test --test tgbot_coordinator_bridge_integration`
- **FM:** FM-016++ ✅

### [Workers] Telegram bind + webhook → task
- **Де:** `src/network/api/virtual_nodes.rs` (bind/webhook handlers), `src/services/virtual_node_telegram_binding_service.rs`
- **Патерн:** webhook resolves `telegram_user_id` → `peer_id` → `VirtualNodeTaskService::enqueue`
- **Перевірка:** `cargo test --test virtual_node_telegram_binding_integration`
- **FM:** FM-016+ ✅

### [Docs] OpenAPI virtual-nodes surface
- **Де:** `docs/openapi.yaml` — tags `Discovery`, `VirtualNodes`
- **Патерн:** sync з `src/network/api/virtual_nodes.rs`, `discovery.rs` register-remote / heartbeat / list
- **Перевірка:** ручна звірка маршрутів; `rg virtual-nodes docs/openapi.yaml`

### [FM-012] Telegram OAuth widget verification
- **Де:** `src/enterprise/security.rs` (`verify_telegram_widget_query`, `verify_telegram_oauth_callback`)
- **Патерн:** HMAC-SHA256 + constant-time hash; `POOLAI_TELEGRAM_AUTH_MAX_AGE_SECS`; allowlist trim
- **HTTP:** `GET /api/enterprise/auth/telegram/callback` — `tests/telegram_oauth_callback_integration.rs`
- **Перевірка:** `cargo test --test telegram_oauth_callback_integration --features enterprise`; `cargo test --lib enterprise::security::tests --features enterprise`

### [FM-012] Telegram webhook payload guard
- **Де:** `src/network/api/virtual_nodes.rs` (`TELEGRAM_WEBHOOK_MAX_TEXT`, `webhook_secret_ok`)
- **Патерн:** optional `POOLAI_TELEGRAM_WEBHOOK_SECRET` → header `X-Telegram-Webhook-Secret`; truncate `message.text` before enqueue
- **Перевірка:** `cargo test --test virtual_node_telegram_binding_integration --features test-utils`

### [Workers] Local artifact cache on device (FM-016+++)
- **Де:** `src/workers/artifact_cache.rs`, env `POOLAI_WORKER_CACHE_DIR`
- **Патерн:** після успішного PutArtifact wire → `store_probe` у `{cache}/artifacts/{name}-{ts}.bin`; health → `cached_artifacts`
- **Перевірка:** `cargo test --lib workers::artifact_cache`; `verify-dev-stand` (cached_artifacts >= 1)

### [Workers] RAID artifact probe on virtual node (FM-016+++)
- **Де:** `src/workers/raid_artifact_probe.rs`, task `raid_artifact_probe`, `POST /raid/distributed/artifacts/replicate`
- **Патерн:** bootstrap enqueue → worker builds `PutArtifact` probe → coordinator RAID wire
- **Перевірка:** `cargo test --lib workers::raid_artifact_probe`; `cargo test --test virtual_node_tasks_integration --features test-utils`

### [P4] poolai_health_load baseline row (ops)
- **Де:** `src/bin/poolai_health_load.rs`, `docs/performance/BENCHMARKS.md` таблиця `poolai_health_load --json`
- **Патерн:** coordinator на `:8080` → MSYS2 UCRT64 release: `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo run --release --bin poolai_health_load -- --json http://127.0.0.1:8080/api/v1/health 5 50`; PowerShell без MSVC linker — debug exe у `target/debug/` або лише MSYS2 для release
- **Перевірка:** JSON на stdout → рядок (`rps_ok_only`, `latency_p50_ms`, …); changelog `BENCHMARKS.md`
- **FM:** P4 ✅ **2026-05-18**

### [FM-003] LAN §4 BLOCKED — ops only
- **Де:** `docs/performance/LAN_BENCHMARK_RUNBOOK.md` §6, `docs/performance/BENCHMARKS.md` changelog
- **Сигнал:** немає 2 фізичних хостів → не додавати LAN replication row; `poolai_health_load` **2026-04-10** + **2026-05-18** у таблиці
- **Патерн:** dev stand §5.1 (`bin/verify-dev-stand.*`) + §5 dual-port на одній машині; §4 acceptance — лише після ops-прогону
- **Перевірка:** docs-only спринт; `cargo test-ci` для регресії
- **FM:** FM-003 §4 BLOCKED

### [ML] Pipeline step output metrics
- **Де:** `src/ml/pipeline.rs` (`execute_*_step`), `docs/ml/PIPELINE_MANAGEMENT.md` § «Ключі output кроків»
- **Сигнал:** `step_results[step_id].output["step_kind"]` — `turboquant` vs `quantization`
- **Патерн:** TurboQuant: `turboquant=true` + `weight_rows`; метрики `bytes_in`, `bytes_out`, `max_abs_recon_error`; стандартна квантизація — `size_mb_before` / `size_mb_after`
- **Перевірка:** `cargo test --test ml_pipeline_integration --features ml test_pipeline_turboquant_quantization_step test_pipeline_standard_quantization_metrics`
- **FM:** DIGEST §ML ✅

### [FM-019] Baseline verification runbook
- **Де:** `docs/development/ADMIN_A11Y_RUNBOOK.md`
- **Сигнал:** `rg "cargo test-ci" docs/development/ADMIN_A11Y_RUNBOOK.md`
- **Патерн:** після змін `src/ui/` — `cargo test-ci` + `cargo test -p poolai --features enterprise ui::admin --lib`; ручна клавіатура admin; опційно `npx pa11y` на users/security
- **FM:** FM-019 Baseline Implemented

### [FM-019] Admin tablist + dynamic tables (semantic)
- **Де:** `src/ui/admin/security.rs`, `config.rs` (static `role="tablist"`); `admin_common.js` (`adminSyncTabA11y`, `adminEnhanceTablesA11y`, `adminObserveDynamicA11y`)
- **Сигнал:** `rg 'role="tablist"' src/ui/admin/security.rs`
- **Патерн:** tabs — `aria-selected`, `aria-controls`, `tabindex="-1"` на неактивних; tables — `th scope="col"`, `aria-label` з `h3`
- **Перевірка:** `cargo test -p poolai --features enterprise ui::admin --lib`
- **FM:** FM-019 Partial

### [FM-019] Admin form a11y (aria-required, labels)
- **Де:** `src/ui/admin_common.js` (`adminEnhanceFormA11y`); приклад static — `src/ui/admin/users.rs`
- **Сигнал:** `rg "adminEnhanceFormA11y" src/ui/admin_common.js`
- **Патерн:** `[required]` → `aria-required="true"`; `.required` → `aria-hidden="true"`; orphan label у `.form-group` → `for={id}`; виклик на `DOMContentLoaded` і після `showModal` / `showModalContent`
- **Перевірка:** `cargo test -p poolai --features enterprise ui::admin --lib`
- **FM:** FM-019 Partial

### [FM-019] Admin modal focus trap + closed aria-modal
- **Де:** `src/ui/admin_common.js`; розмітка — `rg 'aria-modal="false" aria-hidden="true"' src/ui/admin`
- **Сигнал:** `keepFocusInModal`, `ADMIN_DYNAMIC_MODAL_ID` у `admin_common.js`
- **Патерн:** закритий static modal — `aria-modal="false"` (не `true` при hidden); відкриття — `attachModalA11y`, Tab/Esc; dynamic — `showModal(title, html)`
- **Перевірка:** `cargo test -p poolai --features enterprise ui::admin --lib`
- **FM:** FM-019 Partial

### [FM-019] pa11y CI (workflow_dispatch)
- **Де:** `bin/pa11y-ci.sh`, `.github/workflows/a11y.yml`
- **Патерн:** `PA11Y_ADMIN_STRICT=1 bash bin/pa11y-ci.sh --start` — login actions (`#username`, `#password`, `#loginBtn`) then strict admin URLs; creds `PA11Y_USER`/`PA11Y_PASSWORD` (default `admin`/`admin123`); unauthenticated URLs — `write_pa11y_simple_config` + `--config` (pa11y v9, без CLI `--chromeLaunchConfig`)
- **Перевірка:** `cargo test --test pa11y_ci_script`; GitHub Actions → **A11y (pa11y)**
- **FM:** FM-019 Partial (pa11y auth)

### [FM-016] Virtual-node integration tests + test-utils feature
- **Де:** `tests/virtual_node_pool_join_integration.rs`, `tests/virtual_node_tasks_integration.rs`, `Cargo.toml` `[[test]]`
- **Патерн:** `required-features = ["test-utils"]` — інакше `attach_*_for_test` не компілюється; `cargo test-ci` уже з `--features …,test-utils`
- **Перевірка:** `cargo test --test virtual_node_pool_join_integration --test virtual_node_tasks_integration --features test-utils`
- **FM:** FM-016+++ ops

### [FM-019] pa11y tune — contrast + dashboard shell IDs
- **Де:** `src/ui/admin_styles.css`, `src/ui/themes.rs`, `src/ui/components.rs` (`--danger: #c62828`); `src/ui/mod.rs` (`mobileUserInfo`, `mobileAuthLoginBtn`, `aria-label` на `#themeSelector`)
- **Сигнал:** `rg 'mobileUserInfo' src/ui/mod.rs`; `PA11Y_ADMIN_STRICT=1 bash bin/pa11y-ci.sh --start` → 0 errors
- **Перевірка:** `cargo test -p poolai dashboard_shell_auth_ids_unique --lib`
- **FM:** FM-019 Partial (pa11y strict pass)

### [FM-019] Dashboard modals (workers/libs/vm/raid)
- **Де:** `src/ui/mod.rs` (`showModal`, `keepFocusInModal`, `attachDashModalA11y`)
- **Патерн:** closed state `aria-modal="false" aria-hidden="true"`; open → trap + Esc via `handleDashModalEscape`
- **Перевірка:** `cargo test -p poolai dashboard_a11y --lib`
- **FM:** FM-019 Partial (dashboard)

### [FM-019] Dashboard nav aria-current
- **Де:** `src/ui/mod.rs` (`dashMarkCurrentNav`, CSS `.nav a[aria-current="page"]`)
- **Сигнал:** `rg "dashMarkCurrentNav" src/ui/mod.rs`
- **Патерн:** як `adminMarkCurrentNav`; виклик після `initDashboardShell` і на `poolai:langchange`
- **Перевірка:** `cargo check -p poolai --features enterprise` (UI в `mod.rs` завжди)
- **FM:** FM-019 Partial

### [FM-018] Admin/login a11y
- **Де:** `src/ui/admin/mod.rs:89-130`, `admin_styles.css:770-808`, `admin_common.js` (`adminMarkCurrentNav`, `adminAnnounceLive`); login у `src/ui/mod.rs`
- **Патерн:** skip links → `#admin_main_content` / `#admin_nav`; `aria-live` для помилок; `aria-current="page"` через `adminMarkCurrentNav()` (DOMContentLoaded + inline shell); у `format!` **не** літерал `#fragment`
- **Перевірка:** `cargo test -p poolai --features enterprise ui::admin::a11y_tests`
- **FM:** FM-018 ✅

### [FM-017] Discovery HttpAppError + virtual-node status-only
- **Де:** `src/network/api/discovery.rs` (`discovery_not_ready`, `discovery_validation`, …); `virtual_nodes.rs` — status-only (коментар FM-017)
- **Патерн:** worker (`poolai-worker`) — `is_success()` без parse body; discovery помилки — `{ "error": { "code", "message" }, "context"? }`
- **Перевірка:** `cargo test --test discovery_remote_register_integration`; `rg "HttpAppError" src/network/api/virtual_nodes.rs` → 0

### [Workers] Virtual node pool join (FM-016+++)
- **Де:** `src/network/api/virtual_nodes.rs` (`POST .../pool/join`), `poolai-worker` after `register-remote`
- **Патерн:** peer must exist in discovery with `metadata.role = virtual_node` → `WorkerPoolService::add_worker` (no JWT)
- **Перевірка:** `cargo test --test virtual_node_pool_join_integration --features test-utils`

### [Workers] Virtual node task executor (FM-016+++)
- **Де:** `src/workers/virtual_node_executor.rs`, `src/bin/poolai-worker.rs`
- **Патерн:** HTTP I/O у worker → `TaskRuntime` → `complete_task(task_type, payload, rt)`; bootstrap + `pool_workers_probe`
- **Telegram:** `/status`, `/raid` у `telegram_command` payload (`text`)
- **Перевірка:** `cargo test --lib workers::virtual_node_executor`; `cargo build --bin poolai-worker`

### [Workers] Virtual node file store
- **Де:** `src/services/virtual_node_store.rs`, env `POOLAI_VIRTUAL_NODE_DATA_DIR`
- **Патерн:** `telegram_bindings.json` + `tasks/{peer_id}.json` (atomic write)
- **Перевірка:** coordinator restart with dir set; tasks/bindings survive

### [Workers] Discovery remote register test harness
- **Де:** `tests/discovery_remote_register_integration.rs`, `tests/virtual_node_tasks_integration.rs:18-40`
- **Патерн:** `attach_raid_manager_for_test` + `DiscoveryService` у `ApiContext` → `create_api_routes().with_state(ctx)`
- **Перевірка:** `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`

---

## Документація (кроки 1–12)

| Крок | Файл | Коли оновлювати |
|------|------|-----------------|
| 11 | `docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md` | нові маршрути / features |
| 12 | `docs/catalog/FUNCTION_MANAGEMENT.md` | FM-*, §5.1 |
| ops | `docs/development/HANDOFF_NEW_SESSION.md` | кожен автопрогін |
| P0 | `docs/development/AUTO_DEV_PATTERNS.md` | після explore + `rg` регресій |

---

## Заборонені / обережно

- `git add -A` без відбору; не комітити `data/audit/*.log.gz`
- Повний `cargo test` з doctests на Windows — ризик os error 1455; канон — `cargo test-ci`
- PowerShell/cmd для `cargo`/`git` у цьому репо — лише MSYS2 bash (див. `chat-context.md`)
