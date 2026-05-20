# Патерни автономної розробки PoolAI

**Призначення:** реєстр **конкретних** повторюваних рішень для наступних сесій авторозробки. Оркестратор доповнює цей файл після P0 (збір) і S6 (закриття).

**Оновлено:** 2026-05-19 (S38 Job/Memory wire types).

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
- **Сигнал:** `rg "async fn.*_" tests/admin_ui_api_contracts.rs` → 27 tests (2026-05-19, S26)
- **Патерн:** `Router::new().nest("/api/v1", create_api_routes()).with_state(ApiContext::default())`; 503 → `assert_structured_error`; модуль `attached_managers` — `attach_*_for_test` для 200 shapes
- **Перевірка:** `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test --test admin_ui_api_contracts --features ml,enterprise,cloud,test-utils -j 1 -- --test-threads=1`
- **Доки:** [`ADMIN_UI_JSON_CONTRACTS.md`](./ADMIN_UI_JSON_CONTRACTS.md) — колонка **Тест**
- **FM:** FM-013–015 ✅

### [UI] Enterprise admin contract slices (S25)
- **Де:** `tests/admin_ui_api_contracts.rs` — `mod enterprise_admin_contract_slices` (після `#[cfg(feature = "enterprise")]`)
- **Сигнал:** `rg "enterprise_tenants_list|enterprise_oauth2|enterprise_monitoring_dashboards" tests/admin_ui_api_contracts.rs`
- **Патерн:** seed через `ApiContext` managers, потім `nest("/api/enterprise", create_enterprise_api_routes())`:
  - tenants — `ctx.tenant_manager.initialize()` + `create_tenant(...)` → `GET /api/enterprise/tenants` (ключі `config.active`, `usage.workers`);
  - OAuth2 — `ctx.security_manager.register_oauth2_provider(...)` → `GET .../security/oauth2/providers` (`name`, `config.client_id`, `enabled`);
  - dashboards — `ctx.enterprise_monitoring_manager.create_dashboard(Dashboard { ... })` → `GET .../monitoring/dashboards` (`id`, `name`, `metrics`, `is_public`, `created_at`).
- **Перевірка:** `cargo test-ci`; UI читає ті самі поля в `src/ui/admin/{tenants,security,monitoring}.rs`
- **FM:** FM-013 (розширення), UI_QUALITY P1

### [UI] Enterprise metrics + alert-rules contracts (S26)
- **Де:** `tests/admin_ui_api_contracts.rs` — `enterprise_monitoring_metrics_json_shape`, `enterprise_monitoring_alert_rules_json_shape`
- **Сигнал:** `rg "enterprise_monitoring_metrics|enterprise_monitoring_alert_rules" tests/admin_ui_api_contracts.rs`
- **Патерн:** `record_metric(MetricDataPoint { metric, value, timestamp, … })` → `GET .../monitoring/metrics?metric=…`; `create_alert_rule(AlertRule { name, metric, operator, threshold, severity, enabled })` → `GET .../alert-rules`
- **Перевірка:** UI `dashboard.rs` / `monitoring.rs` читають `metric`, `value`, `timestamp` та поля rule table
- **FM:** FM-013, UI_QUALITY P1 ✅

### [UI] Enterprise SAML + security policies contracts (S26)
- **Де:** `tests/admin_ui_api_contracts.rs` — `enterprise_saml_providers_json_shape`, `enterprise_security_policies_json_shape`
- **Сигнал:** `rg "enterprise_saml_providers|enterprise_security_policies" tests/admin_ui_api_contracts.rs`
- **Патерн:** `security_manager.register_saml_provider` / `create_security_policy` → `GET .../security/saml/providers` та `GET .../security/policies`
- **Перевірка:** `src/ui/admin/security.rs` — `config.entity_id`, `require_mfa`, `session_timeout`
- **FM:** FM-012/013, UI_QUALITY P1 ✅

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
- **FM:** FM-004 ✅ (scalar default; SIMD опційно)

### [ML] TurboQuant SIMD fast-path (FM-004, S35)
- **Де:** `src/ml/turboquant.rs` — `row_max_abs_simd`, `append_quantized_row_simd`, `push_dequantized_row_simd`, `dot_f32_simd`
- **Сигнал:** `cfg!(feature = "turboquant-simd")` / `simd_fast_path_enabled()`
- **Патерн:** feature `turboquant-simd` → `wide::f32x4`; CI `cargo test-ci` без feature; parity `simd_pack_matches_scalar_reference`
- **Перевірка:** `cargo test turboquant --lib --features ml,enterprise,cloud,test-utils,turboquant-simd`
- **FM:** FM-004 ✅

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
- **Патерн:** `PA11Y_ADMIN_STRICT=1 bash bin/pa11y-ci.sh --start` — login actions then 9 `ADMIN_URLS` (dashboard + admin + libs/vm/raid); creds `PA11Y_USER`/`PA11Y_PASSWORD` (default `admin`/`admin123`); unauthenticated — `write_pa11y_simple_config` + `--config` (pa11y v9)
- **Перевірка:** `cargo test --test pa11y_ci_script`; GitHub Actions → **A11y (pa11y)**
- **FM:** FM-019 Partial (pa11y auth)

### [FM-019] pa11y contract у main CI
- **Де:** `.github/workflows/ci.yml` — job `pa11y-contract`
- **Сигнал:** `cargo test --test pa11y_ci_script`
- **Патерн:** перевірка `bin/pa11y-ci.sh` (ADMIN_URLS, PA11Y_WCAG22, validate) без Node; повний scan — `a11y.yml`
- **Перевірка:** PR/push на `main` → job зелений
- **FM:** FM-019 Partial

### [FM-019] pa11y WCAG 2.2 profile (PA11Y_WCAG22)
- **Де:** `bin/pa11y-ci.sh`, `.github/workflows/a11y.yml` (`PA11Y_WCAG22: "1"`)
- **Сигнал:** `PA11Y_STANDARD=WCAG22AA` → exit 2 (pa11y v9); `PA11Y_WCAG22=1` → axe tags `wcag22aa`
- **Патерн:** CI + локально: `PA11Y_WCAG22=1 PA11Y_ADMIN_STRICT=1 bash bin/pa11y-ci.sh --start` — 0 errors
- **Перевірка:** audit filters — `<label for="audit-*">` у `src/ui/admin/audit.rs`
- **FM:** FM-019 Partial (WCAG 2.2 CI)

### [FM-019] pa11y URL matrix + UI plan archival (docs S4)
- **Де:** [`ADMIN_A11Y_RUNBOOK.md`](./ADMIN_A11Y_RUNBOOK.md) §3.1; [`UI_IMPROVEMENTS_PLAN.md`](../UI_IMPROVEMENTS_PLAN.md) (архів)
- **Патерн:** strict — login + 18 `ADMIN_URLS` (dashboard + status/health/metrics + admin home + users/security/config + tenants/audit/monitoring/instances/topology + workers/libs/vm/raid); бібліотеки — **`/ui/libs`**
- **Перевірка:** `cargo test --test pa11y_ci_script`
- **FM:** FM-019 Partial (pa11y strict URLs)

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

### [OpenAPI] Virtual-node Telegram routes sync
- **Де:** `src/network/api/virtual_nodes.rs` (`create_virtual_node_routes`), `docs/openapi.yaml`
- **Сигнал:** `rg '\.route\(' src/network/api/virtual_nodes.rs` vs `rg 'virtual-nodes/telegram' docs/openapi.yaml`
- **Патерн:** шляхи в OpenAPI без префікса `/api/v1` (servers.url вже задає base); FM-016+ list `GET .../telegram/bindings`, unbind `DELETE .../bindings/{telegram_user_id}` → 204
- **Перевірка:** docs-only — `cargo test-ci` зріз; при зміні handlers — integration `virtual_node_*`
- **FM:** FM-016+

### [OpenAPI] Admin, topology, model instances
- **Де:** `src/network/api/admin.rs`, `topology.rs`, `instances.rs`; `docs/openapi.yaml`
- **Сигнал:** `rg '\.route\(' src/network/api/{admin,topology,instances}.rs` vs `rg '^  /(admin|topology|instance|state)' docs/openapi.yaml`
- **Патерн:** `GET /admin/overview` → `AdminOverview`; topology 4 GET paths; instance previews require `model_id` query; POST/DELETE `/instance` need `bearerAuth`
- **Перевірка:** `cargo test-ci`; `tests/admin_ui_api_contracts.rs`
- **FM:** FM-013–015 (contracts), admin UI

### [OpenAPI] Discovery FM-016 routes sync
- **Де:** `src/network/api/discovery.rs` (`create_discovery_routes`), `docs/openapi.yaml`
- **Сигнал:** `rg '\.route\(' src/network/api/discovery.rs` vs `rg '^  /discovery' docs/openapi.yaml`
- **Патерн:** peers `GET /discovery/peers`, `GET .../peers/{peer_id}`; local announce `POST /discovery/register` (200/503); health `GET .../virtual-nodes/{peer_id}/health` → `RemoteHealthProbe` (200/404/503)
- **Перевірка:** `cargo test-ci`; `discovery_remote_register_integration`, `virtual_node_*`
- **FM:** FM-016

### [OpenAPI] Config, UI, completions, enterprise ML pipeline
- **Де:** `src/network/api/{system,ui,completions,ai_ml}.rs`; `docs/openapi.yaml`
- **Сигнал:** `rg '\.route\(' src/network/api/{system,ui,completions,ai_ml}.rs` vs `rg '^  /(config|ui/|v1/chat|ai-ml/)' docs/openapi.yaml`
- **Патерн:** v1 base — `/config` GET public, PUT `admin:all`; `/ui/*` dashboards need `enterprise`; chat at `/v1/chat/completions` → full URL `/api/v1/v1/chat/completions`; `/ai-ml/*` — path-level `servers: /api/enterprise` (features `enterprise`+`ml`)
- **Перевірка:** `cargo test-ci`; `tests/admin_ui_api_contracts.rs` (`config_get`); `tests/network_api_integration.rs` (`test_config_get`, chat completions URI)
- **FM:** FM-014 (config), FM-012/013 (UI), DIGEST §ML

### [OpenAPI] Enterprise OAuth, monitoring, OAuth2 providers
- **Де:** `src/network/enterprise_api/{oauth,monitoring,security}.rs`, `mod.rs`; `docs/openapi.yaml`
- **Сигнал:** `rg '\.route\(' src/network/enterprise_api/mod.rs` vs `rg '^  /(auth/|monitoring/|security/oauth2)' docs/openapi.yaml`
- **Патерн:** base `/api/enterprise`; OAuth `GET /auth/{github,google,telegram}` → 302/HTML; callbacks → `/ui/auth?token=`; monitoring alerts query `severity|tenant_id|acknowledged`; `POST .../alerts/{id}/acknowledge` needs JWT; security OAuth2 CRUD `admin:all`
- **Перевірка:** `cargo test-ci`; unit tests у `oauth.rs` (Telegram HMAC/allowlist)
- **FM:** FM-012 (OAuth/Telegram)

### [OpenAPI] Enterprise tenants, audit, SAML
- **Де:** `src/network/enterprise_api/{tenants,audit,saml,security}.rs`; `docs/openapi.yaml`
- **Сигнал:** `rg '\.route\(' src/network/enterprise_api/mod.rs` vs `rg '^  /(tenants|audit/|auth/saml|security/saml)' docs/openapi.yaml`
- **Патерн:** tenants `POST /tenants/{id}` = update (не PUT); quota `POST .../quota` + `QuotaCheckRequest`; audit `GET /audit/events` + 8 query filters; SAML `POST .../callback` form `SAMLResponse`/`RelayState`
- **Перевірка:** `cargo test-ci` (docs-only OK)
- **FM:** FM-012 (SAML SSO), multi-tenancy

### [OpenAPI] Enterprise security policies (S20)
- **Де:** `src/network/enterprise_api/security.rs`; `docs/openapi.yaml`
- **Сигнал:** `rg 'security/policies' src/network/enterprise_api/mod.rs docs/openapi.yaml`
- **Патерн:** `POST /security/policies` body `{ name, policy }`; `PUT .../{name}` body = full `SecurityPolicy` with matching `name`; delete returns 200 JSON (не 204)
- **Перевірка:** `cargo test-ci`; wave S17–S20 push одним `git push origin main`
- **FM:** FM-012 (enterprise security)

### [OpenAPI] Gap audit v1 surface (S28)
- **Де:** `docs/openapi.yaml`, `docs/development/OPENAPI_GAP_AUDIT_2026-05-19.md`
- **Сигнал:** `rg '\.route\(' src/network/api/users.rs`; `rg '^  /users' docs/openapi.yaml`
- **Патерн:** enterprise/ai-ml вже в yaml (S14–S21); S28 закрив v1 Users, `POST/DELETE /workers`, `/libraries/upload`, RAID+admin, VM templates/networks; backlog — `/raid/distributed/*`
- **Перевірка:** `cargo test-ci` (docs-only OK)
- **FM:** FM-014 (OpenAPI sync)

### [Horizon] Layer C → 100% (S35+)

- **Де:** `docs/development/HORIZON_TO_100_PLAN.md`, `AUTO_RUN_SESSION_2026_HORIZON.md`, FM §5.6
- **Сигнал:** `rg "Horizon S35|GridEnvelope|poolai-solana-adapter" docs/ src/`
- **Черга:** S35–S40 ✅ — **Horizon закрито**; далі maintenance (`NEXT_SESSION_PROMPT.md`)

### [Post-Horizon] FM-020…031 — одна FM за сесію
- **Де:** `AUTO_RUN_SESSION_2026_POST_HORIZON.md`, FM §5.1/§5.7, `NEXT_SESSION_PROMPT.md`
- **Черга:** FM-020 scheduler → FM-021 OpenAPI jobs → … → FM-031 WCAG
- **BLOCKED:** FM-027 (2 хости) — лише runbook без sign-off

### [Maintenance] test-ci на main (після Horizon)
- **Команда:** `export K8S_OPENAPI_ENABLED_VERSION=1.28`; `rustup run stable-x86_64-pc-windows-gnu cargo test-ci` (PATH з `~/.cargo/bin`, MSYS2 UCRT64)
- **Коли:** після змін у `src/`/`tests/` або періодичний зріз `main` (~14 хв локально)
- **Сигнал:** оновити FM «Останній cargo test-ci» + HANDOFF §maintenance

### [Horizon] S40 closure — Layer C + project 100% (docs)
- **Де:** `DEVELOPMENT_PROGRESS_2026-05-19.md`, `HORIZON_TO_100_PLAN.md` §перевірка, FM §5.6, `NEXT_SESSION_PROMPT.md`
- **Сигнал:** C=100%, (A+B+C)/3=100%; S40 [x] у `AUTO_RUN_SESSION_2026_HORIZON.md`
- **Патерн:** без змін `src/`; ops BLOCKED (FM-003 §4 LAN) не знижує %

### [Cloud] FM-006 Azure/GCP REST scope (S39)
- **Де:** `src/cloud/providers/azure.rs`, `gcp.rs`, `docs/cloud/CLOUD_SDK_STATUS.md`
- **Сигнал:** `rg "TODO" src/cloud/providers/` → порожньо; `resolve_azure_location`, `set_base_url_override`
- **Патерн:** Management/Compute через **reqwest REST** (не `azure_mgmt_compute` — version skew); токени Azure: `AZURE_ACCESS_TOKEN` → `az` → IMDS; GCP: metadata → `GOOGLE_APPLICATION_CREDENTIALS` JWT
- **Конфіг:** `AZURE_LOCATION` (default `eastus`); mock: `cargo test --test cloud_mock_integration --features cloud,cloud-sdk`
- **FM:** FM-006 ✅

### [Job] Wire types + API stub (S38, P6)
- **Де:** `src/job/types.rs`, `src/memory/types.rs`, `src/network/api/jobs.rs`
- **Сигнал:** `JobSpec`, `MemoryShardRef`, `envelope_from_job_spec`
- **Патерн:** `JobStore::global()`; `POOLAI_JOB_DATA_DIR` → `jobs.json` (atomic write); без env — in-memory; `POST /api/v1/jobs` → 201; map ↔ `GridEnvelope`
- **Перевірка:** `cargo test --lib round_trip`; `cargo test-ci` після змін у `src/`

### [Job] Scheduler MVP (FM-020)
- **Де:** `src/job/scheduler.rs`, `src/job/store.rs` (`promote_submitted_to_scheduled`), `src/network/api/jobs.rs`
- **Сигнал:** `schedule_pending`, `POST /api/v1/jobs/schedule` → `{ "scheduled": N }`
- **Патерн:** `POST /jobs` push + tick → response `status: scheduled`; priority desc; без VM bind; persist одним `persist()` після batch
- **Перевірка:** `cargo test job --lib --features ml,enterprise,cloud,test-utils`
- **FM:** FM-020 ✅

### [Solana] Adapter crate schema v1 (FM-010, S37)
- **Де:** `crates/poolai-solana-adapter/src/events.rs`, `src/sidecar.rs`, bin `poolai-solana-adapter`
- **Сигнал:** `DomainEventEnvelope::from_json`, `process_event_line`
- **Патерн:** workspace member; **no** `solana-sdk` у `poolai`; NDJSON stdin → ack stdout
- **Перевірка:** `cargo test -p poolai-solana-adapter` (не повний test-ci, якщо main `src/` не змінювався)
- **FM:** FM-010 ✅

### [Grid] Envelope v1 JSON (FM-009, S36)
- **Де:** `src/grid/envelope.rs`, `src/grid/map.rs`
- **Сигнал:** `GridEnvelope::from_json`, `envelope_from_peer_info`, `envelope_from_put_artifact`
- **Патерн:** поле `v: 1`; `type` = `job` | `result` | `memory_shard` | `peer_status`; map ↔ `PeerInfo` / `PutArtifactPayload`
- **Перевірка:** `cargo test grid --lib`; `cargo test --test grid_network_scalability_tests test_grid_envelope`
- **FM:** FM-009 ✅
- **Концепти перед кодом:** `GRID_PROTOCOL_CONCEPT`, `SOLANA_ADAPTER_CONCEPT`, `JOB_LAYER_CONCEPT`
- **Перевірка:** `cargo test-ci` після кожного спринту з `src/`
- **Поза scope:** FM-003 §4 LAN (2 хости), mainnet Solana

### [FM] Autoprogon 100% — закриття (S34)

- **Де:** `DEVELOPMENT_PROGRESS_2026-05-19.md`, `FUNCTION_MANAGEMENT.md` §5.5, `NEXT_SESSION_PROMPT.md`
- **Сигнал:** шар A+B **100%**; Layer C **100%** (S40)
- **Патерн:** відкриті Architect `[ ]` для LAN/cloud-sdk — **horizon**, не backlog autoprogon
- **E2E:** усі admin nav routes в `admin.spec.ts` (S34: libs `#libraries-list`)
- **FM:** підтримка / ops лише за запитом

### [E2E] axe Playwright (S33 / FM-019)
- **Де:** `e2e/tests/a11y.spec.ts`, `@axe-core/playwright` у `e2e/package.json`
- **Сигнал:** `rg "AxeBuilder|a11y.spec" e2e/`
- **Патерн:** `critical` + `serious` violations → `[]`; tags `wcag2a`, `wcag2aa`, `wcag21a`, `wcag21aa`; login + `loginAsAdmin` → `/ui/admin/users`
- **Перевірка:** `bash bin/e2e-playwright.sh --start` (потрібен poolai + `npm install` у `e2e/`)
- **FM:** FM-019 ✅ (scope A)

### [Ops] Локальний запуск — `run-poolai` (S32)
- **Де:** `bin/run-poolai.sh`, `bin/run-poolai.ps1`, `docs/development/RUN_LOCAL.md`
- **Сигнал:** `bash bin/run-poolai.sh help`
- **Патерн:** `single` (1× poolai :8080) | `virtual-node` | `lan` | `docker` | `stop` | `status`; dev features `enterprise,ml,cloud,test-utils`
- **Не стаджити:** `data/dev/` (runtime), `data/audit/*.log*`
- **Перевірка:** `bash bin/run-poolai.sh single --bg --skip-build` → `curl -sf http://127.0.0.1:8080/api/v1/health`

### [OpenAPI] Distributed RAID wire protocol (S31)
- **Де:** `docs/openapi.yaml` tag `RAID Distributed`; handlers `src/network/raid_distributed_handlers.rs`
- **Сигнал:** `rg "raid/distributed" docs/openapi.yaml src/network/api/raid.rs`
- **Патерн:** 7× `POST`, body `ProtocolMessage` (`type`, `id`, `timestamp`, `node_id`, `payload`); response `{type}_response` + typed payload in `payload`
- **Без JWT** — node-to-node; не плутати з `/raid/artifacts` (authenticated REST)
- **Перевірка:** `rg '^  /raid/distributed' docs/openapi.yaml | wc -l` → 7
- **FM:** FM-007/008 distributed RAID

### [E2E] Playwright admin після smoke (S27 / S29 / S31)
- **Де:** `e2e/tests/admin.spec.ts`, `e2e/tests/helpers.ts`, `e2e/tests/smoke.spec.ts`
- **Сигнал:** `rg "loginAsAdmin|#tenants-list|#security-content|#audit-events" e2e/`
- **Патерн:** `loginAsAdmin(page)` → admin routes; контейнер з `.admin-table`, `.muted`, або `.admin-fetch-error` — **`.first()`** якщо кілька `.muted` (monitoring: alerts + dashboards)
- **S29:** `/ui/admin/security` → `#oauth2-providers-list` + кнопка `/register|зареєстр/i` (i18n UA); `/ui/admin/audit` → `#audit-events` (auto `queryAuditLogs()`)
- **S31:** `/ui/admin/raid` → `#raid-admin` + `#raid-artifacts` + Upload button; `/ui/admin/topology` → `#topology-node-count`, `#topology-nodes-list`
- **Перевірка:** `bash bin/e2e-playwright.sh --start` (MSYS2; `enterprise,ml,cloud,test-utils`); CI — `.github/workflows/e2e.yml` `workflow_dispatch`
- **FM:** FM-019 (UI E2E backlog)

### [FM] Legacy docs — не канон (S30)
- **Де:** `docs/development/DOCS_LEGACY_AUDIT_2026-05-19.md`, `FUNCTION_MANAGEMENT.md` §5.3
- **Сигнал:** `rg "Stale|Archived|не канон" docs/status docs/development --glob '*.md'`
- **Патерн:** січневі `STATUS_*`, `ADMIN_PANEL_*`, `UI_UX_*` — банер → STABLE/FM/DOCS_LEGACY_AUDIT; пріоритети лише з §5.1
- **Перевірка:** не `rg "\- \[ \]" docs/status` для черги автопрогону
- **FM:** крок 12

### [FM] Прогрес і «ніколи не зроблено» (аудит 2026-05-19)
- **Де:** `docs/status/DEVELOPMENT_PROGRESS_2026-05-19.md`, `FUNCTION_MANAGEMENT.md` §5.5
- **Сигнал:** шар A **93%**; BLOCKED FM-003 §4; Concept FM-009/010; Deferred FM-004/006
- **Патерн:** legacy `[ ]` у `docs/archive/` і січневих планах — **не канон**; черга S28+ у `NEXT_SESSION_PROMPT.md`
- **Перевірка:** `rg "\- \[ \]" docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md` → 2 (LAN BLOCKED, cloud-sdk optional)

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
