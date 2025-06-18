# PoolAI - AI Mining Pool Management System

**Version: Beta_bolvanka_v1**  
**Build Date: 2024-12-19**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/Version-Beta_bolvanka_v1-blue.svg)](https://github.com/platinoff/poolAI)

PoolAI - это инновационная система управления майнинг пулами с интеграцией генеративных AI моделей. Система оптимизирует использование GPU, ASIC и CPU ресурсов для максимальной эффективности майнинга и AI вычислений.

## 🚀 Основные возможности

### 🤖 AI Интеграция
- **Генеративные модели**: Поддержка GPT, BERT, T5 и других моделей
- **Автоматическая оптимизация**: AI-управляемая настройка параметров майнинга
- **Модель-ассистированный майнинг**: Использование AI для улучшения алгоритмов

### ⚡ Оптимизация ресурсов
- **GPU Optimization**: Автоматическая настройка CUDA/OpenCL параметров
- **ASIC Management**: Управление ASIC устройствами и их настройками
- **CPU Tuning**: Оптимизация CPU для майнинга и AI задач
- **Memory Management**: Умное управление памятью между майнингом и AI

### 📱 Управление
- **Telegram Bot**: Полное управление системой через Telegram
- **Web Dashboard**: Современный веб-интерфейс для мониторинга
- **REST API**: Программный доступ к функциям системы
- **Admin Panel**: Панель администратора с расширенными возможностями

### 🛡️ Надежность
- **RAID System**: Отказоустойчивость и репликация данных
- **Monitoring**: Комплексный мониторинг системы
- **Alerting**: Система оповещений о проблемах
- **Auto-recovery**: Автоматическое восстановление после сбоев

### 🎯 Система наград
- **Fair Distribution**: Справедливое распределение наград
- **Activity Tracking**: Отслеживание активности воркеров
- **Performance Metrics**: Метрики производительности
- **Reward Optimization**: Оптимизация наград на основе AI

## 🏗️ Архитектура

```
PoolAI/
├── src/
│   ├── core/           # Базовые интерфейсы и трейты
│   ├── libs/           # Управление моделями и GPU
│   ├── pool/           # Управление пулом и воркерами
│   ├── monitoring/     # Мониторинг и метрики
│   ├── runtime/        # Управление экземплярами
│   ├── network/        # Сетевая инфраструктура
│   ├── platform/       # Платформенная абстракция
│   ├── vm/            # Виртуализация и passthrough
│   ├── tgbot/         # Telegram бот
│   ├── raid/          # RAID система
│   ├── ui/            # Веб-интерфейс
│   ├── admin/         # Панель администратора
│   └── workers/       # Управление воркерами
├── cursor-core/       # Основная библиотека
├── src/tgbot/         # Telegram бот модуль
└── src/raid/          # RAID модуль
```

## 📦 Установка

### Требования

- **ОС**: Windows 10+, Linux (Ubuntu 20.04+), macOS 10.15+
- **Rust**: 1.70+ ([установка](https://rustup.rs/))
- **RAM**: 8GB+ (рекомендуется 16GB+)
- **GPU**: NVIDIA RTX 4090+ или эквивалент
- **ASIC**: Поддержка основных ASIC устройств
- **Сеть**: Стабильное интернет-соединение

### Быстрая установка

```bash
# Клонирование репозитория
git clone https://github.com/platinoff/poolAI.git
cd poolAI

# Сборка проекта
cargo build --release

# Запуск
cargo run --release
```

### Docker установка

```bash
# Сборка образа
docker build -t poolai:beta-bolvanka-v1 .

# Запуск контейнера
docker run -d \
  --name poolai \
  --gpus all \
  -p 8080:8080 \
  -v /path/to/config:/app/config \
  poolai:beta-bolvanka-v1
```

## ⚙️ Конфигурация

Создайте файл `config.toml` в корне проекта:

```toml
[system]
version = "Beta_bolvanka_v1"
debug = false
log_level = "info"

[gpu]
enabled = true
optimization = true
memory_limit = 16384  # MB
cuda_version = "12.0"

[models]
default_model = "gpt-3.5-turbo"
max_models = 10
auto_load = true
model_path = "./models"

[pool]
name = "PoolAI Beta"
description = "AI-powered mining pool"
max_workers = 1000
reward_algorithm = "ai_optimized"

[telegram]
enabled = true
token = "YOUR_BOT_TOKEN"
admin_users = ["your_telegram_id"]

[web]
enabled = true
host = "127.0.0.1"
port = 8080
tls_enabled = false

[monitoring]
enabled = true
metrics_port = 9090
alert_email = "admin@poolai.com"

[raid]
enabled = true
replication_factor = 3
auto_repair = true
```

## 🚀 Использование

### Запуск системы

```bash
# Запуск с конфигурацией
cargo run --release -- --config config.toml

# Запуск в фоновом режиме
nohup cargo run --release > poolai.log 2>&1 &

# Запуск с отладкой
RUST_LOG=debug cargo run --release
```

### Telegram бот

После настройки токена в конфигурации:

```
/start - Запуск бота
/status - Статус системы
/pool_stats - Статистика пула
/worker_stats - Статистика воркеров
/gpu_status - Статус GPU
/ai_models - Список AI моделей
/restart - Перезапуск системы
```

### Веб-интерфейс

Откройте браузер и перейдите на `http://localhost:8080`

- **Dashboard**: Общий обзор системы
- **Pool Management**: Управление пулом
- **Worker Management**: Управление воркерами
- **AI Models**: Управление AI моделями
- **Monitoring**: Мониторинг и метрики
- **Admin Panel**: Панель администратора

### API Endpoints

```bash
# Статус системы
curl http://localhost:8080/api/v1/status

# Статистика пула
curl http://localhost:8080/api/v1/pool/stats

# Статистика воркеров
curl http://localhost:8080/api/v1/workers/stats

# Добавление воркера
curl -X POST http://localhost:8080/api/v1/workers/add \
  -H "Content-Type: application/json" \
  -d '{"name": "worker1", "gpu": "RTX4090"}'
```

## 🔧 Разработка

### Структура проекта

```bash
# Основные модули
src/core/          # Базовые интерфейсы
src/libs/          # AI модели и GPU
src/pool/          # Управление пулом
src/monitoring/    # Мониторинг
src/runtime/       # Runtime системы
src/network/       # Сеть и API
src/platform/      # Платформы
src/vm/           # Виртуализация
src/tgbot/        # Telegram бот
src/raid/         # RAID система
src/ui/           # Веб-интерфейс
src/admin/        # Админ панель
src/workers/      # Воркеры

# Зависимости
cursor-core/       # Основная библиотека
src/tgbot/        # Telegram модуль
src/raid/         # RAID модуль
```

### Сборка для разработки

```bash
# Установка зависимостей
cargo build

# Запуск тестов
cargo test

# Проверка кода
cargo clippy
cargo fmt

# Документация
cargo doc --open
```

### Добавление новых функций

1. Создайте новый модуль в соответствующей директории
2. Добавьте модуль в `lib.rs`
3. Реализуйте необходимые трейты
4. Добавьте тесты
5. Обновите документацию

## 📊 Мониторинг

### Метрики

Система предоставляет метрики в формате Prometheus:

- **pool_workers_total**: Общее количество воркеров
- **pool_hashrate**: Общий хешрейт пула
- **gpu_utilization**: Утилизация GPU
- **ai_model_requests**: Запросы к AI моделям
- **system_memory_usage**: Использование памяти
- **network_connections**: Сетевые соединения

### Алерты

Настраиваемые алерты для:

- Низкий хешрейт
- Высокая температура GPU
- Проблемы с AI моделями
- Сетевые проблемы
- Недостаток памяти

## 🤝 Вклад в проект

Мы приветствуем вклад в развитие PoolAI!

1. **Fork** репозитория
2. Создайте **feature branch** (`git checkout -b feature/amazing-feature`)
3. **Commit** изменения (`git commit -m 'Add amazing feature'`)
4. **Push** в branch (`git push origin feature/amazing-feature`)
5. Откройте **Pull Request**

### Руководство по стилю

- Следуйте [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- Используйте `cargo fmt` для форматирования
- Добавляйте тесты для новых функций
- Обновляйте документацию

## 📄 Лицензия

Этот проект лицензирован под MIT License - см. файл [LICENSE](LICENSE) для деталей.

## 🆘 Поддержка

- **GitHub Issues**: [Создать issue](https://github.com/platinoff/poolAI/issues)
- **Discussions**: [Обсуждения](https://github.com/platinoff/poolAI/discussions)
- **Email**: platinovubuntu@gmail.com
- **Telegram**: @platinov

## 🙏 Благодарности

- **Rust Community** за отличный язык программирования
- **Tokio Team** за асинхронную runtime
- **Actix Team** за веб-фреймворк
- **PyTorch Team** за ML библиотеки
- **Telegram Team** за Bot API

## 📈 Roadmap

### Beta_bolvanka_v2 (Планируется)
- Улучшенная GPU оптимизация
- Поддержка новых AI моделей
- Расширенная аналитика
- Cloud интеграция

### Release_stable_v1 (Планируется)
- Production-ready версия
- Enterprise функции
- Расширенная документация
- Коммерческая поддержка

---

**PoolAI Beta_bolvanka_v1** - Инновационное решение для AI-powered майнинга! 🚀 