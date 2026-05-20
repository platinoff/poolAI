# 📊 Enterprise Monitoring Persistence - Implementation Plan
## План реалізації persistence для metrics history

**Дата створення**: 2026-01-17  
**Версія**: 1.0  
**Статус**: **MVP Implemented ✅** (FM-030, 2026-05-20)  
**Оцінка**: 1-2 дні

---

## 🎯 Мета

Додати persistent storage для historical metrics в Enterprise Monitoring, щоб зберігати метрики довше, ніж дозволяє in-memory storage (зараз 1000 точок).

---

## 📋 Поточний стан

**Файл**: `src/enterprise/monitoring.rs`

**Статус**: ⚠️ In-memory storage only
- `metrics_history: Vec<MetricDataPoint>` - зберігає максимум 1000 точок в пам'яті
- `get_metric_history()` - фільтрує в пам'яті
- Немає persistence для long-term storage

**TODO коментарі**: 
- `src/enterprise/monitoring.rs:192-194` - Initialize dashboard storage

---

## 🔧 План реалізації

### Варіант 1: SQLite (Рекомендований) ⭐⭐⭐

**Переваги**:
- Легкий, не потребує окремого сервера
- Підтримує SQL запити з індексами
- Добре працює для time-series data
- Простий deployment (один файл)

**Недоліки**:
- Може бути повільним при великій кількості записів (>1M)
- Немає built-in time-series оптимізацій

**Оцінка**: 1-2 дні

#### Крок 1: Додати rusqlite dependency (0.5 дня)

**Файл**: `Cargo.toml`

```toml
[dependencies]
# ... existing dependencies ...

# SQLite support for metrics persistence (optional)
rusqlite = { version = "0.32", optional = true, features = ["bundled"] }

[features]
enterprise = ["rusqlite"]  # Enable SQLite when enterprise feature is enabled
```

**Альтернатива**: `sqlx` з SQLite driver (більш async-friendly)

#### Крок 2: Створити database schema (0.5 дня)

**Файл**: `src/enterprise/monitoring.rs`

```rust
// SQL schema for metrics_history table
const CREATE_METRICS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS metrics_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,
    metric TEXT NOT NULL,
    value REAL NOT NULL,
    tags TEXT,  -- JSON string
    tenant_id TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_metrics_timestamp ON metrics_history(timestamp);
CREATE INDEX IF NOT EXISTS idx_metrics_metric ON metrics_history(metric);
CREATE INDEX IF NOT EXISTS idx_metrics_tenant ON metrics_history(tenant_id);
CREATE INDEX IF NOT EXISTS idx_metrics_metric_timestamp ON metrics_history(metric, timestamp);
"#;
```

#### Крок 3: Інтегрувати SQLite в MonitoringManager (1 день)

**Зміни**:
1. Додати `db_conn: Option<Arc<Mutex<Connection>>>` до `MonitoringManager`
2. Оновити `initialize()` для створення БД та таблиць
3. Оновити `record_metric()` для INSERT в БД
4. Оновити `get_metric_history()` для SELECT з БД
5. Додати cleanup task для видалення старих metrics (retention policy)

**Приклад коду**:

```rust
use rusqlite::{Connection, params, Result as SqlResult};

pub struct MonitoringManager {
    // ... existing fields ...
    db_conn: Option<Arc<Mutex<Connection>>>,
}

impl MonitoringManager {
    pub async fn initialize(&self) -> Result<(), AppError> {
        // ... existing init code ...

        // Initialize SQLite if db_path is configured
        if let Some(ref db_path) = self.db_path {
            let conn = Connection::open(db_path)?;
            conn.execute(CREATE_METRICS_TABLE, [])?;
            // Store connection (with Mutex for thread safety)
            // self.db_conn = Some(Arc::new(Mutex::new(conn)));
            info!("SQLite database initialized: {}", db_path);
        }

        *initialized = true;
        Ok(())
    }

    pub async fn record_metric(&self, data_point: MetricDataPoint) -> Result<(), AppError> {
        // Store in-memory (cache)
        // ... existing code ...

        // Persist to SQLite if configured
        if let Some(ref conn) = self.db_conn {
            let conn = conn.lock().await;
            let tags_json = serde_json::to_string(&data_point.tags)?;
            conn.execute(
                "INSERT INTO metrics_history (timestamp, metric, value, tags, tenant_id) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    data_point.timestamp.to_rfc3339(),
                    data_point.metric,
                    data_point.value,
                    tags_json,
                    data_point.tenant_id.map(|id| id.to_string())
                ]
            )?;
        }

        // Check alert rules
        self.check_alert_rules(&data_point).await?;
        Ok(())
    }

    pub async fn get_metric_history(
        &self,
        metric: Option<&str>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        tenant_id: Option<Uuid>,
        limit: Option<usize>,
    ) -> Result<Vec<MetricDataPoint>, AppError> {
        // If SQLite is configured, query from database
        if let Some(ref conn) = self.db_conn {
            let conn = conn.lock().await;
            // Build SQL query with filters
            // SELECT timestamp, metric, value, tags, tenant_id
            // FROM metrics_history
            // WHERE (? IS NULL OR metric = ?)
            //   AND (? IS NULL OR timestamp >= ?)
            //   AND (? IS NULL OR timestamp <= ?)
            //   AND (? IS NULL OR tenant_id = ?)
            // ORDER BY timestamp DESC
            // LIMIT ?
            // ... execute query and deserialize results ...
            return Ok(results);
        }

        // Fallback to in-memory history
        // ... existing code ...
    }
}
```

#### Крок 4: Додати retention policy та cleanup (0.5 дня)

```rust
// Cleanup old metrics (run periodically, e.g., daily)
pub async fn cleanup_old_metrics(&self, retention_days: u32) -> Result<usize, AppError> {
    if let Some(ref conn) = self.db_conn {
        let cutoff = Utc::now() - chrono::Duration::days(retention_days as i64);
        let conn = conn.lock().await;
        let deleted = conn.execute(
            "DELETE FROM metrics_history WHERE timestamp < ?1",
            params![cutoff.to_rfc3339()]
        )?;
        info!("Cleaned up {} old metrics (retention: {} days)", deleted, retention_days);
        Ok(deleted)
    } else {
        Ok(0)
    }
}
```

---

### Варіант 2: PostgreSQL (Для production) ⭐⭐

**Переваги**:
- Краще для великих обсягів даних (>1M records)
- Підтримує advanced queries
- Можна використовувати TimescaleDB для time-series оптимізацій

**Недоліки**:
- Потребує окремого сервера
- Складніше deployment
- Більше overhead

**Оцінка**: 2-3 дні

**Dependency**: `sqlx` з PostgreSQL driver

---

### Варіант 3: File-based JSON/TOML (Простий) ⭐

**Переваги**:
- Не потребує dependencies
- Простий в реалізації
- Легко читати та debug

**Недоліки**:
- Неефективно для запитів (повний scan файлу)
- Не підходить для production з великим обсягом даних

**Оцінка**: 0.5 дня

---

## 📊 Рекомендація

**Варіант 1: SQLite** - оптимальний для початку:
- ✅ Простий deployment
- ✅ Ефективні SQL запити
- ✅ Готово для майбутнього переходу на PostgreSQL
- ✅ Не потребує зовнішніх сервісів

---

## 📝 Database Schema

### metrics_history table

```sql
CREATE TABLE metrics_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,        -- ISO 8601 format (RFC3339)
    metric TEXT NOT NULL,           -- Metric name (e.g., "cpu_usage")
    value REAL NOT NULL,            -- Metric value
    tags TEXT,                      -- JSON string with key-value pairs
    tenant_id TEXT,                 -- UUID string (nullable)
    created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for efficient queries
CREATE INDEX idx_metrics_timestamp ON metrics_history(timestamp);
CREATE INDEX idx_metrics_metric ON metrics_history(metric);
CREATE INDEX idx_metrics_tenant ON metrics_history(tenant_id);
CREATE INDEX idx_metrics_metric_timestamp ON metrics_history(metric, timestamp);
```

### retention_policy table (optional, for future)

```sql
CREATE TABLE retention_policies (
    metric TEXT PRIMARY KEY,
    retention_days INTEGER NOT NULL,
    enabled BOOLEAN DEFAULT 1
);
```

---

## 🧪 Testing Plan

### Unit Tests
- [ ] Test database initialization
- [ ] Test `record_metric()` with SQLite
- [ ] Test `get_metric_history()` queries (metric filter, time range, tenant filter, limit)
- [ ] Test retention policy cleanup

### Integration Tests
- [ ] Test metrics persistence across restarts
- [ ] Test concurrent metric recording
- [ ] Test query performance with large dataset (10K+ records)

---

## 🔄 Migration Path

### Phase 1: Hybrid Approach (Поточний стан)
- In-memory cache (1000 points) для швидкого доступу
- SQLite для long-term storage
- `get_metric_history()` читає з обох джерел

### Phase 2: Full SQLite (Майбутнє)
- Прибрати in-memory cache (або залишити тільки для alert checking)
- Всі queries з SQLite
- Додати batch inserts для performance

### Phase 3: PostgreSQL (Опціонально, для enterprise)
- Migrate schema to PostgreSQL
- Використати TimescaleDB для time-series оптимізацій
- Додати replication для high availability

---

## 📈 Performance Considerations

### Оптимізації:
1. **Batch inserts** - зберігати metrics пакетами (100-1000 за раз)
2. **Connection pooling** - використовувати pool для SQLite (якщо через sqlx)
3. **Indexes** - додати indexes для часто використовуваних queries
4. **Partitioning** - розділити таблицю за timestamp (місяць/рік) для старих даних
5. **Vacuum** - періодичний VACUUM для SQLite для оптимізації

### Retention Policy:
- Default: 30 days
- Configurable per metric type
- Automatic cleanup task (daily)

---

## ✅ Критерії успіху

1. ✅ Metrics зберігаються в SQLite після restart
2. ✅ `get_metric_history()` працює з SQLite queries
3. ✅ Retention policy автоматично видаляє старі metrics
4. ✅ Performance: query < 100ms для 10K records
5. ✅ Integration tests passing

---

**Підготовлено**: Rust Architect  
**Дата**: 2026-01-17  
**Версія**: 1.0 - Monitoring Persistence Plan
