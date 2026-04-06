# 🎯 Rust Architect — План наступних кроків (2026-03-17)

**Статус проєкту**: 100% модулів завершено, продакшн‑готовий бекенд на Rust  
**Ціль цього плану**: не додати ще features, а **архітектурно стабілізувати** кодову базу за останніми best practices великих Rust/Axum/Tokio‑сервісів.

---

## 📊 Орієнтири та принципи

- **Базова функціональність**: завершена (див. `RUST_ARCHITECT_STATUS_2026-01-19.md`, `RUST_ARCHITECT_SUMMARY_2026-01-21.md`, `PERCENTAGE_PLAN.md`).
- **Архітектура**: модульна, з чітким поділом доменів (`core`, `raid`, `vm`, `enterprise`, `cloud`, `ui`, …).
- **Мета цього плану**: довести до кінця **архітектурні покращення**, які вже згадані в документації (Global State, Service Layer, Error Context, Performance profiling), щоб:
  - спростити підтримку великого коду,
  - покращити тестованість,
  - узгодити структуру з типовою 100% Rust/Axum архітектурою.

---

## Наступні кроки за пріоритетом (стан розробки)

Упорядковано за залежностями та поточним статусом (checkbox’и в секціях нижче — джерело правди).

| Порядок | Пріоритет | Що робити зараз |
|--------:|-----------|-----------------|
| **1** | **Priority 1** | **Закрито по суті**: `ARCHITECTURE_REVIEW.md` (розділ AppState), `DEVELOPMENT_PLAN_UPDATED.md` (посилання на покровий план), feature **`test-utils`** + `attach_*_for_test` на `AppState`. Опційно пізніше: distributed RAID / Raft без глобальних згадок у коментарях. |
| **2** | **Priority 2** | Сервіси **`RaidService`**, **`VmService`**, **`LibraryService`**, enterprise/cloud/admin — у проді; REST `/raid/*` через **`RaidService`**. Опційно: **`raid_distributed_handlers`** vs сервісний шар. |
| **3** | **Priority 2b** | TurboQuant **фаза 1** ✅. Далі: **wire-reплікація** + порівняння розміру артефакта до/після TQ01 на стенді; **P4** / SIMD за потреби. |
| **4** | **Priority 3** | **REST/enterprise/raid закрито** для узгодженого JSON (`api_json_error`, `enterprise_err`, `raid_api_err`, …). **Також**: `auth.rs`, **`ws.rs`** (upgrade + WS error payload), **`rate_limit.rs`**. Опційно: уточнення статусів для `ResourceError` / not-found. |
| **5** | **Priority 4** | Hot-path профілювання, **бенчмарки** (у т.ч. після TurboQuant для артефактів/RAID). |
| **6** | **Priority 5** | **Закрито (концепт):** архівні плани + інвентар TODO у `src/`; optional `cloud-sdk` доробки окремо. |
| **7** | **Priority 6** | Grid / Job / Memory / Solana **концепти** у `docs/` — зроблено; код/on-chain прототип — за потреби. |

*Опційно паралельно з 1–2*: стабілізація `cargo test --all-features` на Windows (GNU toolchain / розбиття тестів) — не блокує рядок 1–3, але зменшує фрикцію CI.

---

## ⭐ Priority 1 — Єдиний `AppState` / `ApiContext` (централізований global state + DI)

**Мета**: одна точка входу для залежностей (конфіг, storage, менеджери модулів, клієнти), у стилі `AppState` в Axum‑проєктах.

**Кроки**:
- [x] Створити структурований `AppState` / `ApiContext`:
  - [x] Окремий модуль `src/core/state.rs` з `AppState`.
  - [x] Ввести alias `ApiContext = Arc<AppState>` для HTTP та сервісного шару.
- [ ] Замінити розрізнаний доступ до синглтонів на інʼєкцію через цей контекст:
  - [x] Протягнути `ApiContext` у `network::start_server` і підʼєднати до `Router` як state.
  - [x] В HTTP‑шарі приймати `ApiContext` замість глобалів — **зроблено** для основного REST (`src/network/api/*` + enterprise + WS). **`api_legacy.rs` прибрано** (не використовувався). **`discovery.rs`**: метрики на announce через **`Option<Arc<RwLock<InstanceManager>>>`**, передається з `AppState::instance_manager` у `start_server` (після `attach_core_http_singletons` у `main`). Деталі вже реалізованого:
    - `UserManager`, `oauth2_pending_states`, `discovery` slot, `ws_manager`.
    - Enterprise: `tenant_manager`, `audit_logger`, `enterprise_monitoring_manager`, `security_manager` + `sync_enterprise_globals()` у `main`.
    - Core: `pool`, `raid_manager`, `vm_manager`, `library_manager`, `instance_manager`, `topology_manager` — `OnceLock` у `AppState`, **`attach_core_http_singletons()`** після ініціалізації модулів у `main`.
    - Handler’и: workers, raid, raid distributed, vm, libraries, instances, completions, topology, enterprise API, UI dashboards — **`State<ApiContext>`**.
    - Модульні **`get_global_*`** лишаються для старту, фонових задач і тестів; HTTP узгоджений з тими самими `Arc`, що виставляє `main`.
  - [x] У тестах: feature **`test-utils`** — `AppState::attach_*_for_test` для прикріплення `Arc` без повного `main` (див. `core/state.rs`).
- [x] Оновити відповідні розділи в:
  - [x] `docs/ARCHITECTURE_REVIEW.md` — розділ `AppState` / `ApiContext`.
  - [x] `docs/development/DEVELOPMENT_PLAN_UPDATED.md` — посилання на цей план.

**Критерії готовності**:
- [ ] Всі публічні HTTP‑handler’и отримують залежності через `AppState/ApiContext`.
- [ ] Інтеграційні тести можуть підняти сервер із тестовим `AppState` без глобальних синглтонів.

---

## ⭐ Priority 2 — Service Layer поверх доменних модулів

**Мета**: чітко відокремити:
- HTTP‑handler’и (`network/api/*`) ⟶ тільки транспорт (парсинг запиту / формування відповіді),
- **сервісний шар** (`services/*`) ⟶ бізнес‑логіка,
- доменні модулі (`raid/*`, `vm/*`, `enterprise/*`, `cloud/*`, `core/*`) ⟶ примітиви та інфраструктура.

**Кроки**:
- [x] Створити `src/services/` з сервісами (нарощувати далі):
  - [x] `raid_service.rs`: list nodes/artifacts; далі артефакти, реплікація, метрики.
  - [x] `vm_service.rs`: усі VM HTTP-операції через сервіс (instances lifecycle, health, templates, networks).
  - [x] `library_service.rs`: list/get/install/uninstall/update/upload бібліотек.
  - [x] `enterprise_service.rs`: multi‑tenancy, audit, monitoring, security (OAuth2/SAML/policies + OAuth/SAML старт і callback-виклики до `SecurityManager` через сервіс) — HTTP → `EnterpriseService` ✅.
  - [x] `cloud_service.rs`: операції з провайдерами та Kubernetes‑оператором (див. `AppState::cloud_manager`, `services/cloud_service.rs`).
  - [x] `admin_service.rs`: агрегація даних для адмін‑панелі (`GET /api/v1/admin/overview`, дашборд `/ui/admin`).
- [ ] Поступово мігрувати логіку з `network/api/*.rs` у відповідні сервіси:
  - [ ] Handler’и роблять мінімум: екстрактують вхідні дані, викликають метод сервісу, маплять результат у HTTP‑відповідь.
  - [ ] Сервіси отримують залежності через `AppState/ApiContext`.
- [ ] Додати короткий опис service‑шару в:
  - [x] `docs/ARCHITECTURE_BEST_PRACTICES.md` (дерево `src/services/`),
  - [x] `docs/development/DEVELOPMENT_PLAN_UPDATED.md` (посилання на покровий план).

**Критерії готовності**:
- [ ] Кожен основний домен (RAID, VM, Cloud, Enterprise, Admin) має сервісний модуль.
- [ ] Handler‑файли читабельні як thin‑контролери (без складної бізнес‑логіки).
- [ ] Сервіси можна тестувати окремо від HTTP‑шару.

---

## ⭐ Priority 2b — Welcome TurboQuant (ML data-plane, **лише Rust**)

**Мета**: зменшити **обсяг ML-даних** (KV cache, ваги, векторні блоки) за ідеями **TurboQuant / PolarQuant / QJL** (Google Research), **без Python і без зовнішніх інтерпретаторів** — увесь код у дереві `poolai` на Rust.

**Чому після Priority 2 / бекендів pipeline**: стабільний **Rust-executor** кроків і формат артефактів спрощує підключення нового кроку або гілки в `Quantization`. Деталі: `docs/ml/TURBOQUANT_INTEGRATION.md`.

**Кроки**:
- [x] Специфікація формату **`TQ01`** + юніт-тести в **`src/ml/turboquant.rs`** (див. також `docs/ml/TURBOQUANT_INTEGRATION.md`).
- [x] Модуль **`src/ml/turboquant.rs`** — Rust-only, без subprocess.
- [x] Контракт кроку pipeline: гілка **`Quantization`** з конфігом `turboquant` / `quantization=turboquant`; метрики в результаті кроку.
- [x] Інтеграційний тест **`tests/ml_pipeline_integration.rs`** (`test_pipeline_turboquant_quantization_step`) при `--features ml`.
- [x] Проксі замірів replication control-plane: Criterion-група **`raid_replication_engine`** у `benches/runtime_benchmarks.rs` (див. `docs/performance/BENCHMARKS.md`).
- [x] In-tree **HTTP wire harness** для distributed `PutArtifact`: **`Cargo.toml`** `[[test]] distributed_raid_wire_integration` (`--features test-utils`; з **`ml`** — порівняння розміру JSON TQ01 vs сирий f32); команди в **`docs/performance/BENCHMARKS.md`** (секція P2b).
- [ ] Повні заміри реплікації артефактів по мережі та порівняння розміру даних до/після TurboQuant/TQ01 на одному стенді (**Priority 4** / LAN-стенд; harness лише готує шлях handler+serde).
- [ ] Опційно: SIMD / прискорений підшлях у Rust.

**Критерії готовності**:
- [x] Увімкнення TurboQuant керується конфігом кроку pipeline (і feature **`ml`**).
- [x] Є автоматизовані тести модуля та шляху pipeline (`cargo test`, без зовнішніх binary).
- [x] Метрики стиснення у відповіді pipeline: крок **Quantization** з TurboQuant заповнює `StepResult.output` (`bytes_in`, `bytes_out`, `compression_ratio`, `max_abs_recon_error`, …); після **`POST .../execute`** вони доступні в тілі **`GET .../pipeline/{id}`** (enterprise **`ai_ml`**). Опційно пізніше: окремий UI-дашборд під ці поля.

---

## ⭐ Priority 3 — Структурований контекст помилок (`ErrorContext` + `AppError`)

**Мета**: єдина “мова помилок” для всього бекенда, сумісна з HTTP‑шаром і логуванням, у стилі продакшн Axum‑сервісів.

**Кроки**:
- [x] Ввести доменний тип помилки `AppError` у `src/core/error.rs` (коди через `error_code()`, `thiserror`).
- [x] Додати `ErrorContext` (`operation`, `resource`, `resource_id`, `details`, `hint`) та хелпери `with_*`.
- [x] Базова конверсія в HTTP JSON у `src/network/api/common.rs`: `api_error_response`, **`api_json_error`**, `http_status_for_app_error`; RBAC — `AppError::Forbidden` + `api_error_response`.
- [x] Покрити **основний** публічний REST узгодженим форматом `{ "error": { "code", "message" }, "context"?: … }`: `src/network/api/*` (у т.ч. `raid.rs`, `ui`, `users`, `system`, `completions`, `raid_admin`), **повний** `enterprise_api.rs` (+ попередні модулі з попередніх комітів).
- [x] **`src/network/auth.rs`** (логін, middleware) — `api_json_error` + `ErrorContext` (узгоджено з `json_errors.rs`).
- [ ] За потреби спростити handler’и до `Result<T, AppError>` там, де ще ручні tuple‑відповіді.

**Критерії готовності**:
- [x] Усі **основні** HTTP‑модулі в `network/api/` та enterprise router використовують `api_json_error` / `api_error_response` / `ErrorContext` для помилок.
- [x] HTTP‑шар узгоджений з **`auth.rs`** (структуровані помилки); **WS** (`websocket_handler` upgrade, невідомий `message_type` у payload) та **rate limit** 429 — той самий формат `error` / `context`.
- [x] Для шляхів через `api_error_response` / `api_json_error` — структуровані логи (код, контекст, рівень за класом статусу).

---

## ⭐ Priority 4 — Performance Profiling & Benchmarks (RAID / VM / Cloud / API)

**Мета**: перейти від одноразових оптимізацій до **регулярного профілювання** та вимірювань для ключових шляхів.

**Кроки**:
- [x] Визначити hot‑paths (на базі наявних доків і інтуїції архітектора):
  - [x] RAID: **локальний** `put_artifact` — Criterion у `benches/runtime_benchmarks.rs` (`raid_local_put`).
  - [x] RAID: **проксі мережевого шару** — serde `PutArtifactPayload` (`raid_protocol_put_payload`); реальна реплікація з пірами — окремий harness.
  - [x] RAID replication **control-plane** — `raid_replication_engine` у `runtime_benchmarks` (`select_replication_nodes`, `calculate_quorum`).
  - [x] VM: **ін‑процес** lifecycle — `vm_lifecycle` у `runtime_benchmarks` (не hypervisor).
  - [x] Cloud: **`cloud_benchmarks`** (`--features cloud`) — validate + init/shutdown менеджера (SDK виклики не в бенчі).
  - [x] API: **проксі** JSON health — `http_health_json` у `runtime_benchmarks`; RPS — `wrk` вручну.
- [x] Створити або оновити бенчмарки (інкремент 2026‑04):
  - [x] Criterion: `runtime_benchmarks` (memory pool, LRU, model request, cache key, local RAID put, **VM**, **RAID protocol**, **health JSON**), `turboquant_benchmarks` (`--features ml`), **`cloud_benchmarks`**, **`service_layer_benchmarks`** (`test-utils`).
- [x] Оновити `docs/performance/BENCHMARKS.md` — команди `cargo bench`, групи Criterion, приклад CI; ілюстративні таблиці позначені як неконтрольні CI.
  - [x] Перші **числові** рядки (dev sample) у таблиці baseline — замінити на референс‑хост, коли буде стенд.
  - [x] Задокументувати зміни після оптимізацій (розділ *Changelog* у `BENCHMARKS.md`).

**Критерії готовності**:
- [x] Є повторюваний сценарій **локально**: `cargo bench -j 1 --bench runtime_benchmarks` та `cargo bench -j 1 --bench turboquant_benchmarks --features ml` (+ опційні `cloud_benchmarks`, `service_layer_benchmarks`).
- [x] CI для Criterion — **опційно**: `.github/workflows/benchmarks.yml` (`workflow_dispatch` + неділя 06:00 UTC), артефакт `target/criterion/`; baseline у `BENCHMARKS.md` — **оновлювати** вручну після зміни коду або референс‑машини.
- [x] Для основних сценаріїв зафіксовані **цільові** метрики (таблиця *Target metrics* у `BENCHMARKS.md`; поруч із dev/ref рядками).

---

## ⭐ Priority 5 — Cleanup, TODOs, оновлення документації

**Мета**: синхронізувати код і документацію **після** виконання пріоритетів 1–4.

**Кроки**:
- [x] Інвентар **TODO/FIXME** у `src/*.rs` (2026-04-06): лише **`cloud/providers/azure.rs`** (3 маркери: credential/compute/location), **`cloud/providers/gcp.rs`** (1 — майбутній crate); **`core/model_interface.rs`** — `todo!()` тільки всередині **згорнутого** rustdoc-прикладу. У виконуваному прод-коді **`todo!()` / `unimplemented!()` немає**.
- [ ] Опційно пізніше: реалізувати відкладені пункти Azure/GCP SDK (feature **`cloud-sdk`**) — не блокує основний CI-матрицю.
- [x] Архівні зрізи планів **примарковані** посиланням на покровий план:
  - [x] `RUST_ARCHITECT_STATUS_2026-01-19.md` — банер → `NEXT_STEPS_ARCHITECT_2026-03-17.md`.
  - [x] `RUST_ARCHITECT_STATUS_2026-01-21.md` — те саме.
  - [x] `RUST_ARCHITECT_NEXT_STEPS_2026-01-19.md` — те саме.
  - [x] `docs/status/PERCENTAGE_PLAN.md` — те саме + `STABLE_STATE_SUMMARY.md`.
- [x] Оновити або доповнити (інкремент 2026‑04):
  - [x] `docs/development/DEVELOPMENT_PLAN_UPDATED.md` — зріз стану + посилання на цей план, `BENCHMARKS.md`, витяг функціоналу.
  - [x] `docs/status/STABLE_STATE_SUMMARY.md` — канонічні посилання (HANDOFF, Architect plan), CI/бенчі.
  - [x] `docs/ARCHITECTURE_REVIEW.md` — розділ про perf‑цикл і Criterion (доповнення до наявного блоку AppState / сервісний шар).

**Критерії готовності**:
- [x] Покровий план, HANDOFF, STABLE_STATE, ARCHITECTURE_REVIEW (інкремент 2026‑04) та **архівні** `RUST_ARCHITECT_*` / `PERCENTAGE_PLAN` (банер на Architect plan) узгоджені з поточною моделлю робіт.
- [x] Залишкові **TODO** у дереві `src/` локалізовані (див. кроки вище); розширення cloud-sdk винесено в опційний пункт без вимоги до поточного релізу.

---

## ⭐ Priority 6 — Grid / Job / Memory / Tokenization (концептуальний шар)

**Мета**: накрити вже реалізоване продакшн‑ядро (15 модулів, RAID/VM/Cloud/ML/Enterprise) єдиною моделлю `PoolAI Node` + Grid/Job/Memory/Token‑шарів, не ламаючи існуючий код.

**Кроки**:
- [x] Описати `PoolAI Node` як building block Grid‑мережі:
  - [x] Створити `docs/concept/POOLAI_GRID_NODE.md` з ролями `miner` / `hub` / `hybrid`.
- [x] Описати Memory Layer поверх RAID/ML:
  - [x] Створити `docs/concept/POOLAI_MEMORY_LAYER.md` з моделлю “AGI‑памʼяті” та seeds‑поведінкою (аналог торентів).
- [x] Формалізувати Job / Mining Layer в концептах:
  - [x] Додати окремий development‑док `JOB_LAYER_CONCEPT_2026-03-17.md` з описом AI‑Job (ресурси, дедлайни, тип задачі, верифікація) і життєвим циклом `submitted → scheduled → executed → verified → rewarded`.
- [x] Описати Grid Protocol (поверх Discovery/RAID):
  - [x] Типи повідомлень: `Job`, `Result`, `MemoryShard`, `PeerStatus` — див. [`GRID_PROTOCOL_CONCEPT_2026-04-06.md`](GRID_PROTOCOL_CONCEPT_2026-04-06.md).
  - [x] Звʼязок із peer/discovery API (`/api/v1/discovery/*`, `DiscoveryMessage`), distributed RAID (`/raid/distributed/*`), тестами [`tests/grid_network_scalability_tests.rs`](../../tests/grid_network_scalability_tests.rs).
- [x] Визначити Solana‑adapter як окремий шар:
  - [x] У docs описати події `JobCompleted`, `SeedProvided`, `MemoryUpdated` → on‑chain семантика — див. [`SOLANA_ADAPTER_CONCEPT_2026-04-06.md`](SOLANA_ADAPTER_CONCEPT_2026-04-06.md).
  - [x] Відокремлення core‑runtime (Rust) від billing/tokenization‑адаптера — таблиця меж і принципи в тому ж документі.

**Критерії готовності**:
- [x] Окремі узгоджені концепт‑ та development‑доки: PoolAI Node, Grid Protocol, Job Layer, Memory Layer, Solana‑adapter (реалізація програми на Solana — поза scope концептів).
- [x] Документи посилаються на наявні модулі / API / тести де доречно; суперечностей з базовим концептом немає (он-chain деталі — TBD до прототипу).

---

## 🧩 Взаємозалежності між кроками

```text
AppState / ApiContext (Priority 1)
    ↓
Service Layer (Priority 2)
    ↓
TurboQuant / ML data-plane (Priority 2b) — після контракту бекендів pipeline
    ↓
Error Context (Priority 3)
    ↓
Performance Profiling (Priority 4)
    ↓
Docs & Cleanup (Priority 5)
    ↓
Grid / Job / Memory / Tokenization (Priority 6)
```

**Примітка**: Кроки 3–4 можуть виконуватись частково паралельно для різних модулів, але **AppState і Service Layer бажано стабілізувати першими**, щоб не плодити дублікати патернів. **Priority 2b** (TurboQuant) стартує після **стабільних Rust-бекендів кроків pipeline** і контракту артефактів (усі виклики — з коду Rust).

---

## Верифікація 2026-04-05 (Cursor, toolchain, Git, тести)

**Останні коміти на `main` (орієнтир, `git log`)**: **Priority 1** — `AppState` + модульний HTTP; видалено `api_legacy.rs`; `DiscoveryService` з інжектованим `instance_manager` з `AppState`. **Priority 2 (частково)**: `src/services/` — `raid_service` (list nodes/artifacts), `vm_service` (усі VM-маршрути), `library_service` (усі `/libraries/*`); відповідні handlers у `network/api/*` через сервіси.

**2026-04-03**: `UserManager` у `AppState`. **2026-04-04–05**: discovery + OAuth2 pending у `AppState`; інтеграційні тести `tests/network_api_integration.rs` для VM/RAID list — **не 404** (типово **503** без менеджерів на `ApiContext::default()`).

**Cursor / правила**: каталог `.cursor/rules/` (`rust-architect.md`, `ai-assistant.md`, `chat-context.md`, `.cursorrules`, …). Оновлено `rust-architect.md`: канонічний push через зовнішній MSYS2; агент/CI можуть використовувати PowerShell; опис Dependabot; узгодження з `cargo test` як у CI; примітка про MSVC vs `rust-toolchain.toml` (GNU).

**Toolchain (локально, Windows)**: `rustc`/`cargo` **1.92.0**; `rust-toolchain.toml` — GNU target + clippy/rustfmt. Перевірка: `rustup show` (за потреби `rustup override set 1.92.0-x86_64-pc-windows-gnu`).

**Dependabot**: `.github/dependabot.yml` — **cargo** та **github-actions**, weekly Monday 09:00 UTC; відкриті PR залежностей дивитись на GitHub (label `dependencies`).

**Тести (обовʼязковий набір як у CI)**:  
`K8S_OPENAPI_ENABLED_VERSION=1.28`  
`cargo test --lib --tests --features ml,enterprise,cloud` — проганяти після змін; раніше виправлено `tests/ml_pruning_integration.rs`, `tests/saml_auth_flow_integration.rs`; **2026-04-04+** — для `GET /vm/instances`, `GET /raid/nodes`, `GET /raid/artifacts`: маршрут існує (≠ 404); 200 не обовʼязково без ініціалізованих менеджерів.

**Наступні кроки розробки (коротко)**:
1. Таблиця **«Наступні кроки за пріоритетом»** на початку цього файлу — головний порядок робіт.  
2. **Priority 1**: документація AppState + **`test-utils`** — зроблено; опційно — Raft/distributed без глобальних згадок.  
3. **Priority 2**: далі **`enterprise_service`**, **`cloud_service`**, розширення **RaidService** (операції крім list); ML pipeline step backends (реальні Rust-бекенди кроків). VM / libraries / RAID list — уже через сервіси.  
4. **TurboQuant** — Priority 2b (`docs/ml/TURBOQUANT_INTEGRATION.md`).

---

## Handoff — остання сесія (2026-04-05): ApiContext + service layer

**Концепція (коротко)**: **`ApiContext` = `Arc<AppState>`** у Axum; HTTP залежить від полів `AppState` + `State<T>`. Пізнє підключення підсистем — **`OnceLock`** на `AppState`, **`attach_core_http_singletons()`** / **`sync_enterprise_globals()`** у `main`.

**Що вже на `main` (перевірка: `git log`)**:
- WebSocket, enterprise HTTP, core HTTP — через `ctx` / `AppState` (як раніше).
- **Сервіси**: `VmService`, `LibraryService`, `RaidService` (list nodes/artifacts); відповідні API-модулі викликають сервіси, не прямі синглтони в handler’ах для цих маршрутів.
- Документація: узгоджено **README**, **`docs/README.md`**, **`docs/INDEX_2026-03-17.md`**, **`file_list.csv`** (прибрано зіпсовані рядки).

**Історично (на момент цього зрізу)** — наступним були P2/P2b; актуальний фокус див. таблицю на початку файлу та **останню** секцію «Верифікація» внизу.

**Перевірка збірки (як у недавніх CI-прогонах)**:
`K8S_OPENAPI_ENABLED_VERSION=1.28`  
`cargo check --all-targets --features ml,enterprise,cloud`  
`cargo test --lib --tests --features ml,enterprise,cloud` (за потреби розширити).

---

## Верифікація 2026-04-06 (документація + стан гілки `main`)

- **Доки / інвентар**: оновлено кореневий [`README.md`](../../README.md) (блок статусу та Next Focus), [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md), [`INDEX_2026-03-17.md`](../INDEX_2026-03-17.md), [`docs/README.md`](../README.md); у [`file_list.csv`](../../file_list.csv) додано `src/ml/turboquant.rs`.
- **Код (вже в історії `main`)**: розширений `RaidService`; ML pipeline core steps + TurboQuant; частковий P3 — `api_error_response` для RAID operation errors та enterprise AI-ML pipeline handlers.
- **Наступні кроки (актуалізовано пізніше)**: див. верифікацію 2026-04-07 та **2026-04-06 (P3 повний)** нижче.

## Верифікація 2026-04-07 (P3 розширення + доки + тести)

- **Код (коміт на `main`)**: P3 — `api_json_error`, `AppError::Forbidden`; handlers: `instances`, `libraries`, `vm`, `workers`, `topology`, `rewards`, tenant CRUD у `enterprise_api.rs` (поруч із уже наявними RAID operation + `ai_ml` pipeline).
- **Тести**: `K8S_OPENAPI_ENABLED_VERSION=1.28` `cargo test --lib --tests --features ml,enterprise,cloud` — успішний прогін з `-j 1` та `--test-threads=1` при обмеженій пам’яті на Windows.
- **Документація**: узгоджено README, INDEX, HANDOFF, NEXT_STEPS, `docs/README.md`, `development/README.md`; інвентар `file_list.csv` без нових шляхів (ключові файли P3 уже в списку).

## Верифікація 2026-04-06 (P3 — `raid.rs` + повний `enterprise_api.rs`)

- **Код (`main`)**: узгоджені JSON-помилки для **`src/network/api/raid.rs`** (`raid_api_err`, `raid_event_store_unavailable`, …) та **`src/network/enterprise_api.rs`** (хелпер **`enterprise_err`**, security / OAuth / SAML / monitoring / tenant тощо). Раніше в тому ж напрямку: `users`, `ui`, `system`, `completions`, `raid_admin`.
- **Залишок P3**: **`src/network/auth.rs`** (плоский `"error"` у відповідях) — див. кореневий README «Next Focus» та [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md).
- **Наступний фокус**: **P4** (бенчмарки / профілювання), опційно **P2** (RAID → `RaidService`), оновлення чекбоксів **TurboQuant** у цьому файлі під наявний код.

## Верифікація 2026-04-06 (P4 — Criterion targets)

- **Код**: `benches/runtime_benchmarks.rs` — групи `vm_lifecycle`, `raid_protocol_put_payload`, `http_health_json`; `benches/cloud_benchmarks.rs` (`--features cloud`); `benches/service_layer_benchmarks.rs` (`--features test-utils`, `RaidService`); реєстрація в кореневому `Cargo.toml`.
- **Документація**: `docs/performance/BENCHMARKS.md` — команди, групи, таблиця під baseline, опційні рядки CI.
- **Залишок P4**: заповнити baseline після прогону; end-to-end HTTP — `wrk` / `hey` (див. той самий документ).

## Верифікація 2026-04-06 (P4 — baseline sample + план)

- **Прогін**: `runtime_benchmarks` з `--sample-size 20 --warm-up-time 0.3 --measurement-time 0.5` (Windows, release); медіани занесені в `BENCHMARKS.md` як **dev-win-sample**.
- **`service_layer_benchmarks`**: виправлено панік «no reactor running» — `AppState::new()` під `Runtime::enter()`.
- **Доки**: `NEXT_STEPS` — оновлені чекбокси P3 (auth), P4 (hot-path / бенчі); README «Next Focus».

## Верифікація 2026-04-06 (P6 — Grid Protocol concept)

- **Документ**: [`development/GRID_PROTOCOL_CONCEPT_2026-04-06.md`](GRID_PROTOCOL_CONCEPT_2026-04-06.md) — типи повідомлень `Job`, `Result`, `MemoryShard`, `PeerStatus`; мапінг на `/api/v1/discovery/*`, `DiscoveryMessage`, `/raid/distributed/*`, `tests/grid_network_scalability_tests.rs`.
- **Оновлено**: [`concept/POOLAI_GRID_NODE.md`](../concept/POOLAI_GRID_NODE.md) (посилання на Grid Protocol), [`INDEX_2026-03-17.md`](../INDEX_2026-03-17.md), [`catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`](../catalog/FUNCTIONALITY_DIGEST_2026-04-06.md), `file_list.csv`.
- **Залишок P6**: Solana‑adapter (on‑chain mapping), опційно єдиний wire envelope для Grid.

## Верифікація 2026-04-06 (P6 — Solana adapter concept)

- **Документ**: [`development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md`](SOLANA_ADAPTER_CONCEPT_2026-04-06.md) — межі core vs адаптер; мапінг `JobCompleted` / `SeedProvided` / `MemoryUpdated`; варіанти інтеграції (sidecar, черга, pull).
- **P6 критерії готовності (концепт)**: закрито; наступний горизонт — прототип on-chain програми та schema подій core↔adapter.
- **Залишок**: єдиний Grid wire envelope; реальний Solana crate / repo.

## Верифікація 2026-04-06 (P5 — архівні плани)

- **Банер** на покровий план додано до: `RUST_ARCHITECT_STATUS_2026-01-19.md`, `RUST_ARCHITECT_STATUS_2026-01-21.md`, `RUST_ARCHITECT_NEXT_STEPS_2026-01-19.md`, `docs/status/PERCENTAGE_PLAN.md`.
- **Таблиця пріоритетів** (рядок Priority 5) оновлена під цей інкремент.

## Верифікація 2026-04-06 (P5 — інвентар TODO у `src/`)

- **Перевірка**: пошук по `src/**/*.rs` на `TODO`, `FIXME`, `todo!(`, `unimplemented!(` (наприклад ripgrep) — маркери лише в `cloud/providers/{azure,gcp}.rs` та rustdoc у `model_interface.rs`; виконуваних заглушок немає.
- **Критерії P5** (доки + інвентар коду): закриті; **cloud-sdk** залишається дорожньою картою optional features.

## Верифікація 2026-04-06 (README — Next Focus після P5/P6)

- Оновлено блок **Next Focus** у кореневому [`README.md`](../../README.md): P5/P6 закриті на рівні доків; пріоритетний горизонт — **P4**, **P2b** (стенд), опційно **P2** distributed / **P3** / **P1**.
- Таблицю **«Наступні кроки за пріоритетом»** у цьому файлі узгоджено з тим самим зрізом (рядки P2 / P2b).

## Верифікація 2026-04-06 (документація — таксономія та ML-тести)

- **Таксономія `docs/`**: оновлено [`docs/STRUCTURE.md`](../STRUCTURE.md) (канонічні точки входу, каталоги, політика щодо плоских `docs/*.md`, Cursor rules, інвентар, примітка про doctests на Windows).
- **Індекси**: [`docs/README.md`](../README.md), [`docs/INDEX_2026-03-17.md`](../INDEX_2026-03-17.md), [`docs/development/HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md), кореневий [`README.md`](../../README.md) — посилання на `STRUCTURE.md` та `.cursor/rules/documentation.md`.
- **Тести**: у `Cargo.toml` додано `[[test]]` з `required-features = ["ml"]` для інтеграцій **`ml_*`** (`automl`, `experiments`, `federated`, `pipeline`, `pruning`, `versioning`), щоб **`cargo test`** без `--features ml` не компілював optional `poolai::ml`.

