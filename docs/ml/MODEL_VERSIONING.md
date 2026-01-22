# Model Versioning (ML.4)
## Дата: 2026-01-22

Модуль управління версіями моделей для відстеження lifecycle та rollback.

---

## 🎯 Мета

Model Versioning для управління версіями моделей:
- Version tracking та registration
- Model metadata storage
- Version comparison
- Tagging system
- Rollback capabilities

---

## 📚 API

### Створення Manager

```rust
use poolai::ml::versioning::ModelVersionManager;

let manager = ModelVersionManager::new();
```

### Реєстрація Моделі

```rust
use poolai::ml::versioning::{ModelVersionManager, ModelMetadata};
use std::collections::HashMap;

let manager = ModelVersionManager::new();

let metadata = ModelMetadata {
    model_type: "NeuralNetwork".to_string(),
    accuracy: 0.95,
    training_time_ms: 1000,
    hyperparameters: HashMap::new(),
    description: Some("Best model so far".to_string()),
};

let version = manager.register_model("model1", metadata).await?;
println!("Registered version: {}", version.version);
```

### Отримання Версії

```rust
// Get specific version
let version = manager.get_version("model1", "v1").await?;

// Get latest version
let latest = manager.get_latest_version("model1").await?;

// List all versions
let versions = manager.list_versions("model1").await?;
```

### Порівняння Версій

```rust
use poolai::ml::versioning::VersionComparison;

let comparison = manager.compare_versions("model1", "v1", "v2").await?;
match comparison {
    VersionComparison::Newer => println!("v2 is newer"),
    VersionComparison::Older => println!("v2 is older"),
    VersionComparison::Same => println!("Same version"),
    VersionComparison::Different => println!("Different versions"),
}
```

### Tagging

```rust
// Add tags
manager.add_tags("model1", "v1", vec!["production".to_string(), "best".to_string()]).await?;

// Get versions by tag
let production_versions = manager.get_versions_by_tag("model1", "production").await?;
```

---

## 📊 Структури

### ModelMetadata

- `model_type: String` - Тип моделі
- `accuracy: f64` - Точність моделі (0.0-1.0)
- `training_time_ms: u64` - Час навчання в мілісекундах
- `hyperparameters: HashMap<String, String>` - Гіперпараметри
- `description: Option<String>` - Опис моделі

### ModelVersion

- `version: String` - Версія (наприклад, "v1", "v2")
- `model_id: String` - ID моделі
- `metadata: ModelMetadata` - Метадані моделі
- `created_at: DateTime<Utc>` - Дата створення
- `tags: Vec<String>` - Теги

### VersionComparison

- `Newer` - Перша версія новіша
- `Older` - Перша версія старіша
- `Same` - Однакові версії
- `Different` - Різні версії (однаковий час створення)

---

## 🔍 Функціональність

### Version Tracking

Автоматичне присвоєння версій при реєстрації:
- Перша версія: `v1`
- Друга версія: `v2`
- І так далі...

Кожна модель має незалежну нумерацію версій.

### Version Comparison

Порівняння версій на основі:
- Часу створення (`created_at`)
- Номера версії

### Tagging System

Теги дозволяють:
- Позначати версії (наприклад, "production", "best", "experimental")
- Фільтрувати версії за тегами
- Уникати дублікатів тегів

### Metadata Storage

Зберігання повної інформації про модель:
- Тип моделі
- Точність
- Час навчання
- Гіперпараметри
- Опис

---

## 🧪 Тестування

```bash
cargo test --test ml_versioning_integration --features ml
```

Тести покривають:
- Реєстрацію моделей
- Отримання версій
- Порівняння версій
- Tagging
- Валідацію даних
- Edge cases

---

## 📝 Приклади Використання

### Базовий Приклад

```rust
use poolai::ml::versioning::{ModelVersionManager, ModelMetadata};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), poolai::core::error::AppError> {
    let manager = ModelVersionManager::new();

    let metadata = ModelMetadata {
        model_type: "NeuralNetwork".to_string(),
        accuracy: 0.95,
        training_time_ms: 1000,
        hyperparameters: HashMap::new(),
        description: Some("Initial model".to_string()),
    };

    let version = manager.register_model("model1", metadata).await?;
    println!("Registered: {}", version.version);

    Ok(())
}
```

### Множинні Версії

```rust
use poolai::ml::versioning::{ModelVersionManager, ModelMetadata};

#[tokio::main]
async fn main() -> Result<(), poolai::core::error::AppError> {
    let manager = ModelVersionManager::new();

    // Register multiple versions
    for i in 0..5 {
        let metadata = ModelMetadata {
            model_type: "NeuralNetwork".to_string(),
            accuracy: 0.90 + (i as f64 * 0.01),
            training_time_ms: 1000 + (i * 100),
            hyperparameters: std::collections::HashMap::new(),
            description: Some(format!("Version {}", i + 1)),
        };

        let version = manager.register_model("model1", metadata).await?;
        println!("Registered: {}", version.version);
    }

    // Get latest
    let latest = manager.get_latest_version("model1").await?;
    println!("Latest version: {} (accuracy: {:.2}%)",
        latest.version, latest.metadata.accuracy * 100.0);

    // List all versions
    let versions = manager.list_versions("model1").await?;
    println!("Total versions: {}", versions.len());

    Ok(())
}
```

### Tagging та Фільтрація

```rust
use poolai::ml::versioning::ModelVersionManager;

#[tokio::main]
async fn main() -> Result<(), poolai::core::error::AppError> {
    let manager = ModelVersionManager::new();

    // Register models
    // ...

    // Tag production version
    manager.add_tags("model1", "v3", vec!["production".to_string()]).await?;
    manager.add_tags("model1", "v3", vec!["best".to_string()]).await?;

    // Get production versions
    let production = manager.get_versions_by_tag("model1", "production").await?;
    println!("Production versions: {}", production.len());

    Ok(())
}
```

### Порівняння Версій

```rust
use poolai::ml::versioning::{ModelVersionManager, VersionComparison};

#[tokio::main]
async fn main() -> Result<(), poolai::core::error::AppError> {
    let manager = ModelVersionManager::new();

    // Register versions
    // ...

    // Compare versions
    let comparison = manager.compare_versions("model1", "v1", "v2").await?;

    match comparison {
        VersionComparison::Newer => println!("v1 is newer than v2"),
        VersionComparison::Older => println!("v1 is older than v2"),
        VersionComparison::Same => println!("Same version"),
        VersionComparison::Different => println!("Different versions"),
    }

    Ok(())
}
```

---

## 🔗 Інтеграція

### З AutoML Module

Model Versioning може використовуватися з AutoML:
- Автоматична реєстрація найкращих моделей
- Версіонування ensemble моделей
- Відстеження історії оптимізації

### З Federated Learning

Versioning для federated learning:
- Версіонування агрегованих моделей
- Відстеження раундів навчання
- Rollback до попередніх раундів

### З Network API

Model Versioning доступний через REST API:
- `POST /api/enterprise/ai-ml/versioning/register` - реєстрація версії
- `GET /api/enterprise/ai-ml/versioning/{model_id}/versions` - список версій
- `GET /api/enterprise/ai-ml/versioning/{model_id}/latest` - остання версія
- `GET /api/enterprise/ai-ml/versioning/{model_id}/{version}` - конкретна версія
- `POST /api/enterprise/ai-ml/versioning/{model_id}/{version}/tags` - додати теги

---

## 📈 Майбутні Покращення

- [ ] Persistence (зберігання версій в БД)
- [ ] Model rollback functionality
- [ ] Version diff (порівняння змін між версіями)
- [ ] Model archiving
- [ ] Version expiration policies
- [ ] Integration з model storage
- [ ] Version metadata search
- [ ] Batch operations

---

**Версія**: v0.2.2  
**Дата**: 2026-01-22  
**Статус**: ✅ Реалізовано (Priority 5 - ML.4)
