# Ролі GSV (Galaxy StarWalker Vision VDT)

Канон ролей для проєкту GSV — окремого Rust-first проєкту в `GSV/` репо PoolAI.
Дзеркало [`poolai-agent-roles.mdc`](../../.cursor/rules/poolai-agent-roles.mdc) +
[`poolai-session-iteration.mdc`](../../.cursor/rules/poolai-session-iteration.mdc),
адаптоване під статус GSV (FM §5.12 band 102 `PH-S1659…S1668` ✅, band 108 `PH-S1719…S1728`).

## Ролі

| Роль | Хто | Відповідальність |
|------|-----|------------------|
| **Власник / креативний директор** | Людина | Візія (Galaxy StarWalker Vision), пріоритети, BLOCKED/Deferred, фінальний push за бажанням |
| **Оркестратор** | Головний агент Cursor (Composer) | Звичайна сесія: один **PH-S***. **`абракадабра`:** S0 (**диск/clean першим**) → project scan (**warnings першими**) → 10 PH-S* у §5.12 → drain → **`git push` + самарі завжди в кінці** |
| **Субагенти** | Task tool | Вузькі підзадачі (explore/shell/generalPurpose); результат повертається оркестратору |

## Канон сесії (GSV)

1. **S0 — диск/git першим**: `df -h /s` / `check_target_disk.sh` → `cargo clean` якщо <5G (12G дешево) → `git fetch` → `git status -sb` → `git log -1`. Обидва проєкти живі: poolAI + GSV.
2. **Project scan — warnings першими**: пріоритети з §5.12 (GSV перед master-backlog Ratio96), roadmap, docs, code. FM-лічильники вимірювати (`wc -l`, `rg`), не з пам'яті.
3. **Drain**: до 10 PH-S* у §5.12. **Rust-first** тести: API/grid/job/telegram wire → contract tests; API-only acceptance у Rust `tests/*_contracts.rs` / `*_integration.rs`; **без Python**.
4. **Speeds + Rust panel**: `record-test-ci-speed.sh` + `record-rust-diagnostics.sh` (poolAI canon).
5. **Vision-sync**: `poolai-vision-sync --check`.
6. **Один commit + `git push` + самарі** в кінці сесії. **Не** робити mid-push.

**Не делегувати:** фінальний `git push`, закриття спринту в FM §5.12, оновлення `NEXT_SESSION_PROMPT.md`, amend після push.

## Rust ratio канон (band 108)

- **Rust 95–100% / wasm 0–5% (завжди), без Python/Java.** Bins — лише `src/bin/`.
- Ratio тримаємо через `GSV/src/boxes/ratio.rs` + bin `gsv-loc-audit` (дзеркало `poolai_loc_audit.rs`):
  ```
  cargo run --bin gsv-loc-audit                 # write GSV/data/rust_ratio.json
  cargo run --bin gsv-loc-audit -- --print      # print report, no write
  cargo run --bin gsv-loc-audit -- --min-ratio 0.95 --advisory
  ```
- Gate: `rust_ratio >= 0.95` (формальна смуга). Нижче — **compact UI/CSS** (тонкий JS/DOM glue), не додавати Rust-обхід.
- JSON звіт: `GSV/data/rust_ratio.json` (gitignored, не комітимо); live UI-бейдж через `GET /api/ratio`.

## Бокси GSV (панелі/можливості)

Tracker · SLI console · Toolchain · IDE · Update · Box preview · SLI terminal ·
Tests/bench hooks · **Ratio** · **OmniRouter** (Rust AI-проксі/роутер). Канон —
[`GSV/docs/gsv/GSV_BOXES.md`](gsv/GSV_BOXES.md), архітектура —
[`GSV/docs/gsv/GSV_ARCHITECTURE.md`](gsv/GSV_ARCHITECTURE.md).

## Збірка / тести

- Terminal — **MSYS2 bash** для `cargo`/`git`; з кореня репо:
  ```
  export PATH="/c/Users/${USER}/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
  export RUSTUP_TOOLCHAIN="stable-x86_64-pc-windows-gnu"
  cd GSV && cargo build --all-targets && cargo test && cargo clippy --all-targets
  ```
- Запущений `gsv-server` **блокує `target/debug/gsv-server.exe`** → `cargo test`/`build` падає
  з `Access is denied (os error 5)` → спочатку зупинити сервер (PID), потім build/test.
- Роутинг: `--repo-root S:/rust/poolAI --data-dir S:/rust/poolAI/GSV/data --port 8891`;
  `GSV/data/*` gitignored (секрети/API-ключі безпечні).

## Поза чергою

- **BLOCKED:** ні (band 102 повністю ✅).
- **Deferred:** Vision docs sync / migration → `GSV/docs/gsv/GSV_MIGRATION.md` (future sprints).
