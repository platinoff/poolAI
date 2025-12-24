# 🎯 Наступні кроки розробки - Rust Architect
## Оновлено: 2025-12-19

---

## 📊 Поточний стан проекту

**Поточний branch**: `stage3/vm-health-checks`  
**Статус збірки**: ✅ `cargo check` проходить без помилок (0 errors, 0 warnings)  
**Статус тестів**: ✅ 37 tests passing (6 unit + 31 integration)  
**Загальний прогрес Stage 3**: ~85% готово

### Статус модулів

| Модуль | Статус | Прогрес | Тести | Останні досягнення |
|--------|--------|---------|-------|-------------------|
| **Libs** | ✅ COMPLETED | 100% | 10 passing | RAID integration, manifest persistence |
| **RAID** | ✅ MAJOR PROGRESS | 85% | 9 passing | GC/quota, retention policies |
| **VM** | ✅ IN PROGRESS | 75% | 18 passing | Health checks, resource limits infrastructure |
| **UI** | ✅ IN PROGRESS | 80% | - | Read-only dashboard |

---

## ✅ Завершені завдання (останні)

1. ✅ **VM Health Checks Integration** (2025-12-19)
   - Інтеграція з HealthMonitor
   - Periodic health checks (30s interval)
   - Auto-restart on failure
   - 6 integration tests

2. ✅ **VM Resource Limits Infrastructure** (2025-12-19)
   - ResourceLimits struct
   - ResourceLimiter trait
   - PlatformResourceLimiter з stubs
   - API endpoints
   - 7 integration tests

---

## 🎯 Наступні кроки (від простого до складного, від менш залежного до більш залежного)

### Phase 1: Незалежні завдання (можна робити зараз)

#### 1. Security (JWT/HTTPS) ⭐ **РЕКОМЕНДОВАНО НАСТУПНИМ**
**Залежності**: Network Module (✅), Toolchain stability  
**Складність**: Середня  
**Оцінка**: 1-2 тижні  
**Блокує**: UI Write Operations

**Чому наступним**:
- ✅ Network готовий
- ⚠️ Блокує UI Write Operations
- ⚠️ Потребує toolchain stability (gcc/dlltool або MSVC)
- ✅ Відповідає принципу "від менш залежного до більш залежного"

**Завдання**:
- [ ] Feature flags для `jsonwebtoken`/`axum-server`
- [ ] Toolchain stability (gcc/dlltool або MSVC target)
- [ ] Let's Encrypt автоматичне оновлення сертифікатів
- [ ] JWT middleware integration
- [ ] Integration tests

**Ризики**:
- ⚠️ Потребує native toolchain (gcc/dlltool на Windows GNU)
- ⚠️ Може вимагати перехід на MSVC target
- ⚠️ Залежності `ring`/`jsonwebtoken` можуть мати проблеми з компіляцією

**Альтернативи**:
- Використати pure-Rust альтернативи (якщо можливо)
- Відкласти до стабілізації toolchain
- Реалізувати базову авторизацію без JWT (токени в пам'яті)

---

#### 2. UI Write Operations
**Залежності**: Network API (✅), Auth (JWT) ← залежить від Phase 1.1  
**Складність**: Низька-Середня  
**Оцінка**: 1-2 тижні

**Чому другим**:
- ✅ Network готовий
- ⚠️ Залежить від Security (JWT)
- ✅ Простий task після Security
- ✅ Відповідає принципу "від простого до складного"

**Завдання**:
- [ ] JWT authentication в UI
- [ ] Write endpoints з RBAC checks
- [ ] Confirmation dialogs для деструктивних операцій
- [ ] Form validation
- [ ] Integration tests

---

### Phase 2: Складніші завдання (після Phase 1)

#### 3. Actual Resource Limits Enforcement (VM)
**Залежності**: VM Resource Limits Infrastructure (✅), Platform APIs  
**Складність**: Висока  
**Оцінка**: 2-3 тижні

**Чому третім**:
- ✅ Infrastructure готова
- ⚠️ Потребує platform-specific код (cgroups/Job Objects)
- ⚠️ Більш складне завдання
- ✅ Не блокує інші завдання

**Завдання**:
- [ ] Windows Job Objects для CPU/memory limits
- [ ] Linux cgroups v2 для CPU/memory limits
- [ ] GPU scheduling policy
- [ ] Platform-specific implementations
- [ ] Integration tests

**Ризики**:
- ⚠️ Потребує native APIs (Windows API, Linux syscalls)
- ⚠️ Може вимагати додаткові залежності (`windows` crate, `libc`)
- ⚠️ Тестування на різних платформах

---

#### 4. Distributed RAID (BurstRAID/SmallWorld)
**Залежності**: Local RAID (✅), Network (✅), Consensus, Event Sourcing  
**Складність**: Дуже висока  
**Оцінка**: 4+ тижні (окрема фаза з ADR)

**Чому останнім**:
- ✅ Local RAID готовий
- ⚠️ Найскладніше завдання
- ⚠️ Потребує окремий design document
- ⚠️ Може бути окремим проектом

**Завдання**:
- [ ] Протокол для distributed storage
- [ ] Raft consensus для consistency
- [ ] Event sourcing для auditability
- [ ] Circuit breaker pattern
- [ ] Test strategy для distributed scenarios

---

## 📋 Dependency Matrix

| Завдання | Залежності | Блокує | Складність | Пріоритет | Статус |
|----------|------------|--------|------------|-----------|--------|
| Security (JWT) | Network (✅), Toolchain | UI Write | Середня | ⭐⭐⭐ Високий | 🔄 Ready |
| UI Write | Network (✅), Auth (JWT) | Нічого | Низька-Середня | ⭐⭐ Середній | ⏳ Blocked |
| Resource Limits (actual) | VM Infrastructure (✅), Platform | Нічого | Висока | ⭐ Середній | 🔄 Ready |
| Distributed RAID | RAID (✅), Network (✅), Consensus | Нічого | Дуже висока | ⭐ Низький | 🔄 Future |

---

## 🎯 Рекомендований порядок виконання

### Крок 1: Security (JWT/HTTPS) ⭐ **ПОЧАТИ З ЦЬОГО**
**Причини**:
- ✅ Блокує UI Write Operations
- ✅ Network готовий
- ✅ Відповідає принципу "від менш залежного до більш залежного"
- ⚠️ Потребує toolchain stability

**Очікуваний результат**: JWT authentication + HTTPS support з feature flags

**Ризики та мітигація**:
- ⚠️ **Ризик**: Проблеми з toolchain (gcc/dlltool)
  - **Мітигація**: Feature flags, можливість відключити JWT/HTTPS
- ⚠️ **Ризик**: Проблеми з компіляцією `ring`/`jsonwebtoken`
  - **Мітигація**: Альтернативні pure-Rust рішення, fallback на базову авторизацію

---

### Крок 2: UI Write Operations
**Причини**:
- ✅ Network готовий
- ⚠️ Залежить від Security (JWT)
- ✅ Простий task після Security

**Очікуваний результат**: Write operations через UI з авторизацією

---

### Крок 3: Actual Resource Limits Enforcement
**Причини**:
- ✅ Infrastructure готова
- ⚠️ Складніше, але можна робити зараз
- ⚠️ Потребує platform-specific код

**Очікуваний результат**: CPU/memory/GPU limits для VM instances

---

### Крок 4: Distributed RAID (окрема фаза)
**Причини**:
- ✅ Local RAID готовий
- ⚠️ Найскладніше завдання
- ⚠️ Може бути окремим проектом

**Очікуваний результат**: Distributed storage з Raft consensus

---

## ✅ Рішення: Почати з Security (JWT/HTTPS)

**Обґрунтування**:
1. ✅ Блокує UI Write Operations
2. ✅ Network готовий
3. ✅ Відповідає принципу "від менш залежного до більш залежного"
4. ⚠️ Потребує toolchain stability (можна вирішити через feature flags)

**Наступний крок**: Після Security → UI Write Operations

---

## 📝 Примітки

- **Toolchain Stability**: Якщо виникнуть проблеми з gcc/dlltool, можна:
  - Використати MSVC target
  - Відключити JWT/HTTPS через feature flags
  - Використати pure-Rust альтернативи

- **Resource Limits**: Infrastructure готова, actual enforcement можна робити паралельно з Security

- **Distributed RAID**: Може бути окремим проектом/фазою з окремим ADR

---

**Підготовлено**: Rust Architect  
**Дата**: 2025-12-19  
**Версія**: 2.0

