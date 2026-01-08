# 🔐 Як потрапити в Admin Panel

## 📍 URL для доступу

**Головна адреса адмін панелі:**
```
http://localhost:8080/ui/admin
```

**Або якщо використовується HTTPS:**
```
https://localhost:8443/ui/admin
```

## 🔑 Авторизація

### Тестові облікові дані

Для доступу до адмін панелі потрібна роль **Admin**:

- **Username:** `admin`
- **Password:** `admin123`

### Інші тестові облікові дані:

- **Operator:** `operator` / `op123` (обмежений доступ)
- **Viewer:** `viewer` / `view123` (тільки перегляд)

## 🚀 Запуск з enterprise feature

Адмін панель доступна тільки з увімкненим feature `enterprise`:

```bash
# Запуск з enterprise feature
cargo run --features enterprise

# Або збірка з enterprise
cargo build --features enterprise
```

## 📋 Кроки для доступу

1. **Запустіть сервер з enterprise feature:**
   ```bash
   cd S:\rust\poolAI
   cargo run --features enterprise
   ```

2. **Відкрийте браузер і перейдіть на:**
   ```
   http://localhost:8080/ui/auth
   ```

3. **Увійдіть з обліковими даними Admin:**
   - Username: `admin`
   - Password: `admin123`

4. **Після успішної авторизації перейдіть на:**
   ```
   http://localhost:8080/ui/admin
   ```

## 🎯 Доступні розділи адмін панелі

- **Dashboard** - `/ui/admin` - Загальний огляд системи
- **Tenants** - `/ui/admin/tenants` - Управління tenants
- **Security** - `/ui/admin/security` - OAuth2, SAML, політики безпеки
- **Audit Logs** - `/ui/admin/audit` - Перегляд audit подій
- **Monitoring** - `/ui/admin/monitoring` - Dashboards та alerts
- **VM Instances** - `/ui/admin/vm` - Управління VM
- **Workers** - `/ui/admin/workers` - Управління workers
- **Libraries** - `/ui/admin/libs` - Управління libraries
- **RAID** - `/ui/admin/raid` - Управління artifacts
- **Users** - `/ui/admin/users` - Управління користувачами
- **Configuration** - `/ui/admin/config` - Системні налаштування

## ⚠️ Важливо

- Адмін панель доступна **тільки для користувачів з роллю Admin**
- Якщо ви не авторизовані або маєте іншу роль, вас перенаправить на `/ui/auth`
- Для production використання змініть тестові облікові дані!

## 🔧 Налаштування порту

За замовчуванням сервер запускається на `127.0.0.1:8080`. 

Щоб змінити порт, встановіть змінну середовища:
```bash
# Windows PowerShell
$env:PORT="3000"
cargo run --features enterprise

# Linux/Mac
PORT=3000 cargo run --features enterprise
```

Або змініть адресу в `main.rs`.
