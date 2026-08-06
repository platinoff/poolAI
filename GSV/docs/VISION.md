# GSV — Vision box (poolAI vision canon mirror)

Дзеркало poolAI vision canon (`docs/vision/manifest.json` + `docs/vision/feed.json` +
`docs/vision/extensions.json`) у Rust-бокс GSV. Band 112 (PH-S1759…S1768, ✅).

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

## Ratio-safe політика

`vision.js` (161 KB) / `vision.css` (50 KB) legacy не переносяться у `GSV/ui/` — це знищило б
canon Rust 95–100%. UI — тонкий glue: Vision card (summary + ticker), **Vision Map card**
(per-layer chips + edge kinds + посилання на `vision.svg`), **Sprint Map card**
(sprint-queue modules/kinds/links), **Doc Preview card** (node + 1-hop neighbors,
input `galaxy_grid`), **Vision Sync card** (Resync snapshot button + drift status) та
**Sprint Queue card** (next/active/open + planned details) у `ui/index.html`.
`.svg` у ratio-аудиті Ignored — порт діаграми ratio-neutral.
Див. [`GSV_MIGRATION.md`](../../docs/gsv/GSV_MIGRATION.md).
