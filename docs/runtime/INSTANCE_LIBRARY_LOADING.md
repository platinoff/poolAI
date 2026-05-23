# Runtime Instance Library Model Loading
## Дата: 2026-01-22

Покращення Runtime Instance для завантаження моделей з бібліотек.

---

## 🎯 Мета

Додати підтримку завантаження моделей з LibraryManager в Runtime Instance:
- Автоматичне визначення моделей з бібліотек
- Зберігання метаданих бібліотеки в instance
- Обробка запитів через library models

---

## 📚 API

### Автоматичне Завантаження

При створенні instance, система автоматично перевіряє LibraryManager:

```rust
use poolai::runtime::instance::{InstanceManager, InstancePlacement, PlacementStrategy};
use std::collections::HashMap;

let manager = InstanceManager::new();

let placement = InstancePlacement {
    strategy: PlacementStrategy::Single,
    node_ids: vec!["node1".to_string()],
    memory_by_node: HashMap::new(),
    memory_delta: 0,
    error: None,
};

let instance_id = manager.create_instance(
    "model-from-library".to_string(),
    placement,
    HashMap::new(),
).await?;

// Якщо бібліотека знайдена, метадані автоматично додаються:
// - library_path: шлях до бібліотеки
// - library_version: версія бібліотеки
// - library_loaded: "true"
```

### Обробка Запитів

Instance автоматично обробляє запити через library models:

```rust
use poolai::core::model_interface::{ModelRequest, ModelParameters};

let request = ModelRequest {
    input: "Test input".to_string(),
    parameters: ModelParameters::default(),
    session_id: None,
    priority: 5,
    timeout: Some(30),
};

let response = manager.process_request_via_instance(&instance_id, request).await?;
```

---

## 🔍 Функціональність

### Автоматичне Визначення

При створенні instance:
1. Перевіряє ModelManager
2. Якщо не знайдено, перевіряє LibraryManager
3. Якщо бібліотека знайдена, зберігає метадані в instance

### Метадані Бібліотеки

Instance зберігає:
- `library_path`: Шлях до бібліотеки
- `library_version`: Версія бібліотеки
- `library_loaded`: Прапорець завантаження

### Обробка Запитів

Пріоритет обробки:
1. ModelInterface з instance (якщо завантажено)
2. ModelManager (якщо модель зареєстрована)
3. LibraryManager (якщо бібліотека доступна)

---

## 📝 Приклади Використання

### Базовий Приклад

```rust
use poolai::runtime::instance::{InstanceManager, InstancePlacement, PlacementStrategy};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), poolai::core::error::AppError> {
    let manager = InstanceManager::new();

    // Створити placement
    let placement = InstancePlacement {
        strategy: PlacementStrategy::Single,
        node_ids: vec!["node1".to_string()],
        memory_by_node: HashMap::new(),
        memory_delta: 0,
        error: None,
    };

    // Створити instance - автоматично завантажить з бібліотеки якщо доступна
    let instance_id = manager.create_instance(
        "my-model".to_string(),
        placement,
        HashMap::new(),
    ).await?;

    // Перевірити метадані
    let instance = manager.get_instance(&instance_id).await.unwrap();
    if let Some(library_path) = instance.metadata.get("library_path") {
        println!("Model loaded from library: {}", library_path);
    }

    Ok(())
}
```

---

## 🔗 Інтеграція

### З LibraryManager

Instance автоматично інтегрується з LibraryManager:
- Перевірка наявності бібліотек
- Зберігання метаданих
- Використання для обробки запитів

### З ModelManager

Instance також підтримує ModelManager:
- Пріоритет ModelManager над LibraryManager
- Fallback до LibraryManager якщо модель не знайдена

---

## 📈 Майбутні Покращення

- [x] Реальна реалізація ModelInterface для library models (FM-035 — `model_loader.rs`)
- [x] Інтеграція з libtorch, onnx path detect + weight validation (native inference — optional future feature)
- [ ] Lazy loading моделей
- [ ] Model caching
- [ ] Performance optimization

---

**Версія**: v0.2.2  
**Дата**: 2026-01-22  
**Статус**: ✅ Реалізовано (Runtime Instance Enhancement)
