# PoolAI Enterprise Features Quick Start

## Запуск з Enterprise Features

### Базовий запуск з Enterprise features:

```bash
cargo run --features enterprise
```

### З JWT та HTTPS:

```bash
cargo run --features enterprise,jwt,https
```

### Збірка release версії:

```bash
cargo build --release --features enterprise
```

### Запуск release версії:

```bash
cargo run --release --features enterprise
```

## Enterprise API Endpoints

Після запуску з feature `enterprise`, будуть доступні наступні endpoints:

### Tenant Management
- `GET /api/enterprise/tenants` - Список всіх tenants
- `POST /api/enterprise/tenants` - Створити нового tenant
- `GET /api/enterprise/tenants/{id}` - Отримати tenant за ID
- `POST /api/enterprise/tenants/{id}` - Оновити tenant
- `DELETE /api/enterprise/tenants/{id}` - Видалити tenant
- `GET /api/enterprise/tenants/{id}/usage` - Отримати використання ресурсів
- `POST /api/enterprise/tenants/{id}/quota` - Перевірити квоту

### Security Management
- `GET /api/enterprise/security/oauth2/providers` - Список OAuth2 провайдерів
- `POST /api/enterprise/security/oauth2/providers` - Зареєструвати OAuth2 провайдер
- `GET /api/enterprise/security/saml/providers` - Список SAML провайдерів
- `POST /api/enterprise/security/saml/providers` - Зареєструвати SAML провайдер
- `GET /api/enterprise/security/policies` - Список security policies
- `POST /api/enterprise/security/policies` - Створити security policy

### Audit Logs
- `GET /api/enterprise/audit/events` - Запит audit events

### Monitoring
- `GET /api/enterprise/monitoring/alerts` - Список alerts
- `POST /api/enterprise/monitoring/alerts/{id}/acknowledge` - Підтвердити alert
- `GET /api/enterprise/monitoring/dashboards` - Список dashboards
- `POST /api/enterprise/monitoring/dashboards` - Створити dashboard
- `GET /api/enterprise/monitoring/metrics` - Отримати metrics

## Приклади використання

### 1. Створити Tenant

```bash
curl -X POST http://localhost:8080/api/enterprise/tenants \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "tenant-1",
    "config": {
      "max_workers": 10,
      "max_memory_mb": 10240,
      "max_cpu_cores": 8,
      "active": true
    }
  }'
```

### 2. Отримати список OAuth2 провайдерів

```bash
curl http://localhost:8080/api/enterprise/security/oauth2/providers \
  -H "Authorization: Bearer YOUR_TOKEN"
```

### 3. Зареєструвати OAuth2 провайдер

```bash
curl -X POST http://localhost:8080/api/enterprise/security/oauth2/providers \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "google",
    "config": {
      "client_id": "your-client-id",
      "client_secret": "your-client-secret",
      "authorization_url": "https://accounts.google.com/o/oauth2/auth",
      "token_url": "https://oauth2.googleapis.com/token",
      "redirect_uri": "https://poolai.example.com/callback",
      "scopes": ["openid", "profile", "email"]
    },
    "enabled": true
  }'
```

### 4. Перевірити квоту для Tenant

```bash
curl -X POST http://localhost:8080/api/enterprise/tenants/{tenant-id}/quota \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "workers": 5,
    "memory_mb": 5120,
    "cpu_cores": 4,
    "storage_mb": 10000,
    "vm_instances": 2
  }'
```

## Конфігурація

Enterprise features не потребують додаткової конфігурації в `config.toml`. Всі managers ініціалізуються автоматично при старті сервера.

## Тестування

### Запуск тестів з Enterprise features:

```bash
cargo test --features enterprise
```

### Запуск інтеграційних тестів:

```bash
cargo test --features enterprise --test integration_test_name
```

## Troubleshooting

**Проблема**: Enterprise endpoints не доступні
**Рішення**: Переконайтеся, що feature `enterprise` увімкнено при запуску:
```bash
cargo run --features enterprise
```

**Проблема**: `401 Unauthorized` на enterprise endpoints
**Рішення**: Більшість enterprise endpoints потребують автентифікації. Отримайте token через `/api/v1/login` та використовуйте його в заголовку `Authorization: Bearer TOKEN`.

**Проблема**: `403 Forbidden` на enterprise endpoints
**Рішення**: Переконайтеся, що ваш користувач має права `admin:all`. За замовчуванням користувач `admin` має всі права.
