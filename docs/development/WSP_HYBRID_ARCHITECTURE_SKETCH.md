# WSP hybrid architecture sketch (ECO-9.2)

**Status:** sketch only — owner review before any code · **Date:** 2026-11-09
**Parent verdict:** `S:/rust/GSV/docs/superpowers/plans/2026-11-08-poolai-wsp-feasibility.md` §Verdict (ECO-9.1)
**Brief:** [`WSP_WEIGHT_SPACE_PROGRAMMING_BRIEF.md`](WSP_WEIGHT_SPACE_PROGRAMMING_BRIEF.md)

## 0. Scope / non-goals

Sketch of the **hybrid** scenario from the ECO-9.1 verdict (~55%): WSP як нові job-kind
поверх наявного Job lifecycle; RL/on-policy **не замінюється** — лишається доменом
routing/mining-політики. Без production kernel merge, без змін on-chain контрактів,
без бінарних ваг у git.

## 1. Компоненти

| Компонент | Де живе | Суть |
|---|---|---|
| **ActivationAudit** | poolAI job-kind + probe-хук у engine | Пасивний збір статистик активацій на probe-сеті: per-layer mean/cov drift, z-score спайки. Вихід — кандидати-аномалії (tensor, layer, stats). |
| **WeightPatch pack** | poolAI artifact (RAID/MemoryShard) | Sparse список `{tensor, block_idx, new_block_bits, base_hash}` — **блокова гранулярність** (GGUF Q4_K_M суперблок 256 / Q8_0 блок 32), + eval-звіт (perplexity delta, укр. golden, task-метрики). |
| **Validator quorum** | poolAI grid (існуючий Job/Result) | K нод застосовують патч до своїх реплік, ганяють фіксований eval-набір, підписують Result. Прийняття — кворум 2/3. |
| **Solana anchor** | Solana adapter (існуючий verify→reward) | On-chain **тільки** hash патча + кворум-підписи; тіла патчів — через RAID replication plane (hash-якорна trust-модель). Reward — після прийняття. |
| **Hot-apply (rusty-engine)** | engine (друга фаза, після owner go) | `block-bitmap patch-at-dequant`: на load — бітмапа патчених (tensor, block_idx); dequant-ядро гілкується тільки на патчених блоках. Zero-downtime; опційно A/B shadow-runner з канарковим трафіком %. |
| **Offline apply (llama-rs)** | engine wrapper | FFI чорна скриня — hot-apply неможливий: staged GGUF rewrite (block-level) + рестарт двигуна. Другий клас або поза WSP-лейною. |
| **RL lane** | без змін | Routing/mining/економічна політика — як є. WSP не торкається цієї лейни. |

## 2. Потік даних (lifecycle mapping)

```
ActivationAudit (job)          WeightPatch (job)
  probe-set активацій            pack {tensor,block,delta,hash} + eval
        │                              │
        ▼                              ▼
  anomaly candidates  ───────►  fan-out K validator нод (існуючий Job fan-out)
                                       │ кожна: apply → eval → sign Result
                                       ▼
                              кворум 2/3? ── ні → reject (без reward)
                                       │ так
                                       ▼
                              Solana: anchor(hash, quorum) → reward
                                       │
                                       ▼
                              RAID replication патч-тіла → ноди hot-apply
                              (rusty: block-bitmap / A/B canary; llama-rs: restart)
```

Існуючий lifecycle `submitted → scheduled → executing → verifying → rewarded → completed`
використовується як є — нові kind, не новий рушій.

## 3. Гарди (обов'язкові)

1. **Freeze router/gate ваг** — патчіться тільки FFN/attention-ваги експертів, ніколи
   MoE-роутинг (меншинні мовні кластери — найбільший ризик).
2. **Український golden eval** — фіксований набір укр. фікстур до/після патча; регресія
   = автоматичний rollback (block-bitmap дозволяє миттєвий відкат — зняти біт).
3. **Block checksums** — патч валідний тільки якщо base_hash блока зійшовся.
4. **Canary traffic %** — A/B shadow перед повним розгортанням.
5. Числові golden-fingerprint тести rusty-engine — не рухаються (regression gate).

## 4. Фазування (після owner go)

| Фаза | Зміст | Gate |
|---|---|---|
| 9.2 (цей документ) | Sketch + owner review | owner go |
| 9.3a spike | ActivationAudit capture на 1 ноді (offline, без p2p) | статистики збираються, без регресії |
| 9.3b spike | WeightPatch pack format + block-bitmap apply у rusty-engine за feature-flag | golden-fingerprint не зрушив |
| 9.3c | Validator quorum + Solana hash-anchor на devnet | кворум-тест на 3 нодах |
| 9.4 | Production lane (canary → full) | owner |

## 5. Відкриті питання до owner

1. Пороговий eval-набір: розмір/склад укр. golden фікстур і task-метрик (хто верифікує?).
2. K (кворум-розмір) та економіка reward за validator-роботу.
3. Чи llama-rs взагалі входить у WSP-лейну (offline-only), чи виключаємо.
4. Де живе canon архітектури (цей sketch у poolAI — тимчасово?).
