# 🚀 PoolAI - Наступні кроки розробки
## Rust Architect Analysis - 2025-12-19

---

## 📊 Поточний стан

**Статус збірки**: ✅ `cargo check` проходить без помилок  
**Статус тестів**: ✅ **22 tests passing** (6 unit + 16 integration)  
**Останній коміт**: `99fed50` - fix: resolve compiler warnings for https feature and unused imports

### Завершені модулі (100%)
- ✅ Core, Pool, Monitoring, Network, Platform, Runtime, Rewards, TGBot
- ✅ **Security (JWT/HTTPS)** — **НОВЕ ЗАВЕРШЕННЯ** 🎉

### Модулі в розробці
- ✅ Libs Module (~95%) - production-ready
- ✅ RAID Module (~70%) - local reliable store
- ✅ VM Module (~60%) - process runner integrated
- ✅ UI Module (~80%) - read-only dashboard

---

## 🎯 Наступні кроки (від простого до складного)

### ⭐ Пріоритет 1: RAID-Libs Integration (1 тиждень)

**Мета**: Libs зберігає завантажене як artifact в RAID, runtime читає з RAID

**Чому це пріоритет**:
- ✅ Обидва модулі готові (Libs ~95%, RAID ~70%)
- ✅ Найменш залежне завдання
- ✅ Логічне продовження розробки
- ✅ Не блокує інші завдання

**Завдання**:
1. Модифікувати `libs/manager.rs::download_and_install()`:
   - Після успішного download/extract → зберегти як artifact в RAID
   - Використати `raid::get_global_manager().put_artifact()`
   - Оновити `LibraryInfo` з `ArtifactRef`
2. Runtime читає artifacts з RAID:
   - Модифікувати `libs/manager.rs` для читання з RAID
   - Використати `raid::get_global_manager().get_artifact()`
   - Fallback на локальний шлях якщо artifact не знайдено
3. Integration tests:
   - Тест: library install → artifact в RAID
   - Тест: runtime читає з RAID
   - Тест: fallback на локальний шлях

**Оцінка**: 1 тиждень

---

### Пріоритет 2: Resource Limits Enforcement (VM) (2-3 тижні)

**Мета**: Platform-specific resource limiting (cgroups на Linux, Job Objects на Windows)

**Залежності**: VM Process Runner (✅), Platform APIs (✅)

**Завдання**:
1. Створити `src/vm/resources.rs`:
   - `ResourceLimiter` trait з методами `apply_limits`, `get_usage`, `is_supported`
   - `ResourceLimits` struct (cpu_cores, memory_mb, gpu_device)
   - `PlatformResourceLimiter` implementation
2. Platform-specific implementations:
   - Linux: cgroups для CPU/memory limits
   - Windows: Job Objects для CPU/memory limits
   - GPU scheduling policy (загальна для обох платформ)
3. Інтеграція з VM Module:
   - `VmManager::apply_resource_limits()` викликає `ResourceLimiter`
   - `VmManager::get_instance_resource_usage()` повертає поточне використання
4. API endpoints:
   - `GET /api/v1/vm/instances/:id/resources` - поточне використання
   - `GET /api/v1/vm/resource-limits-supported` - підтримка платформи

**Оцінка**: 2-3 тижні

---

### Пріоритет 3: Health Checks Integration (VM) (1 тиждень)

**Мета**: Інтеграція VM instances з HealthMonitor для auto-restart

**Залежності**: VM Process Runner (✅), Health Monitor (✅)

**Завдання**:
1. Інтеграція з HealthMonitor:
   - `VmManager::start_instance()` реєструє health check
   - `VmManager::stop_instance()` видаляє health check
   - Periodic health checks для running VM processes
2. Auto-restart logic:
   - При health check failure → автоматичний restart
   - Максимальна кількість restarts (запобігання loop)
   - Логування всіх restart events
3. API endpoint:
   - `GET /api/v1/vm/instances/:id/health` - health status

**Оцінка**: 1 тиждень

---

### Пріоритет 4: UI Write Operations (1-2 тижні)

**Мета**: Write endpoints з JWT authentication та RBAC checks

**Залежності**: Network API (✅), Auth (JWT) (✅) — **ГОТОВО!**

**Завдання**:
1. JWT authentication в UI:
   - Login form (`/ui/login`)
   - Token storage (localStorage)
   - Token refresh logic
2. Write endpoints з RBAC:
   - Create operations (workers, libs, vm instances)
   - Update operations (config, resources)
   - Delete operations (з confirmation dialogs)
3. User feedback:
   - Success/error notifications
   - Loading states
   - Form validation

**Оцінка**: 1-2 тижні

---

### Пріоритет 5: Distributed RAID (BurstRAID/SmallWorld) (4+ тижні)

**Мета**: Distributed storage з fault tolerance

**Залежності**: Local RAID (✅), Network (✅), Consensus, Event Sourcing

**Завдання**:
1. Протокол для distributed storage
2. Raft consensus для consistency
3. Event sourcing для auditability
4. Circuit breaker pattern для fault tolerance
5. Test strategy для distributed scenarios

**Оцінка**: 4+ тижні (окрема фаза з ADR/design doc)

---

## 📅 Рекомендований порядок виконання

### Тиждень 1: RAID-Libs Integration
- ✅ Найменш залежне завдання
- ✅ Обидва модулі готові
- ✅ Логічне продовження

### Тиждень 2-4: Resource Limits Enforcement (VM)
- Platform-specific implementations
- CPU/memory/GPU limits
- API endpoints

### Тиждень 5: Health Checks Integration (VM)
- HealthMonitor integration
- Auto-restart logic
- API endpoints

### Тиждень 6-7: UI Write Operations
- JWT authentication в UI
- Write endpoints з RBAC
- User feedback

### Тиждень 8+: Distributed RAID
- Distributed storage protocol
- Consensus mechanism
- Fault tolerance

---

## 🎯 Критерії успіху

### Для кожного завдання:
1. ✅ Код компілюється без помилок (`cargo check`)
2. ✅ Тести проходять (unit + integration)
3. ✅ API endpoints працюють
4. ✅ Документація оновлена
5. ✅ Git коміти з описовими повідомленнями

---

**Підготовлено**: Rust Architect  
**Дата**: 2025-12-19  
**Версія**: 1.0

