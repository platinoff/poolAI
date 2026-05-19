# TurboQuant — дослідження та інтеграція в PoolAI

**Дата**: 2026-04-04 (оновлено: лише **Rust**, без Python)  
**Статус**: фаза 1 у коді ✅; **FM-004 SIMD** ✅ (Horizon S35) — feature `turboquant-simd`, `wide::f32x4`

---

## Політика імплементації

**У PoolAI TurboQuant реалізується виключно в Rust** (`src/ml/…`, feature `ml`). Жодних Python sidecar, subprocess до інтерпретатора чи PyPI-залежностей у runtime. Екосистемні пакети на PyPI згадуються лише як **орієнтир для валідації** (наукова відповідність), не як шар деплою.

---

## Що таке TurboQuant

**TurboQuant** — сімейство алгоритмів стиснення від **Google Research** для великих мовних моделей і векторного пошуку. Публічний опис: [TurboQuant: Redefining AI efficiency with extreme compression](https://research.google/blog/turboquant-redefining-ai-efficiency-with-extreme-compression/). Наукова робота: [arXiv:2504.19874](https://arxiv.org/abs/2504.19874). Пов’язані компоненти: **PolarQuant** ([arXiv:2502.02617](https://arxiv.org/abs/2502.02617)), **QJL** (Quantized Johnson–Lindenstrauss).

### Механіка (спрощено)

1. **PolarQuant** — перетворення векторів (полярні / рекурсивні кути + радіуси) для ефективної скалярної квантизації з меншим overhead констант.
2. **QJL** — корекція залишкової похибки з мінімальним біт-бюджетом (зокрема знакові проєкції), щоб зберегти якість attention / inner product.

Заявлені ефекти: сильне зменшення **KV cache**, менший бітрейт на елемент, застосовність до **vector search**.

---

## Наскільки це підходить PoolAI

| Аспект | Оцінка |
|--------|--------|
| **Передача даних** | Менший обсяг **ML-артефактів** (KV-буфери, квантизовані ваги, ембеддинги) → менше байтів по **RAID/мережі**; не замінює TCP/RAID tuning. |
| **Стек** | Реалізація з **пейперів** і референс-логіки в **чистому Rust** (`no_std`-дружні шари за потреби для ядра алгоритму). Опційно пізніше: **SIMD** / окремий crate під GPU через офіційні Rust-біндинги (якщо з’являться), без Python. |
| **ML.6** | Після стабільного контракту кроків **Quantization** / pipeline: вхід `&[f32]` / буфери артефактів, вихід стислий формат + метрики в коді. |
| **Ризики** | Обсяг інженерії (відтворення алгоритму), чисельна валідація проти пейпера; ліцензії власного коду = MIT проєкту. |

**Висновок**: TurboQuant доречний як **Rust-модуль** у ML data-plane; очікуваний шлях — **поетапна імплементація** (спрощені режими → повна відповідність пейперу).

---

## План імплементації (фази, лише Rust)

1. **Специфікація та тести** — зафіксувати внутрішній формат пакета (заголовок + квантизовані блоки), property-тести на round-trip / похибку inner product для малих розмірностей.
2. **Модуль `src/ml/turboquant.rs` (або підмодуль)** — ізоляція від HTTP; публічні функції на зразок `compress_vectors`, `decompress_for_dot_product` (назви уточнити під API пейпера).
3. **Інтеграція з pipeline** — розширення конфігу кроку `Quantization` / окремий прапор `turboquant: true` у `StepType` / `HashMap` конфігу кроку; виклик лише з існуючого Rust executor pipeline.
4. **Метрики** — `bytes_in`, `bytes_out`, `target_bits` у результаті кроку (структури вже в стилі `PruningResult` / pipeline status).
5. **Бенчмарки** — `criterion` у `benches/` або розділ у `docs/performance/BENCHMARKS.md` (Priority 4).
6. **SIMD (FM-004)** — опційний feature **`turboquant-simd`** (`wide`); див. § SIMD нижче.

---

## SIMD (FM-004, Horizon S35)

| Що | Де |
|----|-----|
| Feature | `Cargo.toml`: `turboquant-simd = ["dep:wide"]` |
| Код | `src/ml/turboquant.rs` — `row_max_abs`, `append_quantized_row`, `push_dequantized_row`, `dot_f32` |
| API | `turboquant::simd_fast_path_enabled()` → `true` лише з feature |
| Збірка | `cargo build --features ml,turboquant-simd` |
| Тести | `cargo test turboquant --lib --features ml,turboquant-simd` (parity `simd_pack_matches_scalar_reference`) |
| Бенч | `cargo bench --bench turboquant_benchmarks --features ml,turboquant-simd` |

Без feature залишається **scalar 4-wide unroll** (стабільний default, CI `cargo test-ci` без `turboquant-simd`).

---

## Зв’язані документи

- `docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md` — **Priority 2b** та блок «Наступні кроки за пріоритетом».
- `docs/ml/PIPELINE_MANAGEMENT.md` — ML.6.
- `docs/concept/poolAI_concept_root.txt` — Stage 4.4.
