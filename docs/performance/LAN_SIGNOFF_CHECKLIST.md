# LAN sign-off checklist (FM-027 / FM-003 §4)

**Призначення:** ops-чеклист для **двох фізичних хостів** перед закриттям FM-003 §4. **Sign-off** — лише після виконання всіх обов’язкових пунктів і запису evidence у [`BENCHMARKS.md`](./BENCHMARKS.md).

**Статус sign-off:** **BLOCKED** поки немає другого фізичного хоста. До того — **§5** / **§5.1** у [`LAN_BENCHMARK_RUNBOOK.md`](./LAN_BENCHMARK_RUNBOOK.md) + `bin/verify-lan-prep.*`.

**Пов’язано:** FM-027 (runbook prep), FM-028 (single-host TQ01+RAID metrics), `distributed_raid_wire_integration` (CI wire).

---

## 0. Метадані прогону (заповнити на початку)

| Поле | Node A | Node B |
|------|--------|--------|
| Hostname / OS | | |
| LAN IP:port | e.g. `192.168.1.10:8080` | e.g. `192.168.1.11:8080` |
| `git rev-parse HEAD` | | |
| `poolai --version` або build id | | |
| `POOLAI_RAID_BASE_PATH` | (окремий шлях) | (окремий шлях) |
| Features | `enterprise,ml,cloud,test-utils` | same |

**Ops operator / date:** _______________

---

## 1. Pre-flight (обов’язково)

- [ ] Обидва хости: одна гілка/коміт PoolAI; `K8S_OPENAPI_ENABLED_VERSION=1.28` при cloud parity.
- [ ] API-порт відкритий між A↔B (firewall / security group задокументовано).
- [ ] `POOLAI_RAID_BASE_PATH` **різні** на A і B; достатньо вільного диска (≥ 2× тестового артефакта).
- [ ] З ops-ноутбука (або з A): `bash bin/verify-lan-prep.sh` з `POOLAI_NODE_A_URL` / `POOLAI_NODE_B_URL` → exit 0.
- [ ] Логи старту збережені: `data/lan-stand/logs/` (single-host) або ops note для remote.

---

## 2. Connectivity (§4.1)

- [ ] `GET /api/v1/health` → 200 на A (локально і з B).
- [ ] `GET /api/v1/health` → 200 на B (локально і з A).
- [ ] Peers зареєстровані (discovery): `GET /api/v1/discovery/peers` на A містить B (або документований manual `register-remote`).
- [ ] Distributed RAID wire route доступний: `POST /api/v1/raid/distributed/...` без 503 (RAID manager initialized).

---

## 3. Metrics capture (§3 runbook)

### 3.1 HTTP baseline

- [ ] `poolai_health_load --json` на A (30s, concurrency 400) → рядок у `BENCHMARKS.md`.
- [ ] Те саме на B → другий рядок.

### 3.2 Replication (FM-003 / FM-007)

- [ ] **Put** тестового артефакта (≥ 4 MiB) на A.
- [ ] **Push** sync A→B: wall time, `missing_artifacts` / `conflicts` у JSON — у ops note.
- [ ] **Pull** sync B→A (інший artifact або reverse): timings записані.
- [ ] Артефакт читабельний на B (checksum / id збігається).

### 3.3 TurboQuant (P2b, `--features ml`)

- [ ] TQ01 artifact: `size_bytes` до/після на A і B; ratio у `BENCHMARKS.md` або ops note.
- [ ] Якщо `ml` недоступний на стенді — skip з посиланням на CI `distributed_raid_wire_integration` + `ml`.

### 3.4 LeaveCluster (FM-008)

- [ ] Graceful leave: replication перед `delete_worker` за сценарієм wire-тесту; JSON поля як у `distributed_raid_wire_integration`.

---

## 4. Sign-off gate (FM-003 §4)

Усі пункти **§4 LAN_BENCHMARK_RUNBOOK** (Acceptance):

- [ ] Two nodes reach each other over LAN (health + wire route).
- [ ] At least one **Push** and one **Pull** sync з timings у `BENCHMARKS.md` або attached ops note.
- [ ] TQ01 artifact: documented size ratio on the stand.
- [ ] LeaveCluster graceful сценарій пройдений.

**Підпис ops:** _______________ **Дата:** _______________

Після sign-off: оновити `FUNCTION_MANAGEMENT.md` FM-003 §4, `STABLE_STATE_SUMMARY`, changelog у `BENCHMARKS.md`.

---

## 5. Якщо лише один фізичний хост

| Дія | Не замінює §4 sign-off |
|-----|------------------------|
| `bin/run-lan-nodes.*` (8080+8081) | Wire/dev на одній машині |
| `bin/verify-lan-prep.*` (default) | Pre-flight dual-port |
| `bin/verify-dev-stand.*` | FM-016 virtual node stack |

FM-003 §4 залишається **BLOCKED**; FM-027 prep ✅.

**Last updated:** 2026-05-20 (FM-027).
