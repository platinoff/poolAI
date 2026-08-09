# GSV — Legacy vision app parity (docs/vision)

Inventory of the legacy Galaxy vision app (`docs/vision/index.html` + `docs/vision/vision.js` +
`docs/vision/vision.css`, 12.2 + 157.3 + 49.7 KB) against the GSV Rust-first UI. Bands 115–116
(PH-S1789…S1808, ✅) — supersession; **band 117 (PH-S1809…S1818, ✅) — legacy deactivation**:
`docs/vision/index.html` → GSV pointer page; `vision.js`/`vision.css` → DEACTIVATED banner
(kept as canon archive, not loaded). **band 119 (PH-S1829…S1838, ✅) — Galaxy UI full parity**
(colors + box behaviors: full `:root` palette wire, starfield/galaxy backdrop, header chrome,
panel dock/fullscreen).

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
| Speeds panel (`speed_index.json`) | speed_index.json | `GET /api/vision/speeds` + Speed Index card (band 115); `GET /api/vision/speeds.svg` + Speed history chart (band 116) | ✅ migrated (PH-S1790) |
| Rust panel (`rust_diagnostics.json`) | rust_diagnostics.json | `GET /api/vision/rust-diagnostics` + Rust Diagnostics card (band 115); `GET /api/vision/rust-diagnostics.svg` + Rust history chart (band 116) | ✅ migrated (PH-S1791) |
| Links panel (related edges = node 1-hop neighbors) | manifest edges | `GET /api/vision/doc-preview?id=` + Doc Preview card | ✅ superseded |
| Preview panel (doc preview) | manifest nodes/edges | `GET /api/vision/doc-preview?id=` + Doc Preview card | ✅ covered |
| Explorer sidebar file-tree | manifest paths | `GET /api/vision/node-search?q=` (case-insensitive id/label/path) | ✅ superseded |
| Eco/FX/Ms GPU mode + starfield canvas | — (cosmetic) | `GET /api/vision/starfield.svg?mode=eco\|fx\|ms` + GPU mode button (cycle) + `body.vision-(eco\|fx\|ms)` | ✅ migrated (PH-S1831) |
| Galaxy backdrop (nebula) | — (cosmetic) | `GET /api/vision/galaxy.svg` + `.galaxy-backdrop` `<img>` | ✅ migrated (PH-S1832) |
| Full legacy `:root` palette | `vision.css` `:root` | `GET /api/vision/palette` (`GalaxyPalette` wire) + `loadGalaxyPalette` CSS-змінні | ✅ migrated (PH-S1830) |
| RSS ticker (header) | feed.json | `#rssTicker` (`/api/vision/feed?status=all`, ≤30 items) | ✅ migrated (PH-S1833) |
| Power menu (shutdown/reboot/soft/hard Vision) | `POST /api/v1/ops/power` (poolAI) | `#btnPower` menu: Soft sync Vision → `/api/vision/sync`, Reload UI → resync, Force offline | ✅ superseded (PH-S1833) |
| Skip links / panel collapse / fullscreen / dock | — (layout chrome) | `–` collapse → panel dock chips (restore on click); `□` fullscreen + `Esc` exits | ✅ migrated (PH-S1834) |

## Підсумок

- **Мігровано в Rust (bands 115–119):** Speeds + Rust diagnostics (115), Rust-rendered
  SVG history charts (116), sprint UI theme + focus map (118), Galaxy UI full parity (119):
  повний legacy `:root` palette wire (`/api/vision/palette`), starfield/galaxy backdrop
  SVG (Rust-rendered), header chrome (RSS ticker, GPU mode cycle, power menu), panel dock +
  Esc-fullscreen. Усі legacy панелі тепер мають GSV еквівалент (endpoint + compact card).
- **Superseded:** auto-reload/queue/links/file-tree покриваються наявними `GET /api/vision/*` +
  компактними картками (bands 109–114); power menu → `#btnPower` (soft sync/reload/offline).
- **Out-of-scope:** — (band 119 закрив останні прогалини; `vision.css` `:root` = Rust wire,
  `vision.js`/`vision.css` не переносяться).

**Band 117 (PH-S1809…S1818, ✅):** legacy deactivated — `docs/vision/index.html` переписаний у
GSV pointer page; `vision.js`/`vision.css` отримали DEACTIVATED banner. Файли не видаляємо —
`docs/vision/` лишається канон-джерелом (manifest/feed/extensions/speed_index/rust_diagnostics/
vision.svg); живий UI — GSV (`gsv-server` → `http://127.0.0.1:8891/`).

**Band 119 (PH-S1829…S1838, ✅):** Galaxy UI full parity — повна legacy-палітра (`/api/vision/palette`),
starfield + galaxy backdrop (Rust SVG), header chrome (RSS ticker / GPU mode / power menu), panel
dock + Esc-fullscreen — закрито останні legacy-прогалини без переносу `vision.js`/`vision.css`.
