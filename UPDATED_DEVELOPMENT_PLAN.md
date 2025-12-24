# 🏗️ Оновлений план розробки - Rust Architect Perspective

**Дата**: 2025-12-19 (Updated)  
**Статус**: 🚧 **АКТИВНА РОЗРОБКА**  
**Поточний етап**: Stage 3 - Completion & Stabilization (Libs ~95%, RAID ~70%, VM ~60%, Security ✅)

---

## 📊 Поточний стан проекту

### ✅ Завершені модулі

- ✅ **Core Module** - Base structures, config, error handling, state
- ✅ **Pool Module** - Worker pool management
- ✅ **Monitoring Module** - Metrics and monitoring
- ✅ **Network Module** - REST API and WebSocket
- ✅ **Platform Module** - GPU management
- ✅ **Runtime Module** - Advanced runtime management (Stage 4.1)
- ✅ **Rewards System** - Achievement-based rewards
- ✅ **TGBot Module** - Telegram bot integration
- ✅ **Security Module (JWT/HTTPS)** - **НОВЕ ЗАВЕРШЕННЯ** 🎉
  - ✅ JWT authentication з feature flags (`jwt`)
  - ✅ HTTPS/TLS support з feature flags (`https`)
  - ✅ Fallback authentication (base64 encoding для dev)
  - ✅ RBAC (Admin, Operator, Viewer roles)
  - ✅ Integration tests (9 tests passing)

### 🚧 В розробці

- ✅ **Libs Module** (~95% готово) - **MAJOR PROGRESS**
  - ✅ Базова структура (mod.rs, manager.rs, registry.rs, versioning.rs, dependencies.rs, constraints.rs, download.rs, manifest.rs, integration.rs)
  - ✅ API endpoints інтегровані (5 endpoints)
  - ✅ Semantic versioning
  - ✅ Dependency resolution з constraints (>=, <=, ==, ~, ^, >, <)
  - ✅ Завантаження/розпакування (HTTP stream, tar/zip, sha256 checksum)
  - ✅ Atomic install + manifest persistence (production-min) — **ЗАВЕРШЕНО**
  - ✅ Повна інтеграція з model_interface (compat checks, auto-update policy) — **ЗАВЕРШЕНО**
  - ✅ Тестування (6 unit + 4 integration = 10 tests passing)
  - 🔄 SAT solver для складних dependency conflicts (опціонально)

- ✅ **RAID Module** (~70% готово) - **MAJOR PROGRESS**
  - ✅ Local artifact storage (`/raid/artifacts`)
  - ✅ Node registry primitives (register/list)
  - ✅ Artifact manifest persistence (atomic write) + list/delete APIs
  - ✅ GC (garbage collection) для старих artifacts — **ЗАВЕРШЕНО**
  - ✅ Quota management (size-based limits) — **ЗАВЕРШЕНО**
  - ✅ Retention policies (quota_bytes, retention_days, gc_on_startup) — **ЗАВЕРШЕНО**
  - ✅ API endpoints: GET /api/v1/raid/quota, POST /api/v1/raid/gc — **ЗАВЕРШЕНО**
  - ✅ Integration tests (4 tests passing)
  - 🔄 Інтеграція libs → raid artifacts (libs зберігає завантажене як artifact)
  - 🔄 BurstRAID/SmallWorld distributed (окрема фаза)

### 🚧 Модулі Stage 3 (базові скелети реалізовано)

- ✅ **VM Module** - Instance lifecycle + Process Runner (~60% готово)
  - ✅ Basic instance management (create, start, stop, delete)
  - ✅ Resource model (cpu/memory/gpu) + isolation policy placeholders
  - ✅ Process runner інтеграція з `runtime/process.rs` — **ЗАВЕРШЕНО**
  - ✅ Process lifecycle management (spawn, stop, logs, timeouts) — **ЗАВЕРШЕНО**
  - ✅ Integration tests (5 tests passing)
  - 🔄 Resource limits enforcement (platform-specific)
  - 🔄 Health checks integration

- ✅ **UI Module** - Read-only dashboard (mounted at `/ui`)
  - ✅ 8 pages з auto-refresh (status, health, metrics, workers, libs, vm, raid)
  - ✅ Shared HTML layout з navigation
  - ✅ JavaScript auto-refresh (polling кожні 5 секунд)
  - 🔄 Write operations (через API, з авторизацією)
  - 🔄 UI components library

---

## 🎯 Стратегічний план (від більш залежного до менш залежного)

### 🔗 Dependency-Based Development Order

**Принцип**: Робимо спочатку завдання, які мають багато залежностей або блокують інші, потім переходимо до незалежних.

**Граф залежностей**:
- **Distributed RAID** ← залежить від: Local RAID (✅), Network (✅), Consensus
- **Security (JWT/HTTPS)** ← залежить від: Network (✅), Toolchain
- **UI Write Operations** ← залежить від: Network (✅), Auth (JWT)
- **Resource Limits (VM)** ← залежить від: VM Process Runner (✅), Platform APIs
- **Health Checks (VM)** ← залежить від: VM Process Runner (✅), Health Monitor (✅)
- **RAID-Libs Integration** ← залежить від: Libs (✅), RAID (✅) - **НАЙМЕНШ ЗАЛЕЖНЕ**

### Принципи виконання
- **Нульова регресія збірки**: `cargo check` має проходити на Windows GNU (MSYS2 UCRT64) без native toolchain surprises.
- **Мінімальний, але завершений вертикальний slice**: спочатку “read-only visibility”, потім “safe write paths”.
- **Feature flags для важких залежностей**: JWT/HTTPS повертаємо тільки коли toolchain стабільний.

### Пріоритет 0: Build & Toolchain Stability (постійно)
- **Windows-gnu friendly**: уникати `zstd-sys/bzip2-sys/ring` за замовчуванням.
- **Контроль features**: `zip` без default-features; `reqwest` без native-tls.
- **Ціль**: `cargo check` завжди зелений.
 - **Runtime external deps**: все, що викликається через `Command` (воркери/утиліти), має мати
   - sibling-binary path (поруч з `poolai.exe`) або
   - чіткий fallback/warn без падіння всього процесу.

### Пріоритет 1: UI Module (read-only dashboard) — ✅ ЗАВЕРШЕНО
**Мета**: дати оператору "скло" для огляду системи без ризиків запису.
- ✅ Сторінки `/ui/`: status/health/metrics/workers/libs/vm/raid
- ✅ Авто-оновлення (простий JS polling)
- ✅ Без нових важких залежностей

**Критерій готовності**
- [x] UI показує стан системи і ключові списки (libs/vm/raid/workers)
- [x] Немає write-операцій з UI (тільки read)

### Пріоритет 2: Libs Module (production-min) — ✅ ~95% ЗАВЕРШЕНО
**Мета**: зробити інсталяцію бібліотек безпечною і відтворюваною.
- ✅ Atomic install (tmp dir → rename)
- ✅ Manifest/metadata (installed versions, checksum, source URL, installed_at)
- ✅ Version constraints: повний парсинг + перевірка (>=, <=, ==, ~, ^, >, <)
- ✅ Інтеграція з `model_interface`: ensure_libtorch + compat checks policy
- ✅ Тести: unit (6 tests) + integration (4 tests для manifest persistence)

**Критерій готовності**
- [x] Install/Uninstall/Update працюють і є атомарними
- [x] Manifest збережений на диску
- [x] Мінімальні тести для критичних шляхів (10 tests passing)
- [ ] SAT solver для складних dependency conflicts (опціонально)

### Пріоритет 3: RAID Module (local → reliable) — ✅ ~70% ЗАВЕРШЕНО
**Мета**: надійний локальний artifact store для libs/models.
- ✅ CRUD артефактів + індекс (manifest)
- ✅ GC/cleanup + quota — **ЗАВЕРШЕНО**
- ✅ Retention policies (quota_bytes, retention_days, gc_on_startup) — **ЗАВЕРШЕНО**
- ✅ API endpoints для quota та GC — **ЗАВЕРШЕНО**
- ✅ Integration tests (4 tests passing)
- 🔄 Інтеграція: libs зберігає завантажене як artifact, runtime читає артефакти

### Пріоритет 4: VM Module (process-runner → isolation) — ✅ ~60% ЗАВЕРШЕНО
**Мета**: контроль запуску воркерів/моделей з життєвим циклом і базовими лімітами.
- ✅ Process runner на базі `runtime/process.rs` (+ статус/логи/таймаути) — **ЗАВЕРШЕНО**
- ✅ Integration tests (5 tests passing) — **ЗАВЕРШЕНО**
- 🔄 Resource limits enforcement (CPU/mem/gpu scheduling policy)
- 🔄 Health checks integration
- 🔄 Isolation (sandbox/containers/real VM) — окрема підфаза

### Пріоритет 5: Security (JWT/HTTPS) — ✅ ЗАВЕРШЕНО
**Залежності**: Network Module (✅), Toolchain stability — **ВИКОНАНО**
- ✅ `jsonwebtoken`/`axum-server` під feature flags (`jwt`, `https`)
- ✅ Fallback authentication (base64 encoding для dev)
- ✅ RBAC (Admin, Operator, Viewer roles)
- ✅ Integration tests (9 tests passing)
- ✅ **Більше не блокує**: UI Write Operations — **ГОТОВО ДО РЕАЛІЗАЦІЇ**

### Пріоритет 6: Distributed RAID (BurstRAID/SmallWorld) — найбільш залежне
**Залежності**: Local RAID (✅), Network (✅), Consensus, Event Sourcing
- Протокол, інваріанти, тест-стратегія, fault-tolerance
- Планувати як окремий етап з окремим ADR/design doc
- **Найбільш залежне завдання** (залежить від багатьох модулів)

## 🔧 Виправлення попереджень

### Dead Code Warnings

Всі поля з попередженнями `dead_code` позначені як `#[allow(dead_code)]` з коментарями про майбутнє використання:

- ✅ `ModelManager.config` - для конфігурації моделей
- ✅ `RuntimeManager.config` - для реконфігурації
- ✅ `Worker.task_channel` - для розподілу завдань
- ✅ `TaskQueue.capacity` - для перевірки ємності
- ✅ `CacheManager.size_mb` - для обмежень розміру
- ✅ `HealthMonitor.interval` - для планування health checks

**Архітектурне рішення**: Ці поля будуть використовуватися в наступних ітераціях розробки, тому вони залишені з `#[allow(dead_code)]` замість видалення.

---

## 📅 Timeline (оновлено: від простого до складного)

### Тиждень 1: UI read-only
- Дашборд сторінки + навігація + авто-оновлення

### Тиждень 2-3: Libs production-min
- Atomic install + manifest
- Constraint checking + conflict reporting
- Базові тести + fixtures

### Тиждень 4: RAID local reliable store
- Artifact manifest + GC/quota
- Інтеграція libs → raid

### Тиждень 5-6: VM process runner
- VmManager ↔ runtime/process + lifecycle
- Базові ліміти/таймаути/логи

### Далі: Security (JWT/HTTPS) → Distributed RAID
- Виноситься в окрему фазу через toolchain/складність

---

## 🏗️ Архітектурні принципи Rust

### 1. Zero-Cost Abstractions
- Trait-based polymorphism
- Compiler optimizations
- Zero-copy operations

### 2. Memory Safety
- Ownership and Borrowing
- `Arc<RwLock<>>` для shared mutable state
- `OnceLock` для глобальної ініціалізації
- Lifetimes для безпеки

### 3. Concurrency-First
- Async/await для I/O
- Tokio runtime
- Actor model для ізоляції стану

### 4. Type Safety
- Strong typing
- Pattern matching
- `Result<T, E>` для error handling
- `Option<T>` для nullable values

---

## ✅ Критерії готовності модулів

### Для кожного модуля:

1. **Функціональність**
   - [ ] Всі основні функції реалізовані
   - [ ] API endpoints працюють
   - [ ] Інтеграція з іншими модулями

2. **Якість коду**
   - [ ] Немає unsafe блоків
   - [ ] Thread-safe реалізація
   - [ ] Proper error handling
   - [ ] Документація Rustdoc
   - [ ] Мінімальні попередження компілятора

3. **Тестування**
   - [ ] Unit tests (>80% coverage)
   - [ ] Integration tests
   - [ ] Performance benchmarks

4. **Документація**
   - [ ] API documentation
   - [ ] Usage examples
   - [ ] Architecture decisions

---

## 🎯 Наступні кроки (Від більш залежного до менш залежного)

### Phase 1: Найбільш залежні завдання (блокують інші)

#### 1. Security (JWT/HTTPS) — ✅ ЗАВЕРШЕНО
**Залежності**: Network (✅), Toolchain — **ВИКОНАНО**
**Блокує**: ~~UI Write Operations~~ — **БІЛЬШЕ НЕ БЛОКУЄ**
**Оцінка**: ✅ ЗАВЕРШЕНО

**Завдання**:
- [x] Feature flags для `jsonwebtoken`/`axum-server`
- [x] Toolchain stability (feature flags з fallback)
- [x] Fallback authentication (base64 encoding для dev)
- [x] JWT middleware integration
- [x] Integration tests (9 tests passing)

#### 2. Resource Limits Enforcement (VM) — ⭐
**Залежності**: VM Process Runner (✅), Platform APIs
**Оцінка**: 2-3 тижні

**Завдання**:
- [ ] CPU limits (cgroups на Linux, Job Objects на Windows)
- [ ] Memory limits enforcement
- [ ] GPU scheduling policy
- [ ] Platform-specific implementations

### Phase 2: Середні залежності

#### 3. Health Checks Integration (VM)
**Залежності**: VM Process Runner (✅), Health Monitor (✅)
**Оцінка**: 1 тиждень

**Завдання**:
- [ ] Інтеграція VM instances з HealthMonitor
- [ ] Periodic health checks для running VM processes
- [ ] Auto-restart on health check failure

#### 4. UI Write Operations — ГОТОВО ДО РЕАЛІЗАЦІЇ
**Залежності**: Network API (✅), Auth (JWT) (✅) — **ГОТОВО!**
**Оцінка**: 1-2 тижні

**Завдання**:
- [ ] JWT authentication в UI (login form)
- [ ] Write endpoints з RBAC checks (create/update/delete operations)
- [ ] Confirmation dialogs для деструктивних операцій
- [ ] Error handling та user feedback

### Phase 3: Найменш залежні (можна робити паралельно)

#### 5. RAID-Libs Integration — ⭐ ПРІОРИТЕТ 1 (найменш залежне)
**Залежності**: Libs (✅), RAID (✅) - обидва готові!
**Оцінка**: 1 тиждень

**Завдання**:
- [ ] Модифікувати `libs/manager.rs::download_and_install()` для збереження artifacts в RAID
- [ ] Оновити `LibraryInfo` з `ArtifactRef`
- [ ] Runtime читає artifacts з RAID замість прямого доступу
- [ ] Integration tests для RAID-Libs integration

### Phase 4: Найскладніші (окрема фаза)

#### 6. Distributed RAID (BurstRAID/SmallWorld)
**Залежності**: Local RAID (✅), Network (✅), Consensus, Event Sourcing
**Оцінка**: 4+ тижні (окрема фаза з ADR)

**Завдання**:
- [ ] Протокол для distributed storage
- [ ] Raft consensus для consistency
- [ ] Event sourcing для auditability
- [ ] Circuit breaker pattern

---

**План оновлено та готовий до виконання!** 🚀

