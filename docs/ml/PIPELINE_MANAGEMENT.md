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

**Версія**: v0.2.2  
**Дата**: 2026-01-22 (оновлено 2026-04-04 — TurboQuant у плані)  
**Статус**: ✅ Реалізовано (Priority 7 - ML.6)
