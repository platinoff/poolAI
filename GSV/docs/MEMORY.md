# GSV — Memory mark (what/why)

Стан проєкту **Galaxy StarWalker Vision** — окремого Rust-first проєкту в `GSV/` репо PoolAI.
Оновлюється в кінці кожного band. Лічильники — вимірювані (`wc -l`, `cargo test`,
`cargo run --bin gsv-loc-audit`), не з пам'яті.

## Стан (2026-08-05 · band 108 ✅)

- **Канон:** Rust **95–100%** / wasm **0–5%** (завжди), без Python/Java; bins — лише `src/bin/`.
- **Ratio (виміряно):** `cargo run --bin gsv-loc-audit` → **95.52%** (rust 4223 / product 4421) — gate ≥95% ✅.
  Звіт: `GSV/data/rust_ratio.json` (gitignored). 292-рядковий `ui/index.html` стиснуто до **198** LOC
  (95.48% → 95.52%) — при цьому збережено всі бокси + додано **Ratio card**.
- **Тести (виміряно):** `cargo test` → **87** (46 unit + 18 `gsv_server_contracts` + 8 `gsv_omni_contracts`
  + 7 `gsv_ratio_contracts` + 8 `gsv_update_flow`). `cargo clippy --all-targets` → **0** warnings. `cargo fmt` clean.
- **Бокси:** Tracker · SLI console · Toolchain · IDE · Update/offline · Box preview · SLI terminal ·
  Tests/bench hooks · **Ratio** · **OmniRouter** (Rust AI-проксі/роутер, catalog/config/proxy).

## Що зроблено

### Band 102 (PH-S1659…S1668, ✅ 2026-08-01) — GSV migration
- `docs/gsv/` канон + `GSV/Cargo.toml` (окремий workspace, `.cargo/config.toml` → `target-dir`).
- `gsv-server` bin (axum + tokio, SSE `/events`, single-page UI `ui/index.html` embedded).
- Бокси: Tracker, SLI console, Toolchain, IDE, Update/offline, Box preview, SLI terminal, Tests/bench hooks.
- 52 tests green (на той момент), clippy 0. FM §5.12 §5.83 ✅.

### Band 108 (PH-S1719…S1728, ✅ 2026-08-05) — roles/ratio/roles canon (poolAI дисципліна)
- **PH-S1719** `GSV/docs/GSV_ROLES.md` — ролі VDT (Власник/Оркестратор/Субагенти) + канон сесії
  (S0 disk-first → project scan warnings-first → drain ≤10 PH-S* → Speeds + Rust panel → vision-sync
  → один commit → `git push` + самарі).
- **PH-S1720** `GSV/src/bin/gsv_loc_audit.rs` + `GSV/src/boxes/ratio.rs` — LOC ratio audit
  (дзеркало poolAI `poolai_loc_audit.rs`): `git ls-files --full-name`, `classify_product_path`,
  `--print/--no-write/--advisory/--min-ratio/--output/--data-dir`, gate ≥95%.
- **PH-S1721** `tests/gsv_ratio_contracts.rs` — 7 integration contracts (audit/save/load/wire/API).
- **PH-S1722** Ratio box + `GET /api/ratio` + UI Ratio card.
- **PH-S1723** `GSV/docs/MEMORY.md` (цей файл) + `GSV/docs/README.md` індекс.
- **PH-S1724** `GSV/docs/HANDOFF_NEW_SESSION.md` + `NEXT_SESSION_PROMPT.md`.
- **PH-S1725** FM §5.12 §5.89 band 108 + `docs/gsv/GSV_TECH_ROADMAP.md` band 108.
- **PH-S1726** poolAI docs parity (FUNCTIONALITY_DIGEST / vision README / GSV rows).
- **PH-S1727** poolAI `docs/development/HANDOFF_NEW_SESSION.md` + `NEXT_SESSION_PROMPT.md`.
- **PH-S1728** Band close: ratio hold, fmt, clippy, cargo test, docs canon, vision-sync, push.

## Важливі факти (не забувати)

1. **GSV — окремий Rust-проєкт** у `S:\rust\poolAI\GSV` (own workspace, own `target/`).
2. **Ratio аудит іде по git-tracked файлах** репо poolAI під префіксом `GSV/` (не `GSV/target/`, не `data/`).
   git-топ має MSYS-стиль `/s/rust/poolAI` — нормалізуємо в `S:/rust/poolAI` (`normalize_git_root`).
3. **Запущений `gsv-server` блокує `target/debug/gsv-server.exe`** → `cargo test`/`build` падає
   з `Access is denied (os error 5)` → спочатку зупинити сервер.
4. **Data dir:** `GSV/data/*` gitignored (омні-конфіг, rust_ratio.json, трекер). Запуск:
   `--repo-root S:/rust/poolAI --data-dir S:/rust/poolAI/GSV/data --port 8891`.
5. **Збірка:** terminal MSYS2 bash; PATH префікс `C:\Users\plati\.cargo\bin`.
6. **OmniRouter** прокидає через OpenAI-сумісний proxy; dry-run заголовок `X-Omni-Dry-Run: 1` —
   жодного реального мережевого запиту в тестах.
7. **UI канон:** тонкий JS/DOM glue; якщо ratio падає <95% — **compact UI/CSS**, не Rust-обхід.
