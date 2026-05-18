# Передача контексту новій сесії (PoolAI)

**Оновлено:** 2026-05-18 (менеджер функціоналу — §5.3 аудит, FM-017/018).

**Автономний прогін:** [`AUTO_RUN_SESSION_2026-05-29.md`](./AUTO_RUN_SESSION_2026-05-29.md) — **FM-017** (HttpAppError залишок). **FM-003 §4** BLOCKED.

**FM-003:** dev stand ✅ (`bin/run-virtual-node-dev.*`, `verify-dev-stand.*`, `core::dev_stand`). Real LAN §4 — **BLOCKED** (два фізичні хости).

**FM-016 ✅:** virtual nodes + `poolai-worker`. **FM-016+ ✅:** bind/webhook/store. **FM-016++ ✅:** `poolai-telegram-bot`. **FM-016+++ ✅:** pool join, `raid_artifact_probe`, artifact cache, verify-dev-stand e2e. **FM-012 ✅:** OAuth (2026-05-27). **Далі (§5.1):** **FM-017** HttpAppError залишок → FM-003 §4 (2 хости) → FM-018 a11y → FM-004/006 deferred → FM-009/010 concept.

**Зріз:** FM-015 ✅, FM-012 ✅. §5.1 [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md).

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

## 2a. Virtual node / Telegram env (FM-016+)

| Змінна | Де | Призначення |
|--------|-----|-------------|
| `POOLAI_COORDINATOR_URL` | worker | Base URL coordinator (без trailing `/`) |
| `POOLAI_TELEGRAM_ID` | worker | Telegram user id → `POST .../telegram/bind` після register |
| `POOLAI_WORKER_CACHE_DIR` | worker | Локальний кеш probe-артефактів після успішного `raid_artifact_probe` |
| `POOLAI_VIRTUAL_NODE_DATA_DIR` | coordinator | Персистентні tasks/bindings (напр. `data/virtual_nodes`) |
| `POOLAI_TELEGRAM_WEBHOOK_SECRET` | coordinator | Опційно: header `X-Telegram-Webhook-Secret` для webhook |
| `POOLAI_TELEGRAM_AUTH_MAX_AGE_SECS` | enterprise OAuth | Max вік `auth_date` для Telegram Login Widget (default 86400) |
| `TELEGRAM_BOT_TOKEN` | `poolai-telegram-bot` | Token від @BotFather |

Збірка бота: `cargo build --bin poolai-telegram-bot --features tgbot`. Запуск: `TELEGRAM_BOT_TOKEN=... POOLAI_COORDINATOR_URL=http://127.0.0.1:8080 poolai-telegram-bot`.

Секрети — лише в env на хості, не в репо.

## 2b. FM-003 dev stand (одна машина)

| Скрипт | Призначення |
|--------|-------------|
| `bin/run-lan-nodes.ps1` / `.sh` | Два `poolai` на 8080+8081 |
| `bin/run-virtual-node-dev.ps1` / `.sh` | Coordinator + `poolai-worker` |
| `bin/verify-dev-stand.ps1` / `.sh` | Health + discovery + pool join + bootstrap tasks (>=4 completed) |

Runbook: [`LAN_BENCHMARK_RUNBOOK.md`](../performance/LAN_BENCHMARK_RUNBOOK.md) §5–5.1.

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
- **Clippy (2026-04-10):** перед push доцільно прогнати ті самі команди, що в [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml): `cargo clippy --all-targets --no-default-features -- -D warnings`, `cargo clippy --all-targets --features jwt,https -- -D warnings`, і з `K8S_OPENAPI_ENABLED_VERSION=1.28` — `cargo clippy --all-targets --features cloud,cloud-sdk -- -D warnings`. Для змін у **enterprise** / UI — також `cargo clippy -p poolai --features enterprise -- -D warnings`. Код і `tests/*` вирівняні під ці матриці.
- **FM-012 ✅ (2026-05-16):** i18n UA/EN + Telegram OAuth hardening — [`oauth.rs`](../../src/network/enterprise_api/oauth.rs), [`security.rs`](../../src/enterprise/security.rs), [`i18n_core.js`](../../src/ui/i18n_core.js); unit-тести allowlist/expiry/RBAC.

## 4. Наступні кроки (канон: FM-* + Architect)

**Єдине зведення порядку робіт** — [`catalog/FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) **§5.1** (таблиця FM-003 → FM-012 та далі). Коротко:

1. **Telegram workers → virtual nodes** (майбутній **FM-016**) — воркери на пристроях з discovery/pool/RAID; потім FM-003 реальний LAN.
2. **Відкладено** — **FM-003 (real LAN)**, **cloud-sdk** (FM-006), SIMD TurboQuant (FM-004).
3. **Концепт** — Grid envelope (FM-009), Solana (FM-010).

**FM-005** ✅ (узгоджений JSON) — закрито; див. таблицю **FM-*** у [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md).

Деталі, чекбокси Architect і верифікації — [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](./NEXT_STEPS_ARCHITECT_2026-03-17.md). Індекс тікетів — таблиця **FM-*** у [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md).

## 5. Автономний режим (наступна сесія → git push)

1. Відкрити [`AUTO_RUN_SESSION_2026-05-17.md`](./AUTO_RUN_SESSION_2026-05-17.md) і вставити **стартовий промпт** з §2.
2. Агент працює як **оркестратор** (`autonomous-orchestrator.mdc`): **P0** збір патернів (subagent `explore`) → **S1–S4**; після коду — `cargo fmt` + `cargo test-ci`.
3. **Ціль:** `AUTO_DEV_PATTERNS.md` (≥15 записів з `path:line`); FM-003 LAN або runbook; Architect `- [ ]`; оновити HANDOFF / FUNCTION_MANAGEMENT §5.1.
4. **Не в обсязі:** FM-004, FM-006, FM-009, FM-010.
5. **Push:** MSYS2 bash, [`.cursor/commands/git-push.md`](../../.cursor/commands/git-push.md), **Summary** (шаблон AUTO_RUN 2026-05-17 §8).
