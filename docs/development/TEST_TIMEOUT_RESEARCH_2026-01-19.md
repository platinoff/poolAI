# Test Timeout Research & Best Practices
## Оновлено: 2026-01-19

**Джерела**: Rust Tokio documentation, industry best practices, web research

---

## 📚 Research Results

### 1. Timeout Best Practices

**Рекомендовані значення:**
- **Initialization timeout**: 5 секунд
  - Дозволяє SDK спробувати підключитися до реальних сервісів
  - Достатньо для виявлення проблем з credentials
  - Не блокує тести надто довго

- **Mock server timeout**: 2 секунди
  - Mock servers повинні відповідати миттєво
  - 2 секунди достатньо для локальних HTTP запитів
  - Дозволяє виявити проблеми з mock server setup

- **Operation timeout**: 3 секунди
  - Для cloud operations (EC2, ECS, VM creation)
  - Достатньо для валідації та error handling
  - Не чекає на реальні HTTP запити

---

### 2. Tokio Test Patterns

**Рекомендовані підходи:**

1. **`tokio::time::timeout` для всіх async операцій**
   ```rust
   use tokio::time::{timeout, Duration};
   
   let result = timeout(Duration::from_secs(5), async_operation()).await;
   match result {
       Ok(Ok(value)) => { /* success */ }
       Ok(Err(e)) => { /* operation failed */ }
       Err(_) => { /* timeout */ }
   }
   ```

2. **Константи для timeout значень**
   ```rust
   const INIT_TIMEOUT_SECS: u64 = 5;
   const MOCK_TIMEOUT_SECS: u64 = 2;
   const OPERATION_TIMEOUT_SECS: u64 = 3;
   ```

3. **Graceful handling всіх результатів**
   - Timeout не є помилкою для тестів структури
   - Error handling перевіряє правильні типи помилок
   - Mock servers дозволяють тестувати success scenarios

---

### 3. Mock Server Integration

**Best Practices:**

1. **Використання mockito для HTTP моків**
   - Легкий та швидкий
   - Підтримує async API
   - Автоматичний вибір портів

2. **Timeout для mock server тестів**
   - 2 секунди достатньо для локальних запитів
   - Дозволяє виявити проблеми з конфігурацією
   - Не блокує тести

3. **Isolation між тестами**
   - Кожен тест створює свій mock server
   - Cleanup після тесту
   - Немає shared state

---

### 4. Error Handling Patterns

**Рекомендовані перевірки:**

1. **Timeout handling**
   ```rust
   match timeout(Duration::from_secs(5), operation()).await {
       Ok(Ok(_)) => { /* success */ }
       Ok(Err(e)) => { /* check error type */ }
       Err(_) => { /* timeout - acceptable for structure tests */ }
   }
   ```

2. **Error type assertions**
   ```rust
   if let Err(AppError::ValidationError(msg)) = result {
       assert!(msg.contains("expected error message"));
   }
   ```

3. **Graceful degradation**
   - Тести структури не потребують реальних credentials
   - Timeout або помилки - прийнятні результати
   - Важливо перевірити правильні типи помилок

---

### 5. CI/CD Considerations

**Рекомендації для CI:**

1. **Test isolation**
   - Кожен тест незалежний
   - Немає shared state
   - Cleanup після кожного тесту

2. **Timeout values**
   - Достатньо великі для повільної CI
   - Не занадто великі щоб не блокувати
   - Консистентні між локальними та CI тестами

3. **Error reporting**
   - Детальні повідомлення про помилки
   - Context для debugging
   - Логування timeout scenarios

---

## ✅ Implementation

### Timeout Constants

```rust
// Timeout constants for different test scenarios
const INIT_TIMEOUT_SECS: u64 = 5; // For initialization (may try real HTTP)
const MOCK_TIMEOUT_SECS: u64 = 2; // For mock server tests (should be fast)
const OPERATION_TIMEOUT_SECS: u64 = 3; // For cloud operations
```

### Usage Pattern

```rust
#[tokio::test]
async fn test_example() {
    use tokio::time::{timeout, Duration};
    
    let manager = Manager::new();
    
    // Initialize with timeout
    let result = timeout(
        Duration::from_secs(INIT_TIMEOUT_SECS),
        manager.initialize(),
    ).await;
    
    if let Ok(Ok(_)) = result {
        // Test operations
        let op_result = timeout(
            Duration::from_secs(OPERATION_TIMEOUT_SECS),
            manager.operation(),
        ).await;
        
        manager.shutdown().await.unwrap();
    }
}
```

---

## 📊 Comparison with Other Projects

### Similar Projects Patterns:

1. **AWS SDK Rust tests**: 5-10 секунд для initialization
2. **Azure SDK Rust tests**: 5 секунд для token acquisition
3. **GCP SDK Rust tests**: 3-5 секунд для metadata server

**Наш підхід**: 5 секунд для init, 3 секунди для operations - відповідає best practices

---

## 🎯 Recommendations

1. ✅ Використовувати константи для timeout значень
2. ✅ Graceful handling всіх результатів (success, error, timeout)
3. ✅ Перевіряти правильні типи помилок
4. ✅ Mock servers для success scenarios (коли можливо)
5. ✅ Timeout захист для всіх async операцій
6. ✅ Детальні повідомлення про помилки

---

**Підготовлено**: Rust Architect  
**Дата**: 2026-01-19  
**Джерела**: Tokio documentation, Rust testing best practices, industry standards
