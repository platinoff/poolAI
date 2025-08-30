# PoolAI - Система управління AI майнінг пулами

> 🇺🇸 English version available: [README.md](./README.md)

PoolAI - це комплексна розподілена система для управління AI майнінг пулами з інтеграцією генеративних моделей, оптимізацією GPU та автоматизованим управлінням ресурсами.

## 🎉 **STAGE 3 ЗАВЕРШЕНО!** 🚀

**Поточний статус**: Stage 3 повністю реалізовано з передовими функціями  
**Наступна ціль**: Stage 4 - Enterprise функції та Cloud інтеграція

---

## ⚡️ План архітектурних покращень (2025)

1. **Healthcheck endpoint** — /api/v1/health для CI/CD та моніторингу ✅ **ЗАВЕРШЕНО**
2. **Global version/uptime state** — реалізовано через модуль `version.rs` ✅ **ЗАВЕРШЕНО**
3. **Public API експортується тільки з lib.rs** — всі внутрішні компоненти приватні ✅ **ЗАВЕРШЕНО**
4. **JWT & RBAC** — middleware для перевірки токенів та ролей ✅ **ЗАВЕРШЕНО**
5. **Endpoint access restriction** — /metrics, /workers, /shutdown тільки для авторизованих ✅ **ЗАВЕРШЕНО**
6. **CI/CD** — GitHub Actions workflow для тестів та збірки 🔄 **ПЛАНУЄТЬСЯ**
7. **Swagger/OpenAPI** — генерація та публікація API специфікації 🔄 **ПЛАНУЄТЬСЯ**
8. **Документація** — Quick Start, curl приклади, секція безпеки ✅ **ЗАВЕРШЕНО**
9. **Live metrics (WebSocket)** — /ws/metrics для моніторингу в реальному часі ✅ **ЗАВЕРШЕНО**
10. **UI/UX** — Кнопки копіювання, посилання безпеки, favicon/logo ✅ **БАЗОВО ЗАВЕРШЕНО**

---

## 🎯 Статус розробки

**Поточна фаза**: Stage 3 ЗАВЕРШЕНО 🎉  
**Ціль**: Розширена AI майнінг пула з Enterprise функціями

### 🚀 Дорожня карта розробки

#### ✅ MVP (Stage 1) - ЗАВЕРШЕНО
- ✅ **Core Module** - Основні структури та трейти
- ✅ **Pool Module** - Управління пулом та воркерами  
- ✅ **Monitoring Module** - Базові метрики та моніторинг

#### ✅ Stage 2 - ЗАВЕРШЕНО
- ✅ **Network Module** - REST API та WebSocket з HTTPS/TLS підтримкою
- ✅ **Platform Module** - Управління GPU та оптимізація
- ✅ **TGBot Module** - Telegram бот для управління
- ✅ **Security Module** - JWT аутентифікація, rate limiting та управління сертифікатами

#### ✅ Stage 3 - ЗАВЕРШЕНО! 🎉
- ✅ **Runtime Module** - Управління життєвим циклом та контроль процесів
- ✅ **Libs Module** - Управління бібліотекою моделей та контроль версій
- ✅ **VM Module** - Підтримка віртуалізації та ізоляції
- ✅ **RAID Module** - Відмовостійкість та реплікація даних
- ✅ **UI Module** - Веб-інтерфейс та панель керування
- ✅ **Rewards System** - Система нагород на основі ендорфінів
- ✅ **WebSocket Security** - Оновлення в реальному часі з JWT аутентифікацією
- ✅ **Enhanced API** - Комплексні REST endpoints

#### 🔄 Stage 4 - В РОЗРОБЦІ (Q2 2025)
- **Stage 4.1: Advanced Runtime** - Управління процесами, оркестрація ресурсів
- **Stage 4.2: Enterprise Features** - Multi-tenancy, розширена безпека, аудит логування
- **Stage 4.3: Cloud Integration** - Підтримка Kubernetes, cloud провайдери, auto-scaling
- **Stage 4.4: AI/ML Enhancement** - Оптимізація моделей, інтеграція AutoML, федеративне навчання

---

## 🌟 Нові функції Stage 3

### 🎁 **Система нагород**
- **Нагороди на основі ендорфінів** за продуктивність та співпрацю
- **Система досягнень** з бейджами та рівнями
- **Відстеження прогресу** та статистика користувачів
- **Бонуси за продуктивність** та нагороди за streak

### 🔐 **Розширена безпека**
- **JWT аутентифікація** з role-based access control
- **Безпека WebSocket** з валідацією токенів
- **Підтримка HTTPS/TLS** з self-signed сертифікатами
- **Rate limiting** та захист від DDoS

### 🌐 **Комунікація в реальному часі**
- **WebSocket endpoints** для live метрик
- **Оновлення в реальному часі** статусу системи
- **Live моніторинг** з миттєвими сповіщеннями
- **Безпечні протоколи** комунікації

### 📊 **Розширений API**
- **Health check endpoints** для моніторингу
- **Комплексний збір метрик**
- **Управління користувачами** та аутентифікація
- **Моніторинг ресурсів** та оптимізація

---

## 📋 Вимоги

### Системні вимоги
- **OS**: Linux (Ubuntu 20.04+) або Windows 10+
- **CPU**: 4+ ядра рекомендується
- **RAM**: 8GB+ рекомендується
- **Storage**: 50GB+ доступного місця
- **GPU**: NVIDIA GPU з підтримкою CUDA (опціонально)

### Програмні вимоги
- **Rust**: 1.70+ (остання стабільна)
- **MSYS2** (Windows): Для нативних залежностей
- **CUDA**: 11.0+ (опціонально, для GPU підтримки)
- **OpenSSL**: 1.1.1+ (для HTTPS/TLS підтримки)
- **Certbot**: Для Let's Encrypt сертифікатів (production)

## 🛠️ Встановлення

### Швидкий старт

1. **Клонувати репозиторій**
   ```bash
   git clone https://github.com/poolai/poolai.git
   cd poolai
   ```

2. **Встановити залежності**
   ```bash
   cargo build --features "stage3 https"
   ```

3. **Згенерувати сертифікати (для HTTPS)**
   ```bash
   mkdir certs
   openssl req -x509 -newkey rsa:4096 -keyout certs/key.pem -out certs/cert.pem -days 365 -nodes
   ```

4. **Запустити з Stage 3 функціями**
   ```bash
   cargo run --features "stage3 https"
   ```

## 🚀 Використання

### Запуск системи

```bash
# Stage 3 з HTTPS
cargo run --features "stage3 https"

# З конкретною конфігурацією
POOLAI_CONFIG_PATH=./custom_config.toml cargo run --features "stage3 https"

# З логуванням
RUST_LOG=debug cargo run --features "stage3 https"
```

### Поточні функції (Stage 3)

- **Управління пулом**: Розширений пул воркерів з інтелектуальним балансуванням навантаження
- **Інтеграція моделей**: Основний інтерфейс моделей та обробка з управлінням бібліотекою
- **Розширений моніторинг**: Системні метрики, health checks та оновлення в реальному часі
- **Управління ресурсами**: Виділення GPU та пам'яті з оптимізацією
- **Безпека**: JWT аутентифікація, HTTPS/TLS, role-based access control
- **Система нагород**: Система мотивації на основі досягнень
- **WebSocket**: Комунікація в реальному часі та live метрики
- **API**: Комплексні REST endpoints з документацією

### Плановані функції (Stage 4)

- **Enterprise функції**: Multi-tenancy, розширена безпека, аудит логування
- **Cloud інтеграція**: Підтримка Kubernetes, cloud провайдери, auto-scaling
- **AI/ML покращення**: Оптимізація моделей, інтеграція AutoML, федеративне навчання
- **Розширений UI**: Сучасна панель з моніторингом в реальному часі
- **CI/CD**: Автоматизовані тести та deployment pipelines

## 🔒 Безпека та HTTPS

### Архітектура безпеки

PoolAI реалізує комплексну модель безпеки з кількома варіантами розгортання:

#### Режим розробки (HTTPS)
- HTTPS на localhost з self-signed сертифікатами
- JWT аутентифікація для API доступу
- CORS увімкнено для локальної розробки

#### Продакшн режим (HTTPS)
- TLS 1.3 шифрування для всіх комунікацій
- Автоматичне управління сертифікатами з Let's Encrypt
- HSTS заголовки для розширеної безпеки
- Rate limiting та захист від DDoS

### Функції безпеки

- **Аутентифікація**: JWT-based API аутентифікація ✅
- **Авторизація**: Role-based access control (Admin, Operator, Viewer) ✅
- **Шифрування**: TLS 1.3 для транспорту, AES-256 для даних ✅
- **Rate Limiting**: Налаштовувані ліміти запитів ✅
- **CORS**: Налаштовуване cross-origin resource sharing ✅
- **Security Headers**: HSTS, CSP, X-Frame-Options ✅
- **WebSocket Security**: WSS з JWT аутентифікацією ✅

## 🧪 Тестування

### Unit тести

```bash
cargo test
```

### Інтеграційні тести

```bash
cargo test --test integration
```

### Тести безпеки

```bash
# Запустити security audit
cargo audit

# Тестувати HTTPS endpoints
curl -k https://localhost:8080/api/v1/status

# Тестувати WebSocket безпечне з'єднання
wscat -c wss://localhost:8080/ws/metrics

# Тестувати систему нагород
curl -k https://localhost:8080/api/v1/rewards
```

## 🤝 Внесок у проект

1. Fork репозиторію
2. Створити feature branch (`git checkout -b feature/amazing-feature`)
3. Commit зміни (`git commit -m 'Add amazing feature'`)
4. Push до branch (`git push origin feature/amazing-feature`)
5. Відкрити Pull Request

### Гайдлайни розробки

- Дотримуватися Stage 4 roadmap підходу
- Фокусуватися на enterprise функціях та cloud інтеграції
- Підтримувати чистий, документований код
- Писати тести для нової функціональності

## 📄 Ліцензія

Цей проект ліцензовано під MIT License - див. файл [LICENSE](LICENSE) для деталей.

## 🆘 Підтримка

- **Issues**: [GitHub Issues](https://github.com/poolai/poolai/issues)
- **Discussions**: [GitHub Discussions](https://github.com/poolai/poolai/discussions)

## 🙏 Подяки

- Rust спільноті за відмінну екосистему
- NVIDIA за CUDA та GPU computing інструменти
- Всім контрибьюторам та користувачам PoolAI

---

**PoolAI** - Надаємо AI можливості розподіленого обчислення 🚀  
**Статус**: Stage 3 ЗАВЕРШЕНО! 🎯  
**Наступна ціль**: Stage 4 - Enterprise функції та Cloud інтеграція 🚀

