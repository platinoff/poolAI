# AutoML Integration (ML.2)
## Дата: 2026-01-22

Модуль автоматизованого машинного навчання для автоматичного вибору моделей, оптимізації гіперпараметрів та створення ансамблів.

---

## 🎯 Мета

Автоматизоване машинне навчання з підтримкою:
- Автоматичний вибір моделі
- Оптимізація гіперпараметрів
- Створення ансамблів моделей
- Агрегація передбачень
- Feature engineering (планується)

---

## 📚 API

### Створення Pipeline

```rust
use poolai::ml::automl::{AutoMLPipeline, AutomlConfig};

let config = AutomlConfig::default_config();
let pipeline = AutoMLPipeline::new(config);
```

### Навчання Моделі

```rust
use poolai::ml::automl::{AutoMLPipeline, TrainingData};

let pipeline = AutoMLPipeline::default();

let data = TrainingData {
    features: vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
    ],
    labels: vec![0.0, 1.0],
};

let model = pipeline.train(data).await?;
println!("Best model: {:?}, Accuracy: {:.2}%", 
    model.model_type, model.accuracy * 100.0);
```

### Створення Ансамблю

```rust
let ensemble = pipeline.create_ensemble(3).await?;
println!("Ensemble with {} models", ensemble.len());
```

### Агрегація Передбачень

```rust
let models = vec![/* trained models */];
let predictions = vec![vec![0.5, 0.3], vec![0.6, 0.4]];

let aggregated = pipeline.aggregate_predictions(&models, &predictions).await?;
```

---

## 📊 Структури

### AutomlConfig

- `auto_feature_engineering: bool` - Автоматична feature engineering
- `max_trials: u32` - Максимальна кількість спроб
- `timeout_seconds: u64` - Таймаут навчання
- `ensemble_size: usize` - Розмір ансамблю
- `cross_validation_folds: u32` - Кількість фолдів для cross-validation

### TrainingData

- `features: Vec<Vec<f64>>` - Вектори ознак
- `labels: Vec<f64>` - Мітки

### TrainedModel

- `model_type: ModelType` - Тип моделі
- `accuracy: f64` - Точність моделі
- `hyperparameters: HashMap<String, String>` - Гіперпараметри
- `training_time_ms: u64` - Час навчання
- `model_id: String` - ID моделі

### ModelType

- `LinearRegression` - Лінійна регресія
- `RandomForest` - Випадковий ліс
- `GradientBoosting` - Градієнтний бустинг
- `NeuralNetwork` - Нейронна мережа
- `SupportVectorMachine` - SVM

---

## 🔍 Функціональність

### Model Selection

Pipeline автоматично оцінює всі доступні типи моделей та вибирає найкращу на основі точності.

### Hyperparameter Optimization

Для кожного типу моделі генеруються оптимальні гіперпараметри:
- LinearRegression: learning_rate, max_iterations
- RandomForest: n_estimators, max_depth, min_samples_split
- GradientBoosting: n_estimators, learning_rate, max_depth
- NeuralNetwork: hidden_layers, learning_rate, epochs
- SupportVectorMachine: kernel, C, gamma

### Ensemble Methods

Створення ансамблю з топ-N моделей для покращення точності.

### Aggregation

Weighted average aggregation з вагами на основі точності моделей.

---

## 🧪 Тестування

```bash
cargo test --test ml_automl_integration --features ml
```

Тести покривають:
- Створення pipeline
- Навчання моделей
- Отримання найкращої моделі
- Отримання кандидатів
- Створення ансамблю
- Агрегацію передбачень
- Обробку помилок

---

## 📝 Приклади Використання

### Базовий Приклад

```rust
use poolai::ml::automl::{AutoMLPipeline, TrainingData};

#[tokio::main]
async fn main() -> Result<(), poolai::core::error::AppError> {
    let pipeline = AutoMLPipeline::default();

    let data = TrainingData {
        features: vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ],
        labels: vec![0.0, 1.0, 0.0],
    };

    let model = pipeline.train(data).await?;
    println!("Best model: {:?}", model.model_type);
    println!("Accuracy: {:.2}%", model.accuracy * 100.0);

    Ok(())
}
```

### Створення Ансамблю

```rust
use poolai::ml::automl::AutoMLPipeline;

#[tokio::main]
async fn main() -> Result<(), poolai::core::error::AppError> {
    let pipeline = AutoMLPipeline::default();

    // Train models
    let data = TrainingData {
        features: vec![vec![1.0, 2.0], vec![3.0, 4.0]],
        labels: vec![0.0, 1.0],
    };
    pipeline.train(data).await?;

    // Create ensemble with top 3 models
    let ensemble = pipeline.create_ensemble(3).await?;
    println!("Ensemble created with {} models", ensemble.len());

    for (i, model) in ensemble.iter().enumerate() {
        println!("Model {}: {:?} (accuracy: {:.2}%)",
            i + 1, model.model_type, model.accuracy * 100.0);
    }

    Ok(())
}
```

### Агрегація Передбачень

```rust
use poolai::ml::automl::{AutoMLPipeline, TrainedModel, ModelType};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), poolai::core::error::AppError> {
    let pipeline = AutoMLPipeline::default();

    let models = vec![
        TrainedModel {
            model_type: ModelType::LinearRegression,
            accuracy: 0.8,
            hyperparameters: HashMap::new(),
            training_time_ms: 100,
            model_id: "model1".to_string(),
        },
        TrainedModel {
            model_type: ModelType::RandomForest,
            accuracy: 0.9,
            hyperparameters: HashMap::new(),
            training_time_ms: 200,
            model_id: "model2".to_string(),
        },
    ];

    let predictions = vec![
        vec![0.5, 0.3, 0.2],
        vec![0.6, 0.4, 0.3],
    ];

    let aggregated = pipeline.aggregate_predictions(&models, &predictions).await?;
    println!("Aggregated predictions: {:?}", aggregated);

    Ok(())
}
```

---

## 🔗 Інтеграція

### З Network API

AutoML доступний через REST API:
- `GET /api/enterprise/ai-ml/automl` - статус AutoML
- `POST /api/enterprise/ai-ml/automl/train` - навчання моделі
- `GET /api/enterprise/ai-ml/automl/best` - найкраща модель
- `POST /api/enterprise/ai-ml/automl/ensemble` - створення ансамблю

### З Optimization Module

AutoML використовує ML.1 Optimization для:
- Hyperparameter tuning
- Model profiling
- Quantization

---

## 📈 Майбутні Покращення

- [ ] Реальна реалізація навчання моделей (зараз симуляція)
- [ ] Feature engineering автоматизація
- [ ] Cross-validation інтеграція
- [ ] Distributed training
- [ ] Model persistence
- [ ] Model versioning
- [ ] Experiment tracking

---

**Версія**: v0.2.2  
**Дата**: 2026-01-22  
**Статус**: ✅ Реалізовано (Priority 2 - Ітерація 2)
