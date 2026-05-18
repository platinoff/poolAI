# 🚀 Параметри запуску PoolAI

## Огляд

PoolAI підтримує кілька способів конфігурації та запуску:
- **Environment Variables** (змінні середовища)
- **Configuration File** (файл конфігурації `config.toml`)
- **Cargo Features** (features для збірки)
- **Runtime Configuration** (налаштування Tokio runtime)

---

## 📋 Environment Variables (Змінні середовища)

### Основні параметри

#### `POOLAI_CONFIG_PATH`
Шлях до файлу конфігурації (за замовчуванням: `./config.toml`)

```bash
# Linux/macOS
POOLAI_CONFIG_PATH=./custom_config.toml cargo run

# Windows (CMD)
set POOLAI_CONFIG_PATH=.\custom_config.toml && cargo run

# Windows (PowerShell)
$env:POOLAI_CONFIG_PATH=".\custom_config.toml"; cargo run
```

#### `POOLAI_DATA_PATH`
Шлях до директорії для зберігання даних (за замовчуванням: `./data`)

```bash
POOLAI_DATA_PATH=/var/lib/poolai cargo run
```

#### `POOLAI_HTTP_PORT`
HTTP-порт API-сервера (за замовчуванням: `8080`). Потрібен для **кількох вузлів на одній машині** (LAN dev stand, FM-003).

```powershell
# Windows — node B on 8081
$env:POOLAI_HTTP_PORT="8081"
$env:POOLAI_RAID_BASE_PATH="S:\rust\poolAI\data\lan-stand\node-B\raid"
cargo run --features enterprise,ml,cloud,test-utils
```

```bash
# MSYS2 — node A on 8080
POOLAI_HTTP_PORT=8080 POOLAI_RAID_BASE_PATH=./data/lan-stand/node-A/raid cargo run --features enterprise,ml,cloud,test-utils
```

Або скрипт: **`bin/run-lan-nodes.ps1`** (PowerShell) / **`bin/run-lan-nodes.sh`** (bash).

#### `POOLAI_RAID_BASE_PATH`
Базова директорія RAID-артефактів (за замовчуванням: `C:\poolai\raid` на Windows, `/var/lib/poolai/raid` на Linux, `./data/raid` інакше). **Окремий шлях на кожен вузол** при multi-node на одному хості.

```bash
export POOLAI_RAID_BASE_PATH=/var/lib/poolai/raid-node-b
```

Див. [`performance/LAN_BENCHMARK_RUNBOOK.md`](performance/LAN_BENCHMARK_RUNBOOK.md).

#### `RUST_LOG`
Рівень логування (за замовчуванням: `info`)

**Доступні рівні:**
- `error` - тільки помилки
- `warn` - попередження та помилки
- `info` - інформаційні повідомлення (за замовчуванням)
- `debug` - детальне логування
- `trace` - максимальне логування

**Приклади:**
```bash
# Детальне логування
RUST_LOG=debug cargo run

# Логування тільки для poolai модуля
RUST_LOG=poolai=debug cargo run

# Логування для конкретних модулів
RUST_LOG=poolai::network=debug,poolai::monitoring=info cargo run
```

---

## ⚙️ Tokio Runtime Configuration

### `TOKIO_WORKER_THREADS`
Кількість worker threads для Tokio runtime (за замовчуванням: кількість CPU cores)

```bash
# Встановити 8 worker threads
TOKIO_WORKER_THREADS=8 cargo run

# Windows (PowerShell)
$env:TOKIO_WORKER_THREADS="8"; cargo run
```

**Рекомендації:**
- Для production: залишити за замовчуванням (автоматичне визначення)
- Для development: можна встановити менше (наприклад, 2-4) для економії ресурсів
- Для high-load: можна встановити більше (наприклад, 16-32)

### `TOKIO_BLOCKING_THREADS`
Кількість blocking threads для I/O операцій (за замовчуванням: `2 * worker_threads`)

```bash
# Встановити 16 blocking threads
TOKIO_BLOCKING_THREADS=16 cargo run
```

**Рекомендації:**
- За замовчуванням: `2 * worker_threads` (оптимально для більшості випадків)
- Для high I/O workload: можна збільшити до `4 * worker_threads`
- Для CPU-bound workload: можна зменшити до `worker_threads`

---

## 🎯 Cargo Features (Features для збірки)

### Базовий запуск
```bash
cargo run
```
- Стандартний UI на `http://localhost:8080/ui`
- Без enterprise features
- Без HTTPS/JWT

### Enterprise Features
```bash
cargo run --features enterprise
```
- Admin Panel на `http://localhost:8080/ui/admin`
- Multi-tenancy
- Advanced monitoring
- Audit logging
- SAML SSO

### JWT Authentication
```bash
cargo run --features jwt
```
- JWT-based authentication
- Secure token management
- Role-based access control

### HTTPS/TLS
```bash
cargo run --features https
```
- HTTPS на `https://localhost:8443`
- TLS encryption
- Self-signed certificate (development)

### Комбінації Features
```bash
# Enterprise + HTTPS + JWT (рекомендовано для development)
cargo run --features enterprise,https,jwt

# Enterprise + Cloud SDK
cargo run --features enterprise,cloud,cloud-sdk

# Всі features
cargo run --features enterprise,https,jwt,cloud,cloud-sdk,raft
```

---

## 📄 Configuration File (`config.toml`)

### Розташування
- За замовчуванням: `./config.toml`
- Кастомний шлях: через `POOLAI_CONFIG_PATH`

### Структура конфігурації

```toml
[server]
host = "0.0.0.0"
port = 8080
https_port = 8443

[logging]
level = "info"

[monitoring]
enabled = true
interval_seconds = 5

[raid]
mode = "Local"
base_path = "./data/raid"
```

### Приклад використання
```bash
# Використати кастомний config файл
POOLAI_CONFIG_PATH=./production_config.toml cargo run --features enterprise
```

---

## 🐳 Docker Deployment

### Environment Variables в Docker

```yaml
# docker-compose.yml
services:
  poolai:
    environment:
      - RUST_LOG=info
      - POOLAI_CONFIG_PATH=/config/config.toml
      - POOLAI_DATA_PATH=/data
      - TOKIO_WORKER_THREADS=8
      - TOKIO_BLOCKING_THREADS=16
```

### Docker Run
```bash
docker run -d \
  --name poolai \
  -p 8080:8080 \
  -p 8443:8443 \
  -e RUST_LOG=info \
  -e POOLAI_CONFIG_PATH=/config/config.toml \
  -e TOKIO_WORKER_THREADS=8 \
  -v poolai-data:/data \
  -v poolai-config:/config \
  poolai:latest
```

---

## 📊 Production Deployment

### Рекомендовані параметри для Production

```bash
# Environment variables
export RUST_LOG=info
export POOLAI_CONFIG_PATH=/etc/poolai/config.toml
export POOLAI_DATA_PATH=/var/lib/poolai
export TOKIO_WORKER_THREADS=$(nproc)  # Автоматично за кількістю CPU
export TOKIO_BLOCKING_THREADS=$(( $(nproc) * 2 ))

# Build
cargo build --release --features enterprise,https,jwt

# Run
./target/release/poolai
```

### Systemd Service Example

```ini
[Unit]
Description=PoolAI Service
After=network.target

[Service]
Type=simple
User=poolai
WorkingDirectory=/opt/poolai
Environment="RUST_LOG=info"
Environment="POOLAI_CONFIG_PATH=/etc/poolai/config.toml"
Environment="POOLAI_DATA_PATH=/var/lib/poolai"
ExecStart=/opt/poolai/poolai
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

---

## 🔧 Development Parameters

### Швидкий запуск для розробки

```bash
# Мінімальні ресурси, детальне логування
RUST_LOG=debug \
TOKIO_WORKER_THREADS=2 \
TOKIO_BLOCKING_THREADS=4 \
cargo run --features enterprise
```

### Windows PowerShell

```powershell
$env:RUST_LOG="debug"
$env:TOKIO_WORKER_THREADS="2"
$env:TOKIO_BLOCKING_THREADS="4"
cargo run --features enterprise
```

### Windows CMD

```cmd
set RUST_LOG=debug
set TOKIO_WORKER_THREADS=2
set TOKIO_BLOCKING_THREADS=4
cargo run --features enterprise
```

---

## 📝 Приклади використання

### Приклад 1: Локальна розробка
```bash
RUST_LOG=debug cargo run --features enterprise
```
- Детальне логування
- Enterprise features
- HTTP на порту 8080

### Приклад 2: Тестування з HTTPS
```bash
RUST_LOG=info cargo run --features enterprise,https,jwt
```
- HTTPS на порту 8443
- JWT authentication
- Enterprise features

### Приклад 3: Production-like
```bash
RUST_LOG=info \
POOLAI_CONFIG_PATH=./production_config.toml \
TOKIO_WORKER_THREADS=8 \
cargo run --release --features enterprise,https,jwt
```
- Release build
- Production config
- Оптимізовані параметри runtime

### Приклад 4: High Performance
```bash
RUST_LOG=warn \
TOKIO_WORKER_THREADS=16 \
TOKIO_BLOCKING_THREADS=32 \
cargo run --release --features enterprise,https,jwt
```
- Мінімальне логування
- Максимальна продуктивність
- Багато потоків для high-load

---

## 🔍 Діагностика

### Перевірка поточних параметрів

При запуску PoolAI виводить інформацію про конфігурацію:

```
🚀 Starting PoolAI v0.2.1
📅 Build time: 2026-01-21 12:00:00
⚙️  Tokio runtime: 8 worker threads, 16 blocking threads
```

### Логування конфігурації

Для перевірки всіх параметрів:
```bash
RUST_LOG=debug cargo run --features enterprise 2>&1 | grep -i "config\|runtime\|thread"
```

---

## 📚 Додаткові ресурси

- [QUICK_START.md](QUICK_START.md) - Швидкий старт
- [FEATURES_COMMANDS.md](FEATURES_COMMANDS.md) - Детальна інформація про features
- [configuration/PRODUCTION.md](configuration/PRODUCTION.md) - Production конфігурація
- [deployment/DOCKER.md](deployment/DOCKER.md) - Docker deployment

---

**Останнє оновлення**: 2026-01-21  
**Версія**: v0.2.1
