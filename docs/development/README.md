# 🚀 Плани Розробки PoolAI

**Версія**: v0.2.2 → v0.3.0+  
**Останнє оновлення**: 2026-06-19

**Структура доків і правила агента:** [`../STRUCTURE.md`](../STRUCTURE.md) · [`.cursor/rules/documentation.md`](../../.cursor/rules/documentation.md)

---

## 🎯 Актуальні Документи

### Нова сесія / передача контексту
- **[`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md)** — порядок документації, гілка `main`, посилання на `git-push`, короткий стан P2 і наступні кроки.
- **[`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md)** — **ціль 90–95% Rust** + portability (wasm horizon); FM **§5.13** PH-S143…S150.
- **[`GALAXY_GRID_ROADMAP_2026-05-27.md`](./GALAXY_GRID_ROADMAP_2026-05-27.md)** — роадмеп Galaxy Grid; §5.12 **0** · PH-S524…S533 ✅; **`абракадабра`** = project scan.
- **[`AUTO_RUN_SESSION_2026_HORIZON.md`](./AUTO_RUN_SESSION_2026_HORIZON.md)** — **поточна фаза:** Horizon Layer C → 100% (S35–S40).
- **[`HORIZON_TO_100_PLAN.md`](./HORIZON_TO_100_PLAN.md)** — методика доведення проєкту до 100%.
- **[`NEXT_SESSION_PROMPT.md`](./NEXT_SESSION_PROMPT.md)** — copy-paste для агента; **`абракадабра`** · §5.12 **10** · band 70 ✅ · vision rev **377**; [`OPENAPI_GAP_AUDIT_2026-05-19.md`](./OPENAPI_GAP_AUDIT_2026-05-19.md).
- **[`SSO_HORIZON.md`](./SSO_HORIZON.md)** — band 70 SSO horizon-close matrix (`--sso-horizon`).
- **[`SSO_RATIO_ADVISORY.md`](./SSO_RATIO_ADVISORY.md)** — band 69 SSO ratio-advisory matrix (`--sso-ratio-advisory`).
- **[`SSO_VISION_SYNC.md`](./SSO_VISION_SYNC.md)** — band 68 SSO vision-sync matrix (`--sso-vision-sync`).
- **[`TENANT_HORIZON.md`](./TENANT_HORIZON.md)** — band 60 tenant phase-A horizon close (`--tenant-horizon`).
- **[`TENANT_RATIO_ADVISORY.md`](./TENANT_RATIO_ADVISORY.md)** — band 59 tenant ratio-advisory matrix (`--tenant-ratio-advisory`) + SQLite CRUD.
- **[`TENANT_VISION_SYNC.md`](./TENANT_VISION_SYNC.md)** — band 58 tenant vision-sync matrix (`--tenant-vision-sync`).
- **[`TENANT_DOCS_CANON.md`](./TENANT_DOCS_CANON.md)** — band 57 tenant docs-canon matrix (`--tenant-docs-canon`).
- **[`PROMETHEUS_METRICS.md`](./PROMETHEUS_METRICS.md)** · **[`OPENTELEMETRY_TRACING.md`](./OPENTELEMETRY_TRACING.md)** · **[`DESIGN_SYSTEM.md`](./DESIGN_SYSTEM.md)** · **[`VISUAL_REGRESSION_E2E.md`](./VISUAL_REGRESSION_E2E.md)** · **[`../security/TLS.md`](../security/TLS.md)** — PH-S07…S14 ✅; **PH-S03…S06** ✅ (VM, Raft); черга PH закрита — FM **§5.9**.
- **[`AUTO_RUN_SESSION_2026-07-01.md`](./AUTO_RUN_SESSION_2026-07-01.md)** — autoprogon S21–S34 ✅ (архів черги).
- **[`DOCS_LEGACY_AUDIT_2026-05-19.md`](./DOCS_LEGACY_AUDIT_2026-05-19.md)** — менеджер функціоналу: stale docs, не повторювати архівні `[ ]`.
- **[`AUTO_RUN_SESSION_2026-05-29.md`](./AUTO_RUN_SESSION_2026-05-29.md)** — попередній (FM-017 ✅).
- **[`AUTO_RUN_SESSION_2026-05-28.md`](./AUTO_RUN_SESSION_2026-05-28.md)** — ops hygiene ✅.
- **[`AUTO_DEV_PATTERNS.md`](./AUTO_DEV_PATTERNS.md)** — реєстр патернів для авторозробки.
- **[`AUTO_RUN_SESSION_2026-05-16.md`](./AUTO_RUN_SESSION_2026-05-16.md)** — попередній прогін (S0–S6, завершено).

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

- Формат: `src/network/json_errors.rs`, **`HttpAppError`**, **`AppError::RestError`**; **`enterprise_err`** / **`enterprise_json_err`** — `network/enterprise_api/mod.rs`. **`auth.rs`**, **`ws.rs`**, **`rate_limit.rs`**, **`login`/`refresh`**, **`check_permission`**, **`auth_middleware`** узгоджені. **FM-005** ✅. Джерело правди: **`HANDOFF`**, **`NEXT_STEPS_ARCHITECT`**, **`FUNCTION_MANAGEMENT.md` §5.1**.

### Priority 4: Продуктивність і бенчі

- Команди та baseline: [`../performance/BENCHMARKS.md`](../performance/BENCHMARKS.md). Criterion (`runtime_benchmarks`, `turboquant_benchmarks` + `ml`, `cloud_benchmarks` + `cloud`, `service_layer_benchmarks` + `test-utils`) + MSVC short-profile рядки в таблиці baseline. HTTP **`GET /api/v1/health`**: in-tree **`poolai_health_load`** (`cargo run --release --bin poolai_health_load -- …`). Опційний CI: [`.github/workflows/benchmarks.yml`](../../.github/workflows/benchmarks.yml) (`workflow_dispatch` + cron).

### Priority 2: Service layer + опціональні features

- У коді: `admin_service`, `cloud_service` (feature `cloud`), `enterprise_service`, разом із `raid` / `vm` / `library` сервісами — деталі в `NEXT_STEPS_ARCHITECT_2026-03-17.md`.

1. **Stage 4.4 AI/ML** (див. [`FUNCTIONALITY_DIGEST`](../catalog/FUNCTIONALITY_DIGEST_2026-04-06.md) §ML):
   - ✅ ML.1–ML.6 scaffolding + pipeline orchestration (enterprise HTTP)
   - 🔄 Hardening: метрики кроків, operational playbooks — [`ml/PIPELINE_MANAGEMENT.md`](../ml/PIPELINE_MANAGEMENT.md)

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
