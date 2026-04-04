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

## ⭐ Priority 1 — Єдиний `AppState` / `ApiContext` (централізований global state + DI)

**Мета**: одна точка входу для залежностей (конфіг, storage, менеджери модулів, клієнти), у стилі `AppState` в Axum‑проєктах.

**Кроки**:
- [x] Створити структурований `AppState` / `ApiContext`:
  - [x] Окремий модуль `src/core/state.rs` з `AppState`.
  - [x] Ввести alias `ApiContext = Arc<AppState>` для HTTP та сервісного шару.
- [ ] Замінити розрізнаний доступ до синглтонів на інʼєкцію через цей контекст:
  - [ ] Протягнути `ApiContext` у `network::start_server` і підʼєднати до `Router` як state.
  - [ ] В HTTP‑шарі (`src/network/api/*.rs`) приймати `ApiContext` (через state/extractors) замість окремих глобальних доступів.
  - [ ] У тестах створювати lightweight‑версію `ApiContext` для ізольованого тестування handler’ів/сервісів.
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
Error Context (Priority 3)
    ↓
Performance Profiling (Priority 4)
    ↓
Docs & Cleanup (Priority 5)
    ↓
Grid / Job / Memory / Tokenization (Priority 6)
```

**Примітка**: Кроки 3–4 можуть виконуватись частково паралельно для різних модулів, але **AppState і Service Layer бажано стабілізувати першими**, щоб не плодити дублікати патернів.

---

## Верифікація 2026-04-03 (Cursor, toolchain, Git, тести)

**Останні коміти на `main` (орієнтир)**: ML pipeline / enterprise AI-ML API, federated/automl, документація та `.cursor` (див. `git log`).

**Cursor / правила**: каталог `.cursor/rules/` (`rust-architect.md`, `ai-assistant.md`, `chat-context.md`, `.cursorrules`, …). Оновлено `rust-architect.md`: канонічний push через зовнішній MSYS2; агент/CI можуть використовувати PowerShell; опис Dependabot; узгодження з `cargo test` як у CI; примітка про MSVC vs `rust-toolchain.toml` (GNU).

**Toolchain (локально, Windows)**: `rustc`/`cargo` **1.92.0**; `rust-toolchain.toml` — GNU target + clippy/rustfmt. Перевірка: `rustup show` (за потреби `rustup override set 1.92.0-x86_64-pc-windows-gnu`).

**Dependabot**: `.github/dependabot.yml` — **cargo** та **github-actions**, weekly Monday 09:00 UTC; відкриті PR залежностей дивитись на GitHub (label `dependencies`).

**Тести (обовʼязковий набір як у CI)**:  
`K8S_OPENAPI_ENABLED_VERSION=1.28`  
`cargo test --lib --tests --features ml,enterprise,cloud` — **успішно** після виправлень: `tests/ml_pruning_integration.rs` (семантика `weights_after`), `tests/saml_auth_flow_integration.rs` (унікальні імена провайдерів для глобального `SecurityManager`).

**Наступні кроки розробки (коротко)**:
1. Пріоритети 1–2 з цього документа (`ApiContext` у всіх handlers, service layer).  
2. Stage 4.4 — бекенди кроків ML pipeline + тести навколо enterprise AI/ML HTTP API.  
3. За потреби — відновити/спростити повний `cargo test --all-features` на Windows (окремі крейти або GNU-only).

