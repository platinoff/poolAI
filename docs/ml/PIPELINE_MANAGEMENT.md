# Pipeline Management (ML.6)
## Дата: 2026-01-22

Модуль оркестрації ML pipeline для управління кроками та залежностями.

---

## 🎯 Мета

Pipeline Management для:
- Визначення pipeline з кроками
- Виконання pipeline з dependency resolution
- Відстеження статусу кроків
- Управління конфігурацією кроків

---

## 📚 API

### Створення Manager

```rust
use poolai::ml::pipeline::MLPipelineManager;

let manager = MLPipelineManager::new();
```

### Створення Pipeline

```rust
use poolai::ml::pipeline::{MLPipelineManager, PipelineStep, StepType};
use std::collections::HashMap;

let manager = MLPipelineManager::new();

let steps = vec![
    PipelineStep {
        id: "preprocess".to_string(),
        step_type: StepType::Preprocessing,
        config: HashMap::new(),
        dependencies: vec![],
    },
    PipelineStep {
        id: "train".to_string(),
        step_type: StepType::Training,
        config: HashMap::new(),
        dependencies: vec!["preprocess".to_string()],
    },
];

let pipeline = manager.create_pipeline("pipeline1", steps).await?;
```

### Виконання Pipeline

```rust
manager.execute_pipeline(pipeline.id.as_str()).await?;

let got = manager.get_pipeline(pipeline.id.as_str()).await?;
println!("Status: {:?}", got.status);
```

---

## 📊 Структури

### StepType

- `Preprocessing` - Попередня обробка даних
- `Training` - Навчання моделі
- `Evaluation` - Оцінка моделі
- `Deployment` - Розгортання моделі

### PipelineStep

- `id: String` - Унікальний ID кроку
- `step_type: StepType` - Тип кроку
- `config: HashMap<String, String>` - Конфігурація кроку
- `dependencies: Vec<String>` - Залежності (ID інших кроків)

### PipelineStatus

- `Created` - Pipeline створено
- `Running` - Pipeline виконується
- `Completed` - Pipeline завершено
- `Failed` - Pipeline завершено з помилкою

### StepStatus

- `Pending` - Крок очікує виконання
- `Running` - Крок виконується
- `Completed` - Крок завершено
- `Failed` - Крок завершено з помилкою

---

## 🔍 Функціональність

### Dependency Resolution

Автоматичне визначення порядку виконання на основі залежностей:
- Topological sort для правильного порядку
- Перевірка на circular dependencies
- Паралельне виконання незалежних кроків (планується)

### Step Execution

- Послідовне виконання кроків
- Відстеження статусу кожного кроку
- Зберігання результатів виконання
- Обробка помилок

### Pipeline Lifecycle

- **Created**: Pipeline створено, готовий до виконання
- **Running**: Pipeline виконується
- **Completed**: Всі кроки виконано успішно
- **Failed**: Один з кроків завершився з помилкою

---

## 📏 Ключі output кроків (runbook метрик)

Після `execute_pipeline` результати кожного кроку — у `pipeline.step_results[step_id].output` (`HashMap<String, String>`). Усі успішні кроки містять **`status`** (`completed`) та **`step_id`**.

| `step_kind` | `StepType` | Ключі метрик (додатково) | Конфіг (приклад) |
|-------------|------------|--------------------------|------------------|
| `preprocessing` | `Preprocessing` | `feature_dim`, `sample_count`, `normalize_enabled`, `estimated_bytes`, `pipeline_checksum` | `feature_dim`, `sample_count`, `normalize` |
| `training` | `Training` | `epochs_run`, `learning_rate`, `final_loss`, `converged` | `epochs`, `learning_rate` / `lr` |
| `evaluation` | `Evaluation` | `accuracy`, `f1_proxy`, `samples_evaluated` | `baseline_accuracy`, `samples_evaluated` |
| `deployment` | `Deployment` | `environment`, `rollout_percent`, `revision`, `artifact_uri` | `environment`, `rollout_percent` |
| `profiling` | `Profiling` | `latency_ms`, `memory_mb`, `flops` | — |
| `pruning` | `Pruning` | `pruned_count`, `sparsity_ratio`, `accuracy_drop_est`, … | `ratio`, `target_sparsity` |
| `quantization` | `Quantization` | `quantization_level`, `size_mb_before`, `size_mb_after`, `compression_ratio` | `quantization` / `level` = `int8`, `fp16`, … |
| **`turboquant`** | `Quantization` | **`bytes_in`**, **`bytes_out`**, `target_bits`, `compression_ratio`, `rows`, `cols`, `max_abs_recon_error` | `turboquant=true` або `quantization=turboquant`; `weight_rows` (`1,2;3,4`) або `weights` + `turboquant_cols` |
| `hyperparameter_tuning` | `HyperparameterTuning` | `suggested_learning_rate`, `suggested_batch_size`, … | `lr_min`, `lr_max`, … |
| `automl` | `AutoMl` | `model_id`, `accuracy`, `model_type`, `hyperparameters_json`, … | AutoML config |
| `federated_aggregation` | `FederatedAggregation` | `federated_round`, `clients_count`, `total_samples`, `weight_dim`, … | `round`, `client_weights` |

### TurboQuant (операційний зріз)

Увімкнення: `turboquant` / `use_turboquant` = `true`, або `quantization=turboquant` (див. `MLPipeline::turboquant_enabled` у `src/ml/pipeline.rs`).

```rust
let mut cfg = HashMap::new();
cfg.insert("turboquant".to_string(), "true".to_string());
cfg.insert("weight_rows".to_string(), "0.1,0.2,0.3;-0.5,1.0,0.0".to_string());
// крок StepType::Quantization → output["step_kind"] == "turboquant"
```

Інтерпретація:

| Ключ | Значення |
|------|----------|
| `bytes_in` | Розмір сирих `f32` рядків перед пакуванням TQ01 |
| `bytes_out` | Розмір стислого буфера після `turboquant::pack_uniform_rows` |
| `compression_ratio` | `bytes_in / bytes_out` (1.0 якщо `bytes_out == 0`; для малих матриць `bytes_out` може перевищувати `bytes_in` через заголовок TQ01) |
| `max_abs_recon_error` | Макс. \|оригінал − unpack\| по елементах (sanity для тестів/логів) |

Деталі формату TQ01: [`TURBOQUANT_INTEGRATION.md`](./TURBOQUANT_INTEGRATION.md). Інтеграційний тест: `tests/ml_pipeline_integration.rs` → `test_pipeline_turboquant_quantization_step`.

### Стандартна квантизація (без TurboQuant)

`quantization` ≠ turboquant → `step_kind` = `quantization`, метрики `size_mb_*` / `compression_ratio` з `apply_quantization` (`src/ml/optimization.rs`).

---

## ⚙️ Ops verification (runbook)

Операційний зріз після змін у `src/ml/pipeline.rs` або кроках TurboQuant:

```bash
export K8S_OPENAPI_ENABLED_VERSION=1.28
# Повний зріз як CI (рекомендовано перед push):
cargo test-ci
# Або лише ML pipeline:
cargo test --test ml_pipeline_integration --features ml
cargo test pipeline:: --lib --features ml
```

**Перевірка метрик кроку** (після `execute_pipeline`):

```rust
let got = manager.get_pipeline(pipeline.id.as_str()).await?;
let out = &got.step_results["quantize"].output;
assert_eq!(out.get("status").map(String::as_str), Some("completed"));
assert!(out.contains_key("step_kind")); // turboquant | quantization | training | …
```

Ключі output — таблиця [«Ключі output кроків»](#-ключі-output-кроків-runbook-метрик) вище. Інтеграція з RAID-артефактами: після `deployment` перевір `artifact_uri` у output; реплікація — distributed RAID (`/raid/distributed/*`, OpenAPI tag **RAID Distributed**).

**Last ops review:** 2026-05-19 (автопрогін S31).

---

## 🧪 Тестування

```bash
cargo test --test ml_pipeline_integration --features ml
```

Тести покривають:
- Створення pipeline
- Виконання pipeline
- Dependency resolution
- Валідацію залежностей
- Статус tracking
- Edge cases

---

## 📝 Приклади Використання

### Базовий Pipeline

```rust
use poolai::ml::pipeline::{MLPipelineManager, PipelineStep, StepType};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), poolai::core::error::AppError> {
    let manager = MLPipelineManager::new();

    let steps = vec![
        PipelineStep {
            id: "preprocess".to_string(),
            step_type: StepType::Preprocessing,
            config: HashMap::new(),
            dependencies: vec![],
        },
        PipelineStep {
            id: "train".to_string(),
            step_type: StepType::Training,
            config: HashMap::new(),
            dependencies: vec!["preprocess".to_string()],
        },
    ];

    let pipeline = manager.create_pipeline("pipeline1", steps).await?;
    manager.execute_pipeline(pipeline.id.as_str()).await?;

    let got = manager.get_pipeline(pipeline.id.as_str()).await?;
    println!("Pipeline status: {:?}", got.status);

    Ok(())
}
```

### Pipeline з Конфігурацією

```rust
use poolai::ml::pipeline::{MLPipelineManager, PipelineStep, StepType};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), poolai::core::error::AppError> {
    let manager = MLPipelineManager::new();

    let mut train_config = HashMap::new();
    train_config.insert("batch_size".to_string(), "32".to_string());
    train_config.insert("learning_rate".to_string(), "0.001".to_string());
    train_config.insert("epochs".to_string(), "100".to_string());

    let steps = vec![
        PipelineStep {
            id: "preprocess".to_string(),
            step_type: StepType::Preprocessing,
            config: HashMap::new(),
            dependencies: vec![],
        },
        PipelineStep {
            id: "train".to_string(),
            step_type: StepType::Training,
            config: train_config,
            dependencies: vec!["preprocess".to_string()],
        },
        PipelineStep {
            id: "evaluate".to_string(),
            step_type: StepType::Evaluation,
            config: HashMap::new(),
            dependencies: vec!["train".to_string()],
        },
    ];

    let pipeline = manager.create_pipeline("pipeline1", steps).await?;
    manager.execute_pipeline(pipeline.id.as_str()).await?;

    Ok(())
}
```

### Повний ML Pipeline

```rust
use poolai::ml::pipeline::{MLPipelineManager, PipelineStep, StepType};

#[tokio::main]
async fn main() -> Result<(), poolai::core::error::AppError> {
    let manager = MLPipelineManager::new();

    let steps = vec![
        PipelineStep {
            id: "preprocess".to_string(),
            step_type: StepType::Preprocessing,
            config: std::collections::HashMap::new(),
            dependencies: vec![],
        },
        PipelineStep {
            id: "train".to_string(),
            step_type: StepType::Training,
            config: std::collections::HashMap::new(),
            dependencies: vec!["preprocess".to_string()],
        },
        PipelineStep {
            id: "evaluate".to_string(),
            step_type: StepType::Evaluation,
            config: std::collections::HashMap::new(),
            dependencies: vec!["train".to_string()],
        },
        PipelineStep {
            id: "deploy".to_string(),
            step_type: StepType::Deployment,
            config: std::collections::HashMap::new(),
            dependencies: vec!["evaluate".to_string()],
        },
    ];

    let pipeline = manager.create_pipeline("full_ml_pipeline", steps).await?;
    manager.execute_pipeline(pipeline.id.as_str()).await?;

    let got = manager.get_pipeline(pipeline.id.as_str()).await?;
    println!("Pipeline completed: {} steps", got.step_results.len());

    Ok(())
}
```

---

## 🔗 Інтеграція

### З AutoML Module

Pipeline Management інтегрується з AutoML:
- Автоматичне створення pipeline для AutoML
- Відстеження кроків оптимізації
- Управління ensemble pipeline

### З Experiment Tracking

Pipeline + Experiments:
- Логування pipeline як експериментів
- Відстеження метрик pipeline
- Порівняння різних pipeline

### З Model Versioning

Pipeline + Versioning:
- Версіонування pipeline
- Відстеження змін у pipeline
- Rollback pipeline

---

## 📈 Майбутні Покращення

- [ ] Паралельне виконання незалежних кроків
- [ ] Retry logic для failed steps
- [ ] Step caching
- [ ] Pipeline templates
- [ ] Conditional execution
- [ ] Pipeline scheduling
- [ ] Integration з workflow engines
- [ ] Real step execution (замість симуляції)
- [ ] **Welcome TurboQuant** — крок/режим стиснення KV / ваг / векторів (**лише Rust** у репозиторії); див. `docs/ml/TURBOQUANT_INTEGRATION.md` та Priority 2b у `docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md`

---

## 🔗 TurboQuant (план)

Коротко: алгоритми **TurboQuant / PolarQuant / QJL** зменшують обсяг **ML-даних** (зокрема KV), що допомагає **швидшій передачі артефактів** після реплікації в RAID. Інтеграція — **конфіг кроку pipeline** + **Rust-модуль** у `src/ml/` (без Python). Деталі: `TURBOQUANT_INTEGRATION.md`.

---

**Версія**: v0.2.3  
**Дата**: 2026-01-22 (оновлено 2026-05-19 — ops runbook, `cargo test-ci`)  
**Статус**: ✅ Реалізовано (Priority 7 - ML.6)
