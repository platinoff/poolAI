# TurboQuant — дослідження та інтеграція в PoolAI

**Дата**: 2026-04-04  
**Статус**: планова фіча (Welcome TurboQuant track)

---

## Що таке TurboQuant

**TurboQuant** — сімейство алгоритмів стиснення від **Google Research** для великих мовних моделей і векторного пошуку. Публічний опис: [TurboQuant: Redefining AI efficiency with extreme compression](https://research.google/blog/turboquant-redefining-ai-efficiency-with-extreme-compression/) (березень 2026). Наукова робота: [arXiv:2504.19874](https://arxiv.org/abs/2504.19874) (ICLR 2026). Пов’язані компоненти: **PolarQuant** ([arXiv:2502.02617](https://arxiv.org/abs/2502.02617)), **QJL** (Quantized Johnson–Lindenstrauss).

### Механіка (спрощено)

1. **PolarQuant** — обертання / полярне представлення векторів, щоб застосувати ефективний квантизатор з меншим «overhead» констант квантизації.
2. **QJL** — корекція залишкової похибки з мінімальним біт-бюджетом (зокрема знакові проєкції), щоб зберегти якість attention / inner product.

Заявлені ефекти в блозі та пейпері: сильне зменшення **KV cache** (орієнтири **~4–6×** і менший бітрейт на елемент), покращення швидкості обчислення attention logits на GPU у порівнянні з повноточністю, застосовність до **vector search** без важкого dataset-specific тюнінгу.

---

## Наскільки це підходить PoolAI

| Аспект | Оцінка |
|--------|--------|
| **Вузьке місце «передачі даних»** | TurboQuant знижує **обсяг і пропускну потребу для ML-даних** (кеші, ваги, ембеддинги), а не замінює оптимізацію мережевого стеку RAID/TCP. Для **реплікації квантизованих артефактів** і **стримінгу inference** менший payload = менше часу на дроті. |
| **Стек проєкту** | Ядро PoolAI — **Rust**. Готові пакети екосистеми орієнтовані на **Python** ([turboquant](https://pypi.org/project/turboquant/), [turboquant-torch](https://pypi.org/project/turboquant-torch/), [turboquant-hf](https://pypi.org/project/turboquant-hf/) на PyPI). Прямого стабільного **Rust crate** на момент огляду немає — реалістичний шлях: **окремий worker / sidecar** (Python або майбутній CUDA/JAX шар) + контракт з `MLPipelineManager`. |
| **Зв’язок з ML.6** | Логічно лягає **після** або **поруч** з кроком **Quantization** і з реальними бекендами pipeline: вхід/вихід артефакту, метрики `bytes_before` / `bytes_after`, якість (proxy метрики). |
| **Ризики** | Ліцензії залежностей, версії CUDA/GPU, узгодження з **enterprise** multi-tenant (ізоляція воркерів уже є в VM/runtime). |

**Висновок**: для цілей PoolAI TurboQuant **доречний** як **прискорення ML data-plane** (KV / ваги / індекси), а не як універсальне рішення всіх мережевих bottleneck. Його варто **закласти в план** поряд із бекендами pipeline (Priority 2 / Stage 4.4).

---

## План імплементації (фази)

1. **Контракт і конфіг** — розширити тип кроку pipeline або підкрок `Quantization` (наприклад, `compression: turboquant`, `target_bits`, `mode: kv_cache | weights | vectors`). Документувати HTTP/gRPC або файловий контракт input/output артефакту.
2. **Sidecar / VM worker** — опційний процес (Python + офіційні/підтримувані пакети) у зоні worker; PoolAI викликає його з runtime (аналогічно до інших зовнішніх ML інструментів). Логи й метрики в існуючий monitoring.
3. **RAID / бібліотека моделей** — зберігати **вже стиснені** блоби як артефакти; порівняти P95 часу реплікації до/після на однаковому каналі (див. `docs/performance/BENCHMARKS.md`).
4. **Довгостроково** — відстежити появу **Rust/native** біндингів або винести критичний шлях у **окремий сервіс** з GPU; не блокувати core на Python, якщо політика деплою забороняє.

---

## Зв’язані документи

- `docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md` — **Priority 2b (TurboQuant track)**.
- `docs/ml/PIPELINE_MANAGEMENT.md` — оркестрація ML.6.
- `docs/concept/poolAI_concept_root.txt` — Stage 4.4, оновлення «Welcome TurboQuant».
