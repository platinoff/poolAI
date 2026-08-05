# GSV Migration — що мігруємо з PoolAI у GSV

План міграції vision-системи в окремий проєкт GSV.

## Що переносимо (з `docs/vision/`)

| Джерело | У GSV | Статус |
|---------|-------|--------|
| `index.html` (Galaxy UI) | `GSV/ui/index.html` (UI glue поверх Rust) | **✅ (single-page UI: health/tracker/sli/toolchain/ide/update/preview/terminal/hooks, SSE)** |
| `manifest.json` (граф) | `GSV/data/gsv_manifest.json` (генерується Rust) | **✅ (band 109: `boxes/vision`, `gsv-vision-sync`, `GET /api/vision/manifest`)** |
| `feed.json` (RSS ticker) | `GSV/data/gsv_feed.json` | **✅ (band 109: `GET /api/vision/feed`, Vision UI card)** |
| `vision.js` / `vision.css` | `GSV/ui/` (тонкий UI glue) | ⏳ future (ratio-safe: не переносимо 161 KB legacy JS; canon Rust 95–100%) |
| `vision.svg` | `GSV/ui/vision.svg` | ⏳ future (можливо окремим спринтом) |
| README (відкриття) | → `docs/gsv/` | **✅ (адаптовано)** |

## Що переносимо (з `src/`)

| Джерело | У GSV | Статус |
|---------|-------|--------|
| `poolai-vision-sync` (граф/drift gate) | `gsv-vision-sync` bin (`GSV/src/bin/gsv_vision_sync.rs`) | **✅ (band 109: `--check` drift gate, mirror manifest/feed у `GSV/data/`)** |
| Vision UI-логіка (map, sprint-queue, doc-preview) | `gsv_server` + `GSV/ui/` | ⏳ future (band 109: Vision card = summary + feed ticker) |
| `crates/poolai-ui-core` / `poolai-ui-wasm` | за потреби (Rust-first; wasm 0–5%) | ⏳ future |

## Що лишається в PoolAI

- `docs/vision/` — **не видаляємо** (канон-джерело, діє до завершення GSV).
- `src/bin/poolai_vision_sync.rs` — лишається для поточного vision; після міграції — за бажанням.
- FM §5.12, HANDOFF, NEXT — лишаються в PoolAI (репозиторій один).

## Як мігруємо (принцип)

1. **Документація** (ця сесія): `docs/gsv/` створено, GSV-архітектура канонізована.
2. **Сервер scaffold** (PH-S1660): `GSV/Cargo.toml` + `gsv_server.rs` — мінімальний server + static UI.
3. **Бокси** (PH-S1662…S1667): поступово, кожен — окремий спринт.
4. **Дублювання vs заміна**: GSV-код пишеться в `GSV/`; `docs/vision/` лишається джерелом, поки GSV не покриє функціонал (vision parity check в band close).

## Критерії готовності (vision parity)

- GSV-UI відкривається через `gsv-server` на `http://127.0.0.1:PORT`.
- Tracker показує параметри останнього workflow.
- SLI console показує команди + пропоновані SLI-функції з `bin/`+`scripts/`.
- Update flow: «Update» замість reload; offline-стійкість; metrics resync — працює.
- Rust 95–100% / wasm 0–5% дотримано (loc-audit).
