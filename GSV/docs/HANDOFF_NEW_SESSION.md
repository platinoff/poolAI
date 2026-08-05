# Передача контексту новій сесії (GSV)

**Оновлено:** 2026-08-05 (band 110 **PH-S1739…S1748** ✅ · ratio **96.01%** · tests **106** · clippy **0**)

**Наступна сесія:** **`абракадабра`** → S0 диск/git → project scan (warnings first) → drain ≤10 PH-S*
→ Speeds + Rust panel → vision-sync → **один commit** → **`git push` + самарі**. Канон:
[`GSV_ROLES.md`](GSV_ROLES.md).

## Стан зараз

- **GSV** — окремий Rust-first проєкт (`GSV/`), bands 102 · 108 · 109 · 110 **✅**.
- **Ratio:** `cargo run --bin gsv-loc-audit` → **96.01%** (gate ≥95% ✅) → `GSV/data/rust_ratio.json`.
- **Тести:** `cargo test` → **106** green · **clippy 0** · **fmt clean**.
- **Сервер живий:** `gsv-server` на порту **8870** (8891 був транзитивно зарезервований Windows dynamic exclusion у band 110 close; canon порт **8891** — `--repo-root S:/rust/poolAI --data-dir S:/rust/poolAI/GSV/data`).
- **FM:** band 110 = §5.91 (PH-S1739…S1748 ✅). Master horizon poolAI: band 111 (PH-S1749…S1758).
- **Vision rev:** 461 (band 110 vision-sync). Vision box: `boxes/vision.rs` + `gsv-vision-sync` bin +
  `GET /api/vision*`; snapshot `GSV/data/gsv_manifest.json` + `gsv_feed.json` (rev 461).
  Band 110: `GET /api/vision/map` (layers L0..L5 z-sorted + edge kinds), `GET /assets/vision.svg`
  (порт `docs/vision/vision.svg`), `GET /api/vision/feed?status=` фільтр, Vision Map card у UI.
- **poolAI ratio:** **95.02%** (advisory hold, `--ratio96-docs-canon --advisory --min-ratio 0.95`).

## S0 (кожна сесія, disk/git first)

1. `df -h /s | tail -1` → `bash scripts/check_target_disk.sh` → `cargo clean` якщо <5G (12G дешево).
2. `git fetch` → `git status -sb` → `git log -1 --oneline`.
3. Прочитати цей HANDOFF + `NEXT_SESSION_PROMPT.md` + FM §5.12 §5.91.

## Project scan (якщо §5.12 < 10 відкритих)

- Warnings/diagnostics першими: `cargo run --bin poolai-rust-diagnostics -- --print` (poolAI),
  clippy warnings GSV (`cargo clippy --all-targets`).
- Роадмапи/архітектор-ряди: `docs/gsv/GSV_TECH_ROADMAP.md`, `GSV/docs/`, FM §5.1.
- Fallback-смуга: ratio contracts, UI compact, docs canon, vision sync, stand smoke.

## Build/test (MSYS2 bash)

```bash
export PATH="/c/Users/${USER}/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN="stable-x86_64-pc-windows-gnu"
cd GSV
cargo fmt -- --check && cargo clippy --all-targets && cargo test && cargo run --bin gsv-loc-audit
```

**⚠️ Перед build/test зупинити `gsv-server`** (блокує `gsv-server.exe`, os error 5); після — перезапустити.

## Git (кінець сесії)

- Один commit (код + docs + FM/HANDOFF/NEXT). Не `git add -A` — тільки файли спринту.
- **`git push` + самарі** — обов'язково останній крок.
