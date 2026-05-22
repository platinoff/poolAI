# Передача контексту новій сесії (PoolAI)

**Оновлено:** 2026-05-22 (FM-032 ✅ OpenAPI VM network; наступна **FM-033**; Post-Horizon **FM-020…032** ✅; HEAD `e49e92ef+`; A+B+C **100%**).

**Autoprogon:** [`AUTO_RUN_SESSION_2026-07-01.md`](./AUTO_RUN_SESSION_2026-07-01.md) S21–S34 ✅. **Horizon:** [`AUTO_RUN_SESSION_2026_HORIZON.md`](./AUTO_RUN_SESSION_2026_HORIZON.md) · [`HORIZON_TO_100_PLAN.md`](./HORIZON_TO_100_PLAN.md).

**FM-003:** dev stand ✅; LAN §4 — **BLOCKED** (2 хости).

**FM-016 ✅:** virtual nodes + `poolai-worker`. **FM-016+ ✅:** bind/webhook/store. **FM-016++ ✅:** `poolai-telegram-bot`. **FM-016+++ ✅:** pool join, `raid_artifact_probe`, artifact cache, verify-dev-stand e2e. **FM-012 ✅:** OAuth (2026-05-27). **P4 ✅ (2026-05-18):** `poolai_health_load` → [`BENCHMARKS.md`](../performance/BENCHMARKS.md). **FM-019 partial ✅ (S7–S12):** pa11y 18 auth + login; `PA11Y_WCAG22=1`; `a11y.yml` PR; `ci.yml` `pa11y-contract`. **OpenAPI (S14–S21)** ✅. **FM-019 CI (S22)** ✅ — `ci.yml` `pa11y-contract` + `pa11y-wcag22` (paths-filter → reusable `a11y.yml`). **S23 ✅:** Playwright smoke. **S24 ✅:** `DELETE /ui/dashboards/{id}` → 204. **S25 ✅:** UI_QUALITY P1 — tenants, OAuth2, dashboards (+3 tests). **S26 ✅:** metrics, alert-rules, SAML, policies (+4 tests; **UI_QUALITY P1 закрито**, 27 contract tests). **S27 ✅:** Playwright admin E2E — tenants + monitoring. **S28 ✅:** OpenAPI gap audit — [`OPENAPI_GAP_AUDIT_2026-05-19.md`](./OPENAPI_GAP_AUDIT_2026-05-19.md). **S29 ✅:** Playwright — `/ui/admin/security`, `/ui/admin/audit` (`admin.spec.ts`). **S30 ✅:** FM legacy docs — [`DOCS_LEGACY_AUDIT_2026-05-19.md`](./DOCS_LEGACY_AUDIT_2026-05-19.md), stale banners. **S33 ✅:** OpenAPI DTO, axe, E2E vm/workers. **S34 ✅:** docs sync A+B 100%, Playwright libs, `data/dev/` gitignore. **Прогрес autoprogon:** **100%** (A+B). **Horizon:** S35–S40 ✅ · **Post-Horizon:** FM-020…031 — [`AUTO_RUN_SESSION_2026_POST_HORIZON.md`](./AUTO_RUN_SESSION_2026_POST_HORIZON.md). **FM-026 ✅:** Jobs API contracts — `tests/jobs_api_contracts.rs`. **FM-027 ✅:** LAN sign-off prep. **FM-028 ✅:** P2b dual-port metrics — `capture-p2b-single-host-metrics.*`, `poolai-p2b-tq01-snapshot`, `BENCHMARKS.md` §FM-028. **FM-029 ✅:** Job store SQLite. **FM-030 ✅:** Monitoring SQLite MVP (`POOLAI_MONITORING_DATA_DIR`). **FM-031 ✅:** pa11y/axe — 21 auth URLs (`/ui/admin/vm|workers|libs|raid` + matrix у `a11y.spec.ts`). **Post-Horizon FM-020…031 закрито.** **Ops BLOCKED:** FM-003 §4 / FM-027 (2 хости). **Нещодавно:** FM-025 `VmTemplate` OpenAPI; FM-024 Solana devnet mock RPC. **FM-020…025 ✅.** Звірка — FM **§5.1**, **§5.7**.

**Зріз:** FM-015 ✅, FM-012 ✅. §5.1 [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md).

**Гілка роботи:** `main` (`git push origin main` → `origin/main`).

**Maintenance (2026-05-22):** FM-032 — `poolai-openapi-gap-audit` exit 0 (docs-only; без `src/`). **Наступна розробка:** **FM-033** Solana on-chain + devnet RPC (§5.1 FM).

## 1. Канонічний порядок документації та планів

Той самий список, що в кореневому [`README.md`](../../README.md) (*Documentation map*) і [`docs/README.md`](../README.md) (*Canonical reading order*), кроки **1–12**.

| Крок | Що читати |
|------|-----------|
| 0b | [`REPOSITORY_LAYOUT.md`](./REPOSITORY_LAYOUT.md) — `src/` vs `src/bin/` vs `bin/` vs `scripts/` vs `crates/`. |
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
| `POOLAI_JOB_DATA_DIR` | coordinator | Персистентні jobs (напр. `data/jobs`; default `jobs.json`) |
| `POOLAI_JOB_STORE` | coordinator | `sqlite` — `jobs.db` (потрібен `--features job-store-sqlite`); інакше JSON |
| `POOLAI_MEMORY_DATA_DIR` | coordinator | Персистентні memory shards (напр. `data/memory`, `shards.json`) |
| `POOLAI_MONITORING_DATA_DIR` | coordinator | Enterprise monitoring SQLite (`monitoring.db`: metrics, dashboards, alert_rules) |
| `POOLAI_SOLANA_CONFIG` | sidecar | Шлях до TOML (default: bundled `config/devnet.toml`) |
| `POOLAI_SOLANA_CLUSTER` | sidecar | `devnet` / `localnet` (mainnet rejected) |
| `POOLAI_SOLANA_MOCK_RPC` | sidecar | `1` — mock submit у stdout ack (`rpc` block) |

**Стек (агенти):** Rust-only runtime — [`.cursor/rules/runtime-stack-policy.mdc`](../../.cursor/rules/runtime-stack-policy.mdc); `docs/STRUCTURE.md` §7. **Не** пропонувати Python для ML/API.
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
| `bin/verify-lan-prep.ps1` / `.sh` | FM-027: dual-port or `POOLAI_NODE_*_URL` health + discovery peers |
| `bin/capture-p2b-single-host-metrics.ps1` / `.sh` | FM-028: `run-lan-nodes` + health_load ×2 + TQ01 snapshot → `data/lan-stand/metrics-fm028-*.json` |

Runbook: [`LAN_BENCHMARK_RUNBOOK.md`](../performance/LAN_BENCHMARK_RUNBOOK.md) §5–5.1. **Запуск усього проєкту:** [`RUN_LOCAL.md`](./RUN_LOCAL.md) (`bin/run-poolai.sh`).

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

**Єдине зведення** — [`catalog/FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) **§5.1** (черга **FM-033…042**), legacy audit **§5.8**, «не зроблено» **§5.3**.

| Порядок | FM | Що | Стан |
|--------|-----|-----|------|
| — | **FM-003** | LAN §4 sign-off | **BLOCKED** (2 хости); prep ✅ FM-027 |
| **1** | **FM-033** | Solana on-chain + real RPC | **Planned** — наступна сесія |
| **2** | **FM-035** | Real model loading (EXO) | **Planned** |
| **3** | **FM-034** | Job scheduler → VM/worker | **Planned** |
| **4–9** | **FM-036…042** | Sharding, UI audit, topology graph, Playwright CI, OTel, perf | Planned / Partial |
| **10** | **FM-041** | Cloud SDK deep auth | **Deferred** |

**Закрито:** FM-001…032 (Post-Horizon + autoprogon). **Не повторювати** FM-020…032.

**Промпт:** [`NEXT_SESSION_PROMPT.md`](./NEXT_SESSION_PROMPT.md).

## 5. Автономний режим (Horizon → git push)

1. Старт: [`NEXT_SESSION_PROMPT.md`](./NEXT_SESSION_PROMPT.md) — **FM-033** (Solana); ops LAN **BLOCKED**.
2. Оркестратор: [`.cursor/rules/autonomous-orchestrator.mdc`](../../.cursor/rules/autonomous-orchestrator.mdc); після змін у `src/` — `cargo fmt` + `cargo test-ci`.
3. **Не в обсязі:** FM-003 §4 LAN (2 хости); mainnet Solana; native Azure Compute SDK crate.
4. **Push:** MSYS2 UCRT64, [`git-push.md`](../../.cursor/commands/git-push.md); якщо в коміті є `src/`/`tests/`/`crates/`/`Cargo.toml` — **обов’язковий** Summary у тілі + самарі в чат після push.
5. Не стаджити `data/audit/*.log.gz`, `data/dev/`, `.commit-msg-*.txt`.
