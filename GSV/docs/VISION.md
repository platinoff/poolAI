# GSV — Vision box (poolAI vision canon mirror)

Дзеркало poolAI vision canon (`docs/vision/manifest.json` + `docs/vision/feed.json` +
`docs/vision/extensions.json`) у Rust-бокс GSV. Band 113 (PH-S1769…S1778, ✅).

## Що це

- **`GSV/src/boxes/vision.rs`** — serde-структури (`Manifest`, `ManifestNode`, `ManifestEdge`,
  `Layer`, `Feed`, `FeedItem`, `Extensions`) + read/save/load/wire.
- **Джерело:** `docs/vision/manifest.json` (galaxy graph: layers, nodes, edges),
  `docs/vision/feed.json` (sprint ticker) та `docs/vision/extensions.json`
  (extension manifest: `active_sprint`, `revision`, `ui_version`, planning `scopes`)
  на корені poolAI.
- **Знімки:** `GSV/data/gsv_manifest.json` + `gsv_feed.json` + `gsv_extensions.json`
  (генеруються Rust, gitignored).
- **API:**
  - `GET /api/vision` — summary (revision, git_head, nodes/edges counts, next/last sprint, feed ticker).
  - `GET /api/vision/manifest` — повний граф (nodes/edges/layers).
  - `GET /api/vision/map` — **band 110**: легкий map-звіт для UI (layers L0..L5 z-sorted з
    `node_count`/`edges_from`, `edge_kinds` tally, totals) — `MapReport`/`LayerStats`/`EdgeKindStats`.
  - `GET /api/vision/feed` — RSS-тикер; `?status=closed|open|all` фільтр (band 110).
  - `GET /api/vision/sprint-map` — **band 111**: sprint-queue map (scoping/tracking edges:
    `sprint-scope`, `queue`, `session-tracks`) → `SprintMapReport` (`links` з `NodeRef`,
    `modules` per-node targets tally, `kinds`/`layers` stats).
  - `GET /api/vision/doc-preview?id=<node>` — **band 111**: docs ↔ code preview для одного node
    (1-hop neighbors) → `DocPreviewReport` (`links_out`/`links_in`, `link_count`).
  - `GET /api/vision/sync` — **band 112**: auto-sync: повторний mirror canon у знімки +
    drift gate у відповіді (`drift: []` = зелено) → `wire_sync`.
  - `GET /api/vision/extensions` — **band 112**: extension manifest mirror (`active_sprint`,
    `revision`, `ui_version`, `scope_count` + sorted `scopes`) → `wire_extensions`.
  - `GET /api/vision/sprint-queue` — **band 112**: sprint-queue planning report
    (`next_sprint`/`last_sprint_closed`, `open_count`, `entries` з manifest.sprint_queue,
    `active_sprint` з extensions, `planned` = entries ∪ active) → `SprintQueueReport`.
  - `GET /api/vision/node-search?q=&layer=` — **band 113**: galaxy node search
    (case-insensitive match по id/label/path/sections, `top-N 25` layer-z-sorted,
    `links_out`/`links_in` tallies) → `NodeSearchReport`/`NodeSearchResult`.
  - `GET /assets/vision.svg` — **band 110**: порт `docs/vision/vision.svg` (isometric diagram,
    `image/svg+xml`, include_str! з `GSV/ui/vision.svg`).

## Синхронізація

```text
cargo run --bin gsv-vision-sync                # mirror у GSV/data/ (manifest+feed+extensions)
cargo run --bin gsv-vision-sync -- --check     # drift gate (source parse + revision parity)
```

`--check` повертає exit 0, коли: source manifest+feed+extensions парсяться, revision > 0, і
persisted `gsv_manifest.json` revision збігається з source.

## Sprint-queue планування

`SprintQueueReport` (band 112) об'єднує два джерела:

- `manifest.sprint_queue` → `entries` (`SprintQueueEntry`: id/title/summary/status/category),
  `open_count` = `sprint_queue_open_count`.
- `extensions.active_sprint` → `active_sprint`; `planned` = entries ∪ активний спринт
  (статус `open`, category `sprint`), коли він ще не в черзі.

## Node search + інтерактивна мапа (band 113)

`NodeSearchReport` (band 113):

- `query`/`layer` — відбиток запиту; `total_matches` — повна кількість збігів.
- `results` — `NodeSearchResult` (id/label/layer/path/sections + `links_out`/`links_in`),
  топ-25 (`NODE_SEARCH_LIMIT`), відсортовані за z шару, потім за id.
- Порожній `q` = браузинг шару (усі nodes шару).

UI (`ui/index.html`): Vision Map card тепер рендерить **inline** `assets/vision.svg` +
клікабельні layer chips (фільтр мапи/пошуку, active chip), node-search input + results table
→ deep-link у Doc Preview card (`openSearchNode`).

## Ratio-safe політика

`vision.js` (161 KB) / `vision.css` (50 KB) legacy не переносяться у `GSV/ui/` — це знищило б
canon Rust 95–100%. UI — тонкий glue: Vision card (summary + ticker), **Vision Map card**
(inline SVG + per-layer chips + edge kinds + node search), **Sprint Map card**
(sprint-queue modules/kinds/links), **Doc Preview card** (node + 1-hop neighbors,
input `galaxy_grid`), **Vision Sync card** (Resync snapshot button + drift status) та
**Sprint Queue card** (next/active/open + planned details) у `ui/index.html`.
`.svg` у ratio-аудиті Ignored — порт діаграми ratio-neutral.
Див. [`GSV_MIGRATION.md`](../../docs/gsv/GSV_MIGRATION.md).
