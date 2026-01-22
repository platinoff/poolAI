# Federated Learning (ML.3)
## Дата: 2026-01-22

Модуль федеративного навчання для розподіленого навчання моделей з підтримкою приватності та безпечної агрегації.

---

## 🎯 Мета

Федеративне навчання з підтримкою:
- Client-server communication protocol
- Model updates aggregation (FedAvg, FedProx)
- Privacy-preserving techniques
- Secure aggregation
- Round management

---

## 📚 API

### Створення Pipeline

```rust
use poolai::ml::federated::{FederatedLearningPipeline, FederatedConfig};

let config = FederatedConfig::default_config();
let pipeline = FederatedLearningPipeline::new(config);
```

### Додавання Client Updates

```rust
use poolai::ml::federated::{FederatedLearningPipeline, ClientUpdate};

let pipeline = FederatedLearningPipeline::default();

let update = ClientUpdate {
    client_id: "client1".to_string(),
    model_weights: vec![0.5, 0.3, 0.2],
    sample_count: 100,
    round: 0,
};

pipeline.add_client_update(update).await?;
```

### Агрегація Updates

```rust
let aggregated = pipeline.aggregate_updates().await?;
println!("Aggregated model: {} weights, {} clients",
    aggregated.weights.len(), aggregated.clients_count);
```

### Управління Rounds

```rust
// Start new round
let round = pipeline.start_round().await;

// Get current round
let current = pipeline.get_current_round().await;

// Get model for specific round
let model = pipeline.get_round_model(1).await;
```

---

## 📊 Структури

### FederatedConfig

- `aggregation: AggregationMode` - Режим агрегації (FedAvg, FedProx)
- `min_clients_per_round: u32` - Мінімальна кількість клієнтів
- `max_clients_per_round: u32` - Максимальна кількість клієнтів
- `rounds: u32` - Кількість раундів
- `privacy_budget: f64` - Бюджет приватності
- `secure_aggregation: bool` - Безпечна агрегація

### ClientUpdate

- `client_id: String` - ID клієнта
- `model_weights: Vec<f64>` - Ваги моделі
- `sample_count: usize` - Кількість зразків
- `round: u32` - Номер раунду

### AggregatedModel

- `weights: Vec<f64>` - Агреговані ваги
- `total_samples: usize` - Загальна кількість зразків
- `clients_count: usize` - Кількість клієнтів
- `round: u32` - Номер раунду
- `aggregation_mode: AggregationMode` - Режим агрегації

### AggregationMode

- `FedAvg` - Federated Averaging (за замовчуванням)
- `FedProx` - Federated Proximal

---

## 🔍 Функціональність

### Federated Averaging (FedAvg)

Weighted average aggregation на основі кількості зразків кожного клієнта:

```
aggregated_weight[i] = Σ(client_weight[i] * client_samples / total_samples)
```

### Federated Proximal (FedProx)

Подібно до FedAvg, але з додаванням proximal term для регуляризації:

```
aggregated_weight[i] = fedavg_weight[i] * (1 - μ)
```

де μ - proximal parameter (за замовчуванням 0.01).

### Round Management

- Автоматичне управління раундами
- Зберігання історії раундів
- Валідація синхронізації клієнтів

### Privacy & Security

- Підтримка privacy budget (планується)
- Secure aggregation flag (планується)
- Differential privacy (планується)

---

## 🧪 Тестування

```bash
cargo test --test ml_federated_integration --features ml
```

Тести покривають:
- Створення pipeline
- Додавання client updates
- Агрегацію (FedAvg, FedProx)
- Управління раундами
- Обробку помилок
- Великі моделі
- Weighted averaging

---

## 📝 Приклади Використання

### Базовий Приклад

```rust
use poolai::ml::federated::{FederatedLearningPipeline, ClientUpdate};

#[tokio::main]
async fn main() -> Result<(), poolai::core::error::AppError> {
    let pipeline = FederatedLearningPipeline::default();

    // Add client updates
    let update1 = ClientUpdate {
        client_id: "client1".to_string(),
        model_weights: vec![1.0, 2.0, 3.0],
        sample_count: 100,
        round: 0,
    };

    let update2 = ClientUpdate {
        client_id: "client2".to_string(),
        model_weights: vec![2.0, 3.0, 4.0],
        sample_count: 200,
        round: 0,
    };

    pipeline.add_client_update(update1).await?;
    pipeline.add_client_update(update2).await?;

    // Aggregate
    let aggregated = pipeline.aggregate_updates().await?;
    println!("Aggregated model: {} weights", aggregated.weights.len());
    println!("Total samples: {}", aggregated.total_samples);

    Ok(())
}
```

### Множинні Раунди

```rust
use poolai::ml::federated::{FederatedLearningPipeline, ClientUpdate};

#[tokio::main]
async fn main() -> Result<(), poolai::core::error::AppError> {
    let mut config = FederatedConfig::default_config();
    config.min_clients_per_round = 2;
    config.rounds = 5;

    let pipeline = FederatedLearningPipeline::new(config);

    for round in 1..=5 {
        pipeline.start_round().await;

        // Simulate client updates
        for client_id in 0..3 {
            let update = ClientUpdate {
                client_id: format!("client{}", client_id),
                model_weights: vec![0.5, 0.3, 0.2],
                sample_count: 100,
                round,
            };
            pipeline.add_client_update(update).await?;
        }

        // Aggregate
        let aggregated = pipeline.aggregate_updates().await?;
        println!("Round {}: {} clients, {} samples",
            aggregated.round, aggregated.clients_count, aggregated.total_samples);

        // Retrieve model later
        let model = pipeline.get_round_model(round).await;
        assert!(model.is_some());
    }

    Ok(())
}
```

### FedProx Aggregation

```rust
use poolai::ml::federated::{FederatedLearningPipeline, FederatedConfig, AggregationMode, ClientUpdate};

#[tokio::main]
async fn main() -> Result<(), poolai::core::error::AppError> {
    let mut config = FederatedConfig::default_config();
    config.aggregation = AggregationMode::FedProx;
    config.min_clients_per_round = 2;

    let pipeline = FederatedLearningPipeline::new(config);

    let update1 = ClientUpdate {
        client_id: "client1".to_string(),
        model_weights: vec![1.0, 2.0],
        sample_count: 100,
        round: 0,
    };

    let update2 = ClientUpdate {
        client_id: "client2".to_string(),
        model_weights: vec![2.0, 3.0],
        sample_count: 100,
        round: 0,
    };

    pipeline.add_client_update(update1).await?;
    pipeline.add_client_update(update2).await?;

    let aggregated = pipeline.aggregate_updates().await?;
    assert_eq!(aggregated.aggregation_mode, AggregationMode::FedProx);

    Ok(())
}
```

### Перевірка Готовності

```rust
use poolai::ml::federated::FederatedLearningPipeline;

#[tokio::main]
async fn main() -> Result<(), poolai::core::error::AppError> {
    let pipeline = FederatedLearningPipeline::default();

    // Add updates
    // ...

    // Check if ready
    if pipeline.is_ready_for_aggregation().await {
        let aggregated = pipeline.aggregate_updates().await?;
        println!("Aggregated successfully");
    } else {
        println!("Waiting for more clients...");
        println!("Pending: {}", pipeline.get_pending_updates_count().await);
    }

    Ok(())
}
```

---

## 🔗 Інтеграція

### З Network API

Federated Learning доступний через REST API:
- `GET /api/enterprise/ai-ml/federated` - статус
- `POST /api/enterprise/ai-ml/federated/update` - додати client update
- `POST /api/enterprise/ai-ml/federated/aggregate` - агрегувати
- `GET /api/enterprise/ai-ml/federated/round/{round}` - отримати модель раунду

### З AutoML Module

Federated Learning може використовувати AutoML для:
- Model selection на клієнтах
- Hyperparameter optimization
- Ensemble aggregation

---

## 📈 Майбутні Покращення

- [ ] Реальна реалізація secure aggregation
- [ ] Differential privacy інтеграція
- [ ] Client authentication та authorization
- [ ] Model compression перед передачею
- [ ] Asynchronous client updates
- [ ] Fault tolerance та recovery
- [ ] Performance optimization для великих моделей
- [ ] Real-time monitoring та metrics

---

**Версія**: v0.2.2  
**Дата**: 2026-01-22  
**Статус**: ✅ Реалізовано (Priority 3 - Ітерація 3)
