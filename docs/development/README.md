# 🚀 Плани Розробки PoolAI

**Версія**: v0.2.2 → v0.3.0+  
**Останнє оновлення**: 2026-04-10

**Структура доків і правила агента:** [`../STRUCTURE.md`](../STRUCTURE.md) · [`.cursor/rules/documentation.md`](../../.cursor/rules/documentation.md)

---

## 🎯 Актуальні Документи

### Нова сесія / передача контексту
- **[`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md)** — порядок документації, гілка `main`, посилання на `git-push`, короткий стан P2 і наступні кроки.

### Головний план (Rust Architect, 2026-03-17+)
- **[`NEXT_STEPS_ARCHITECT_2026-03-17.md`](./NEXT_STEPS_ARCHITECT_2026-03-17.md)** — **канонічний** покроковий план (таблиця P1–P7, TurboQuant, верифікації CI) + підрозділ **«Операційний порядок»** (дзеркало **§5.1** у [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md)). Старіші файли `NEXT_STEPS_ARCHITECT_2026-01-22.md` тощо — історичні; див. [`../archive/development/`](../archive/development/).

### Інші основні плани:
- [`NEXT_STEPS_2026-01-19.md`](./NEXT_STEPS_2026-01-19.md) — наступні кроки v0.2.2 → v0.3.0+ (контекст)
- [`FUTURE_DEVELOPMENT_ROADMAP.md`](./FUTURE_DEVELOPMENT_ROADMAP.md) — майбутній roadmap
- [`PERFORMANCE_OPTIMIZATION_PLAN_2026-03-17.md`](./PERFORMANCE_OPTIMIZATION_PLAN_2026-03-17.md) — оптимізація продуктивності (Tokio/AppState/кеш)

### Допоміжні документи:
- [`CONCEPT_IMPLEMENTATION_CHECKLIST.md`](./CONCEPT_IMPLEMENTATION_CHECKLIST.md) - Чеклист реалізації концепції
- [`UI_QUALITY_AND_E2E_PLAN_2026-04-06.md`](./UI_QUALITY_AND_E2E_PLAN_2026-04-06.md) — узгодження API↔UI, автотести, E2E (план)

---

## 📊 Поточні Пріоритети (v0.3.0+)

**Єдиний нумерований порядок робіт** — [`../catalog/FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) **§5.1** (FM-003 … FM-010). Нижче — довідкові теми за P\*; деталі та чекбокси — у **`NEXT_STEPS_ARCHITECT_2026-03-17.md`**.

### Priority 3: Узгоджені HTTP / транспортні помилки

- Формат: `src/network/json_errors.rs`, реекспорт **`HttpAppError`** у `api/common.rs`; **`AppError::RestError`** для стабільних REST-кодів; **`enterprise_err`** — `network/enterprise_api/mod.rs`. **`auth.rs`**, **`ws.rs`**, **`rate_limit.rs`** узгоджені. **FM-005 (Partial)**: більшість **`api/*.rs`** на **`HttpAppError`/`RestError`**; лишаються **`raid*`**, **`enterprise_api/`**. Джерело правди: **`HANDOFF_NEW_SESSION.md`**, **`NEXT_STEPS_ARCHITECT_2026-03-17.md`**, **`FUNCTION_MANAGEMENT.md` §5.1**.

### Priority 4: Продуктивність і бенчі

- Команди та baseline: [`../performance/BENCHMARKS.md`](../performance/BENCHMARKS.md). Criterion (`runtime_benchmarks`, `turboquant_benchmarks` + `ml`, `cloud_benchmarks` + `cloud`, `service_layer_benchmarks` + `test-utils`) + MSVC short-profile рядки в таблиці baseline. HTTP **`GET /api/v1/health`**: in-tree **`poolai_health_load`** (`cargo run --release --bin poolai_health_load -- …`). Опційний CI: [`.github/workflows/benchmarks.yml`](../../.github/workflows/benchmarks.yml) (`workflow_dispatch` + cron).

### Priority 2: Service layer + опціональні features

- У коді: `admin_service`, `cloud_service` (feature `cloud`), `enterprise_service`, разом із `raid` / `vm` / `library` сервісами — деталі в `NEXT_STEPS_ARCHITECT_2026-03-17.md`.

1. **Stage 4.4 AI/ML**:
   - ✅ ML.1 Model Optimization (profiling, tuning, quantization) - завершено
   - ⏸️ ML.2 AutoML - stub готовий, потрібна implementation
   - ⏸️ ML.3 Federated Learning - stub готовий, потрібна implementation
   - ⏸️ ML.1 pruning strategies

2. **Mock Server Integration**:
   - ✅ Harness + Azure + GCP + AWS base_url_override - завершено
   - ✅ e2e mock tests - завершено

---

## 📋 Завершені Завдання

- ✅ v0.2.2 release prep - Changelog, release notes, README
- ✅ Cloud SDK 100% - AWS/Azure/GCP, Auto-scaling, Load Balancing, HPA
- ✅ RAID Strategy 100% - BurstRAID, SmallWorld, Admin Control Plane
- ✅ Enterprise Features 100% - SQLite, OAuth2, SAML SSO

---

## 📦 Архівні Документи

Застарілі документи переміщені в [`../archive/development/`](../archive/development/)

**Примітка**: Багато файлів з датами 2026-01-16, 2026-01-17, 2026-01-18 є застарілими та переміщені в архів. Використовуй тільки актуальні документи з датою 2026-01-19 або пізніше.

---

## 🔗 Посилання

- [`../../README.md`](../../README.md) — кореневий README (швидкий старт, карта доків 1–12)
- [`../README.md`](../README.md) — вхід у каталог `docs/` (канонічний порядок + короткі вказівки)
- [`../INDEX_2026-03-17.md`](../INDEX_2026-03-17.md) — повна навігація по `docs/`
- [`../catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`](../catalog/FUNCTIONALITY_DIGEST_2026-04-06.md) — витяг функціоналу (крок 11)
- [`../catalog/FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) — керування функціоналом, тікети FM-* (крок 12)
- [`../openapi.yaml`](../openapi.yaml) — OpenAPI
- [`../status/PROJECT_STATUS_REPORT_2026-01-19.md`](../status/PROJECT_STATUS_REPORT_2026-01-19.md) — статус проєкту
- [`../concept/poolAI_concept_root.txt`](../concept/poolAI_concept_root.txt) — концепція

---

**Підготовлено**: Rust Architect  
**Дата останнього узгодження з main**: 2026-04-06
