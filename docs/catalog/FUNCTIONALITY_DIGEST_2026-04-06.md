# PoolAI — витяг функціоналу (зведення за доками та кодом)

**Версія репозиторію:** 0.2.2 (`Cargo.toml`). **Оновлено:** 2026-04-06.

Цей документ — **не автогенерація з коду**, а структурований **витяг можливостей** системи, узгоджений з кореневим [`README.md`](../../README.md), [`docs/status/STABLE_STATE_SUMMARY.md`](../status/STABLE_STATE_SUMMARY.md), [`docs/development/HANDOFF_NEW_SESSION.md`](../development/HANDOFF_NEW_SESSION.md), модулями `src/` та (частково) [`docs/openapi.yaml`](../openapi.yaml). Для повного переліку HTTP-шляхів див. роутери в `src/network/` — OpenAPI може відставати від фактичного API.

---

## Канонічний порядок читання документації

Узгоджено з **кроками 1–12** у [`README.md`](../../README.md), [`docs/README.md`](../README.md), [`docs/INDEX_2026-03-17.md`](../INDEX_2026-03-17.md). **Цей файл — крок 11** (каталог / витяг функціоналу). **Крок 12** — [`FUNCTION_MANAGEMENT.md`](./FUNCTION_MANAGEMENT.md) (беклог, прогалини, тікети). **Таксономія папок `docs/` і правила агента:** [`docs/STRUCTURE.md`](../STRUCTURE.md), [`.cursor/rules/documentation.md`](../../.cursor/rules/documentation.md).

---

## Підсистеми та feature-прапорці (`Cargo.toml`)

| Feature | Призначення |
|---------|-------------|
| `jwt` | JWT для API (потрібен нативний toolchain на Windows GNU). |
| `https` | TLS (axum-server). |
| `raft` | Розподілений RAID / Raft (опційно). |
| `enterprise` | Мультитенантність, audit, monitoring, security (OAuth2/SAML), enterprise REST. |
| `ml` | ML-модулі, пайплайн, TurboQuant-гілка в квантизації тощо. |
| `cloud` | Хмарний модуль (автомасштабування, LB, K8s-обгортки) без повного SDK. |
| `cloud-sdk` | Важкі залежності (K8s OpenAPI, Azure, AWS SDK, GCP-частина). |
| `vm-isolation-linux` / `vm-isolation-windows` | Ізоляція VM на платформі. |
| `test-utils` | `AppState::attach_*_for_test` для тестів; приклад повного `/api/v1` без module globals — `tests/appstate_http_injection_integration.rs`. |

**Типова CI-матриця (див. `.github/workflows/ci.yml`):** `ml`, `enterprise`, `cloud` + `K8S_OPENAPI_ENABLED_VERSION=1.28` для збірки з cloud.

---

## Точки входу та процеси

| Компонент | Опис |
|-----------|------|
| **`poolai` (default-run)** | Основний сервер: HTTP(S), UI, REST, WebSocket, інтеграція модулів. |
| **`poolai-worker`** (`src/bin/poolai-worker.rs`) | Окремий воркер-процес для пулу (збірка тестів може блокувати `poolai-worker.exe` на Windows — завершувати процес перед лінком). |

---

## Модулі ядра (`src/` — функціональні області)

| Область | Файли / пакет | Функціонал (за README та архітектурою) |
|---------|----------------|--------------------------------------|
| **Core** | `core/` | Конфіг, `AppState` / `ApiContext` (у т.ч. `rewards_engine` → `rewards::RewardSystem`), помилки (`AppError`, `ErrorContext`), користувачі, discovery-типи, WS-менеджер, інтерфейс моделі. |
| **Pool** | `pool/` | Пул воркерів, топологія, discovery-інтеграція, розміщення. |
| **Network** | `network/` | Axum: `/api/v1/*`, RAID-маршрути, enterprise API, auth, rate limit, JSON-помилки, WebSocket, distributed RAID handlers. |
| **Platform** | `platform/` | GPU / апаратний рівень. |
| **Monitoring** | `monitoring/` | Метрики, context memory (ML-контекст). |
| **Runtime** | `runtime/` | Інстанси, планувальник, кеш, черги, процеси, сховище, оркестратор. |
| **Libs** | `libs/` | Реєстр бібліотек моделей, версіонування, залежності. |
| **VM** | `vm/` | Менеджер VM, ресурси, ізоляція (Linux/Windows за фічами). |
| **RAID** | `raid/` | Локальний і розподілений RAID, протокол, реплікація, BurstRAID, SmallWorld, події, snapshot, адмін-стратегії. |
| **Enterprise** | `enterprise/` | Тенанти, audit, monitoring, security (OAuth2, SAML, політики). |
| **Cloud** | `cloud/` | Провайдери (AWS/Azure/GCP), Kubernetes manager, operator, autoscaling, load balancing (повна поведінка з `cloud-sdk`). |
| **ML** | `ml/` | Оптимізація, AutoML, federated, pruning, pipeline, versioning, experiments, TurboQuant (`turboquant.rs`, формат TQ01). |
| **Rewards** | `rewards/` | Система нагород / прогресу; процесовий `shared_reward_engine()` (`OnceLock<Arc<RewardSystem>>`), узгоджений із `AppState`. |
| **UI** | `ui/` | Вбудована веб-адмінка (дашборди, теми, доступність). |
| **Services** | `services/` | `RaidService`, `RaidDistributedProtocolService`, `VmService`, `LibraryService`, `InstanceService`, `ChatCompletionService`, `SystemService`, `DiscoveryService`, `TopologyService`, `WorkerPoolService`, `RewardsService`, `EnterpriseService`, `CloudService`, `AdminService` — оркестрація для HTTP. |
| **TGBot** | `tgbot/` | Telegram-бот (керування). |

---

## HTTP / API (узагальнено)

- **REST під `/api/v1/`** — модульні роутери в `src/network/api/` (`system`, `workers`, `vm`, `raid`, `raid_admin`, `libraries`, `users`, `rewards`, `instances`, `completions`, `topology`, `discovery`, `ui`, `admin`, …). Див. `create_api_routes()` у `api/mod.rs`.
- **RAID** — додаткові шляхи під `/raid/…` (артефакти, воркери, події, snapshot, GC, strategies, metrics, rebalance, health) через `raid.rs`.
- **Enterprise** — при `feature enterprise`: маршрути в `enterprise_api.rs` (мультитенантність, audit, monitoring, security).
- **ML enterprise** — при `enterprise` + `ml`: `/api/enterprise/ai-ml/…` (пайплайн), див. `ai_ml.rs`.
- **WebSocket** — наприклад `/ws/metrics` (JWT/безпека залежно від конфігурації).
- **OpenAPI** — [`docs/openapi.yaml`](../openapi.yaml) описує **частину** публічних шляхів; повний перелік — з коду роутерів і `src/network/mod.rs`.

---

## ML (Stage 4.4 — за доками)

| Елемент | Стан (за README / HANDOFF) |
|---------|----------------------------|
| ML.1–ML.6 каркас у `src/ml` | Є (оптимізація, AutoML, federated, context memory, versioning, experiments, pipeline). |
| TurboQuant | Фаза 1 у коді (`turboquant.rs`, крок pipeline); див. [`docs/ml/TURBOQUANT_INTEGRATION.md`](../ml/TURBOQUANT_INTEGRATION.md). |
| Hardening | У доробці: продакшн-кроки, метрики, операційні інструкції. |

---

## Cloud (Stage 4.3 — за доками)

| Елемент | Стан (за README) |
|---------|------------------|
| Модульна структура, K8s, провайдери, autoscaling, LB | Інфраструктура та тести — є. |
| Повний SDK / operator у продакшн-глибину | Планова доробка (`cloud-sdk`). |

---

## Концепція (вісі продукту)

| Документ | Зміст |
|----------|--------|
| [`concept/poolAI_concept_root.txt`](../concept/poolAI_concept_root.txt) | Головна концепція PoolAI. |
| [`concept/POOLAI_GRID_NODE.md`](../concept/POOLAI_GRID_NODE.md) | Вузол грида, ролі, модулі. |
| [`concept/POOLAI_MEMORY_LAYER.md`](../concept/POOLAI_MEMORY_LAYER.md) | Memory layer, зв’язок RAID/ML. |
| [`development/JOB_LAYER_CONCEPT_2026-03-17.md`](../development/JOB_LAYER_CONCEPT_2026-03-17.md) | Job / mining layer, життєвий цикл job. |
| [`development/GRID_PROTOCOL_CONCEPT_2026-04-06.md`](../development/GRID_PROTOCOL_CONCEPT_2026-04-06.md) | Grid protocol: типи повідомлень, Discovery/RAID/тести. |
| [`development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md`](../development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md) | Solana adapter: core vs on-chain, події Job/Memory. |

---

## Безпека та спостережуваність (за доками)

- JWT, HTTPS, RBAC, rate limiting, security headers — див. кореневий README та `docs/security/*`.
- Audit, алерти, метрики enterprise — `enterprise/` + доки `monitoring/`.

---

## Як підтримувати цей витяг

1. Після значних змін у **модулях** або **публічному API** — оновити відповідні рядки таблиць і розділ HTTP.
2. Після нових **кроків у канонічному порядку** доків — оновити посилання вгорі та [`README.md`](../../README.md) (крок 11).
3. **Інвентар:** додати шлях до цього файлу в кореневий [`file_list.csv`](../../file_list.csv).

---

## Див. також

- [`FUNCTION_MANAGEMENT.md`](./FUNCTION_MANAGEMENT.md) — керування функціоналом, індекс vs сталевий стан, чернетки тікетів `FM-*` (крок 12).
- [`docs/INDEX_2026-03-17.md`](../INDEX_2026-03-17.md) — повна карта `docs/`.
- [`docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md`](../development/NEXT_STEPS_ARCHITECT_2026-03-17.md) — архітектурний беклог P1–P6.
