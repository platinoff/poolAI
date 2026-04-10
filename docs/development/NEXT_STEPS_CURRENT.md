# 🚀 PoolAI - Наступні кроки (Поточний стан)

> **Канон на 2026-04-10:** порядок робіт — лише **[`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.1** та **[`NEXT_STEPS_ARCHITECT_2026-03-17.md`](./NEXT_STEPS_ARCHITECT_2026-03-17.md)** (операційний підрозділ). Нижче — **історичний зріз 2026-01-17** (BurstRAID/етапи); не використовуй його як єдине джерело пріоритетів. **Останнє:** Clippy `-D warnings` за CI-матрицями + тести — [`STABLE_STATE_SUMMARY.md`](../status/STABLE_STATE_SUMMARY.md), [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md).

## Детальний план дій - 2026-01-17 (архівний зріз)

---

## 📊 Поточний стан проекту

**Версія**: v0.1.0 Production Ready ✅  
**Загальний прогрес**: 96% ✅  
**Distributed AI Features**: 99.6% ✅  
**Тести**: 773+ passing ✅  
**CI/CD**: 100% passing ✅

### Нещодавно завершено:

**Priority 1.2.5 (Optional Improvements) — ✅ ЗАВЕРШЕНО:**
- ✅ Audit Log Compression
- ✅ Load Balancer Health Checks
- ✅ Windows Isolation State Tracking

**Priority 1.3 (API Endpoints) — ✅ ЗАВЕРШЕНО:**
- ✅ VM Management API (`/api/vm/templates`, `/api/vm/networks`)
- ✅ RAID System API (`/api/raid/workers`, `/api/raid/status`)
- ✅ UI Management API (`/api/ui/dashboards`, `/api/ui/components`, `/api/ui/themes`)
- ✅ Enterprise API Endpoints (всі endpoints реалізовані)

**Priority 1.4.1 (BurstRAID Strategy) — ✅ 100% ЗАВЕРШЕНО:**
- ✅ Базова структура створена (`src/raid/burst_raid.rs`)
- ✅ Burst detection реалізовано
- ✅ Adaptive replication factor реалізовано
- ✅ Інтеграція з ReplicationEngine
- ✅ Інтеграція з RaidManager.put_artifact() (completed)
- ✅ Rebalancing logic реалізовано
- ✅ Background tasks для auto-rebalancing та cleanup
- ✅ Shutdown logic для background tasks

---

## 🎯 Наступні кроки (пріоритетний порядок)

### ⭐⭐⭐ Priority 1.4: RAID Planned Features (5-7 тижнів)

#### 1.4.1 BurstRAID Strategy (2-3 тижні) — 🔄 40% → 100%

**Поточний стан**: Базова структура створена, потрібна інтеграція з RaidManager

**Наступні кроки:**

1. **Інтеграція з RaidManager** (3-5 днів) ⭐⭐⭐
   - Модифікувати `RaidManager.put_artifact()` для підтримки `RaidMode::BurstRaid`
   - Додати `BurstRaidStrategy` instance в `RaidManager`
   - Викликати `burst_strategy.replicate_artifact()` при `RaidMode::BurstRaid`
   - Додати `record_access()` виклики для burst detection
   - **Файли**: `src/raid/mod.rs` (put_artifact, initialize)
   - **Оцінка**: 3-5 днів
   - **Пріоритет**: Високий (продовження поточної роботи)

2. **Rebalancing Logic** (3-5 днів) ⭐⭐
   - Реалізувати аналіз поточного розподілу артефактів
   - Ідентифікувати артефакти для переміщення (на основі access patterns, node capacity)
   - Перемістити артефакти на кращі nodes
   - Оновити replication metadata
   - **Файли**: `src/raid/burst_raid.rs` (rebalance method)
   - **Оцінка**: 3-5 днів
   - **Пріоритет**: Середній

3. **Background Tasks** (2-3 дні) ⭐⭐
   - Background task для периодичного rebalancing
   - Background task для burst monitoring
   - Конфігурація інтервалів через `BurstRaidConfig`
   - **Файли**: `src/raid/burst_raid.rs`
   - **Оцінка**: 2-3 дні
   - **Пріоритет**: Середній

4. **Comprehensive Tests** (2-3 дні) ⭐
   - Unit tests для burst detection
   - Unit tests для replication factor adjustment
   - Integration tests для rebalancing
   - Integration tests для RaidManager integration
   - **Файли**: `tests/raid_burst_raid_*.rs`
   - **Оцінка**: 2-3 дні
   - **Пріоритет**: Низький

5. **Documentation Update** (1 день) ⭐
   - Оновити Rustdoc документацію
   - Додати usage examples
   - Оновити `poolAI_concept_root.txt`
   - **Оцінка**: 1 день
   - **Пріоритет**: Низький

**Загальна оцінка**: 11-17 днів (2-3 тижні)

---

#### 1.4.2 SmallWorld Network (2-3 тижні) — 🔄 0%

**Поточний стан**: Placeholder в `RaidMode::SmallWorld`

**План:**

1. **Створити `src/raid/small_world.rs`** (1 день)
   - Базова структура `SmallWorldStrategy`
   - Конфігурація `SmallWorldConfig`
   - Інтеграція з `ReplicationEngine`

2. **Реалізувати SmallWorld distributed strategy** (5-7 днів)
   - Network topology для replication
   - Short-path routing для artifact placement
   - Clustering coefficient calculation
   - Distributed storage optimization

3. **Додати network topology для replication** (3-4 дні)
   - Topology discovery (latency, bandwidth)
   - Optimal path selection
   - Replication based on network proximity

4. **Додати distributed storage optimization** (3-4 дні)
   - Load balancing across clusters
   - Proximity-based artifact placement
   - Cluster-aware replication

5. **Тести та документація** (3-4 дні)

**Загальна оцінка**: 15-20 днів (2-3 тижні)

---

#### 1.4.3 Administrative Control Plane (1-2 тижні) — 🔄 0%

**План:**

1. **Admin API для RAID management** (3-4 дні)
   - Cluster configuration API
   - Node management API
   - Replication policy management

2. **Cluster management interface** (3-4 дні)
   - Cluster health monitoring
   - Replication status dashboard
   - Node status visualization

3. **Тести та документація** (2-3 дні)

**Загальна оцінка**: 8-11 днів (1-2 тижні)

---

### ⭐⭐ Priority 1.5: VM Module Planned Features (4-6 тижнів)

#### 1.5.1 Resource Limits Enforcement (1-2 тижні) — 🔄 Infrastructure ready

**Поточний стан**: Infrastructure готовий (WindowsJobObjectLimiter, LinuxCgroupLimiter), enforcement - placeholder

**План:**

1. **CPU Limits Enforcement** (3-4 дні)
   - Windows: Реалізувати реальні Windows API виклики (SetInformationJobObject)
   - Linux: Реалізувати cgroups CPU limits
   - **Файли**: `src/vm/resources/windows.rs`, `src/vm/resources/linux.rs`

2. **Memory Limits Enforcement** (2-3 дні)
   - Windows: Реалізувати ProcessMemoryLimit через Job Objects
   - Linux: Реалізувати cgroups memory limits
   - **Файли**: `src/vm/resources/windows.rs`, `src/vm/resources/linux.rs`

3. **GPU Limits Enforcement** (2-3 дні)
   - GPU scheduling policy enforcement
   - GPU memory limits
   - **Файли**: `src/vm/resources/*.rs`

4. **Тести** (2-3 дні)

**Загальна оцінка**: 9-13 днів (1-2 тижні)

---

#### 1.5.2 Full Isolation Implementation (2-3 тижні) — 🔄 Basic isolation ready

**План:**

1. **Full namespace integration (setns)** (4-5 днів)
2. **Windows AppContainer full implementation** (4-5 днів)
3. **Security policy enforcement** (3-4 дні)
4. **Тести** (3-4 дні)

**Загальна оцінка**: 14-18 днів (2-3 тижні)

---

#### 1.5.3 macvlan Support (1 тиждень) — 🔄 Function signature ready

**План:**

1. **Завершити macvlan implementation** (3-4 дні)
2. **Додати direct physical interface access** (2-3 дні)
3. **Тести** (1-2 дні)

**Загальна оцінка**: 6-9 днів (1 тиждень)

---

### ⭐ Priority 2: v0.3.0 - Stage 4.4 AI/ML Enhancement (3.5-5 місяців)

**Загальна оцінка**: 14-21 тиждень

**6 Features** (planned для v0.3.0+):
1. Model Optimization (2-3 тижні)
2. AutoML Integration (3-4 тижні)
3. Federated Learning (4-6 тижнів)
4. Model Versioning (1-2 тижні)
5. Experiment Tracking (2-3 тижні)
6. Pipeline Management (2-3 тижні)

---

## 📅 Рекомендований порядок виконання

### Тиждень 1-3: BurstRAID Strategy Completion ⭐⭐⭐

**Пріоритет #1**: Завершити BurstRAID Strategy

1. **Інтеграція з RaidManager** (3-5 днів) — **НАЙВАЖЛИВІШЕ**
2. **Rebalancing Logic** (3-5 днів)
3. **Background Tasks** (2-3 дні)
4. **Tests & Documentation** (3-4 дні)

**Очікуваний результат**: BurstRAID Strategy 100% complete

---

### Тиждень 4-6: SmallWorld Network Strategy ⭐⭐

**Пріоритет #2**: Реалізувати SmallWorld distributed strategy

1. Створити базову структуру (1 день)
2. Реалізувати distributed strategy (5-7 днів)
3. Network topology integration (3-4 дні)
4. Distributed storage optimization (3-4 дні)
5. Tests & Documentation (3-4 дні)

**Очікуваний результат**: SmallWorld Network 100% complete

---

### Тиждень 7-8: VM Resource Limits Enforcement ⭐⭐

**Пріоритет #3**: Реалізувати реальні resource limits

1. CPU Limits Enforcement (3-4 дні)
2. Memory Limits Enforcement (2-3 дні)
3. GPU Limits Enforcement (2-3 дні)
4. Tests (2-3 дні)

**Очікуваний результат**: Resource Limits Enforcement 100% complete

---

### Тиждень 9+: Інші пріоритети

- Administrative Control Plane (1-2 тижні)
- Full Isolation Implementation (2-3 тижні)
- macvlan Support (1 тиждень)
- v0.3.0 AI/ML Enhancement (3.5-5 місяців)

---

## 🎯 Швидкі перемоги (можна виконати сьогодні)

1. **BurstRAID Strategy - RaidManager Integration** (3-5 днів)
   - Найвищий пріоритет
   - Логічне продовження поточної роботи
   - Результат: BurstRAID працює з RaidManager

2. **Rebalancing Logic** (3-5 днів)
   - Покращує якість розподілу
   - Не блокує інші завдання

3. **VM CPU Limits Enforcement** (3-4 дні)
   - Важливий для production
   - Infrastructure готовий

---

## 📊 Підсумок пріоритетів

### Найвищий пріоритет (зробити першим):
1. ⭐⭐⭐ **BurstRAID Strategy Integration** — продовження поточної роботи
2. ⭐⭐ **SmallWorld Network Strategy** — завершення RAID features
3. ⭐⭐ **VM Resource Limits Enforcement** — важливо для production

### Середній пріоритет:
4. ⭐ **Administrative Control Plane**
5. ⭐ **Full Isolation Implementation**
6. ⭐ **macvlan Support**

### Низький пріоритет (v0.3.0+):
7. 🔄 **Stage 4.4 AI/ML Enhancement** (3.5-5 місяців)

---

**Підготовлено**: Rust Architect  
**Дата**: 2026-01-17  
**Версія**: 1.0 - Current Next Steps Plan  
**Статус**: Active
