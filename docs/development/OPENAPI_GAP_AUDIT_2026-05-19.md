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
| ~~`/admin/secrets/rotation`~~ | Secret rotation status (PH-S24/S37b) | **✅ PH-S47** |
| ~~`/admin/secrets/rotate`~~ | Run rotation hooks (admin RBAC) | **✅ PH-S47** |
| ~~`/raid/distributed/*`~~ | Inter-node RAID protocol | **✅ S31–S33** paths + `RaidDistributed*` DTO schemas |
| ~~VM template body schemas~~ | `VmTemplate`, `GpuSchedulingPolicy` | **✅ FM-025** (2026-05-20) |
| ~~VM network body schemas~~ | `VmNetwork`, `NetworkIsolationConfig` | **✅ FM-032** (2026-05-22) |
| ~~`/grid/network-profiles`~~ | List persisted peer ids (PH-S570) | **✅ PH-S840** |
| ~~`/admin/security-advisories`~~ | Security advisory list (PH-S586) | **✅ PH-S840** |
| ~~`/admin/security-advisories/{id}/acknowledge`~~ | Advisory acknowledge (PH-S573) | **✅ PH-S840** |
| ~~`/virtual-nodes/telegram/wallet/rebind-override`~~ | Admin wallet rebind override | **✅ PH-S840** |

**PH-S840…S849 (2026-06-21):** OpenAPI gap **0 missing** (`poolai-openapi-gap-audit` exit 0); grid metrics examples in yaml; `tests/grid_openapi_contracts.rs`; stand smoke OpenAPI path cases; `galaxy_horizon_s840_integration`; rust_ratio **94.70%** (PH-S845 hold advisory).

**PH-S1059…S1068 (2026-07-18, band 42):** OpenAPI gap **0 missing** (regression gate); contract extend `grid_openapi_contracts` + `memory_api_contracts`; stand smoke `ops_power_openapi`; openapi.yaml tier-2 examples (ops power, memory shards, seed-inventory); `galaxy_horizon_s1059_integration`.

## CI (PH-S19)

Job **`openapi-gap-audit`** у [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml) — `cargo run --bin poolai-openapi-gap-audit` (fail PR/push on exit 1).

**Ігнор аудиту (не публічний REST):** префікси `/ui/`, `/raft/` (Raft wire з `raft_rpc.rs`); exact `/api/workers`.

## Перевірка

```bash
cargo run --bin poolai-openapi-gap-audit
# exit 0 = all routes documented; exit 1 = prints missing paths

rg '\.route\(' src/network/api/users.rs
rg '^  /users' docs/openapi.yaml
rg '^  /raid/admin' docs/openapi.yaml
```

**PH-S50 (2026-05-26):** Jobs tag + `JobStoreBackend` schema (`json`/`sqlite`/`raid`); DIGEST `src/job/` не stub. Gap audit **0 missing**.

**Наступний спринт:** **PH-S51** VM Linux isolation (`NEXT_SESSION_PROMPT.md`). OpenAPI route backlog **закрито** (FM-032 ✅).
