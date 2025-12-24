# 🚀 PoolAI - Наступні кроки розробки (Week 2+)
## Rust Architect Analysis - 2025-12-23

---

## 📊 Поточний стан (після Week 1)

**Статус збірки**: ✅ `cargo check` проходить без помилок  
**Статус тестів**: ✅ **27 tests passing** (6 unit + 21 integration)  
**Останній коміт**: `c5381c1` - feat(libs): RAID-Libs integration - Week 1 complete

### Завершені модулі (100%)
- ✅ Core, Pool, Monitoring, Network, Platform, Runtime, Rewards, TGBot
- ✅ **Security (JWT/HTTPS)** — завершено
- ✅ **RAID-Libs Integration** — **НОВЕ ЗАВЕРШЕННЯ (Week 1)** 🎉

### Модулі в розробці
- ✅ Libs Module (~95%) - production-ready
- ✅ RAID Module (~75%) - local reliable store + libs integration
- ✅ VM Module (~60%) - process runner integrated
- ✅ UI Module (~80%) - read-only dashboard

---

## 🎯 Наступні кроки (Week 2-7)

### ⭐ Пріоритет 1: Resource Limits Enforcement (VM) (Week 2-4)

**Мета**: Platform-specific resource limiting (cgroups на Linux, Job Objects на Windows)

**Залежності**: VM Process Runner (✅), Platform APIs (✅)

**Чому це пріоритет**:
- ✅ Process runner готовий
- ✅ Platform APIs готові
- ⚠️ Більш складне завдання (platform-specific код)
- ✅ Критично для production-ready VM

**Завдання**:

#### Week 2: Структури та Trait
1. Створити `src/vm/resources.rs`:
   - `ResourceLimits` struct (cpu_cores, memory_mb, gpu_device)
   - `ResourceUsage` struct (current CPU/memory/GPU usage)
   - `ResourceLimiter` trait з методами:
     - `apply_limits(process_id, limits) -> Result<()>`
     - `get_usage(process_id) -> Result<ResourceUsage>`
     - `is_supported() -> bool`
   - `PlatformResourceLimiter` struct (placeholder)

2. Інтеграція з VM Module:
   - Додати `resource_limiter: Arc<RwLock<PlatformResourceLimiter>>` до `VmManager`
   - `VmManager::apply_resource_limits()` викликає `ResourceLimiter`
   - `VmManager::get_instance_resource_usage()` повертає поточне використання

3. API endpoints:
   - `GET /api/v1/vm/instances/:id/resources` - поточне використання
   - `GET /api/v1/vm/resource-limits-supported` - підтримка платформи

#### Week 3: Linux Implementation (cgroups)
1. Linux-specific implementation:
   - Використати `cgroups-rs` або `systemd` API
   - CPU limits через `cpu.cfs_quota_us` та `cpu.cfs_period_us`
   - Memory limits через `memory.limit_in_bytes`
   - GPU scheduling (якщо доступно через nvidia-ml-py або подібне)

2. Integration tests:
   - Тест: CPU limits enforcement
   - Тест: Memory limits enforcement
   - Тест: Resource usage monitoring

#### Week 4: Windows Implementation (Job Objects)
1. Windows-specific implementation:
   - Використати Windows API через `winapi` crate
   - Job Objects для CPU/memory limits
   - `SetInformationJobObject` для встановлення лімітів
   - GPU scheduling (якщо доступно)

2. Integration tests:
   - Тест: CPU limits enforcement
   - Тест: Memory limits enforcement
   - Тест: Resource usage monitoring

**Оцінка**: 2-3 тижні

---

### Пріоритет 2: Health Checks Integration (VM) (Week 5)

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

4. Integration tests:
   - Тест: health check registration
   - Тест: auto-restart on failure
   - Тест: max restart limit

**Оцінка**: 1 тиждень

---

### Пріоритет 3: UI Write Operations (Week 6-7)

**Мета**: Write endpoints з JWT authentication та RBAC checks

**Залежності**: Network API (✅), Auth (JWT) (✅) — **ГОТОВО!**

**Завдання**:

#### Week 6: Authentication в UI
1. JWT authentication в UI:
   - Login form (`/ui/login`)
   - Token storage (localStorage)
   - Token refresh logic
   - Logout functionality

2. Protected routes:
   - Middleware для перевірки JWT
   - Redirect на login якщо не авторизовано
   - Role-based route access

#### Week 7: Write Operations
1. Write endpoints з RBAC:
   - Create operations (workers, libs, vm instances)
   - Update operations (config, resources)
   - Delete operations (з confirmation dialogs)

2. User feedback:
   - Success/error notifications
   - Loading states
   - Form validation
   - Error messages

3. Integration tests:
   - Тест: login/logout flow
   - Тест: RBAC checks
   - Тест: Write operations з авторизацією

**Оцінка**: 1-2 тижні

---

### Пріоритет 4: Distributed RAID (BurstRAID/SmallWorld) (Week 8+)

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

### Week 2-4: Resource Limits Enforcement (VM)
- Структури та trait (Week 2)
- Linux implementation (Week 3)
- Windows implementation (Week 4)
- Integration tests

### Week 5: Health Checks Integration (VM)
- HealthMonitor integration
- Auto-restart logic
- API endpoints
- Integration tests

### Week 6-7: UI Write Operations
- JWT authentication в UI (Week 6)
- Write endpoints з RBAC (Week 7)
- User feedback
- Integration tests

### Week 8+: Distributed RAID
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

## 📊 Прогрес Week 1

### ✅ Завершено:
- ✅ RAID-Libs Integration
  - ✅ `ArtifactRef` додано до `LibraryInfo`
  - ✅ `download_and_install()` зберігає artifacts в RAID
  - ✅ `get_library_path_or_load_from_raid()` завантажує з RAID
  - ✅ Integration tests (5 tests passing)
- ✅ 27 tests passing (6 unit + 21 integration)
- ✅ Build stability підтримується

---

**Підготовлено**: Rust Architect  
**Дата**: 2025-12-23  
**Версія**: 1.0 (Post Week 1)

