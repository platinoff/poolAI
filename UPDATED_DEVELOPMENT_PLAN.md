# 🏗️ Оновлений план розробки - Rust Architect Perspective

**Дата**: 2025-12-17  
**Статус**: 🚧 **АКТИВНА РОЗРОБКА**  
**Поточний етап**: Stage 3 - Completion & Stabilization (Libs + VM/RAID/UI scaffolds)

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

### 🚧 В розробці

- 🚧 **Libs Module** (~80% готово)
  - ✅ Базова структура (mod.rs, manager.rs, registry.rs, versioning.rs, dependencies.rs)
  - ✅ API endpoints інтегровані
  - ✅ Semantic versioning
  - ✅ Dependency resolution (базова реалізація)
  - ✅ Завантаження/розпакування (HTTP stream, tar/zip, sha256)
  - 🔄 Atomic install + manifest (production-min)
  - 🔄 Повна інтеграція з model_interface (compat checks, auto-update policy)
  - 🔄 Тестування (unit + integration)

### 🚧 Модулі Stage 3 (базові скелети реалізовано)

- 🚧 **VM Module** - Instance lifecycle scaffold (in-memory) + API (read-only)
- 🚧 **RAID Module** - Local artifact storage scaffold + node registry primitives
- 🚧 **UI Module** - Minimal dashboard scaffold (mounted at `/ui/`)

---

## 🎯 Стратегічний план (від менш складного до більш складного)

### Принципи виконання
- **Нульова регресія збірки**: `cargo check` має проходити на Windows GNU (MSYS2 UCRT64) без native toolchain surprises.
- **Мінімальний, але завершений вертикальний slice**: спочатку “read-only visibility”, потім “safe write paths”.
- **Feature flags для важких залежностей**: JWT/HTTPS повертаємо тільки коли toolchain стабільний.

### Пріоритет 0: Build & Toolchain Stability (постійно)
- **Windows-gnu friendly**: уникати `zstd-sys/bzip2-sys/ring` за замовчуванням.
- **Контроль features**: `zip` без default-features; `reqwest` без native-tls.
- **Ціль**: `cargo check` завжди зелений.

### Пріоритет 1: UI Module (read-only dashboard) — найшвидший value
**Мета**: дати оператору “скло” для огляду системи без ризиків запису.
- Сторінки `/ui/`: status/health/metrics/workers/libs/vm/raid
- Авто-оновлення (простий JS polling)
- Без нових важких залежностей

**Критерій готовності**
- [ ] UI показує стан системи і ключові списки (libs/vm/raid/workers)
- [ ] Немає write-операцій з UI (тільки read)

### Пріоритет 2: Libs Module (production-min)
**Мета**: зробити інсталяцію бібліотек безпечною і відтворюваною.
- Atomic install (tmp dir → rename)
- Manifest/metadata (installed versions, checksum, source URL, installed_at)
- Version constraints: повний парсинг + перевірка (без SAT solver на цьому етапі)
- Інтеграція з `model_interface`: ensure_libtorch + compat checks policy
- Тести: unit (constraints/versioning) + integration (download/extract via local fixtures)

**Критерій готовності**
- [ ] Install/Uninstall/Update працюють і є атомарними
- [ ] Manifest збережений на диску
- [ ] Мінімальні тести для критичних шляхів

### Пріоритет 3: RAID Module (local → reliable)
**Мета**: надійний локальний artifact store для libs/models.
- CRUD артефактів + індекс (manifest)
- GC/cleanup + quota
- Інтеграція: libs зберігає завантажене як artifact, runtime читає артефакти

### Пріоритет 4: VM Module (process-runner → isolation)
**Мета**: контроль запуску воркерів/моделей з життєвим циклом і базовими лімітами.
- Спочатку “process runner” на базі `runtime/process.rs` (+ статус/логи/таймаути)
- Потім ресурси (CPU/mem/gpu scheduling policy)
- Потім isolation (sandbox/containers/real VM) — окрема підфаза

### Пріоритет 5: Security (JWT/HTTPS) — складніше через toolchain
- Повернути `jsonwebtoken`/`axum-server` під feature flags
- Рекомендований шлях: або MSVC target, або гарантована наявність gcc/dlltool

### Пріоритет 6: Distributed RAID (BurstRAID/SmallWorld) — найскладніше
- Протокол, інваріанти, тест-стратегія, fault-tolerance
- Планувати як окремий етап з окремим ADR/design doc

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

## 🎯 Наступні кроки (Негайні)

1. **UI (read-only)**
   - Додати сторінки для libs/vm/raid/workers/metrics
   - Поліпшити UX (шаблон, навігація, авто-оновлення)

2. **Libs (production-min)**
   - Atomic install + manifest
   - Constraint checking + conflict reporting
   - Тести для constraints/versioning/download (fixtures)

3. **RAID (local reliable store)**
   - Artifact manifest + GC/quota
   - Інтеграція libs → raid artifacts

4. **VM (process runner)**
   - В’язка VmManager ↔ runtime/process + lifecycle events

---

**План оновлено та готовий до виконання!** 🚀

