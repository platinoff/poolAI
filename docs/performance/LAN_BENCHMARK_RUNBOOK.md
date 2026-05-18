# LAN benchmark runbook (FM-003 / P2b)

**Status:** Planned (ops) — **FM-016 virtual nodes / worker ✅** (2026-05-25). **§4 sign-off** лишається **BLOCKED** без двох фізичних хостів. Для розробки достатньо **§5.1 virtual-node dev stand** (`verify-dev-stand`).

**Related:** [`BENCHMARKS.md`](./BENCHMARKS.md) (Criterion + `poolai_health_load`), [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](../development/NEXT_STEPS_ARCHITECT_2026-03-17.md) (P2b LAN checkbox).

---

## 1. Topology

| Node | Role | Example |
|------|------|---------|
| **A** | Primary / artifact source | `192.168.1.10:8080` |
| **B** | Peer (distributed RAID wire) | `192.168.1.11:8080` |

Both hosts: same PoolAI build (`--features ml,enterprise,cloud,test-utils` for parity with CI), GNU or Linux ref host preferred for stable `cargo bench`.

---

## 2. Environment

```bash
export PATH="$HOME/.cargo/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
export POOLAI_RAID_BASE_PATH=/var/lib/poolai/raid   # per node, distinct paths
```

On each node:

1. Start PoolAI with distributed RAID enabled and distinct `node_id` / peer registration per deployment docs.
2. Register **B** as a peer of **A** (and vice versa if bidirectional tests).
3. Confirm `GET /api/v1/health` returns 200 from the opposite host.

---

## 3. Metrics to capture

### 3.1 HTTP baseline (per node)

```bash
cargo run --release --bin poolai_health_load -- --json \
  http://<NODE>:8080/api/v1/health 30 400 > health_<NODE>_$(date +%Y%m%d).json
```

Append one row per host to the **`poolai_health_load --json`** table in [`BENCHMARKS.md`](./BENCHMARKS.md) (`rps_ok_only`, `latency_p50_ms`, `latency_p95_ms`, `latency_p99_ms`, `ok_requests`, `error_requests`, `wall_seconds`).

### 3.2 Artifact replication (FM-003 / FM-007)

1. **Put** a known payload (e.g. 4 MiB random) on **A** via distributed `PutArtifact` or REST upload.
2. Trigger **SyncArtifacts** (Push / Pull / Bidirectional) toward **B**.
3. Record: wall-clock sync duration, bytes on disk at **B**, `missing_artifacts` / `conflicts` in JSON response.
4. Repeat with **TQ01**-compressed artifact (`--features ml`): compare `size_bytes` before/after on both nodes.

### 3.3 TurboQuant volume (P2b)

| Step | Command / check |
|------|-----------------|
| Pack | ML pipeline quantization step or `turboquant_benchmarks` on ref host |
| Wire | `cargo test -j 1 --test distributed_raid_wire_integration --features test-utils,ml` (TQ01 size test) |
| LAN | Same artifact id on A and B; `remote_versions` timestamps aligned → no conflict; diverge → `conflicts` populated |

---

## 4. Acceptance (ops sign-off)

- [ ] Two nodes reach each other over LAN (health + wire route).
- [ ] At least one **Push** and one **Pull** sync logged with timings in `BENCHMARKS.md` or an attached ops note.
- [ ] TQ01 artifact: documented size ratio (uncompressed / compressed) on the stand.
- [ ] LeaveCluster graceful: artifacts replicated before `delete_worker` when peers exist (see `distributed_raid_wire_integration` for expected JSON fields).

---

## 5. Single machine (dev stand)

Два вузли на **одній** машині (різні порти, окремі RAID-шляхи) — для розробки wire/sync до повного LAN sign-off §4.

| Node | URL | Env |
|------|-----|-----|
| A | `http://127.0.0.1:8080` | `POOLAI_HTTP_PORT=8080`, `POOLAI_RAID_BASE_PATH=.../node-A/raid` |
| B | `http://127.0.0.1:8081` | `POOLAI_HTTP_PORT=8081`, `POOLAI_RAID_BASE_PATH=.../node-B/raid` |

**Windows (PowerShell, repo root):**

```powershell
.\bin\run-lan-nodes.ps1
# health after ~15s:
Invoke-WebRequest http://127.0.0.1:8080/api/v1/health
Invoke-WebRequest http://127.0.0.1:8081/api/v1/health
Stop-Process -Name poolai -Force -ErrorAction SilentlyContinue
```

**MSYS2 bash:**

```bash
bash bin/run-lan-nodes.sh
curl -s http://127.0.0.1:8080/api/v1/health
curl -s http://127.0.0.1:8081/api/v1/health
```

Дані стенду: `data/lan-stand/` (gitignored). Discovery може бачити peer на сусідньому порту; **§4 acceptance** (Push/Pull timings у `BENCHMARKS.md`) — лише після ops-прогону.

### 5.1 Virtual node stack (FM-016 на одній машині)

Coordinator + `poolai-worker` (реєстрація, tasks, RAID wire) без другого фізичного хоста:

| Процес | URL | Скрипт |
|--------|-----|--------|
| Coordinator | `http://127.0.0.1:8080` | `bin/run-virtual-node-dev.ps1` / `.sh` |
| Worker health | `http://127.0.0.1:9090/health` | те саме |
| Telegram bot (опційно) | long-poll → coordinator | `poolai-telegram-bot --features tgbot` + `TELEGRAM_BOT_TOKEN` |

```powershell
.\bin\run-virtual-node-dev.ps1
.\bin\verify-dev-stand.ps1   # default warmup 50s + bootstrap task retries
```

```bash
bash bin/run-virtual-node-dev.sh
bash bin/verify-dev-stand.sh   # checks health, discovery, pool join, >=4 tasks completed
```

Env: `POOLAI_VIRTUAL_NODE_DATA_DIR` у coordinator (`data/lan-stand/virtual-node/vn-store`), worker `POOLAI_TELEGRAM_ID=dev-stand-user` → auto-bind. Див. HANDOFF §2a.

---

## 6. When blocked

| Situation | Action |
|-----------|--------|
| No second physical host | Use §5.1 virtual-node dev stand + `verify-dev-stand`; FM-003 §4 **BLOCKED** (немає 2 хостів, 2026-05-25) — не блокує інші FM. |
| Firewall | Open TCP between nodes on API port; document rules in ops note. |
| No `ml` feature on stand | Skip TQ01 LAN row; run wire test with `ml` on CI instead. |

**Last updated:** 2026-05-18 — `bin/run-virtual-node-dev.*`, `bin/verify-dev-stand.*`; §5.1 virtual-node stack; `core::dev_stand::resolve_http_port`.
