# OpenAPI gap audit (S28)

**Дата:** 2026-05-19 · **Джерело:** `rg '\.route\(' src/network` vs `docs/openapi.yaml`

## Підсумок

| Область | До S28 | Після S28 (yaml) |
|---------|--------|------------------|
| `/api/v1` Users | 0 paths | `/users`, `/users/{id}` ✅ |
| Pool workers | GET only | + POST `/workers`, DELETE `/workers/{id}` ✅ |
| Libraries | без upload | `/libraries/upload` ✅ |
| RAID core | 4 paths | + status, workers, artifacts CRUD, events, snapshot, strategies, metrics, health, rebalance ✅ |
| RAID admin | 0 paths | 6 admin paths ✅ |
| VM templates/networks | 0 paths | `/vm/templates*`, `/vm/networks*` ✅ |
| `/api/enterprise` | aligned | без змін (S14–S21) |

## Залишок (backlog після S33)

| Шлях | Призначення | Примітка |
|------|-------------|----------|
| ~~`/raid/distributed/*`~~ | Inter-node RAID protocol | **✅ S31–S33** paths + `RaidDistributed*` DTO schemas |
| ~~VM template body schemas~~ | `VmTemplate`, `GpuSchedulingPolicy` | **✅ FM-025** (2026-05-20) |
| VM network body schemas | `VmNetwork`, `NetworkIsolationConfig` | Поступове уточнення (поза FM-025) |

## Перевірка

```bash
rg '\.route\(' src/network/api/users.rs
rg '^  /users' docs/openapi.yaml
rg '^  /raid/admin' docs/openapi.yaml
```

**Наступний спринт:** деталізація distributed payload schemas або axe Playwright (FM-019 backlog).
