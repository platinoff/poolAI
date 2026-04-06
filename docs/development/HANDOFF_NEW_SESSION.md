# Передача контексту новій сесії (PoolAI)

**Оновлено:** 2026-04-06 (P3 по REST/enterprise/raid завершено; доки синхронізовано)  
**Гілка роботи:** `main` (`git push origin main` → `origin/main`).

## 1. Канонічний порядок документації та планів

| Порядок | Що читати |
|--------|-----------|
| 1 | Кореневий [`README.md`](../../README.md) — карта посилань, збірка, CI. |
| 2 | [`docs/INDEX_2026-03-17.md`](../INDEX_2026-03-17.md) — навігація по всьому `docs/`. |
| 3 | [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](./NEXT_STEPS_ARCHITECT_2026-03-17.md) — **головний** план Rust Architect (P1–P6, TurboQuant). |
| 4 | Концепція: [`concept/poolAI_concept_root.txt`](../concept/poolAI_concept_root.txt), Grid/Memory/Job у `docs/concept/` та `JOB_LAYER_CONCEPT_2026-03-17.md`. |
| 5 | Інвентар: кореневий [`file_list.csv`](../../file_list.csv) (ручний зріз); повний список файлів: `git ls-files`. |
| 6 | Архітектура: [`ARCHITECTURE_REVIEW.md`](../ARCHITECTURE_REVIEW.md), [`ARCHITECTURE_BEST_PRACTICES.md`](../ARCHITECTURE_BEST_PRACTICES.md). |

Індекс планів у `docs/development/`: [`development/README.md`](./README.md).

## 2. Git push (Windows / Cursor)

- **Канонічна інструкція:** [`.cursor/commands/git-push.md`](../../.cursor/commands/git-push.md) — MSYS2 UCRT64 **зовнішній** термінал, `PATH` з `~/.cargo/bin`, `K8S_OPENAPI_ENABLED_VERSION=1.28` за потреби cloud-sdk, формат коміта з Summary.
- Не робити `git add -A` без потреби; не стаджити `data/audit/*.log.gz`.
- Старі одноразові нотатки `PUSH_*.md` перенесені в [`docs/archive/`](../archive/); актуальні проблеми — [`docs/troubleshooting/`](../troubleshooting/).

## 3. Що вже зроблено (орієнтир для нової сесії)

- **`src/services/`**: `raid_service`, `vm_service`, `library_service`, `enterprise_service`, `cloud_service`, `admin_service` + `GET /api/v1/admin/overview` (`src/network/api/admin.rs`).
- **RaidService (P2)**: крім list — `put_artifact`, `delete_artifact`, `quota`, `cluster_status`; DTO квоти/статусу в `raid_service.rs`; тонкі handlers у `src/network/api/raid.rs`.
- **ML pipeline (Stage 4.4)**: детерміновані Rust-бекенди для `Preprocessing`, `Training`, `Evaluation`, `Deployment` (`src/ml/pipeline.rs`).
- **TurboQuant (P2b, фаза 1)**: `src/ml/turboquant.rs` (формат `TQ01`), інтеграція в крок `Quantization` за конфігом; див. `docs/ml/TURBOQUANT_INTEGRATION.md`.
- **Priority 3 (основний HTTP-шар)**: `src/network/api/common.rs` — `api_error_response`, **`api_json_error`**, `http_status_for_app_error`; `src/core/error.rs` — **`AppError::Forbidden`**, `ErrorContext` (+ `hint`). Узгоджені відповіді: **`raid.rs`** (у т.ч. `raid_api_err`, `raid_event_store_unavailable`), **повний** **`enterprise_api.rs`** (хелпер **`enterprise_err`**), **`users`**, **`ui`**, **`system`**, **`completions`**, **`raid_admin`**, раніше — **`ai_ml`**, **instances/libraries/vm/workers/topology/rewards**, tenant CRUD, RAID `Operation` через `api_error_response`.
- **P3 (auth)**: **`src/network/auth.rs`** — узгоджено з **`api_json_error`** / **`ErrorContext`** (модуль **`network/json_errors.rs`**); UI читає `error.message`.
- **Перевірка тестів (як CI)**: `K8S_OPENAPI_ENABLED_VERSION=1.28` + `cargo test --lib --tests --features ml,enterprise,cloud`. На Windows при OOM лінкера: `cargo test ... -j 1 -- --test-threads=1`.

## 4. Наступні кроки за тим самим планом

1. **P4 (продовження)** — Criterion таргети як вище; у `BENCHMARKS.md` додано **перший dev-sample baseline** (скорочений профіль Criterion). Далі — ті самі команди на **референс-машині**, **`wrk`**, опційно CI для `cargo bench`. `service_layer_benchmarks`: `AppState::new` під **`rt.enter()`** (WS manager робить `tokio::spawn`).
2. **P2 (опційно)** — RAID workers/events/snapshot тощо через `RaidService`.
3. **P2b / доки** — оновити чекбокси TurboQuant у `NEXT_STEPS_ARCHITECT` під фактичний код у `src/ml/turboquant.rs`.
4. За потреби — `cargo test --all-features` на Windows (`-j 1` при OOM лінкера).

Деталі й чекбокси — [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](./NEXT_STEPS_ARCHITECT_2026-03-17.md).
