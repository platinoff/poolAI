# GSV — Legacy vision app parity (docs/vision)

Inventory of the legacy Galaxy vision app (`docs/vision/index.html` + `docs/vision/vision.js` +
`docs/vision/vision.css`, 12.2 + 157.3 + 49.7 KB) against the GSV Rust-first UI. Band 115
(PH-S1789…S1798, ⏳).

## Принцип (ratio-safe)

Legacy `vision.js`/`vision.css` **не переносимо** у `GSV/ui/` (канон: Rust 95–100% / wasm 0–5%).
Функціонал legacy-панелей забезпечується Rust wire (`GET /api/vision/*`) + компактними
UI-картками `GSV/ui/index.html`. Документ — джерело істини для закриття рядка
`vision.js`/`vision.css` у `GSV_MIGRATION.md` (band 115).

## Панелі / фічі legacy-додатку

| Legacy (docs/vision) | Джерело даних | GSV еквівалент | Статус |
|----------------------|---------------|----------------|--------|
| Header `meta-rev`/`meta-trail` (revision, git HEAD, next sprint) | manifest.json | `GET /api/vision` summary + Vision card | ✅ covered |
| RSS ticker (feed) | feed.json | `GET /api/vision/feed` + Vision card ticker | ✅ covered |
| Auto-reload / Reload | manifest.json + feed.json | `GET /api/vision/sync` Resync (Vision Sync card) | ✅ superseded |
| Layers panel (3D layer-stack) | manifest.layers | `GET /api/vision/map` + Vision Map card (L0..L5 z-sorted SVG) | ✅ covered |
| Sprint queue panel (FM §5.12, eye filter) | manifest.sprint_queue | `GET /api/vision/sprint-queue` + Sprint Queue card; `GET /api/vision/sprint-board` + Sprint Board card | ✅ superseded |
| Galaxy map (interactive SVG) | manifest nodes/edges | `GET /api/vision/map` + inline `assets/vision.svg` + layer chips; `GET /api/vision/node-search` | ✅ covered |
| Speeds panel (`speed_index.json`) | speed_index.json | `GET /api/vision/speeds` + Speed Index card (band 115) | ➡️ migrated (PH-S1790) |
| Rust panel (`rust_diagnostics.json`) | rust_diagnostics.json | `GET /api/vision/rust-diagnostics` + Rust Diagnostics card (band 115) | ➡️ migrated (PH-S1791) |
| Links panel (related edges = node 1-hop neighbors) | manifest edges | `GET /api/vision/doc-preview?id=` + Doc Preview card | ✅ superseded |
| Preview panel (doc preview) | manifest nodes/edges | `GET /api/vision/doc-preview?id=` + Doc Preview card | ✅ covered |
| Explorer sidebar file-tree | manifest paths | `GET /api/vision/node-search?q=` (case-insensitive id/label/path) | ✅ superseded |
| Eco/FX/Ms GPU mode + starfield canvas | — (cosmetic) | — | out-of-scope (app chrome, non-vision) |
| Power menu (shutdown/reboot/soft/hard Vision) | `POST /api/v1/ops/power` (poolAI) | — | out-of-scope (poolAI ops, не GSV) |
| Skip links / panel collapse / fullscreen / dock | — (layout chrome) | — | out-of-scope (compact layout) |

## Підсумок

- **Мігровано в Rust (band 115):** Speeds + Rust diagnostics — єдині функціональні прогалини.
- **Superseded:** auto-reload/queue/links/file-tree покриваються наявними `GET /api/vision/*` +
  компактними картками (bands 109–114).
- **Out-of-scope:** GPU FX / power menu / layout chrome — не vision-канон.

Після закриття band 115 legacy `docs/vision/vision.js`/`vision.css` можуть бути позначені як
superseded (не видаляємо — `docs/vision/` лишається канон-джерелом до завершення GSV).
