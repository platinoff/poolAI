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
| **1** | **Priority 1** | **Майже закрито**: модульний REST + `AppState`; **`api_legacy.rs` видалено** (дублікат, не підключався до router); **`DiscoveryService`** отримує `instance_manager` з `AppState` (без `get_global_instance_manager` у announce). **Залишилось**: розширений тестовий `AppState`, коментарі/код **distributed RAID** ще згадують глобальний Raft — за потреби вирівняти; **`docs/ARCHITECTURE_REVIEW.md`**. |
| **2** | **Priority 2** | Ввести **`src/services/`**, thin handlers; паралельно **Stage 4.4** — **реальні Rust-бекенди кроків ML pipeline** (не заглушки), узгоджені з enterprise AI/ML API. |
| **3** | **Priority 2b** | **TurboQuant лише в Rust** (`src/ml/…`): спека формату, модуль, тести, потім wire у pipeline + метрики (див. `docs/ml/TURBOQUANT_INTEGRATION.md`). |
| **4** | **Priority 3** | Узгодити **`AppError` / `ErrorContext`** і HTTP-мапінг по всьому публічному API. |
| **5** | **Priority 4** | Hot-path профілювання, **бенчмарки** (у т.ч. після TurboQuant для артефактів/RAID). |
| **6** | **Priority 5** | Синхронізація документації та TODO після 1–4. |
| **7** | **Priority 6** | Grid protocol / Solana-adapter у docs і коді за потреби. |

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
  - [ ] У тестах створювати lightweight‑версію `ApiContext` для ізольованого тестування handler’ів/сервісів (`ApiContext::default()` уже для system status; розширити за потреби).
- [ ] Оновити відповідні розділи в:
  - [ ] `docs/ARCHITECTURE_REVIEW.md` (додати опис `AppState/ApiContext`),
  - [ ] `docs/development/DEVELOPMENT_PLAN_UPDATED.md` (посилання на цей план).

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
- [ ] Створити `src/services/` з сервісами:
  - [ ] `raid_service.rs`: операції з артефактами, реплікацією, метриками.
  - [ ] `vm_service.rs`: створення/зупинка/моніторинг процесів/VM.
  - [ ] `enterprise_service.rs`: multi‑tenancy, audit, SAML/OAuth2.
  - [ ] `cloud_service.rs`: операції з провайдерами та Kubernetes‑оператором.
  - [ ] `admin_service.rs`: агрегація даних для адмін‑панелі.
- [ ] Поступово мігрувати логіку з `network/api/*.rs` у відповідні сервіси:
  - [ ] Handler’и роблять мінімум: екстрактують вхідні дані, викликають метод сервісу, маплять результат у HTTP‑відповідь.
  - [ ] Сервіси отримують залежності через `AppState/ApiContext`.
- [ ] Додати короткий опис service‑шару в:
  - [ ] `docs/ARCHITECTURE_BEST_PRACTICES.md`,
  - [ ] `docs/development/DEVELOPMENT_PLAN_UPDATED.md`.

**Критерії готовності**:
- [ ] Кожен основний домен (RAID, VM, Cloud, Enterprise, Admin) має сервісний модуль.
- [ ] Handler‑файли читабельні як thin‑контролери (без складної бізнес‑логіки).
- [ ] Сервіси можна тестувати окремо від HTTP‑шару.

---

## ⭐ Priority 2b — Welcome TurboQuant (ML data-plane, **лише Rust**)

**Мета**: зменшити **обсяг ML-даних** (KV cache, ваги, векторні блоки) за ідеями **TurboQuant / PolarQuant / QJL** (Google Research), **без Python і без зовнішніх інтерпретаторів** — увесь код у дереві `poolai` на Rust.

**Чому після Priority 2 / бекендів pipeline**: стабільний **Rust-executor** кроків і формат артефактів спрощує підключення нового кроку або гілки в `Quantization`. Деталі: `docs/ml/TURBOQUANT_INTEGRATION.md`.

**Кроки**:
- [ ] Специфікація внутрішнього бінарного формату та юніт-тести (малі розмірності, похибка dot-product / recall proxy).
- [ ] Реалізація модуля **`src/ml/turboquant.rs`** (або `src/ml/turboquant/mod.rs`) — чистий Rust, без subprocess.
- [ ] Контракт кроку pipeline: конфіг у `StepType::Quantization` або окремий прапор/режим; метрики `bytes_in`, `bytes_out`, `target_bits` у результаті виконання кроку.
- [ ] Інтеграційний тест **повністю в Rust** (pipeline execute з тестовими вагами).
- [ ] Заміри реплікації артефактів RAID до/після (Priority 4 / `docs/performance/BENCHMARKS.md`).
- [ ] Опційно: SIMD / окремий прискорений підшлях у Rust (без залежності від інших мов).

**Критерії готовності**:
- [ ] Увімкнення TurboQuant керується лише конфігом Rust і feature flags проєкту.
- [ ] Є **автоматизовані** тести модуля та шляху pipeline без зовнішніх binary крім `cargo test`.
- [ ] Метрики стиснення видимі в логах або API статусу pipeline.

---

## ⭐ Priority 3 — Структурований контекст помилок (`ErrorContext` + `AppError`)

**Мета**: єдина “мова помилок” для всього бекенда, сумісна з HTTP‑шаром і логуванням, у стилі продакшн Axum‑сервісів.

**Кроки**:
- [ ] Ввести доменний тип помилки, наприклад `AppError`:
  - [ ] Розмістити в `src/core/error.rs` або аналогічному модулі.
  - [ ] Забезпечити поля для коду помилки, джерела (`source`), контексту, можливо, `hint`.
- [ ] Додати `ErrorContext`:
  - [ ] Легкий struct з полями на кшталт `operation`, `resource`, `id`, `details`.
  - [ ] Хелпери для додавання контексту при `?`‑ланцюжках.
- [ ] Налаштувати конверсію помилок у HTTP‑відповіді:
  - [ ] У HTTP‑шарі повертати `Result<T, AppError>` і імплементувати конверсію в стандартну помилку API.
  - [ ] Переконатися, що чутливі деталі не витікають у зовнішні відповіді, але є в логах.

**Критерії готовності**:
- [ ] Всі основні модулі використовують `AppError` / `ErrorContext` у публічних API.
- [ ] HTTP‑шар завжди повертає узгоджений JSON‑формат помилок.
- [ ] Логи містять достатньо контексту для розбору інцидентів.

---

## ⭐ Priority 4 — Performance Profiling & Benchmarks (RAID / VM / Cloud / API)

**Мета**: перейти від одноразових оптимізацій до **регулярного профілювання** та вимірювань для ключових шляхів.

**Кроки**:
- [ ] Визначити hot‑paths (на базі наявних доків і інтуїції архітектора):
  - [ ] RAID: реплікація артефактів, читання/запис через мережу.
  - [ ] VM: запуск/стоп/моніторинг процесів.
  - [ ] Cloud: Kubernetes/Cloud operations (operator, scaling, LB).
  - [ ] API: найбільш часті REST‑ендпоінти (admin dashboard, monitoring, artifacts).
- [ ] Створити або оновити бенчмарки:
  - [ ] Додати `benches/` для ключових операцій (RAID, VM, Cloud, API).
  - [ ] Застосовувати критерій‑бенчмарки чи еквівалентні інструменти.
- [ ] Зробити оновлення `docs/performance/BENCHMARKS.md`:
  - [ ] Зафіксувати базові метрики (latency/throughput/P95) до оптимізацій.
  - [ ] Задокументувати зміни після оптимізацій.

**Критерії готовності**:
- [ ] Є повторюваний сценарій вимірювання продуктивності (локально і/або в CI).
- [ ] Для основних сценаріїв зафіксовані цільові та фактичні метрики.

---

## ⭐ Priority 5 — Cleanup, TODOs, оновлення документації

**Мета**: синхронізувати код і документацію **після** виконання пріоритетів 1–4.

**Кроки**:
- [ ] Пройтися TODO по коду, зазначені в:
  - [ ] `RUST_ARCHITECT_STATUS_2026-01-19.md`,
  - [ ] `RUST_ARCHITECT_NEXT_STEPS_2026-01-19.md`,
  - [ ] `PERCENTAGE_PLAN.md`.
- [ ] Оновити або доповнити:
  - [ ] `docs/development/DEVELOPMENT_PLAN_UPDATED.md` — посилання на цей план і статус виконання.
  - [ ] `docs/status/STABLE_STATE_SUMMARY.md` — коротке резюме після завершення архітектурних кроків.
  - [ ] `docs/ARCHITECTURE_REVIEW.md` — новий розділ про `AppState/ApiContext`, Service Layer, ErrorContext, perf‑цикл.

**Критерії готовності**:
- [ ] Усі згадані вище документи відображають фактичний стан архітектури.
- [ ] TODO у ключових модулях або закриті, або перенесені в окремі, актуальні плани.

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
- [ ] Описати Grid Protocol (поверх Discovery/RAID):
  - [ ] Типи повідомлень: `Job`, `Result`, `MemoryShard`, `PeerStatus`.
  - [ ] Звʼязок із вже існуючими peer/discovery API та grid‑тестами RAID.
- [ ] Визначити Solana‑adapter як окремий шар:
  - [ ] У docs описати, які події (`JobCompleted`, `SeedProvided`, `MemoryUpdated`) відображаються в on‑chain події.
  - [ ] Чітко відокремити core‑runtime (Rust) від billing/tokenization‑адаптера.

**Критерії готовності**:
- [ ] Існують окремі, узгоджені концепт‑/development‑доки для `PoolAI Node`, Grid Layer, Job/Mining Layer, Memory Layer, Solana‑adapter.
- [ ] Всі нові документи посилаються на вже реалізовані модулі й тести, не суперечать існуючим концептам.

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

## Верифікація 2026-04-04 (Cursor, toolchain, Git, тести)

**Останні коміти на `main` (орієнтир, `git log`)**: серія **Priority 1** — `AppState` + модульний HTTP; **2026-04-04+**: видалено мертвий `api_legacy.rs`; `DiscoveryService::new(..., instance_manager)` з `app_state.instance_manager.get().cloned()` замість `get_global_instance_manager` у `send_announcement`.

**2026-04-03**: `UserManager` у `AppState`, без HTTP‑синглтона `get_global_user_manager`. **2026-04-04**: discovery + OAuth2 pending у `AppState`; інтеграційні тести `tests/network_api_integration.rs` для VM/RAID list очікують **не 404** (типово **503**, якщо менеджери не прикріплені до тестового `ApiContext::default()` — узгоджено з поведінкою після DI).

**Cursor / правила**: каталог `.cursor/rules/` (`rust-architect.md`, `ai-assistant.md`, `chat-context.md`, `.cursorrules`, …). Оновлено `rust-architect.md`: канонічний push через зовнішній MSYS2; агент/CI можуть використовувати PowerShell; опис Dependabot; узгодження з `cargo test` як у CI; примітка про MSVC vs `rust-toolchain.toml` (GNU).

**Toolchain (локально, Windows)**: `rustc`/`cargo` **1.92.0**; `rust-toolchain.toml` — GNU target + clippy/rustfmt. Перевірка: `rustup show` (за потреби `rustup override set 1.92.0-x86_64-pc-windows-gnu`).

**Dependabot**: `.github/dependabot.yml` — **cargo** та **github-actions**, weekly Monday 09:00 UTC; відкриті PR залежностей дивитись на GitHub (label `dependencies`).

**Тести (обовʼязковий набір як у CI)**:  
`K8S_OPENAPI_ENABLED_VERSION=1.28`  
`cargo test --lib --tests --features ml,enterprise,cloud` — проганяти після змін; раніше виправлено `tests/ml_pruning_integration.rs`, `tests/saml_auth_flow_integration.rs`; **2026-04-04** — очікування для `GET /vm/instances`, `GET /raid/nodes`, `GET /raid/artifacts` у router-only тестах: маршрут існує (≠ 404), не обовʼязково 200 без ініціалізованих менеджерів.

**Наступні кроки розробки (коротко)**:
1. Таблиця **«Наступні кроки за пріоритетом»** на початку цього файлу — головний порядок робіт.  
2. **Priority 1 (фінал)**: легкий **`ApiContext` для тестів**; **`ARCHITECTURE_REVIEW.md`**; за бажанням — прибрати згадки глобального Raft у `raid_distributed_handlers` (коли буде дизайн).  
3. **Priority 2**: `src/services/` + реальні ML pipeline step backends.  
4. **TurboQuant** — Priority 2b (`docs/ml/TURBOQUANT_INTEGRATION.md`).

---

## Handoff — остання сесія (архітектура / ApiContext)

**Концепція (коротко)**: один **`ApiContext` = `Arc<AppState>`** у Axum; залежності для HTTP — з полів `AppState` + `State<T>` у handler’ах. Пізнє підключення «великих» підсистем після їхніх `initialize()` — через **`OnceLock`** на `AppState` і виклики **`attach_core_http_singletons()`** / **`sync_enterprise_globals()`** у `main`, щоб HTTP і legacy **`get_global_*`** бачили той самий `Arc`.

**Що вже змерджено на `main` (орієнтир по змісту, перевірка: `git log`)**:
- WebSocket: `WebSocketManager` у `AppState`, без `lazy_static` у `network/ws`.
- Enterprise HTTP: менеджери в `AppState`, `enterprise_api` + UI dashboards через `ctx`, `sync_enterprise_globals()`.
- Core HTTP: `attach_core_http_singletons()`; модульні маршрути `api/` (workers, raid, vm, libraries, instances, completions, topology) + distributed RAID — через `ctx`.

**Наступній сесії**:
1. **Priority 1 — документація та тестовий harness**: `ARCHITECTURE_REVIEW.md`, опційно `DEVELOPMENT_PLAN_UPDATED.md`; розширений `ApiContext`/фабрика для інтеграційних тестів без globals.
2. **Priority 2** — `src/services/` і винесення логіки з `network/api/*`; ML pipeline steps з реальними бекендами.

**Перевірка збірки (як у недавніх CI-прогонах)**:
`K8S_OPENAPI_ENABLED_VERSION=1.28`  
`cargo check --all-targets --features ml,enterprise,cloud`  
`cargo test --lib --tests --features ml,enterprise,cloud` (за потреби розширити).

