# LAN benchmark runbook (FM-003 / P2b)

**Status:** Planned (ops) — requires two PoolAI nodes on the same LAN. Autonomous dev sessions without a stand document steps here; they do not block code merges.

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

## 5. When blocked

| Situation | Action |
|-----------|--------|
| Single machine only | Keep FM-003 **Planned (ops)**; use local `poolai_health_load` + Criterion rows only. |
| Firewall | Open TCP between nodes on API port; document rules in ops note. |
| No `ml` feature on stand | Skip TQ01 LAN row; run wire test with `ml` on CI instead. |

**Last updated:** 2026-05-17 (AUTO_RUN_SESSION 2026-05-17 S1 — runbook звірено; LAN-стенд відсутній, FM-003 лишається **Planned (ops)**).

**Сесія 2026-05-17:** один хост — кроки §1–§3 задокументовані; прийняття §4 відкладено до двох вузлів. Локальний baseline: рядок **2026-04-10** у [`BENCHMARKS.md`](./BENCHMARKS.md) (`poolai_health_load --json`).
