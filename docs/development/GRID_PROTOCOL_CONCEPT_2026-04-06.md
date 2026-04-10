# Grid Protocol — концепт повідомлень (v1, 2026-04-06)

## 1. Мета

Цей документ **не вводить новий бінарний протокол у коді**. Він узгоджує **логічні типи повідомлень** Grid‑рівня з уже реалізованими механізмами (Discovery, RAID, Job/Memory концепти) і задає орієнтири для майбутньої реалізації поверх HTTP/WebSocket або окремого транспорту.

Звʼязані документи:

- [`concept/POOLAI_GRID_NODE.md`](../concept/POOLAI_GRID_NODE.md) — нода як building block грида.
- [`development/JOB_LAYER_CONCEPT_2026-03-17.md`](JOB_LAYER_CONCEPT_2026-03-17.md) — життєвий цикл **Job** і верифікація.
- [`concept/POOLAI_MEMORY_LAYER.md`](../concept/POOLAI_MEMORY_LAYER.md) — **MemoryShard** і seeds поверх RAID/ML.

## 2. Логічні типи повідомлень (Priority 6)

| Тип | Призначення | Мінімальні поля (концепт) |
|-----|-------------|---------------------------|
| **Job** | Запит на виконання AI‑задачі на ноді‑виконавці | `job_id`, `spec` (тип задачі, ресурси, дедлайн), `verification_policy`, посилання на вхідні артефакти / memory keys |
| **Result** | Відповідь виконавця після **Job** | `job_id`, `status`, `outputs` (URI checksum, метрики), `proof` / підпис (за політикою) |
| **MemoryShard** | Опис шару памʼяті / артефакту для реплікації або запиту | `shard_id`, `raid_logical_name` або `artifact_id`, `version`, hints для seeding (див. Memory Layer) |
| **PeerStatus** | Здоровʼя, навантаження, репутація піра для планування | `peer_id`, `capabilities`, `load`, `last_seen`, опційно `role` (miner/hub/hybrid) |

Узгодження з Job Layer: **Job** / **Result** відображаються на фази `submitted → scheduled → executed → verified → rewarded` з `JOB_LAYER_CONCEPT_*`.

## 3. Звʼязок із кодом і API (на зараз)

### Discovery / peer plane

- **HTTP**: `GET/POST /api/v1/discovery/*` — [`src/network/api/discovery.rs`](../../src/network/api/discovery.rs) (`peers`, `peers/{id}`, `register`).
- **UDP / сервіс**: [`src/network/discovery.rs`](../../src/network/discovery.rs) — enum **`DiscoveryMessage`** (`Announce`, `Heartbeat`, `Query`, `Response`) + **`PeerInfo`** / **`PeerCapabilities`** (`core/discovery_types`).
- **Мапінг**: **PeerStatus** на концептуальному рівні = агрегат **`PeerInfo`** + метрики з Monitoring/Runtime; **Announce**/**Heartbeat** — існуючі кроки підтримки «живого» каталогу пірів.

### RAID / replication plane

- **HTTP (distributed RAID)**: маршрути під `/raid/distributed/*` — [`src/network/raid_distributed_handlers.rs`](../../src/network/raid_distributed_handlers.rs) (артефакти, sync, health, cluster join/leave).
- **MemoryShard**: логічно співпадає з **артефактами RAID** та метаданими реплікації (`ReplicationEngine`, події в `raid/events`); точний wire‑формат «Grid» може обгортати існуючі **PutArtifact** / **protocol** типи.

### Масштабування грида (тести)

- Інтеграційні сценарії: [`tests/grid_network_scalability_tests.rs`](../../tests/grid_network_scalability_tests.rs) (топологія багатьох нод, replication engine, статистика мережі).

### Помилки HTTP (узгодження з REST-площиною, 2026-04-10)

Усі згадані HTTP-маршрути живуть під загальним nest **`/api/v1`** (крім **`/api/enterprise/*`**, якщо увімкнено feature `enterprise`). **Невдачі** цих викликів для клієнта мають той самий логічний конверт, що й решта PoolAI REST: поле **`error`** з **`code`** та **`message`**, опційно **`context`** — реалізація [`src/network/json_errors.rs`](../../src/network/json_errors.rs) (**`HttpAppError`**, **`AppError::RestError`** там, де потрібен стабільний machine-readable код; див. **FM-005** у [`docs/catalog/FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md)). Майбутній **Grid envelope** не скасовує цей шар — він може обгортати payload поверх уже узгодженого JSON.

## 4. Що залишається поза цим документом

- **Solana‑adapter** — концепт і мапінг подій: [`SOLANA_ADAPTER_CONCEPT_2026-04-06.md`](SOLANA_ADAPTER_CONCEPT_2026-04-06.md).
- Єдиний серіалізований **Grid envelope** (версія протоколу, підпис, routing) — майбутня специфікація після заморозки транспорту (HTTP vs QUIC vs інше).

## 5. Оновлення концепту Grid Node

У [`POOLAI_GRID_NODE.md`](../concept/POOLAI_GRID_NODE.md) перелік типів повідомлень тепер деталізований тут; для змін у протоколі оновлювати **цей файл** і посилання в **NEXT_STEPS_ARCHITECT** (P6).
