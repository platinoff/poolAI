# Передача контексту новій сесії (GSV)

**Оновлено:** 2026-08-08 (band 120 **PH-S1839…S1848** ✅ · ratio **96.51%** · tests **204** · clippy **0**)

**Наступна сесія:** **`абракадабра`** → S0 диск/git → project scan (warnings first) → drain ≤10 PH-S*
→ Speeds + Rust panel → vision-sync → **один commit** → **`git push` + самарі**. Канон:
[`GSV_ROLES.md`](GSV_ROLES.md).

## Стан зараз

- **GSV** — окремий Rust-first проєкт (`GSV/`), bands 102 · 108 · 109 · 110 · 111 · 112 · 113 · 114 · 115 · 116 · 117 · 118 · 119 · 120 **✅**.
- **Ratio:** `cargo run --bin gsv-loc-audit -- --stretch-96` → **96.51%** (rust 10027 / product 10390, gate ≥95% ✅, stretch-96 ≥96% ✅) → `GSV/data/rust_ratio.json`.
- **Тести:** `cargo test` → **204** green · **clippy 0** · **fmt clean**.
- **Сервер:** порт 8870 (8891 транзитивно зарезервований Windows dynamic exclusion; canon порт **8891**).
- **FM:** band 120 = §5.101 (PH-S1839…S1848 ✅). Master horizon poolAI: band 121.
- **Vision rev:** 472 (band 119 vision-sync close). Vision box: `boxes/vision.rs` + `gsv-vision-sync` bin +
  `GET /api/vision*`; snapshot `GSV/data/gsv_manifest.json` + `gsv_feed.json` + `gsv_extensions.json` (rev 472).
  Band 110: `GET /api/vision/map`, `GET /assets/vision.svg`, `GET /api/vision/feed?status=`, Vision Map card.
  Band 111: `GET /api/vision/sprint-map` (sprint-scope/queue/session-tracks links + modules + kinds) та
  `GET /api/vision/doc-preview?id=` (node + 1-hop neighbors) — Sprint Map + Doc Preview UI cards.
  Band 112: `GET /api/vision/sync` (auto-sync + drift), `GET /api/vision/extensions` (extension mirror:
  active_sprint + scopes), `GET /api/vision/sprint-queue` (entries ∪ active plan) — Vision Sync + Sprint Queue UI cards.
  Band 113: `GET /api/vision/node-search?q=&layer=` (node search, top-N 25, layer-z-sorted) —
  Vision Map card inline SVG + layer filter + search → doc-preview deep-link.
  Band 114: `GET /api/vision/sprint-board` (open/closed/planned columns + progress pct) та
  `GET /api/vision/sprint-progress` (status counts + per-layer nodes/linked distribution) —
  Sprint Board + Sprint Progress UI cards.
  Band 115: `GET /api/vision/speeds` (SpeedIndexReport: latest test-CI + bench + history counts,
  mirror `gsv_speed_index.json`, empty-tolerant) та `GET /api/vision/rust-diagnostics`
  (RustDiagnosticsReport: latest warnings/errors/top_codes + history count, mirror
  `gsv_rust_diagnostics.json`, empty-tolerant) — Speed Index + Rust Diagnostics UI cards.
  Band 116: `GET /api/vision/speeds.svg` (Speed history chart — Rust-rendered SVG: test-CI
  wall bars green ok / red fail, ≤24 runs, footer latest bench) та
  `GET /api/vision/rust-diagnostics.svg` (Rust history chart — warnings orange + errors red
  grouped bars, command footer); `<img>` charts у Speed Index + Rust Diagnostics cards.
  Band 118: `GET /api/vision/sprint-theme` (sprint UI theme wire: `#a78bfa`/`#c4b5fd`,
  pill/chip/queue colors, layer L0–L5 + edge-kind palettes) та
  `GET /api/vision/sprint-focus.svg?sprint=` (Rust-rendered sprint focus map: in-scope accent,
  out-of-scope dim 0.22/0.28, default active sprint) — Sprint Focus card + sprint-pill/queue
  chips у Sprint Queue/Board cards.
  Band 119: `GET /api/vision/palette` (повний legacy `:root` palette wire: bg-deep/bg/panel/
  panel-solid/border/border-bright/text/muted/accent/accent-2/glow/sidebar-w, layers+layers_dim
  L0–L5, edge-docs/code/toml, ext-md/rs/json/toml, sprint, bg-tone, galaxy-bg-opacity) +
  `GET /api/vision/starfield.svg?mode=eco|fx|ms` (Rust-rendered starfield: deterministic LCG,
  eco sparse/fx glow/ms medium) + `GET /api/vision/galaxy.svg` (Rust-rendered nebula backdrop) —
  Galaxy UI full parity: `loadGalaxyPalette` CSS-змінні, RSS ticker, GPU mode button
  (Eco/FX/Ms cycle), power menu (soft sync / reload / force offline), panel dock +
  Esc-fullscreen.
  Legacy parity: [`LEGACY_PARITY.md`](LEGACY_PARITY.md) — всі legacy-панелі закриті
  (bands 115–119); `vision.js`/`vision.css` superseded (band 115); **band 117: legacy
  deactivated** — `docs/vision/index.html` = GSV pointer page, `vision.js`/`vision.css` =
  DEACTIVATED banner (архів, не завантажуються); живий UI — `gsv-server` →
  `http://127.0.0.1:8891/`.
  **band 118: sprint UI (theme + focus) migrated** — legacy sprint colors/`sprint-dim`
  recreated в Rust (`vision.rs`), не legacy JS. **band 119: Galaxy UI full parity
  (colors + box behaviors) migrated** — legacy `:root` palette = Rust wire, starfield/galaxy
  backdrop = Rust SVG, header chrome/dock/fullscreen = compact UI glue (не legacy JS/CSS).
  **band 120: Ratio 96% stretch** — `GET /api/ui/card/{name}` (Rust-rendered card body HTML:
  `boxes/ui.rs` `esc`/`tab`/`bar` + 12 renderers + `CARD_NAMES`); `ui/index.html` thin glue
  (`getText` → `rustCards`); `gsv-loc-audit --stretch-96` advisory (**96.51%** ≥96% ✅).
- **poolAI ratio:** **95.04%** (advisory hold, `--ratio96-docs-canon --advisory --min-ratio 0.95`).

## S0 (кожна сесія, disk/git first)

1. `df -h /s | tail -1` → `bash scripts/check_target_disk.sh` → `cargo clean` якщо <5G (12G дешево).
2. `git fetch` → `git status -sb` → `git log -1 --oneline`.
3. Прочитати цей HANDOFF + `NEXT_SESSION_PROMPT.md` + FM §5.12 §5.100.

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
