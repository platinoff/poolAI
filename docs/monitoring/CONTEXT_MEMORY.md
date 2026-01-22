# Context Memory Monitoring
## Дата: 2026-01-22

Модуль моніторингу контекстної пам'яті для AI моделей, зокрема для інтеграції з Cursor AI.

---

## 🎯 Мета

Моніторинг та відстеження використання контекстної пам'яті для AI моделей:
- Відстеження розміру контексту (поточний, максимальний, середній)
- Відстеження змін (додавання, модифікація, видалення файлів)
- Моніторинг використання пам'яті (RAM, диск, кеш)
- Оптимізація контексту (виявлення проблем та пропозиції)

---

## 📚 API

### Створення Monitor

```rust
use poolai::monitoring::context_memory::ContextMemoryMonitor;

let monitor = ContextMemoryMonitor::new();
```

### Відстеження Змін

```rust
// Додати файл до контексту
monitor.track_file_added("src/main.rs", 1024).await?;

// Модифікувати файл
monitor.track_file_modified("src/lib.rs", 2048).await?;

// Видалити файл
monitor.track_file_deleted("src/old.rs").await?;

// Очистити контекст
monitor.track_context_cleared().await?;
```

### Отримання Метрик

```rust
let metrics = monitor.get_metrics().await;

println!("Current size: {} bytes", metrics.current_size);
println!("Max size: {} bytes", metrics.max_size);
println!("Average size: {:.2} bytes", metrics.average_size);
println!("File count: {}", metrics.file_count);
println!("Changes count: {}", metrics.changes_count);
```

### Отримання Змін

```rust
// Останні 10 змін
let recent = monitor.get_recent_changes(10).await;

// Зміни за останню хвилину
let changes = monitor.get_changes_in_window(Duration::from_secs(60)).await;
```

### Оптимізація

```rust
let suggestions = monitor.suggest_optimizations().await;
for suggestion in suggestions {
    println!("Suggestion: {}", suggestion);
}
```

---

## 📊 Метрики

### ContextMetrics

- `current_size: usize` - Поточний розмір контексту в байтах
- `max_size: usize` - Максимальний розмір контексту
- `average_size: f64` - Середній розмір контексту
- `file_count: usize` - Кількість файлів в контексті
- `changes_count: usize` - Загальна кількість змін
- `memory_usage: MemoryUsage` - Використання пам'яті
- `last_update: Instant` - Час останнього оновлення

### MemoryUsage

- `ram_bytes: usize` - Використання RAM в байтах
- `disk_bytes: usize` - Використання диска в байтах
- `cache_bytes: usize` - Використання кешу в байтах
- `timestamp: Instant` - Час збору метрик

---

## 🔍 Типи Змін

- `FileAdded` - Файл додано до контексту
- `FileModified` - Файл модифіковано
- `FileDeleted` - Файл видалено з контексту
- `ContextCleared` - Контекст очищено

---

## 💡 Оптимізації

Модуль автоматично виявляє проблеми та пропонує оптимізації:

1. **Великий розмір контексту** (>10MB)
   - Пропозиція: Видалити невикористані файли або розбити контекст на менші частини

2. **Багато файлів** (>100)
   - Пропозиція: Групувати пов'язані файли або використовувати резюме

3. **Високе використання RAM** (>50MB)
   - Пропозиція: Використовувати дискове кешування для рідко використовуваних файлів

4. **Висока частота змін** (>50 змін/хв)
   - Пропозиція: Батчувати зміни або зменшити частоту оновлень

---

## 🧪 Тестування

```bash
cargo test --test context_memory_integration
```

Тести покривають:
- Створення monitor
- Відстеження файлів (додавання, модифікація, видалення)
- Очищення контексту
- Обчислення метрик
- Отримання змін
- Оптимізації

---

## 📝 Приклади Використання

### Базовий Приклад

```rust
use poolai::monitoring::context_memory::ContextMemoryMonitor;

#[tokio::main]
async fn main() -> Result<(), poolai::core::error::AppError> {
    let monitor = ContextMemoryMonitor::new();

    // Відстеження файлів
    monitor.track_file_added("src/main.rs", 1024).await?;
    monitor.track_file_added("src/lib.rs", 2048).await?;

    // Отримання метрик
    let metrics = monitor.get_metrics().await;
    println!("Context size: {} bytes", metrics.current_size);
    println!("Files: {}", metrics.file_count);

    // Перевірка оптимізацій
    let suggestions = monitor.suggest_optimizations().await;
    if !suggestions.is_empty() {
        println!("Optimization suggestions:");
        for suggestion in suggestions {
            println!("  - {}", suggestion);
        }
    }

    Ok(())
}
```

### Моніторинг Змін

```rust
use poolai::monitoring::context_memory::ContextMemoryMonitor;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), poolai::core::error::AppError> {
    let monitor = ContextMemoryMonitor::new();

    // Відстеження змін
    monitor.track_file_added("src/file1.rs", 1000).await?;
    monitor.track_file_modified("src/file1.rs", 2000).await?;
    monitor.track_file_deleted("src/file1.rs").await?;

    // Отримання останніх змін
    let recent = monitor.get_recent_changes(10).await;
    for change in recent {
        println!("{:?}: {} ({} bytes)",
            change.change_type,
            change.file_path,
            change.size_bytes
        );
    }

    // Отримання змін за останню хвилину
    let changes = monitor.get_changes_in_window(Duration::from_secs(60)).await;
    println!("Changes in last minute: {}", changes.len());

    Ok(())
}
```

---

## 🔗 Інтеграція з Cursor AI

Модуль призначений для інтеграції з Cursor AI для моніторингу контекстної пам'яті під час розробки:

1. **Відстеження змін файлів** - автоматичне відстеження додавання/модифікації файлів
2. **Метрики використання** - моніторинг розміру контексту та використання пам'яті
3. **Оптимізація** - автоматичні пропозиції для оптимізації контексту

---

## 📈 Майбутні Покращення

- [ ] Інтеграція з системними API для точного вимірювання пам'яті
- [ ] Підтримка дискового кешування
- [ ] Автоматична оптимізація контексту
- [ ] Експорт метрик в Prometheus
- [ ] Інтеграція з Cursor AI API

---

**Версія**: v0.2.2  
**Дата**: 2026-01-22  
**Статус**: ✅ Реалізовано
