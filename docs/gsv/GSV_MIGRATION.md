# GSV Migration — що мігруємо з PoolAI у GSV

План міграції vision-системи в окремий проєкт GSV.

## Що переносимо (з `docs/vision/`)

| Джерело | У GSV | Статус |
|---------|-------|--------|
| `index.html` (Galaxy UI) | `GSV/ui/index.html` (UI glue поверх Rust) | **✅ (single-page UI: health/tracker/sli/toolchain/ide/update/preview/terminal/hooks, SSE)** |
| `manifest.json` (граф) | `GSV/data/gsv_manifest.json` (генерується Rust) | **✅ (band 109: `boxes/vision`, `gsv-vision-sync`, `GET /api/vision/manifest`; band 110: `GET /api/vision/map`)** |
| `feed.json` (RSS ticker) | `GSV/data/gsv_feed.json` | **✅ (band 109: `GET /api/vision/feed`, Vision UI card; band 110: `?status=` filter)** |
| `extensions.json` (active_sprint + planning scopes) | `GSV/data/gsv_extensions.json` | **✅ (band 112: `Extensions` mirror — read/save/load, `GET /api/vision/extensions`, `wire_sync` auto-sync; `sprint-queue` planning `GET /api/vision/sprint-queue`)** |
| `vision.js` / `vision.css` | `GSV/ui/` (тонкий UI glue) | ⏳ future (ratio-safe: не переносимо 161 KB legacy JS; canon Rust 95–100%) |
| `vision.svg` | `GSV/ui/vision.svg` + `GET /assets/vision.svg` | **✅ (band 110: порт isometric diagram; `.svg` = audit Ignored, ratio-neutral)** |
| README (відкриття) | → `docs/gsv/` | **✅ (адаптовано)** |

## Що переносимо (з `src/`)

| Джерело | У GSV | Статус |
|---------|-------|--------|
| `poolai-vision-sync` (граф/drift gate) | `gsv-vision-sync` bin (`GSV/src/bin/gsv_vision_sync.rs`) | **✅ (band 109: `--check` drift gate, mirror manifest/feed у `GSV/data/`; band 112: mirror + `extensions.json` → `gsv_extensions.json`)** |
| Vision UI-логіка (map, sprint-queue, doc-preview) | `gsv_server` + `GSV/ui/` | **✅ (band 109: Vision card = summary + feed ticker; band 110: Vision Map card = layer chips + edge kinds + svg link + `GET /api/vision/map`; band 111: Sprint Map card + Doc Preview card → `GET /api/vision/sprint-map` + `GET /api/vision/doc-preview?id=`; band 112: Vision Sync card + Sprint Queue card → `GET /api/vision/sync` + `GET /api/vision/sprint-queue`; band 113: inline SVG map + layer filter + node search → `GET /api/vision/node-search?q=&layer=` + search → doc-preview deep-link; band 114: Sprint Board card + Sprint Progress card → `GET /api/vision/sprint-board` + `GET /api/vision/sprint-progress`)** |
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
