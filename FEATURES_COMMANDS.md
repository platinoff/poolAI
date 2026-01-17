# Команди з Features для poolAI

## Доступні Features

- `jwt` - JWT аутентифікація (потребує gcc/dlltool на Windows GNU)
- `https` - HTTPS/TLS підтримка (потребує gcc/dlltool на Windows GNU)
- `raft` - Raft консенсус для distributed RAID
- `enterprise` - Enterprise функції (audit, monitoring, multi-tenancy)
- `cloud` - Cloud інтеграція (Kubernetes, AWS, Azure, GCP)
- `cloud-sdk` - Cloud SDK залежності (k8s-openapi, azure SDK)
- `vm-isolation-linux` - VM ізоляція для Linux (потребує root/CAP_*)
- `vm-isolation` - Повна VM ізоляція (platform-specific)

## Команди збірки

### Базові команди
```bash
# Збірка без features
cargo build

# Збірка з одним feature
cargo build --features jwt
cargo build --features https
cargo build --features enterprise
cargo build --features cloud
```

### Комбіновані features
```bash
# JWT + HTTPS (security)
cargo build --features jwt,https

# Enterprise з security
cargo build --features jwt,https,enterprise

# Cloud з SDK
cargo build --features cloud,cloud-sdk

# Всі основні features
cargo build --features jwt,https,enterprise,cloud
```

### Release збірка
```bash
# Production build з security
cargo build --release --features jwt,https

# Enterprise deployment
cargo build --release --features jwt,https,enterprise

# Cloud deployment
cargo build --release --features cloud,cloud-sdk
```

## Команди тестування

```bash
# Тести без features
cargo test

# Тести з JWT
cargo test --features jwt

# Тести з Enterprise
cargo test --features enterprise

# Тести з Cloud
cargo test --features cloud

# Тести з усіма features
cargo test --features jwt,https,enterprise,cloud
```

## Команди запуску

```bash
# Запуск без features
cargo run

# Запуск з JWT
cargo run --features jwt

# Запуск з HTTPS
cargo run --features https

# Запуск з усіма features
cargo run --features jwt,https,enterprise,cloud
```

## Рекомендовані конфігурації

### Для розробки
```bash
cargo build
cargo test
cargo run
```

### Для production (з security)
```bash
cargo build --release --features jwt,https
cargo test --features jwt,https
cargo run --features jwt,https
```

### Для enterprise deployment
```bash
cargo build --release --features jwt,https,enterprise
```

### Для cloud deployment
```bash
cargo build --release --features cloud,cloud-sdk
```

## Windows Setup (для features jwt, https)

Якщо ви отримуєте помилку `Error calling dlltool 'dlltool.exe': program not found` на Windows, виконайте:

### PowerShell (рекомендовано)
```powershell
# Додати MSYS2 до PATH для поточної сесії
.\scripts\setup_msys2_path.ps1

# Після цього можна компілювати
cargo build --features jwt,https
```

### Або вручну додати до PATH
```powershell
# Додати C:\msys64\usr\bin до системного PATH
$env:PATH += ";C:\msys64\usr\bin"
```

### Перевірка
```powershell
# Перевірити, що dlltool доступний
dlltool --version
```

**Примітка:** Скрипт `setup_msys2_path.ps1` додає MSYS2 до PATH лише для поточної PowerShell сесії. Для постійного вирішення додайте `C:\msys64\usr\bin` до системного PATH змінної середовища.

## Примітки

- `jwt` та `https` потребують native toolchain (gcc/dlltool) на Windows GNU
  - Використайте `scripts/setup_msys2_path.ps1` для налаштування PATH на Windows
  - Або додайте `C:\msys64\usr\bin` до системного PATH вручну
- `vm-isolation-linux` потребує root або CAP_NET_ADMIN, CAP_SYS_ADMIN
- `cloud-sdk` додає важкі SDK залежності
- AWS SDK потребує Rust 1.88+ (закоментовано в Cargo.toml)
