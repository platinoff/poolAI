# Experiment Tracking (ML.5)
## Дата: 2026-01-22

Модуль відстеження ML експериментів для управління та порівняння експериментів.

---

## 🎯 Мета

Experiment Tracking для:
- Реєстрації та lifecycle експериментів
- Відстеження метрик (accuracy, loss, custom)
- Порівняння експериментів
- Вибір найкращих експериментів

---

## 📚 API

### Створення Tracker

```rust
use poolai::ml::experiments::ExperimentTracker;

let tracker = ExperimentTracker::new();
```

### Запуск Експерименту

```rust
let exp = tracker.start_experiment("exp1", "NeuralNetwork").await?;
println!("Experiment ID: {}", exp.id);
```

### Логування Метрик

```rust
use poolai::ml::experiments::ExperimentMetrics;

let mut metrics = ExperimentMetrics::default();
metrics.accuracy = 0.95;
metrics.loss = 0.05;
metrics.training_time_ms = 1000;
metrics.custom.insert("f1_score".to_string(), 0.92);

tracker.log_metrics(exp.id.as_str(), metrics).await?;
```

### Завершення Експерименту

```rust
// Успішне завершення
tracker.end_experiment(exp.id.as_str()).await?;

// Або помилка
tracker.fail_experiment(exp.id.as_str()).await?;
```

### Отримання Найкращих Експериментів

```rust
// Найкращий за accuracy
let best = tracker.get_best_by_accuracy().await;

// Найкращий за найменшим loss
let best_loss = tracker.get_best_by_loss().await;
```

---

## 📊 Структури

### ExperimentMetrics

- `accuracy: f64` - Точність моделі
- `loss: f64` - Loss значення
- `training_time_ms: u64` - Час навчання
- `custom: HashMap<String, f64>` - Кастомні метрики

### ExperimentStatus

- `Running` - Експеримент виконується
- `Completed` - Експеримент завершено успішно
- `Failed` - Експеримент завершено з помилкою

### Experiment

- `id: String` - Унікальний ID експерименту
- `name: String` - Назва експерименту
- `model_type: String` - Тип моделі
- `status: ExperimentStatus` - Статус
- `created_at: DateTime<Utc>` - Час створення
- `ended_at: Option<DateTime<Utc>>` - Час завершення
- `metrics: Option<ExperimentMetrics>` - Метрики
- `hyperparameters: HashMap<String, String>` - Гіперпараметри
- `tags: Vec<String>` - Теги

---

## 🔍 Функціональність

### Lifecycle Management

- **Start**: Створення нового експерименту
- **Log Metrics**: Логування метрик під час виконання
- **End/Fail**: Завершення експерименту (успішно або з помилкою)

### Metrics Tracking

- Стандартні метрики: accuracy, loss, training_time
- Кастомні метрики: будь-які додаткові метрики

### Experiment Comparison

- Порівняння за accuracy
- Порівняння за loss
- Вибір найкращих експериментів

### Filtering

- Фільтрація за статусом (Running, Completed, Failed)
- Сортування за часом створення

---

## 🧪 Тестування

```bash
cargo test --test ml_experiments_integration --features ml
```

Тести покривають:
- Створення tracker
- Запуск експериментів
- Логування метрик
- Завершення експериментів
- Вибір найкращих
- Порівняння
- Фільтрацію
- Кастомні метрики

---

## 📝 Приклади Використання

### Базовий Приклад

```rust
use poolai::ml::experiments::{ExperimentTracker, ExperimentMetrics};

#[tokio::main]
async fn main() -> Result<(), poolai::core::error::AppError> {
    let tracker = ExperimentTracker::new();

    // Start experiment
    let exp = tracker.start_experiment("exp1", "NeuralNetwork").await?;

    // Log metrics
    let mut metrics = ExperimentMetrics::default();
    metrics.accuracy = 0.95;
    metrics.loss = 0.05;
    tracker.log_metrics(exp.id.as_str(), metrics).await?;

    // End experiment
    tracker.end_experiment(exp.id.as_str()).await?;

    Ok(())
}
```

### Множинні Експерименти

```rust
use poolai::ml::experiments::{ExperimentTracker, ExperimentMetrics};

#[tokio::main]
async fn main() -> Result<(), poolai::core::error::AppError> {
    let tracker = ExperimentTracker::new();

    // Run multiple experiments
    for i in 0..5 {
        let exp = tracker
            .start_experiment(&format!("exp{}", i), "NeuralNetwork")
            .await?;

        // Simulate training...
        let mut metrics = ExperimentMetrics::default();
        metrics.accuracy = 0.90 + (i as f64 * 0.01);
        metrics.loss = 0.10 - (i as f64 * 0.01);
        tracker.log_metrics(exp.id.as_str(), metrics).await?;

        tracker.end_experiment(exp.id.as_str()).await?;
    }

    // Get best
    let best = tracker.get_best_by_accuracy().await.unwrap();
    println!("Best experiment: {} (accuracy: {:.2}%)",
        best.name, best.metrics.unwrap().accuracy * 100.0);

    Ok(())
}
```

### Фільтрація та Порівняння

```rust
use poolai::ml::experiments::{ExperimentTracker, ExperimentStatus};

#[tokio::main]
async fn main() -> Result<(), poolai::core::error::AppError> {
    let tracker = ExperimentTracker::new();

    // ... create experiments ...

    // List completed experiments
    let completed = tracker
        .list_experiments(Some(ExperimentStatus::Completed))
        .await;
    println!("Completed experiments: {}", completed.len());

    // Compare two experiments
    let ord = tracker
        .compare_by_accuracy("exp1", "exp2")
        .await?;
    println!("Comparison: {:?}", ord);

    Ok(())
}
```

---

## 🔗 Інтеграція

### З AutoML Module

Experiment Tracking інтегрується з AutoML:
- Автоматичне логування експериментів AutoML
- Відстеження найкращих моделей
- Порівняння різних підходів

### З Model Versioning

Versioning + Experiments:
- Версіонування експериментів
- Відстеження історії покращень
- Rollback до попередніх експериментів

---

## 📈 Майбутні Покращення

- [ ] Persistence (зберігання в БД)
- [ ] Experiment visualization
- [ ] Metrics plotting
- [ ] Experiment search
- [ ] Batch operations
- [ ] Experiment templates
- [ ] A/B testing support
- [ ] Integration з MLflow

---

**Версія**: v0.2.2  
**Дата**: 2026-01-22  
**Статус**: ✅ Реалізовано (Priority 6 - ML.5)
