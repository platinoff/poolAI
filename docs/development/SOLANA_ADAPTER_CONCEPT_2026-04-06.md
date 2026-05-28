# Solana adapter — концепт окремого шару (v1, 2026-04-06)

> **Horizon (авторозробка):** реалізація **FM-010** — спринт **S37** у [`HORIZON_TO_100_PLAN.md`](./HORIZON_TO_100_PLAN.md) · [`AUTO_RUN_SESSION_2026_HORIZON.md`](./AUTO_RUN_SESSION_2026_HORIZON.md).

## 1. Мета

Описати **billing / tokenization / on-chain anchoring** як **окремий адаптер** поверх уже реалізованого **PoolAI core** (Rust runtime, RAID, VM, Rewards у сенсі внутрішньої логіки). Цей документ **не** додає залежність `solana-sdk` у дерево `poolai` і **не** фіксує фінальну схему акаунтів — лише межі відповідальності та мапінг доменних подій.

**Galaxy map (PH-S120):** інтерактивний граф — [`docs/vision/`](../vision/) (`solana_concept` → `crate_solana` → `solana_sidecar` / `solana_program`); FM **FM-033** у [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md).

Звʼязані документи:

- [`development/JOB_LAYER_CONCEPT_2026-03-17.md`](JOB_LAYER_CONCEPT_2026-03-17.md) — завершення Job → винагорода / верифікація.
- [`concept/POOLAI_MEMORY_LAYER.md`](../concept/POOLAI_MEMORY_LAYER.md) — seeds / shards.
- [`development/GRID_PROTOCOL_CONCEPT_2026-04-06.md`](GRID_PROTOCOL_CONCEPT_2026-04-06.md) — логічні повідомлення грида.
- Реалізація внутрішніх нагород: модуль [`src/rewards/`](../../src/rewards/) (off-chain; адаптер може підписуватися на ті самі події).

## 2. Відокремлення core (Rust) від адаптера

| Шар | Відповідальність | Залежності |
|-----|-------------------|------------|
| **PoolAI core** | Оркестрація Job (концепт), VM/Runtime, RAID, Discovery, Enterprise API, ML pipeline, внутрішній `rewards` | Без Solana / без обовʼязкового web3 |
| **Solana adapter** | Переклад доменних подій у транзакції / логи програми; гаманці, підпис, RPC; політика gas/rent | Solana Program / клієнт SDK (окремий crate або сервіс) |

**Принципи:**

- Core **не** імпортує Solana types; адаптер отримує **події або HTTP/webhook** з core (або читає узгоджений audit/event stream).
- Ключі підпису **тільки** в адаптері (або HSM), не в основному сервері інференсу.
- Ідемпотентність: повторна доставка тієї ж доменної події не повинна подвоювати виплати — вузол on-chain або off-chain nonce в адаптері.

## 3. Мапінг доменних подій → on-chain (концепт)

Нижче — **орієнтовна** семантика; імена інструкцій / структури логів узгоджуються при реалізації програми.

| Доменна подія (концепт P6) | Зміст у PoolAI | Що фіксується on-chain (ідея) |
|----------------------------|----------------|--------------------------------|
| **JobCompleted** | Job перейшов у `verified` / `rewarded` (див. Job Layer); є `job_id`, виконавець, підсумок верифікації | Інструкція або лог: `job_id` (hash), `executor` (pubkey), `payout_mint` / amount або посилання на merkle batch |
| **SeedProvided** | Учасник надав seed / bandwidth для **MemoryShard** (Memory Layer; реплікація артефакта) | Лог доступності shard: `shard_id`, `provider`, timestamp; може живити репутацію / мікровиплати |
| **MemoryUpdated** | Нова версія памʼяті / артефакта (checksum, версія RAID / ML artifact) | Anchor metadata (CID / hash), без великих blob у ланцюгу; вміст лишається в RAID/IPFS тощо |

Додатково узгоджувані події (майбутнє розширення): `PeerSlashed`, `QuorumWitness`, `StakeLocked` — лише якщо зʼявиться економічний шар стейкінгу.

## 4. Інтеграційні варіанти

1. **Sidecar**: окремий бінарний процес підписує транзакції, підписується на **enterprise audit** або внутрішній event bus (якщо зʼявиться стабільний stream).
2. **Async worker у деплої**: черга (Redis/NATS) між core і адаптером.
3. **Pull-модель**: адаптер періодично читає REST read-only (наприклад audit/metrics) — гірша узгодженість, простіше для MVP.

## 5. Ризики та невизначеність

- Регуляторні / KYC вимоги до виплат — поза scope core; адаптер або off-chain settlement.
- Вибір мережі (mainnet / L2) і версія Anchor — після прототипу.
- Звірка з **існуючим** `rewards` модулем: уникати дублювання бізнес-правил; single source of truth для «хто заслуговує винагороду» — бажано в core, адаптер лише **виконує** вже затверджені рішення.

## 6. Реалізація MVP (S37)

| Що | Де |
|----|-----|
| Crate | `crates/poolai-solana-adapter/` |
| Schema v1 | `events.rs` — `JobCompleted`, `SeedProvided`, `MemoryUpdated` |
| Sidecar | `poolai-solana-adapter` binary (NDJSON stdin → ack stdout) |
| Main `poolai` | **без** `solana-sdk` / без залежності на adapter |

## 7. FM-024 — mock RPC stub (2026-05-20)

| Що | Де |
|----|-----|
| Devnet config | `crates/poolai-solana-adapter/config/devnet.toml`, `src/config.rs` |
| Mock RPC | `src/rpc/mock.rs` — idempotent `event_id` → `mocksig…` |
| Sidecar | `SidecarProcessor` + bin `poolai-solana-adapter` (stdout ack + optional `rpc`) |
| Env | `POOLAI_SOLANA_CONFIG`, `POOLAI_SOLANA_CLUSTER`, `POOLAI_SOLANA_RPC_URL`, `POOLAI_SOLANA_MOCK_RPC` |

**Обмеження:** без мережевого I/O, без `mainnet-beta`, без `solana-sdk` у `poolai`.

## 8. FM-033 — on-chain program + devnet RPC (2026-05-22)

| Що | Де |
|----|-----|
| BPF prototype | `crates/poolai-solana-adapter/program/poolai-events/` (`PoolAiInstruction` Borsh) |
| Instruction builder | `src/instruction.rs` — custom program or Memo fallback (`1111…` placeholder) |
| Devnet JSON-RPC | `src/rpc/devnet.rs` — `getLatestBlockhash`, `sendTransaction`, `getSignatureStatuses` |
| Sidecar | `mock_rpc=false` у bundled `devnet.toml`; ack `rpc` або `rpc_error` |
| Env | `POOLAI_SOLANA_KEYPAIR_PATH`, `POOLAI_SOLANA_PROGRAM_ID`, `POOLAI_SOLANA_MOCK_RPC=1` для stub |

**Обмеження:** без `solana-sdk` у `poolai`; без mainnet; deploy програми — Solana CLI (див. `program/README.md`).

## 9. PH-S38 — core on-chain submit epics (2026-05-25)

| Що | Де |
|----|-----|
| Schema v1 (core mirror) | `src/job/domain_events.rs` — same JSON as sidecar |
| NDJSON sink | `src/job/onchain.rs` — `POOLAI_ONCHAIN_EVENTS_DIR` → `events.ndjson` |
| Emit hooks | `JobStore::update_status` / `force_status` → `JobCompleted`; grid `MemoryShard` → `MemoryUpdated` / `SeedProvided` |
| Tests | `tests/job_onchain_events.rs`; `cargo test job::` unit tests |

**Pipe to sidecar:** `tail -f data/onchain/events.ndjson | poolai-solana-adapter` (devnet config / mock RPC).

## 10. PH-S46 — wire hardening + production devnet path (2026-05-25) ✅

| Що | Де |
|----|-----|
| Shared wire limits | `crates/poolai-solana-adapter/wire/limits.rs` (included by adapter + BPF) |
| Envelope validation | `src/wire_limits.rs`, `DomainEventEnvelope::validate`, sidecar reject |
| BPF checks | `program/poolai-events/src/lib.rs` — empty/oversized fields, instruction data cap |
| Deploy script | `scripts/deploy-poolai-events-devnet.sh` |
| Deployed profile example | `config/devnet.deployed.toml.example` |
| RPC ack | `anchor_mode`: `memo` \| `program` |

**Перевірка:** `cargo test -p poolai-solana-adapter -p poolai-events -j 1`

## 11. Наступні кроки

- Batch / merkle виплати; звірка з `rewards` модулем core.
- **Наступний спринт:** **PH-S41** macvlan (Linux) — [`NEXT_SESSION_PROMPT.md`](./NEXT_SESSION_PROMPT.md).
