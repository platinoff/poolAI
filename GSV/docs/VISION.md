# GSV — Vision box (poolAI vision canon mirror)

Дзеркало poolAI vision canon (`docs/vision/manifest.json` + `docs/vision/feed.json`) у Rust-бокс
GSV. Band 109 (PH-S1729…S1738, ✅).

## Що це

- **`GSV/src/boxes/vision.rs`** — serde-структури (`Manifest`, `ManifestNode`, `ManifestEdge`,
  `Layer`, `Feed`, `FeedItem`) + read/save/load/wire.
- **Джерело:** `docs/vision/manifest.json` (galaxy graph: layers, nodes, edges) та
  `docs/vision/feed.json` (sprint ticker) на корені poolAI.
- **Знімки:** `GSV/data/gsv_manifest.json` + `gsv_feed.json` (генеруються Rust, gitignored).
- **API:**
  - `GET /api/vision` — summary (revision, git_head, nodes/edges counts, next/last sprint, feed ticker).
  - `GET /api/vision/manifest` — повний граф (nodes/edges/layers).
  - `GET /api/vision/feed` — повний RSS-тикер.

## Синхронізація

```text
cargo run --bin gsv-vision-sync                # mirror у GSV/data/
cargo run --bin gsv-vision-sync -- --check     # drift gate (source parse + revision parity)
```

`--check` повертає exit 0, коли: source manifest+feed парсяться, revision > 0, і persisted
`gsv_manifest.json` revision збігається з source.

## Ratio-safe політика

`vision.js` (161 KB) / `vision.css` (50 KB) legacy не переносяться у `GSV/ui/` — це знищило б
canon Rust 95–100%. UI — тонкий glue: Vision card (summary + ticker) у `ui/index.html`.
Див. [`GSV_MIGRATION.md`](../../docs/gsv/GSV_MIGRATION.md).
