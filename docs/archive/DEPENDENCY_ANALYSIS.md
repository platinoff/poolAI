# 🔗 PoolAI Dependency Analysis & Development Order
## Rust Architect Analysis - 2025-12-19

---

## 📊 Dependency Graph

```
┌─────────────────────────────────────────────────────────────┐
│                    INDEPENDENT TASKS                         │
│  (Can be done in parallel, no dependencies)                 │
└─────────────────────────────────────────────────────────────┘
│
├─► UI Write Operations
│   └─► Depends on: Network API (✅), Auth (🔄)
│
├─► Health Checks Integration (VM)
│   └─► Depends on: VM Process Runner (✅)
│
├─► Resource Limits Enforcement (VM)
│   └─► Depends on: VM Process Runner (✅), Platform APIs
│
└─► RAID-Libs Integration
    └─► Depends on: Libs Module (✅), RAID Module (✅)

┌─────────────────────────────────────────────────────────────┐
│                    MODERATE DEPENDENCIES                      │
│  (Depend on 1-2 completed modules)                           │
└─────────────────────────────────────────────────────────────┘
│
├─► Security (JWT/HTTPS)
│   └─► Depends on: Network Module (✅), Toolchain stability
│
└─► VM Health Checks
    └─► Depends on: VM Process Runner (✅), Health Monitor (✅)

┌─────────────────────────────────────────────────────────────┐
│                    HIGH DEPENDENCIES                          │
│  (Depend on multiple modules, complex)                       │
└─────────────────────────────────────────────────────────────┘
│
└─► Distributed RAID (BurstRAID/SmallWorld)
    └─► Depends on: Local RAID (✅), Network (✅), Consensus, Event Sourcing
```

---

## 🎯 Development Order (From Most Dependent to Least Dependent)

### Phase 1: Independent Tasks (Can Start Now)
**Priority: High (block other work)**

#### 1.1 RAID-Libs Integration ⭐ **START HERE**
**Dependencies**: Libs Module (✅ ~95%), RAID Module (✅ ~70%)
**Complexity**: Low
**Estimated Time**: 1 week

**Why first**: 
- Both modules are ready
- No blocking dependencies
- Enables artifact storage for libraries
- Simple integration task

**Tasks**:
- [ ] Modify `libs/manager.rs::download_and_install()` to save artifacts to RAID
- [ ] Update `LibraryInfo` to include `ArtifactRef`
- [ ] Runtime reads artifacts from RAID instead of direct file access
- [ ] Integration tests

---

#### 1.2 Health Checks Integration (VM) ⭐
**Dependencies**: VM Process Runner (✅ ~60%), Health Monitor (✅)
**Complexity**: Low
**Estimated Time**: 1 week

**Why second**:
- Process runner is ready
- Health Monitor exists
- Simple integration
- Improves VM reliability

**Tasks**:
- [ ] Integrate VM instances with HealthMonitor
- [ ] Periodic health checks for running VM processes
- [ ] Auto-restart on health check failure
- [ ] Health status API endpoint

---

#### 1.3 Resource Limits Enforcement (VM) ⭐
**Dependencies**: VM Process Runner (✅ ~60%), Platform APIs
**Complexity**: Medium
**Estimated Time**: 2-3 weeks

**Why third**:
- Process runner is ready
- Platform-specific (cgroups/Job Objects)
- More complex but independent

**Tasks**:
- [ ] CPU limits (cgroups on Linux, Job Objects on Windows)
- [ ] Memory limits enforcement
- [ ] GPU scheduling policy
- [ ] Platform-specific implementations
- [ ] Tests for resource limits

---

### Phase 2: Moderate Dependencies
**Priority: Medium (after Phase 1)**

#### 2.1 Security (JWT/HTTPS)
**Dependencies**: Network Module (✅), Toolchain stability
**Complexity**: Medium
**Estimated Time**: 1-2 weeks

**Why Phase 2**:
- Network module is ready
- Toolchain-dependent (can be done in parallel with Phase 1)
- Feature flags allow gradual rollout

**Tasks**:
- [ ] Feature flags for `jsonwebtoken`/`axum-server`
- [ ] Toolchain stability (gcc/dlltool or MSVC)
- [ ] Let's Encrypt automatic certificate management
- [ ] JWT middleware integration

---

#### 2.2 UI Write Operations
**Dependencies**: Network API (✅), Auth (🔄 - needs JWT)
**Complexity**: Low
**Estimated Time**: 1-2 weeks

**Why Phase 2**:
- Network API is ready
- Depends on Auth (JWT) from 2.1
- Can start UI work in parallel, integrate auth later

**Tasks**:
- [ ] JWT authentication in UI
- [ ] Write endpoints with RBAC checks
- [ ] Confirmation dialogs for destructive operations
- [ ] Form validation

---

### Phase 3: High Dependencies (Complex)
**Priority: Low (after Phase 1-2)**

#### 3.1 Distributed RAID (BurstRAID/SmallWorld)
**Dependencies**: Local RAID (✅), Network (✅), Consensus, Event Sourcing
**Complexity**: Very High
**Estimated Time**: 4+ weeks (separate phase with ADR)

**Why Phase 3**:
- Requires all base infrastructure
- Complex distributed systems work
- Needs separate design document
- Can be done as separate project phase

**Tasks**:
- [ ] Protocol design for distributed storage
- [ ] Raft consensus implementation
- [ ] Event sourcing for auditability
- [ ] Circuit breaker pattern
- [ ] Test strategy for distributed scenarios

---

## 📋 Current Status & Next Steps

### ✅ Completed (No Dependencies)
- Core Module
- Pool Module
- Monitoring Module
- Network Module
- Platform Module
- Runtime Module
- Rewards System
- TGBot Module
- Libs Module (~95%)
- RAID Module (~70%)
- UI Module (~80% - read-only)
- VM Module (~60% - process runner)

### 🚧 In Progress / Ready to Start

#### Immediate Next Steps (Phase 1 - Independent):

1. **RAID-Libs Integration** ⭐ **RECOMMENDED FIRST**
   - **Why**: Both modules ready, simple integration, enables artifact storage
   - **Blocks**: Nothing (can be done now)
   - **Unblocks**: Better library management, artifact persistence

2. **Health Checks Integration (VM)**
   - **Why**: Process runner ready, Health Monitor exists
   - **Blocks**: Nothing (can be done now)
   - **Unblocks**: VM reliability, auto-recovery

3. **Resource Limits Enforcement (VM)**
   - **Why**: Process runner ready, but more complex
   - **Blocks**: Nothing (can be done now)
   - **Unblocks**: Production-ready VM isolation

---

## 🎯 Recommended Development Order

### Week 1-2: RAID-Libs Integration
**Goal**: Libs saves downloaded libraries as artifacts in RAID

**Benefits**:
- Artifact persistence
- Better library management
- Foundation for distributed storage

### Week 3: Health Checks Integration (VM)
**Goal**: VM instances have health monitoring

**Benefits**:
- Auto-recovery
- Better reliability
- Production readiness

### Week 4-6: Resource Limits Enforcement (VM)
**Goal**: CPU/memory/GPU limits for VM instances

**Benefits**:
- Resource isolation
- Production security
- Multi-tenant support

### Week 7-8: Security (JWT/HTTPS)
**Goal**: Feature flags for JWT/HTTPS

**Benefits**:
- Production security
- Authentication/authorization

### Week 9-10: UI Write Operations
**Goal**: Safe write operations through UI

**Benefits**:
- Better UX
- Operational efficiency

### Week 11+: Distributed RAID
**Goal**: Distributed storage with fault tolerance

**Benefits**:
- Scalability
- Fault tolerance
- Enterprise features

---

## 🔄 Dependency Matrix

| Task | Depends On | Blocks | Complexity | Priority |
|------|------------|--------|------------|----------|
| RAID-Libs Integration | Libs (✅), RAID (✅) | Nothing | Low | ⭐⭐⭐ High |
| Health Checks (VM) | VM Process (✅), Health (✅) | Nothing | Low | ⭐⭐ Medium |
| Resource Limits (VM) | VM Process (✅), Platform | Nothing | Medium | ⭐⭐ Medium |
| Security (JWT/HTTPS) | Network (✅), Toolchain | UI Write | Medium | ⭐ Medium |
| UI Write Operations | Network (✅), Auth | Nothing | Low | ⭐ Low |
| Distributed RAID | RAID (✅), Network (✅), Consensus | Nothing | Very High | ⭐ Low |

---

## ✅ Decision: Start with RAID-Libs Integration

**Reasoning**:
1. ✅ Both modules are ready (~95% and ~70%)
2. ✅ No blocking dependencies
3. ✅ Simple integration task
4. ✅ Enables better library management
5. ✅ Foundation for future distributed storage
6. ✅ Low complexity, high value

**Next**: After RAID-Libs, continue with Health Checks (VM), then Resource Limits.

---

**Підготовлено**: Rust Architect  
**Дата**: 2025-12-19  
**Версія**: 1.0

