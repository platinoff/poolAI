# Передача контексту новій сесії (PoolAI)

**Оновлено:** 2026-04-06 (сесія: ML pipeline / RaidService / TurboQuant / API errors)  
**Гілка роботи:** `main` (зазвичай `git push origin main` → `origin/main`).

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
- **Priority 3 (частково)**: `ErrorContext.hint`, `api_error_response` / `http_status_for_app_error` у `src/network/api/common.rs`; використання в RAID (`Operation`) та enterprise **`/api/enterprise/ai-ml/pipeline`** (`src/network/api/ai_ml.rs`).

## 4. Наступні кроки за тим самим планом

- **P3** — поширити узгоджений JSON помилок на решту HTTP handlers; за потреби уточнити статуси для окремих варіантів `AppError`.
- **P4** — бенчмарки / профілювання ключових шляхів.
- **P2 (опційно)** — решта RAID-маршрутів через `RaidService` (workers, events, snapshot, …).
- За потреби — стабілізація `cargo test --all-features` на Windows.

Деталі й чекбокси — лише в [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](./NEXT_STEPS_ARCHITECT_2026-03-17.md).
