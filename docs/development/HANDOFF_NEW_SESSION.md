# Передача контексту новій сесії (PoolAI)

**Оновлено:** 2026-04-12 (кроки 1–12; **FM-012** — **i18n UA/EN** (`src/ui/i18n_core.js`): `/ui/auth` + admin sidebar / мова / `admin_common` hints; **FM-011** — alias **`cargo test-ci`**; **FM-003** / P4 — short Criterion + рядки **`win10-local-26200-*-2026-04-12`** у [`BENCHMARKS.md`](../performance/BENCHMARKS.md); **FM-007** — розширені wire-тести **`SyncArtifacts`** (Pull, Bidirectional, рівні timestamp) у **`distributed_raid_wire_integration`**; **FM-005** ✅ — **`HttpAppError`/`RestError`** по REST + **`enterprise_api`** + **`login`/`refresh`** + **`check_permission`** + **`auth_middleware`**; **§5.1** — [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md); **«Операційний порядок»** — [`NEXT_STEPS_ARCHITECT`](./NEXT_STEPS_ARCHITECT_2026-03-17.md); **FM-007/008**; **FM-011**; Clippy / [`STABLE_STATE_SUMMARY.md`](../status/STABLE_STATE_SUMMARY.md))  
**Гілка роботи:** `main` (`git push origin main` → `origin/main`).

## 1. Канонічний порядок документації та планів

Той самий список, що в кореневому [`README.md`](../../README.md) (*Documentation map*) і [`docs/README.md`](../README.md) (*Canonical reading order*), кроки **1–12**.

| Крок | Що читати |
|------|-----------|
| 1 | Кореневий [`README.md`](../../README.md) — швидкий старт, збірка, CI, карта доків. |
| 2 | [`INDEX_2026-03-17.md`](../INDEX_2026-03-17.md) — навігація по всьому `docs/`. |
| 3 | [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](./NEXT_STEPS_ARCHITECT_2026-03-17.md) — **головний** план Rust Architect (P1–P6, TurboQuant). |
| 4 | **Цей файл** — [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md): гілка, git-push, зріз P2/P3, next steps. |
| 5 | Концепція: [`concept/poolAI_concept_root.txt`](../concept/poolAI_concept_root.txt), Grid/Memory/Job у `docs/concept/` та [`JOB_LAYER_CONCEPT_2026-03-17.md`](./JOB_LAYER_CONCEPT_2026-03-17.md). |
| 6 | Архітектура: [`ARCHITECTURE_REVIEW.md`](../ARCHITECTURE_REVIEW.md), [`ARCHITECTURE_BEST_PRACTICES.md`](../ARCHITECTURE_BEST_PRACTICES.md). |
| 7 | Продуктивність: [`performance/BENCHMARKS.md`](../performance/BENCHMARKS.md), [`performance/PROFILING.md`](../performance/PROFILING.md); **`poolai_health_load --json`** для baseline; опційно [`benchmarks.yml`](../../.github/workflows/benchmarks.yml). |
| 8 | CI: [`ci.yml`](../../.github/workflows/ci.yml). |
| 9 | Інвентар: [`file_list.csv`](../../file_list.csv) (оновлюй також `docs/catalog/` при зміні витягу); повний список: `git ls-files`. |
| 10 | Git push (Windows): [`.cursor/commands/git-push.md`](../../.cursor/commands/git-push.md). |
| 11 | Витяг функціоналу: [`catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`](../catalog/FUNCTIONALITY_DIGEST_2026-04-06.md). |
| 12 | Керування функціоналом: [`catalog/FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) (**§5.1 — наступні кроки за FM-***); правило [`.cursor/rules/functionality-management.mdc`](../../.cursor/rules/functionality-management.mdc). |

Індекс планів у `docs/development/`: [`README.md`](./README.md). **Таксономія каталогу `docs/`:** [`../STRUCTURE.md`](../STRUCTURE.md). OpenAPI: [`docs/openapi.yaml`](../openapi.yaml). UI↔API: [`UI_QUALITY_AND_E2E_PLAN_2026-04-06.md`](./UI_QUALITY_AND_E2E_PLAN_2026-04-06.md). **Крок 11 / витяг функціоналу:** [`catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`](../catalog/FUNCTIONALITY_DIGEST_2026-04-06.md). **Крок 12 / беклог:** [`catalog/FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) (**§5.1** — наступні кроки за FM-*). **Project skill (Cursor):** [`.cursor/skills/poolai-documentation/SKILL.md`](../../.cursor/skills/poolai-documentation/SKILL.md).

## 2. Git push (Windows / Cursor)

- **Канонічна інструкція:** [`.cursor/commands/git-push.md`](../../.cursor/commands/git-push.md) — MSYS2 UCRT64 **зовнішній** термінал, `PATH` з `~/.cargo/bin`, `K8S_OPENAPI_ENABLED_VERSION=1.28` за потреби cloud-sdk, формат коміта з Summary.
- Не робити `git add -A` без потреби; не стаджити `data/audit/*.log.gz`.
- Старі одноразові нотатки `PUSH_*.md` перенесені в [`docs/archive/`](../archive/); актуальні проблеми — [`docs/troubleshooting/`](../troubleshooting/).

## 3. Що вже зроблено (орієнтир для нової сесії)

- **`src/services/`**: `raid_service`, **`raid_distributed_protocol_service`** (distributed RAID JSON protocol; тонкий `raid_distributed_handlers.rs`), `vm_service`, `library_service`, **`instance_service`** (`/api/v1/instance/*`, `/state`), **`chat_completion_service`** (`/v1/chat/completions` — тонкий `completions.rs`), **`system_service`** (status/health/metrics/models/GPU, login, config get/update), **`ui_service`** (теми/компоненти + enterprise-дашборди через `EnterpriseService`), **`discovery_service`**, **`topology_service`**, **`worker_pool_service`**, **`rewards_service`** (`/api/v1/rewards/*`), `enterprise_service`, `cloud_service`, `admin_service` + `GET /api/v1/admin/overview` (`src/network/api/admin.rs`). HTML **`GET /api/v1/status`** — модуль **`network/api/system_status_html.rs`** (не в `SystemService`).
- **RaidService (P2)**: крім list — `put_artifact`, `delete_artifact`, `quota`, `cluster_status`; DTO квоти/статусу в `raid_service.rs`; тонкі handlers у `src/network/api/raid.rs`.
- **ML pipeline (Stage 4.4)**: детерміновані Rust-бекенди для `Preprocessing`, `Training`, `Evaluation`, `Deployment` (`src/ml/pipeline.rs`).
- **TurboQuant (P2b, фаза 1)**: `src/ml/turboquant.rs` (формат `TQ01`), інтеграція в крок `Quantization` за конфігом; див. `docs/ml/TURBOQUANT_INTEGRATION.md`.
- **Priority 3 / FM-005 (HTTP-шар)** ✅: `json_errors.rs` — **`HttpAppError`**, **`IntoResponse`**; **`AppError::RestError`**. Покриття: **`api/*`**, **`raid*`** (**`raid_api_err`**), **`enterprise_api`**, **`authenticate_user`** / **`refresh_access_token`** / **`login`/`refresh` handlers**, **`check_permission`**, **`auth_middleware`** / **`permission_middleware`**.
- **P3 (auth / WS / rate limit)**: **`auth.rs`**, **`ws.rs`**, **`rate_limit.rs`** — той самий JSON-формат помилок (`src/network/json_errors.rs`); UI читає `error.message`. **`http_status_for_app_error`**, **`IntoResponse`** для **`AppError`** / **`HttpAppError`**. Приклад змішаного стилю: **`api/rewards.rs`** — частина GET → **`Result<Json<_>, AppError>`**, **`/rewards/progress/*`** → **`Result<_, HttpAppError>`** (**`ApiNotFound`** / **`NOT_FOUND`**).
- **Перевірка тестів (як CI)**: `K8S_OPENAPI_ENABLED_VERSION=1.28` + `cargo test --lib --tests --features ml,enterprise,cloud,test-utils` (інжектований `AppState`: **`tests/appstate_http_injection_integration.rs`** поряд з **`distributed_raid_wire_integration`**). На Windows при OOM лінкера: `cargo test ... -j 1 -- --test-threads=1`.
- **Clippy (2026-04-10):** перед push доцільно прогнати ті самі команди, що в [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml): `cargo clippy --all-targets --no-default-features -- -D warnings`, `cargo clippy --all-targets --features jwt,https -- -D warnings`, і з `K8S_OPENAPI_ENABLED_VERSION=1.28` — `cargo clippy --all-targets --features cloud,cloud-sdk -- -D warnings`. Код і `tests/*` вирівняні під ці матриці.

## 4. Наступні кроки (канон: FM-* + Architect)

**Єдине зведення порядку робіт** — [`catalog/FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) **§5.1** (таблиця FM-003 → FM-010). Коротко:

1. **Baseline / стенд (FM-003, P4, P2b)** — Criterion на реф-хості; **`poolai_health_load --json`** → [`BENCHMARKS.md`](../performance/BENCHMARKS.md); LAN — повні заміри реплікації + TQ01.
2. **Distributed RAID (FM-007, FM-008)** — у коді: порівняння каталогів у **`SyncArtifacts`**, **`LeaveCluster`** з replication + **`delete_worker`**; далі за потреби: LAN-заміри, **`conflicts`** у payload (remote timestamps), глибша multi-hop реплікація.
3. **Ops (FM-011)** — `Cargo.toml` **`[profile.test] debug = 1`**; канонічний прогін як у CI: **`cargo test-ci`** (alias у **`.cargo/config.toml`**) після **`K8S_OPENAPI_ENABLED_VERSION=1.28`** — лише **`--lib` + `--tests`**, без doctests (на Windows повний **`cargo test`** з doc-тестами може дати **os error 1455**). Інакше: `-j 1`, опційно `CARGO_INCREMENTAL=0`; GNU toolchain або дроблення features за потреби.
4. **Deferred** — **cloud-sdk** (FM-006), SIMD TurboQuant (FM-004); **концепт** — Grid envelope (FM-009), Solana (FM-010).

**FM-005** ✅ (узгоджений JSON) — закрито; див. таблицю **FM-*** у [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md).

Деталі, чекбокси Architect і верифікації — [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](./NEXT_STEPS_ARCHITECT_2026-03-17.md). Індекс тікетів — таблиця **FM-*** у [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md).
