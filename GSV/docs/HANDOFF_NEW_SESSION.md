# Передача контексту новій сесії (GSV)

**Оновлено:** 2026-08-05 (band 113 **PH-S1769…S1778** ✅ · ratio **95.15%** · tests **122** · clippy **0**)

**Наступна сесія:** **`абракадабра`** → S0 диск/git → project scan (warnings first) → drain ≤10 PH-S*
→ Speeds + Rust panel → vision-sync → **один commit** → **`git push` + самарі**. Канон:
[`GSV_ROLES.md`](GSV_ROLES.md).

## Стан зараз

- **GSV** — окремий Rust-first проєкт (`GSV/`), bands 102 · 108 · 109 · 110 · 111 · 112 · 113 **✅**.
- **Ratio:** `cargo run --bin gsv-loc-audit` → **95.15%** (rust 6360 / product 6684, gate ≥95% ✅) → `GSV/data/rust_ratio.json`.
- **Тести:** `cargo test` → **122** green · **clippy 0** · **fmt clean**.
- **Сервер:** порт 8870 (8891 транзитивно зарезервований Windows dynamic exclusion; canon порт **8891**).
- **FM:** band 113 = §5.94 (PH-S1769…S1778 ✅). Master horizon poolAI: band 114.
- **Vision rev:** 465 (band 113 vision-sync). Vision box: `boxes/vision.rs` + `gsv-vision-sync` bin +
  `GET /api/vision*`; snapshot `GSV/data/gsv_manifest.json` + `gsv_feed.json` + `gsv_extensions.json` (rev 465).
  Band 110: `GET /api/vision/map`, `GET /assets/vision.svg`, `GET /api/vision/feed?status=`, Vision Map card.
  Band 111: `GET /api/vision/sprint-map` (sprint-scope/queue/session-tracks links + modules + kinds) та
  `GET /api/vision/doc-preview?id=` (node + 1-hop neighbors) — Sprint Map + Doc Preview UI cards.
  Band 112: `GET /api/vision/sync` (auto-sync + drift), `GET /api/vision/extensions` (extension mirror:
  active_sprint + scopes), `GET /api/vision/sprint-queue` (entries ∪ active plan) — Vision Sync + Sprint Queue UI cards.
  Band 113: `GET /api/vision/node-search?q=&layer=` (node search, top-N 25, layer-z-sorted) —
  Vision Map card inline SVG + layer filter + search → doc-preview deep-link.
- **poolAI ratio:** **95.02%** (advisory hold, `--ratio96-docs-canon --advisory --min-ratio 0.95`).

## S0 (кожна сесія, disk/git first)

1. `df -h /s | tail -1` → `bash scripts/check_target_disk.sh` → `cargo clean` якщо <5G (12G дешево).
2. `git fetch` → `git status -sb` → `git log -1 --oneline`.
3. Прочитати цей HANDOFF + `NEXT_SESSION_PROMPT.md` + FM §5.12 §5.94.

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
