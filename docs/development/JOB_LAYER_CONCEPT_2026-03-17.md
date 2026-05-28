# PoolAI Job / Mining Layer — Концепт (2026-03-17)

## 1. Мета

PoolAI вже реалізує:

- **Runtime + VM + Libs + Cloud** — виконання AI‑тасків;
- **RAID + SmallWorld + Event Sourcing** — розподілене сховище;
- **Rewards** — винагороди за роботу;
- **Enterprise + UI** — керування й моніторинг.

Цей документ формалізує **Job / Mining Layer** як окремий шар поверх цих модулів, без зміни їх реалізації.

## 2. Job як базова одиниця AI‑майнінгу

### 2.1 JobSpec (що описує job)

Концептуально, `JobSpec` містить:

- **Ресурси**:
  - CPU (кількість потоків / відносна потужність),
  - GPU (мінімальна вимога до памʼяті, бажаний тип/рівень),
  - RAM, дисковий простір.
- **Тип задачі**:
  - `Inference` (LLM / Vision / інші моделі),
  - `Training` / `FineTune`,
  - `Indexing` / `Embeddings`,
  - `Memory` (операції з AGI‑памʼяттю: reseed, reindex, federated round),
  - `System` (міграції, оптимізації, maintenance).
- **Обмеження**:
  - дедлайн / максимальна тривалість,
  - пріоритет,
  - політика повторних спроб.

### 2.2 Життєвий цикл Job

1. `submitted` — job створений (через API/UI/grid).
2. `scheduled` — job призначений на ноду/ноді (PoolAI Node).
3. `executing` — VM/Runtime запускають процес(и) для job.
4. `verifying` — результати перевіряються (локально і/або іншими нодами).
5. `rewarded` — Rewards Module (і в майбутньому Solana‑adapter) нараховують винагороду.
6. `completed` / `failed` — фінальний статус, зберігається для історії/метрик.

## 3. Мапінг Job Layer на існуючі модулі

- **Runtime / VM**:
  - виконують job як один або кілька процесів (локально чи в Cloud/K8s).

- **RAID / Memory Layer**:
  - зберігають вхідні/вихідні артефакти job (дані, моделі, ембедінги);
  - Memory‑jobs працюють із shards AGI‑памʼяті (reseeding, reindex, federated updates).

- **Rewards**:
  - нараховують винагороди за успішні job’и (з урахуванням якості/швидкості/ресурсів).

- **Monitoring / Enterprise / UI**:
  - показують статуси job’ів, історію, статистику, помилки.

## 4. Звʼязок з Grid Layer

- У Grid‑мережі job може:
  - надсилатись на конкретну ноду (direct),
  - або бути “розміщеним” і прийнятим тією нодою, яка відповідає вимогам (match‑making).

- Grid‑повідомлення:
  - `JobRequest(JobSpec)` — запит на виконання job.
  - `JobResult(JobId, output, metrics)` — відповідь із результатами.

Цей документ описує логіку Job / Mining Layer; реалізація протоколу й типів повідомлень описується в `GRID_PROTOCOL` (планується).

## 5. Реалізація MVP (S38)

| Тип | Модуль |
|-----|--------|
| `JobId`, `JobKind`, `JobSpec`, `JobStatus` | `src/job/` |
| Grid map | `src/job/map.rs` ↔ `GridEnvelope` Job |
| HTTP | `GET/POST /api/v1/jobs`, `GET/PATCH /jobs/{id}`, `POST /jobs/schedule`, `POST /jobs/{id}/lease` — `src/network/api/jobs.rs` |
| Lease (Galaxy §4.3.1) | `lease_config.rs`, `lease_acquire.rs`, optional `lease_*` на `JobRecord` (PH-S94…S98) |

## 6. Наступні кроки

- [x] Scheduler MVP (`Submitted`→`Scheduled`) — `src/job/scheduler.rs`, `POST /api/v1/jobs/schedule` (FM-020, 2026-05-20).
- [x] `PATCH /api/v1/jobs/{id}` lifecycle status — `lifecycle.rs`, FM-021 (2026-05-20).
- [x] HTTP contract tests — `tests/jobs_api_contracts.rs`, FM-026 (2026-05-20).
- [x] VM/worker binding on schedule — FM-034 (`JobRecord.worker_id`/`vm_id`, pool/VM placement).
- [x] Персистентний job store (JSON file) — `POOLAI_JOB_DATA_DIR`, `src/job/store.rs` (2026-05-20).
- [x] SQLite job store (optional `job-store-sqlite`, `POOLAI_JOB_STORE=sqlite`, migrate JSON) — FM-029 (2026-05-20).
- [x] On-chain submit epics (core NDJSON → sidecar schema v1) — `src/job/domain_events.rs`, `onchain.rs`, `POOLAI_ONCHAIN_EVENTS_DIR` (PH-S38, 2026-05-25).
- [x] Job lease TTL env — `POOLAI_JOB_LEASE_TTL_SECS`, `JobLeaseConfig` (PH-S97, 2026-05-27).
- [x] Job lease renew interval env — `POOLAI_JOB_LEASE_RENEW_INTERVAL_SECS` (PH-S111, 2026-05-28).
- [x] Lease acquire on schedule + `POST /api/v1/jobs/{id}/lease` (PH-S98, 2026-05-27); renew — PH-S99+.

