# 🚀 Як запустити PoolAI

## Швидкий старт

### 1. Базовий запуск (найпростіший)

```bash
cargo run
```

**Що це робить:**
- Запускає PoolAI на `http://localhost:8080`
- Стандартний UI доступний на `http://localhost:8080/ui`
- Без enterprise features

---

### 2. Запуск з Admin Panel (рекомендовано)

```bash
cargo run --features enterprise
```

**Що це робить:**
- Запускає PoolAI з Admin Panel
- UI: `http://localhost:8080/ui`
- Admin Panel: `http://localhost:8080/ui/admin`
- Enterprise features: monitoring, audit, multi-tenancy

---

### 3. Запуск з HTTPS та JWT (безпечний)

```bash
cargo run --features enterprise,https,jwt
```

**Що це робить:**
- HTTPS на `https://localhost:8443`
- JWT authentication
- Admin Panel: `https://localhost:8443/ui/admin`
- Self-signed certificate (для development)

---

## Windows (PowerShell)

### Базовий запуск
```powershell
cargo run
```

### З Enterprise features
```powershell
cargo run --features enterprise
```

### З HTTPS та JWT
```powershell
cargo run --features enterprise,https,jwt
```

### З детальним логуванням
```powershell
$env:RUST_LOG="debug"
cargo run --features enterprise
```

---

## Windows (CMD)

### Базовий запуск
```cmd
cargo run
```

### З Enterprise features
```cmd
cargo run --features enterprise
```

### З логуванням
```cmd
set RUST_LOG=debug
cargo run --features enterprise
```

---

## Linux/macOS (Bash)

### Базовий запуск
```bash
cargo run
```

### З Enterprise features
```bash
cargo run --features enterprise
```

### З HTTPS та JWT
```bash
cargo run --features enterprise,https,jwt
```

### З детальним логуванням
```bash
RUST_LOG=debug cargo run --features enterprise
```

---

## Використання скриптів

### Windows PowerShell скрипт
```powershell
# Запуск з enterprise features
.\scripts\run.ps1 -Enterprise

# Запуск з debug режимом
.\scripts\run.ps1 -Enterprise -Debug

# Запуск з кастомними features
.\scripts\run.ps1 -Features "enterprise,https,jwt"
```

### Bash скрипт (Linux/macOS)
```bash
# Запуск з enterprise features
bash bin/cargo-run.sh --features enterprise
```

---

## Після запуску

### 1. Відкрити UI
- **Стандартний UI**: http://localhost:8080/ui
- **Admin Panel**: http://localhost:8080/ui/admin (якщо `--features enterprise`)

### 2. Створити Admin користувача (через API)

```bash
# Створити admin користувача
curl -X POST http://localhost:8080/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "admin123", "role": "Admin"}'

# Логін для отримання JWT token
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "admin123"}'
```

### 3. Доступ до Admin Panel
- Відкрити `http://localhost:8080/ui/admin`
- Використати створені credentials для входу

---

## Production запуск

### 1. Збірка release версії
```bash
cargo build --release --features enterprise,https,jwt
```

### 2. Запуск release бінарника

**Linux/macOS:**
```bash
./target/release/poolai
```

**Windows:**
```cmd
.\target\release\poolai.exe
```

### 3. З environment variables
```bash
RUST_LOG=info \
POOLAI_CONFIG_PATH=./config.toml \
./target/release/poolai
```

---

## Docker запуск

### З Docker Compose
```bash
docker-compose -f docker/docker-compose.yml up -d
```

### З Docker Run
```bash
docker run -d \
  --name poolai \
  -p 8080:8080 \
  -p 8443:8443 \
  poolai:latest
```

---

## Troubleshooting

### Помилка: "cargo: command not found"
**Рішення:** Встановіть Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Помилка: "dlltool.exe: program not found" (Windows)
**Рішення:** Додайте MSYS2 до PATH:
```powershell
.\scripts\setup_msys2_path.ps1
```

### Порт 8080 зайнятий
**Рішення:** Змініть порт в `config.toml` або зупиніть інший процес:
```bash
# Linux/macOS
lsof -ti:8080 | xargs kill

# Windows
netstat -ano | findstr :8080
taskkill /PID <PID> /F
```

### Не відкривається UI
**Рішення:** Перевірте:
1. Чи запущено сервер (перевірте логи)
2. Чи правильний URL (http://localhost:8080/ui)
3. Чи немає firewall блокування

---

## Корисні команди

### Перевірка статусу
```bash
curl http://localhost:8080/api/v1/status
```

### Health check
```bash
curl http://localhost:8080/api/v1/health
```

### Перегляд логів
```bash
# Якщо запущено через cargo run
# Логи виводяться в консоль

# Якщо запущено через systemd
journalctl -u poolai -f
```

---

## Додаткові ресурси

- [RUN_PARAMETERS.md](RUN_PARAMETERS.md) - Детальна інформація про параметри
- [QUICK_START.md](QUICK_START.md) - Швидкий старт гайд
- [FEATURES_COMMANDS.md](FEATURES_COMMANDS.md) - Всі доступні features

---

**Останнє оновлення**: 2026-01-21
