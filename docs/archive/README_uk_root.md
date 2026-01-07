# PoolAI

AI Mining Pool Management System

## Можливості
- Модульна архітектура Rust: core, pool, monitoring, network, platform, tgbot
- Конфігурується через `config.toml` ([system], [gpu], [pool], [monitoring], [version], [health])
- Автоматичний час збірки через build.rs
- HTTPS сервер із self-signed сертифікатами (dev)
- JWT-аутентифікація та RBAC (планується)
- Live metrics через WebSocket (планується)
- CI/CD через GitHub Actions (планується)
- Swagger/OpenAPI (планується)

## Швидкий старт
1. Згенеруйте self-signed сертифікати: див. docs/ або використайте `openssl`
2. Відредагуйте `config.toml` (див. приклад у репозиторії)
3. Запустіть з HTTPS:
   ```sh
   cargo run --features "stage2 https"
   ```

## Безпека
- Ніколи не комітьте приватні ключі у git!
- Використовуйте змінні середовища для секретів у проді
- Для dev: використовуйте self-signed сертифікати

## API
- `/api/v1/status` — статус (HTML/JSON)
- `/api/v1/metrics` — метрики
- `/api/v1/models` — моделі
- `/api/v1/gpu` — GPU info
- `/api/v1/workers` — воркери
- `/ws/metrics` — live metrics (WebSocket, планується)

## Дорожня карта
- Healthcheck endpoint
- JWT/RBAC
- Swagger/OpenAPI
- CI/CD workflow
- UI/UX покращення

## Документація
- [README.uk.md](./README.uk.md) — українською (основна версія)
- [poolAI_concept.txt](./poolAI_concept.txt) — концепт

---

© 2025 PoolAI Team 