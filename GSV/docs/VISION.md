# GSV — Vision box (poolAI vision canon mirror)

Дзеркало poolAI vision canon (`docs/vision/manifest.json` + `docs/vision/feed.json` +
`docs/vision/extensions.json`) у Rust-бокс GSV. Bands 109–116 (PH-S1729…S1808, ✅);
**band 117 (PH-S1809…S1818, ✅)** — legacy vision deactivation: `docs/vision/index.html` →
GSV pointer page; `vision.js`/`vision.css` → DEACTIVATED banner (архів, не видаляємо);
живий UI — `gsv-server` → `http://127.0.0.1:8891/`. **band 118 (PH-S1819…S1828, ✅)** —
sprint UI migration: sprint theme wire (legacy `--sprint` palette) + Rust-rendered
sprint focus SVG. **band 119 (PH-S1829…S1838, ✅)** — Galaxy UI full parity: full legacy
`:root` palette wire (`GET /api/vision/palette`), Rust-rendered starfield/galaxy backdrop
SVG, header chrome (RSS ticker, GPU mode cycle, power menu), panel dock + Esc-fullscreen.
**band 120 (PH-S1839…S1848, ✅)** — Ratio 96% stretch: `gsv-loc-audit --stretch-96`
advisory (**96.51%** ≥96% ✅) + `GET /api/ui/card/{name}` Rust-rendered card fragments
(`boxes/ui.rs`: `esc`/`tab`/`bar` + 12 renderers) + `ui/index.html` thin glue (`getText` → `rustCards`).
**band 121 (PH-S1849…S1855, ✅)** — OmniRouter box parity: останній hand-rolled JS renderer
`renderOmni` → Rust `boxes/ui.rs` `render_omni` (`GET /api/ui/card/omni`, `CARD_NAMES` 13) —
ratio **96.73%** ≥96% ✅; всі 13 карток server-rendered, `renderOmni` JS видалено.

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
  - `GET /api/vision/sprint-board` — **band 114**: sprint-board report
    (`total`/`open_count`/`closed_count`/`progress_pct`, `next_sprint`/`last_sprint_closed`/
    `active_sprint`, `columns` open/closed/planned з `SprintQueueEntry`) →
    `SprintBoardReport`/`SprintBoardColumn`.
  - `GET /api/vision/sprint-progress` — **band 114**: sprint progress report
    (status counts `open`/`closed`/`planned` + `progress_pct` + per-layer розподіл
    `layers[]` з `node_count`/`linked_count` проти чергових спринтів, z-ascending) →
    `SprintProgressReport`/`SprintLayerProgress`.
  - `GET /api/vision/speeds` — **band 115**: speed-index report
    (`speed_index.json` → latest test-CI + bench + history counts) →
    `SpeedIndexReport`/`SpeedIndexLatest`; empty-tolerant (`ok:true`, `present:false`).
  - `GET /api/vision/rust-diagnostics` — **band 115**: rust clippy diagnostics report
    (`rust_diagnostics.json` → latest warnings/errors/top_codes + history count) →
    `RustDiagnosticsReport`/`RustDiagLatest`; empty-tolerant.
  - `GET /api/vision/speeds.svg` — **band 116**: Rust-rendered SVG chart (test-CI wall-clock
    bars, green = ok / red = fail, ≤24 runs, footer latest bench) з `test_ci_history`;
    empty-state svg (`ok:true`, `present:false`) коли масив порожній.
  - `GET /api/vision/rust-diagnostics.svg` — **band 116**: Rust-rendered SVG chart
    (warnings orange + errors red grouped bars, ≤24 runs, command footer) з `history`;
    empty-state svg коли масив порожній.
  - `GET /api/vision/sprint-theme` — **band 118**: sprint UI theme wire
    (`revision`/`git_head`/`active_sprint`/`next_sprint`, `sprint` `#a78bfa`/`sprint_next`
    `#c4b5fd`, `pill`/`chip`/`queue` colors, `layers` L0–L5, `edge_kinds`) →
    `SprintThemeReport`/`SprintPillTheme`/`SprintChipTheme`/`SprintQueueStateTheme`.
  - `GET /api/vision/sprint-focus.svg?sprint=` — **band 118**: Rust-rendered sprint focus
    map (sprint nodes highlighted, out-of-scope `opacity 0.22`/text `0.28`, edges tinted);
    default = active sprint, empty-state svg коли жоден node не посилається на спринт.
  - `GET /api/vision/palette` — **band 119**: повний legacy Galaxy palette wire
    (`GalaxyPalette`: bg-deep/bg/panel/panel-solid/border/border-bright/text/muted/accent/
    accent-2/glow/sidebar-w, `layers`+`layers_dim` L0–L5, `edge_docs`/`edge_code`/`edge_toml`,
    `ext_md`/`ext_rs`/`ext_json`/`ext_toml`, `sprint`, `bg_tone`, `galaxy_bg_opacity`;
    exact legacy `:root` values) + `ok`/`revision` context.
  - `GET /api/vision/starfield.svg?mode=eco|fx|ms` — **band 119**: Rust-rendered starfield
    backdrop (deterministic LCG seeded by mode; eco sparse/static, fx dense+glow, ms medium;
    `image/svg+xml`, `Cache-Control: no-cache`).
  - `GET /api/vision/galaxy.svg` — **band 119**: Rust-rendered galaxy backdrop
    (radial nebula gradients + spiral-arm hints from the legacy `:root` palette;
    `image/svg+xml`, `Cache-Control: no-cache`).
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

## Sprint-board + progress UI (band 114)

`SprintBoardReport` (band 114) будує доску зі спільного `planned` queue (band 112):

- `columns` — три фіксовані групи: `open` (активний спринт або `status == "open"`),
  `closed` (`closed`/`done`), `planned` (решту статусів).
- `progress_pct` = `closed_count * 100 / total`; `total` = розмір working queue.

`SprintProgressReport` (band 114) — додає per-layer розподіл:

- `layers[]` — з `manifest.layers`, z-ascending, з `node_count` (nodes шару) та
  `linked_count` (nodes, що посилаються на спринт з черги).
- Status counts (`open`/`closed`/`planned`) — над `planned` queue, сума = `total`.

UI: **Sprint Board card** (progress bar + open/closed/planned колонки-details) та
**Sprint Progress card** (progress bar + per-layer таблиця nodes/linked) у `ui/index.html`.

## Speeds + Rust diagnostics wire (band 115)

`SpeedIndexReport` (band 115) — mirror `docs/vision/speed_index.json`:

- `latest` (`SpeedIndexLatest`): `test_ci_wall_secs`/`test_ci_ok`/`test_ci_recorded_at`/
  `test_ci_command` + `last_bench_label`/`last_bench_median_ns`/`last_bench_recorded_at`.
- `test_ci_count`/`bench_count` — довжини історичних масивів.
- Mirror у `GSV/data/gsv_speed_index.json` (sync best-effort); `source_speed_index` =
  live → snapshot → empty default. `wire_speed_index` → `{ok, present, speed_index}`.

`RustDiagnosticsReport` (band 115) — mirror `docs/vision/rust_diagnostics.json`:

- `latest` (`RustDiagLatest`): `warnings`/`errors`/`ok`/`recorded_at`/`command`/`top_codes`.
- `history_count` — довжина історичного масиву.
- Mirror у `GSV/data/gsv_rust_diagnostics.json`; `source_rust_diagnostics` = live →
  snapshot → empty default. `wire_rust_diagnostics` → `{ok, present, rust_diagnostics}`.

UI: **Speed Index card** (test-ci wall time + bench median + rows) та
**Rust Diagnostics card** (warnings/errors/clean + top clippy codes) у `ui/index.html`.

## Speeds + Rust history charts (band 116)

`speed_index_chart_svg` (band 116) — Rust-rendered SVG з `docs/vision/speed_index.json`
`test_ci_history[]`:

- Вертикальні bars тест-CI wall-clock (останні ≤24 записи), **green** = ok / **red** = fail,
  день-мітки MM-DD (`svg_day_label`), footer = latest bench label + median ns.
- Порожній масив → `svg_empty` (title + hint) — картка не падає.
- `#[serde]` типи `SpeedTestCiRecord`/`SpeedBenchRecord`; `read_speed_index` проносить
  `test_ci_history`/`bench_history` через wire (source fallback unchanged).

`rust_diagnostics_chart_svg` (band 116) — Rust-rendered SVG з `docs/vision/rust_diagnostics.json`
`history[]`:

- Групові bars **warnings** (orange) + **errors** (red) на останніх ≤24 записах,
  день-мітки, footer = command (≤48 chars).
- Порожній масив → `svg_empty`. Тип `RustDiagRecord`; `read_rust_diagnostics` проносить
  `history` через wire.

UI: `<img>` per card у `ui/index.html` — `i-speed-chart` → `/api/vision/speeds.svg` та
`i-rust-chart` → `/api/vision/rust-diagnostics.svg` (`image/svg+xml`,
`Cache-Control: no-cache`). Charts — Rust-рендерені (ratio-safe, zero UI JS).

## Sprint UI theme + focus map (band 118)

`sprint_theme_report` (band 118) — sprint UI theme wire з живого manifest + extensions:

- `sprint` `#a78bfa` / `sprint_next` `#c4b5fd` (legacy `--sprint`/`--sprint-next`), `pill`
  (bg/border/color), `chip`, `queue` (`open_border`/`open_bg`/`open_status`/`next_border`/
  `next_glow`/`closed_opacity`) — кольори sprint-pill/queue-state з deactivated `vision.css`.
- `layers[]` L0–L5 per-layer fill (legacy `--L0…--L5`), `edge_kinds[]` per-kind
  (docs/code/toml; невідомі → accent blue `#7eb8ff`).
- `wire_sprint_theme` → `{ok, revision, git_head, active_sprint, next_sprint, sprint,
  sprint_next, pill, chip, queue, layers, edge_kinds}`. Missing source → `{ok:false, error}`.

`sprint_focus_svg` (band 118) — Rust-rendered sprint focus map (legacy `sprint-dim`):

- Scope: nodes чиї `sprints[]` token-matches спринт (`sprint_token_matches` — exact id або
  `PH-S*` glob) або path у matching extension scope (`path_matches_glob` — `**` crossing,
  `*` non-`/`; scope `docs`/`code_globs`/`also_update`).
- Layout: один рядок на layer (L0..L5 z-sorted), in-scope nodes на повній opacity з sprint
  accent, out-of-scope — `circle 0.22` / `text 0.28`; edges touching in-scope tinted.
- `?sprint=` default = active sprint (extensions) → next sprint (manifest).
- Empty state: title `Sprint focus: <id>` + hint «no galaxy nodes reference this sprint yet».
- `GET /api/vision/sprint-focus.svg` повертає `image/svg+xml`, `Cache-Control: no-cache`.

UI: **Sprint Focus card** у `ui/index.html` (input sprint id + «Show focus» button +
`<img id="i-sprint-focus">`); theme wire (`loadSprintTheme`) застосовує CSS-змінні
`--sprint*` на `documentElement`; sprint-pill/queue chips у Sprint Queue + Sprint Board cards.

## Galaxy UI full parity: palette + backdrop + header chrome (band 119)

`GalaxyPalette` (band 119) — повний legacy `:root` palette (`vision.css`), wired verbatim:

- Кольори фону/панелей/тексту (`bg_deep` `#06080f`, `bg` `#0a0e18`, `panel`, `panel_solid`,
  `border`, `border_bright`, `text` `#eef2ff`, `muted` `#8b9cb8`), акценти (`accent` `#7eb8ff`,
  `accent_2` `#c4a5ff`, `glow`), `sidebar_w` `272px`.
- `layers` + `layers_dim` — L0–L5 base/dim per-layer colors; `edge_docs`/`edge_code`/
  `edge_toml`; `ext_md`/`ext_rs`/`ext_json`/`ext_toml`; `sprint` (`#a78bfa`); `bg_tone` `0.8`;
  `galaxy_bg_opacity` `0.15`.
- `wire_palette` → `{ok, revision, ...palette}`; missing source manifest → `revision: 0`.
- UI `loadGalaxyPalette()` застосовує `--*` CSS-змінні на `documentElement` (exact legacy
  look; **без** переносу `vision.css` — ratio-safe).

`starfield_svg(mode)` (band 119) — Rust-rendered backdrop:

- Deterministic LCG (seed per mode) → `star_count` stars (`eco` sparse/static, `fx`
  dense + glow halos, `ms` medium), fill `rgba(200, 220, 255, …)`.
- `GET /api/vision/starfield.svg?mode=` — `image/svg+xml`, `Cache-Control: no-cache`.
- UI `<img id="starfield">` + GPU button (Eco/FX/Ms cycle → `body.vision-(eco|fx|ms)`).
- Empty manifest → `svg_empty` (card не падає).

`galaxy_svg()` (band 119) — Rust-rendered nebula backdrop:

- Radial gradients `g1`/`g2` (accent/`accent_2`/bg-tone stops) + spiral-arm ellipses;
  `GET /api/vision/galaxy.svg` — `image/svg+xml`, `Cache-Control: no-cache`.
- UI `.galaxy-backdrop` `<img>` (fixed, `pointer-events:none`, opacity
  `var(--galaxy-bg-opacity)`).

Header chrome (band 119):

- **RSS ticker** (`#rssTicker`): label + viewport + duplicated track from
  `/api/vision/feed?status=all` (≤30 items, open/closed class).
- **GPU mode button** (`#btnGpu`): Eco → FX → Ms cycle → re-requests
  `/api/vision/starfield.svg?mode=` + `body` class.
- **Power menu** (`#btnPower`): Soft sync Vision → `GET /api/vision/sync` + resync,
  Reload UI → resync, Force offline → `setOffline(true)`.
- **Panel dock + Esc-fullscreen**: card `–` collapse → dock chips (restore on click);
  `□` fullscreen toggle + `Esc` exits (clears `.card.fullscreen`,
  `body.panel-fs-active`). All JS = thin glue (ratio-safe).

## Ratio-safe політика

`vision.js` (161 KB) / `vision.css` (50 KB) legacy не переносяться у `GSV/ui/` — це знищило б
canon Rust 95–100%. UI — тонкий glue: Vision card (summary + ticker), **Vision Map card**
(inline SVG + per-layer chips + edge kinds + node search), **Sprint Map card**
(sprint-queue modules/kinds/links), **Doc Preview card** (node + 1-hop neighbors,
input `galaxy_grid`), **Vision Sync card** (Resync snapshot button + drift status),
**Sprint Queue card** (next/active/open + planned details), **Sprint Board card**
(progress bar + open/closed/planned колонки) та **Sprint Progress card** (progress bar +
per-layer nodes/linked таблиця) у `ui/index.html`. Band 116 додає `<img>` charts:
**Speed Index card** — `speeds.svg` (test-ci wall bars) та **Rust Diagnostics card** —
`rust-diagnostics.svg` (warnings/errors bars); SVG рендерить Rust (`vision.rs`), не JS.
Band 118 додає **Sprint Focus card** (`sprint-focus.svg` + sprint id input) та sprint
theme wire (CSS-змінні `--sprint*` + sprint-pill/queue chips у Sprint Queue/Board cards).
`.svg` у ratio-аудиті Ignored — порт діаграми ratio-neutral.
Band 119 додає **Galaxy UI full parity**: `GET /api/vision/palette` (повний legacy `:root`
wire + `loadGalaxyPalette` CSS-змінні), Rust-rendered starfield + galaxy backdrop
(`/api/vision/starfield.svg?mode=` + `/api/vision/galaxy.svg`), header chrome (RSS ticker,
GPU mode cycle, power menu) та panel dock + Esc-fullscreen у `ui/index.html` — всі фічі
реалізовані Rust wire + компактним UI glue, без переносу legacy `vision.js`/`vision.css`.
Band 120 додає **server-rendered UI fragments**: `GET /api/ui/card/{name}` — Rust HTML
renderers (`boxes/ui.rs`: `esc`/`tab`/`bar` + `render_card` + `CARD_NAMES`), 12 карток
(tracker/sli/toolchain/ratio/hooks-tests/hooks-bench/sprint-map/sprint-queue/sprint-progress/
sprint-board/speed-index/rust-diagnostics) → `ui/index.html` thin glue `getText` → `rustCards`;
8 JS renderers видалено. Це підняло rust ratio **95.18% → 96.51%** (stretch-96 advisory ✅).
Band 121 додає **OmniRouter card** (`GET /api/ui/card/omni`): `render_omni` у `boxes/ui.rs`
(summary/routing + recommended + providers table + models table + `format_number` grouping),
`server/mod.rs` `"omni"` arm → `boxes::omni::wire`, `CARD_NAMES` 13; `renderOmni` JS видалено,
`rustCards` 13 → rust ratio **96.73%** (stretch-96 advisory ✅). Тепер усі 13 UI-карток —
Rust-rendered (`boxes/ui.rs`), жодного JS-рендера карток не залишилось.
Див. [`GSV_MIGRATION.md`](../../docs/gsv/GSV_MIGRATION.md) і [`LEGACY_PARITY.md`](./LEGACY_PARITY.md).
