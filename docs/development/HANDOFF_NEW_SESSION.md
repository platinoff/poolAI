# Передача контексту новій сесії (PoolAI)

**Оновлено:** 2026-04-06  
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

## 3. Що вже зроблено в service layer (P2, орієнтир)

У `src/services/`: `raid_service`, `vm_service`, `library_service`, `enterprise_service`, `cloud_service` (feature `cloud`, `AppState::cloud_manager`), `admin_service` + HTTP `GET /api/v1/admin/overview` (див. `src/network/api/admin.rs`), адмін-дашборд використовує overview.

## 4. Наступні кроки за тим самим планом

- Stage **4.4** ML pipeline — реальні Rust-бекенди кроків, тести.
- **P2b** TurboQuant — `docs/ml/TURBOQUANT_INTEGRATION.md`.
- Розширення **RaidService**; **P3** `AppError` / ErrorContext; стабілізація `cargo test --all-features` на Windows за потреби.

Деталі й чекбокси — лише в [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](./NEXT_STEPS_ARCHITECT_2026-03-17.md).
