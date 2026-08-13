# AGENTS.md — PoolAI (opencode)

Цей файл — адаптація правил з `.cursor/rules/` під opencode. Він застосовується **на кожній сесії** автоматично (аналог `alwaysApply: true`). Детальні правила оригіналу лишаються в `.cursor/rules/` — тут зведений операційний канон.

## Проєкт

- **Назва:** PoolAI · **Версія:** v0.2.2 (див. `Cargo.toml` / `src/version.rs`) · **Корінь:** `S:\rust\poolAI`
- **Стек (канон):** Rust `src/`, `tests/`, `crates/`; UI — JS у `src/ui/`; E2E — `e2e/` (TypeScript); **без Python**.
- **PRIMARY concept:** `docs/concept/poolAI_concept_root.txt` (читати першим).
- **Helper-файл навігації:** `file_list.csv` (репо-відносні шляхи).

## Runtime stack — завжди

1. **Rust-only** для runtime / API / ML / RAID / VM / tools. Rust bins — лише в `src/bin/` (`cargo run --bin …`).
2. **Python заборонено** в репозиторії: 0× `.py`, немає `requirements.txt` / `pyproject.toml` / venv / `pytest`. Архівні згадки в `docs/archive/` — не канон. Перевірка: `git ls-files '*.py'` має бути порожнім.
3. **Java** в репо немає — не додавати без явного запиту.
4. Admin UI: vanilla HTML+CSS+JS у `src/ui/`; WASM — лише горизонт.
5. Ops/CI: bash (MSYS2), dev-launch у `bin/`, toolchain у `scripts/`.

## Термінал — MSYS2 bash, не PowerShell

opencode на Windows за замовчуванням використовує PowerShell — **для цього репо це заборонено**. Усі `cargo` / `git` / скрипти запускати через:

```
C:\msys64\usr\bin\bash.exe -lc 'команда'
```

**Обов’язковий PATH** (перша команда кожної сесії):

```bash
export PATH="/c/Users/${USER:-${USERNAME}}/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN="${RUSTUP_TOOLCHAIN:-stable-x86_64-pc-windows-gnu}"
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI || cd "S:/rust/poolAI"
```

- Шлях у MSYS2: `/s/rust/poolAI` = `S:\rust\poolAI`.
- Toolchain: `rust-toolchain.toml` — rustc/cargo **1.92.0**, target `x86_64-pc-windows-gnu`, rustfmt, clippy.
- **Не** використовувати PowerShell для `git`/`cargo`; не запускати кілька `cargo` паралельно (file lock).
- `npm` — через `/ucrt64/bin` (інакше `npm: command not found`).
- Помилки → `docs/troubleshooting/` (`QUICK_FIX_MSYS2.md`, `GCC_DLLTOOL_NOT_FOUND.md`, `RUST_VERSION_ISSUE.md`, `GIT_PUSH_FAILED.md`).

## S0 — початок сесії (спочатку диск, потім git + docs)

1. **Диск S:** `df -h /s | tail -1` + `bash scripts/check_target_disk.sh`
   - avail **<5G** або warn → `cargo clean` (потім знову `df`).
   - avail **<12G** або `target/` роздутий (>48G) → чистити перед важкими збірками.
2. MSYS2: `git fetch`; `git status -sb`; `git log -1 --oneline`.
3. Прочитати: `docs/development/HANDOFF_NEW_SESSION.md` (кроки 1–12), FM §5.12, `docs/development/NEXT_SESSION_PROMPT.md`.

## Ключові документи (читати першими)

| Призначення | Файл |
|---|---|
| Старт сесії / handoff | `docs/development/HANDOFF_NEW_SESSION.md` |
| Промпт сесії | `docs/development/NEXT_SESSION_PROMPT.md` |
| FM / беклог | `docs/catalog/FUNCTION_MANAGEMENT.md` §5.1 (FM-*) · §5.12 (PH-S*) · §5.13 (Rust ratio) |
| Концепт (PRIMARY) | `docs/concept/poolAI_concept_root.txt` |
| Стабільний стан | `docs/status/STABLE_STATE_SUMMARY.md` |
| Architect-план | `docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md` |
| Rust ratio стратегія | `docs/development/RUST_RATIO_STRATEGY_2026-06-13.md` |
| Roadmap Galaxy | `docs/development/GALAXY_GRID_ROADMAP_2026-05-27.md` |
| Таксономія docs | `docs/STRUCTURE.md` |
| Digest функціоналу | `docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md` |
| Dev-патерни | `docs/development/AUTO_DEV_PATTERNS.md` |

## Git — commit / push

- Усе через **зовнішній MSYS2 bash** (не PowerShell, не вбудований термінал opencode для push).
- **Формат:** Conventional Commits `<type>(<scope>): <subject>`; типи `feat fix docs style refactor perf test build ci chore revert`.
- **Summary у тілі обов’язково**, якщо в коміті є код (`src/`, `tests/`, `crates/`, `Cargo.toml`): що змінено (модулі, FM-id) + які `cargo` перевірки реально прогнані. Docs-only — короткий Summary.
- Staging: **тільки файли спринту** (`git diff --cached --stat`), **не** `git add -A`.
- **Ніколи не стаджити:** `.env*`, `*.pem`/`*.key`, `certs/*.pem`, `data/audit/*` (крім `.gitkeep`), `comitmsg/*.txt`, `data/audit/*.log.gz`.
- Hook/subject fix (якщо subject став `Co-authored-by:`): `bash bin/amend-head-msg.sh comitmsg/<file>.txt`.
- Канон commit+push: `git push origin main` + короткий самарі в чат.
- **Не** `git commit --amend` після push без явного запиту. History rewrite — лише власник.

## Тести — Rust-first

| Scope | Команда |
|---|---|
| API / grid / job / telegram wire | `cargo fmt --all` → `cargo test-ci` (+ після API: `cargo run --bin poolai-openapi-gap-audit` — 0 missing) |
| Admin UI / axe / visual | + `bash bin/e2e-playwright.sh --start` (PATH з `/ucrt64/bin`) |
| Raft | `cargo test-raft-ci` |
| Cloud | `cargo test --test cloud_mock_integration --features cloud,cloud-sdk -- --test-threads=1` |

- **API-only acceptance** (HTTP/4xx/JSON) → Rust `tests/*_integration.rs` / `tests/*_contracts.rs`, **не** новий Playwright spec. Playwright лише для браузера (DOM, axe, visual, admin click-flow).
- Існуючі Playwright API-smoke — legacy; не дублювати новими TS.
- Rust стиль: `AppError` (`src/core/error.rs`), `?`, без `unwrap()`/`expect()` у продукті, `Arc<RwLock<T>>`, `tokio`, `tracing`. Модулі через `mod.rs` (як `src/raid/mod.rs`).
- Перед важкими матрицями: `bash scripts/check_target_disk.sh` (пороги `POOLAI_MIN_FREE_DISK_GB`=12, `POOLAI_MAX_TARGET_DIR_GB`=48).

## Ітераційна розробка (один PH-S*)

- Черга: FM **§5.12** (журнал, ≤10 відкритих) + `NEXT_SESSION_PROMPT.md`; пріоритети — `docs/concept/`, FM **§5.1**, roadmaps, код.
- Scope — лише файли поточного PH-S*; не повторювати закрите з «Не повторювати» у NEXT_SESSION.
- Завершення PH-S*: `cargo fmt` → `cargo test-ci` → ✅ у §5.12 → HANDOFF → NEXT_SESSION → наступний PH-S*.
- FM §5.12 < 10 відкритих → project scan (нижче).

## Тригер «абракадабра» — drain всього проєкту

Коли власник у новій сесії пише **`абракадабра`** — це VDT-цикл по всьому репо **без підтвердження кожного PH-S***:

1. **S0:** диск (`df -h /s` + `check_target_disk.sh` → `cargo clean` за потреби) → `git fetch` → HANDOFF → FM §1–§5.1 → NEXT_SESSION → `cargo run --bin poolai-vision-sync -- --check` ok.
2. **Project scan** (якщо §5.12 <10): **warnings/diagnostics першими** (`rust_diagnostics.json`, `cargo run --bin poolai-rust-diagnostics -- --print`, clippy warnings, compile errors) → потім concept (`docs/concept/POOLAI_GALAXY_GRID.md`), FM §5.1, architect rows (`rg "\- \[ \]"` у `NEXT_STEPS_ARCHITECT_*.md`), roadmaps, gaps (`DOCS_LEGACY_AUDIT`, `OPENAPI_GAP_AUDIT`), `rg "TODO|FIXME" src/`. → **10 PH-S*** у §5.12 (нумерація N+1 від останнього; колонка «Джерело»).
3. **Drain:** усі відкриті PH-S* (код → scope-тести; **без** mid-drain push).
4. **Vision close:** FM §5.12 ✅ + HANDOFF + NEXT → один `poolai-vision-sync` (manifest `revision++`) → FM/NEXT/INDEX rev = `manifest.revision` → `--check`.
5. **Test:** один `cargo fmt --all` → один `cargo test-ci` → `bash bin/record-test-ci-speed.sh` + `bash bin/record-rust-diagnostics.sh`.
6. **Git (кінець сесії):** один commit (код + `GSV/docs/vision/*` + speed/diagnostics JSON + FM/HANDOFF/NEXT) → **`git push origin main` + самарі** — **завжди останній крок**. Не завершувати сесію без push+самарі, якщо був drain/commit.

- **Не** `git add -A`, **не** push mid-drain/mid-scan, **не** паралелити `cargo test-ci`.
- Warnings >0 або errors >0 з виправними пакетами → 1–3 PH-S* **на початок** смуги (Джерело: `rust_diagnostics` / lint code).
- Fallback-смуга, якщо concept/roadmap не заповнюють чергу: galaxy metrics stubs, wasm glue, stand smoke, concept wire, loc-audit, docs canon, vision sync, INDEX.

## Documentation (структура)

- **Уся документація — у `docs/`.** Ніколи не створювати `.md` у корені (окрім README).
- Канонічний порядок — README «Documentation map»: крок 11 = `docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`, крок 12 = `docs/catalog/FUNCTION_MANAGEMENT.md`.
- Каталоги: `status/` (статус), `development/` (плани), `concept/` (концепти), `archive/` (історія), `troubleshooting/` (гайди), `catalog/` (диджести).
- Після зміни продукту (модулі, маршрути, features): оновити `FUNCTIONALITY_DIGEST` + `file_list.csv` (при нових шляхах) + README Next Focus.
- Скрипти: dev/ops launchers → `bin/` (.sh/.ps1), toolchain/deploy → `scripts/` (**.sh лише, без нових .ps1**), Rust CLIs → `src/bin/`.
- Переліки файлів: тільки описові назви, **не** `.ps1`/`.ps` розширеннями.
- Vision map (`GSV/docs/vision/`): після FM/HANDOFF — `poolai-vision-sync` + `--check`; **live UI (band 117):** `gsv-server` → `http://127.0.0.1:8891/` · [`GSV_SERVER.md`](GSV/docs/gsv/GSV_SERVER.md); legacy `bin/open-docs-vision.ps1` → `http://127.0.0.1:8765/GSV/docs/vision/index.html` — архів (deactivated, band 117).

## Безпека та токени

- Ніколи не логувати / не валідувати довжину `GITHUB_TOKEN` / `ghs_*` (може бути JWT ~520 символів) — treat as **opaque**. Регулярка тільки якщо неминуче: `ghs_[A-Za-z0-9\.\-_]{36,}`.
- Canon: `docs/security/SECRETS_MANAGEMENT.md` §1, §5.
- Заборонено в drain/auto-run: force-push, secrets у комітах, rewrite history, Python product files.

## Трансляція Cursor-термінів у opencode

| Cursor | opencode |
|---|---|
| `alwaysApply` / globs у `.mdc` | цей `AGENTS.md` (завжди) |
| subagent `explore` | Task `explore` |
| subagent `generalPurpose` | Task `general` |
| subagent `shell` (MSYS2 команди) | bash tool через `C:\msys64\usr\bin\bash.exe -lc '…'` |
| `bugbot` / `security-review` / `best-of-n-runner` | лише за явним запитом review; не в drain |
| Run Modes / `/multitask` | не застосовні; канон — послідовний MSYS2 flow |
| `git-push.md` команда | push через зовнішній MSYS2 bash + самарі |
| Cursor Router / side chats | не застосовні |
