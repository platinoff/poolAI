# GSV TechPreroadMap — Galaxy StarWalker Vision

**TechPreroadMap**: логічний порядок реалізації проєкту GSV → future sprints.

Дата: 2026-08-05 · **Стан:** band 102 **реалізовано** + band 108 (roles/ratio canon) **✅** +
band 109 (Vision sync/migration) **✅** + band 110 (Vision map UI) **✅** + band 111 (Sprint map + doc-preview) **✅** +
band 112 (Vision auto-sync + sprint-queue planning) **✅** + band 113 (Node search + interactive map) **✅** +
band 114 (GSV Sprint-board + progress UI) **✅** · band 115 (GSV migration completion) **✅** ·
band 116 (GSV history charts — speed/rust analytics) **✅** · band 117 (GSV legacy vision deactivation) **✅** ·
**Спринти:** `PH-S1659…S1668` (FM §5.12 §5.83 ✅) · `PH-S1719…S1728` (FM §5.12 §5.89 ✅) ·
`PH-S1729…S1738` (FM §5.12 §5.90 ✅) · `PH-S1739…S1748` (FM §5.12 §5.91 ✅) ·
`PH-S1749…S1758` (FM §5.12 §5.92 ✅) · `PH-S1759…S1768` (FM §5.12 §5.93 ✅) ·
`PH-S1769…S1778` (FM §5.12 §5.94 ✅) · `PH-S1789…S1798` (FM §5.12 §5.96 ✅) ·
`PH-S1799…S1808` (FM §5.12 §5.97 ✅) · `PH-S1809…S1818` (FM §5.12 §5.98 ✅).

## Логічний порядок (залежності)

```
docs/architecture (✅ ця сесія)
  → server scaffold (bin + static UI)
      → Tracker (джерела даних workflow)
      → SLI console (каталог команд зі скриптів)
      → Toolchain (інвентар тулів)
      → IDE (opencode + cursor сесії)
      → Update/offline/resync (ключова механіка)
      → Box preview (Rust-синтаксис-кольори)
      → SLI terminal (AI → команди)
      → Tests/bench hooks (без перекомпіляції)
  → band close (docs canon, parity, vision-sync, ratio hold)
  → [band 108] roles/ratio canon (GSV як poolAI-grade проєкт):
      GSV_ROLES → gsv-loc-audit → ratio contracts → Ratio box/UI
      → memory mark → HANDOFF/NEXT → FM §5.89 → poolAI parity → band close
```

## Спринти (band 102)

| Sprint | Фокус | Acceptance (ключ) |
|--------|-------|-------------------|
| **PH-S1659** | GSV docs/architecture + Cargo scaffold | `docs/gsv/` канон; `GSV/Cargo.toml`; empty server builds |
| **PH-S1660** | gsv-server bin scaffold | `gsv_server.rs`; `GET /` → UI; `GET /api/health` |
| **PH-S1661** | Tracker box | `tracker/`; `GET /api/tracker`; `gsv_tracker.json`; параметри останнього workflow |
| **PH-S1662** | SLI console box | `sli/`; `GET /api/sli`; каталог з `bin/`+`scripts/`+`src/bin/`; використані команди |
| **PH-S1663** | Toolchain box | `toolchain/`; `GET /api/toolchain`; інвентар (rustc 1.92, clippy, MSYS2, …) |
| **PH-S1664** | IDE box | `ide/`; `GET /api/ide/sessions`; `POST /api/ide/select`; opencode + cursor чати |
| **PH-S1665** | Update box | `update/`; `/api/update`; SSE `update_available`; «Update» замість reload |
| **PH-S1666** | Box preview + SLI terminal | `preview/` Rust-кольори; `POST /api/terminal` (whitelist SLI) |
| **PH-S1667** | Tests/bench hooks (без перекомпіляції) | `hooks/`; `/api/hooks/tests`; `/api/hooks/bench`; read `target/` без build |
| **PH-S1668** | Band close | offline-стійкість + metrics resync; Rust tests; docs canon; vision parity; ratio hold |

## Спринти (band 108) — roles/ratio canon (poolAI дисципліна)

| Sprint | Фокус | Acceptance (ключ) |
|--------|-------|-------------------|
| **PH-S1719** | GSV roles canon | `GSV/docs/GSV_ROLES.md`; README pointer |
| **PH-S1720** | `gsv-loc-audit` bin | `GSV/src/bin/gsv_loc_audit.rs`; `--min-ratio/--advisory`; `GSV/data/rust_ratio.json` |
| **PH-S1721** | Ratio contracts | `tests/gsv_ratio_contracts.rs` (7) |
| **PH-S1722** | Ratio box + wire | `boxes/ratio.rs`; `GET /api/ratio`; UI Ratio card |
| **PH-S1723** | GSV memory mark | `GSV/docs/MEMORY.md` + `GSV/docs/README.md` |
| **PH-S1724** | GSV HANDOFF/NEXT | `GSV/docs/HANDOFF_NEW_SESSION.md` + `NEXT_SESSION_PROMPT.md` |
| **PH-S1725** | FM band 108 + roadmap | FM §5.12 §5.89; цей файл |
| **PH-S1726** | poolAI docs parity | GSV rows у poolAI docs |
| **PH-S1727** | poolAI HANDOFF + NEXT | band 108 ✅ · horizon band 109 |
| **PH-S1728** | Band close | ratio hold (≥95%); fmt/clippy/test; docs canon; vision-sync rev 458 |

## Спринти (band 109) — Vision box (poolAI vision canon mirror)

| Sprint | Фокус | Acceptance (ключ) |
|--------|-------|-------------------|
| **PH-S1729** | Vision box scaffold | `GSV/src/boxes/vision.rs` (manifest/feed serde) + `Cargo.toml` bin |
| **PH-S1730** | Manifest wire | `gsv_manifest.json`; `GET /api/vision/manifest` |
| **PH-S1731** | Feed wire | `gsv_feed.json`; `GET /api/vision/feed` |
| **PH-S1732** | `gsv-vision-sync` bin | mirror + `--check` drift gate |
| **PH-S1733** | Vision UI card | summary + sprint ticker |
| **PH-S1734** | Vision contracts | `tests/gsv_vision_contracts.rs` (7) |
| **PH-S1735** | GSV vision docs | `VISION.md` + `GSV_MIGRATION.md` rows ✅ + MEMORY mark |
| **PH-S1736** | poolAI vision parity | `docs/vision/README.md` + cross-check |
| **PH-S1737** | Ratio hold advisory | `gsv-loc-audit --min-ratio 0.95 --advisory` |
| **PH-S1738** | Band close | ratio hold (≥95%); fmt/clippy/test; docs canon; vision-sync rev 459 |

## Спринти (band 110) — Vision map UI

| Sprint | Фокус | Acceptance (ключ) |
|--------|-------|-------------------|
| **PH-S1739** | Vision map wire | `map_report`/`wire_map`; `GET /api/vision/map` (layers L0..L5 z-sorted + edge kinds) |
| **PH-S1740** | vision.svg port | `GSV/ui/vision.svg` + `GET /assets/vision.svg` (audit Ignored, ratio-neutral) |
| **PH-S1741** | Vision Map UI card | layer chips + edge kinds + svg link у `ui/index.html` |
| **PH-S1742** | Vision map contracts | `tests/gsv_vision_contracts.rs` (10) |
| **PH-S1743** | Feed status filter | `GET /api/vision/feed?status=closed\|open\|all` |
| **PH-S1744** | GSV vision docs | `VISION.md` map/feed-filter/svg; `GSV_MIGRATION.md` rows ✅; MEMORY band 110 |
| **PH-S1745** | poolAI vision parity | `docs/vision/README.md` band 110; roadmap band 110 |
| **PH-S1746** | Ratio hold advisory | `gsv-loc-audit --min-ratio 0.95 --advisory` |
| **PH-S1747** | vision-sync close | `gsv-vision-sync` refresh + poolAI vision rev **461** |
| **PH-S1748** | Band close | ratio hold (≥95%); fmt/clippy/test; docs canon; push |

## Спринти (band 111) — Sprint map + doc-preview (Vision UI логіка)

| Sprint | Фокус | Acceptance (ключ) |
|--------|-------|-------------------|
| **PH-S1749** | Sprint-map wire | `sprint_map_report`/`wire_sprint_map`; `GET /api/vision/sprint-map` (sprint-scope/queue/session-tracks links + modules + kinds) |
| **PH-S1750** | Doc-preview wire | `doc_preview`/`wire_doc_preview`; `GET /api/vision/doc-preview?id=` (node + 1-hop neighbors) |
| **PH-S1751** | Sprint-map contracts | `tests/gsv_vision_contracts.rs` (12) |
| **PH-S1752** | Doc-preview contracts | `tests/gsv_vision_contracts.rs` (**14**) |
| **PH-S1753** | Sprint Map UI card | modules/kinds/links у `ui/index.html` |
| **PH-S1754** | Doc Preview UI card | node id input + out/in links + sections у `ui/index.html` |
| **PH-S1755** | GSV vision docs | `VISION.md` sprint-map/doc-preview; MEMORY band 111; HANDOFF/NEXT band 111 |
| **PH-S1756** | poolAI vision parity | `GSV_MIGRATION.md` row 21 ✅; `docs/vision/README.md`; roadmap band 111 |
| **PH-S1757** | Ratio hold advisory | `gsv-loc-audit --min-ratio 0.95 --advisory`; poolAI parity hold |
| **PH-S1758** | Band close | ratio hold (≥95%); fmt/clippy/test; docs canon; vision-sync; push |

## Спринти (band 112) — Vision auto-sync + sprint-queue planning

| Sprint | Фокус | Acceptance (ключ) |
|--------|-------|-------------------|
| **PH-S1759** | Extensions mirror | `Extensions` struct + read/save/load/source; `gsv_extensions.json` snapshot; `wire_extensions` → `GET /api/vision/extensions`; `sync()`/`collect_drift`/bin include extensions |
| **PH-S1760** | Vision auto-sync wire | `wire_sync` → `GET /api/vision/sync` (re-mirror + drift gate) |
| **PH-S1761** | Sprint-queue planning wire | `SprintQueueReport`/`wire_sprint_queue` → `GET /api/vision/sprint-queue` (entries ∪ active) |
| **PH-S1762** | Extensions contracts | `tests/gsv_vision_contracts.rs` extensions (17) |
| **PH-S1763** | Sprint-queue contracts | sync + sprint-queue endpoints + real-workspace report (**19**) |
| **PH-S1764** | Vision Sync + Sprint Queue UI cards | Resync button + drift status; next/active/open + planned у `ui/index.html` |
| **PH-S1765** | GSV vision docs | `VISION.md` sync/extensions/sprint-queue; MEMORY band 112; HANDOFF/NEXT band 112 |
| **PH-S1766** | poolAI vision parity | `GSV_MIGRATION.md` rows ✅; `docs/vision/README.md`; roadmap band 112 |
| **PH-S1767** | Ratio hold advisory | `gsv-loc-audit --min-ratio 0.95 --advisory` (95.56%) |
| **PH-S1768** | Band close | ratio hold (≥95%); fmt/clippy/test (118); docs canon; vision-sync rev 463; push |

## Спринти (band 113) — Galaxy UI: node search + interactive map

| Sprint | Фокус | Acceptance (ключ) |
|--------|-------|-------------------|
| **PH-S1769** | Node search wire | `NodeSearchReport`/`node_search_report`/`wire_node_search` → `GET /api/vision/node-search?q=&layer=` (case-insensitive id/label/path/sections, top-N 25, layer-z-sorted, links_out/in tallies) |
| **PH-S1770** | Node search contracts | `tests/gsv_vision_contracts.rs` (real-workspace + layer filter + no-match empty/cap, **22**) |
| **PH-S1771** | Node-search endpoint contract | `tests/gsv_server_contracts.rs` (`/api/vision/node-search?q=` ok + results; empty q → ok true, **19**) |
| **PH-S1772** | Inline SVG map card | Vision Map card рендерить `assets/vision.svg` inline (`<img>`) + chips/kinds |
| **PH-S1773** | Layer filter + search UX | клікабельні layer chips (active filter) + node-search input + results → doc-preview deep-link у `ui/index.html` |
| **PH-S1774** | GSV vision docs | `VISION.md` node-search/map UX; MEMORY band 113; HANDOFF/NEXT band 113 |
| **PH-S1775** | poolAI vision parity | `GSV_MIGRATION.md` rows ✅; `docs/vision/README.md`; цей файл band 113 |
| **PH-S1776** | Ratio hold advisory | `gsv-loc-audit --min-ratio 0.95 --advisory` (≥95%) |
| **PH-S1777** | vision-sync close | `gsv-vision-sync` refresh + poolAI vision rev **465** |
| **PH-S1778** | Band close | ratio hold (≥95%); fmt/clippy/test (122); docs canon; vision-sync; push |

## Спринти (band 114) — GSV Sprint-board + progress UI

| Sprint | Фокус | Acceptance (ключ) |
|--------|-------|-------------------|
| **PH-S1779** | Sprint-board wire | `SprintBoardReport`/`SprintBoardColumn`/`sprint_board_report`/`wire_sprint_board` → `GET /api/vision/sprint-board` (open/closed/planned columns + counts + `progress_pct` = closed/total) |
| **PH-S1780** | Progress wire | `SprintProgressReport`/`SprintLayerProgress`/`sprint_progress_report`/`wire_sprint_progress` → `GET /api/vision/sprint-progress` (status counts + per-layer `node_count`/`linked_count`, z-ascending) |
| **PH-S1781** | Sprint-board contracts | `tests/gsv_vision_contracts.rs` (grouping, pct formula, column order, active-in-open, uniqueness, **30**) |
| **PH-S1782** | Progress contracts | sprint-progress contracts (layers match manifest, statuses sum, z-ordered, linked reflect queue, **38**) |
| **PH-S1783** | Endpoint contracts | `tests/gsv_server_contracts.rs` sprint-board + sprint-progress (ok + status sums + columns/layers shape, **21**) |
| **PH-S1784** | Sprint Board card | Sprint Board UI card у `ui/index.html`: progress bar + open/closed/planned колонки-details (`bar()` helper) |
| **PH-S1785** | Sprint Progress card | Sprint Progress UI card: progress bar + per-layer таблиця nodes/linked |
| **PH-S1786** | GSV vision docs | `VISION.md` sprint-board/sprint-progress API + band 114 section; MEMORY band 114; HANDOFF/NEXT band 114 |
| **PH-S1787** | poolAI vision parity | `GSV_MIGRATION.md` rows ✅; `docs/vision/README.md`; цей файл band 114; FM §5.95 |
| **PH-S1788** | Band close | ratio hold (**95.02%**); fmt/clippy/test (140); docs canon; vision-sync rev 467; push |

## Спринти (band 115) — GSV migration completion (legacy vision supersession)

| Sprint | Фокус | Acceptance (ключ) |
|--------|-------|-------------------|
| **PH-S1789** | Legacy parity audit | `GSV/docs/LEGACY_PARITY.md`: кожна legacy-панель (`docs/vision/index.html`) → GSV endpoint+card / superseded / out-of-scope |
| **PH-S1790** | Speeds wire | `SpeedIndexReport`/`SpeedIndexLatest`/`read_speed_index`/`save`/`load`/`source_speed_index`/`wire_speed_index` → `GET /api/vision/speeds` (empty-tolerant) |
| **PH-S1791** | Rust diagnostics wire | `RustDiagnosticsReport`/`RustDiagLatest`/`read_rust_diagnostics`/`save`/`load`/`wire_rust_diagnostics` → `GET /api/vision/rust-diagnostics` (empty-tolerant) |
| **PH-S1792** | Contracts | `gsv_vision_contracts.rs` (real-workspace speed_index/rust_diagnostics + wire shapes) + `gsv_server_contracts.rs` (`/speeds` + `/rust-diagnostics` 200/ok/shape) |
| **PH-S1793** | Speeds + Rust cards | Speed Index card + Rust Diagnostics card у `ui/index.html` (present/empty states, latest metrics, top clippy codes) |
| **PH-S1794** | GSV_MIGRATION rows + roadmap | `GSV_MIGRATION.md` rows ✅ (speed_index/rust_diagnostics/vision.js.css superseded); `GSV_TECH_ROADMAP.md` band 115 |
| **PH-S1795** | GSV vision docs canon | `VISION.md` +band 115 endpoints; MEMORY band 115; HANDOFF/NEXT band 115 |
| **PH-S1796** | poolAI vision parity | FM §5.12 §5.96; HANDOFF/NEXT band 115; `docs/vision/` canon |
| **PH-S1797** | Ratio hold advisory | `gsv-loc-audit` ≥95% (**95.04%**); legacy JS не переносимо (superseded) |
| **PH-S1798** | Band close | ratio hold; fmt/clippy/test (150); docs canon; vision-sync rev 468; push |

## Спринти (band 116) — GSV history charts (speed/rust analytics)

| Sprint | Фокус | Acceptance (ключ) |
|--------|-------|-------------------|
| **PH-S1799** | Scope + queue | FM §5.97 band 116 (PH-S1799…S1808) + manifest sync |
| **PH-S1800** | Speeds history wire | `SpeedTestCiRecord`/`SpeedBenchRecord` + `test_ci_history`/`bench_history` у `SpeedIndexReport`; `read_speed_index` carry arrays (source fallback unchanged) |
| **PH-S1801** | Rust diagnostics history wire | `RustDiagRecord` + `history` у `RustDiagnosticsReport`; `read_rust_diagnostics` carry history |
| **PH-S1802** | Contracts | vision tests 20 → **23**: typed parse, SVG bars + empty state (`data_dir_of` helper) |
| **PH-S1803** | Speed history chart UI | `speed_index_chart_svg` → `GET /api/vision/speeds.svg` (Rust-rendered SVG: test-CI wall bars green ok / red fail, ≤24 runs, footer latest bench) + `<img id="i-speed-chart">` |
| **PH-S1804** | Rust history chart UI | `rust_diagnostics_chart_svg` → `GET /api/vision/rust-diagnostics.svg` (warnings orange + errors red grouped bars, command footer) + `<img id="i-rust-chart">` |
| **PH-S1805** | Stand smoke + wasm defer | stand smoke: обидва SVG 200 `image/svg+xml`; `poolai-ui-wasm` defer row у `GSV_MIGRATION.md` + roadmap |
| **PH-S1806** | GSV vision docs canon | `VISION.md` +band 116 section/endpoints; MEMORY band 116; HANDOFF/NEXT band 116 |
| **PH-S1807** | poolAI vision parity | `docs/vision/README.md`; FM §5.12 §5.97; цей файл band 116; poolAI HANDOFF/NEXT band 116 |
| **PH-S1808** | Band close | ratio hold (**95.26%**); fmt/clippy/test (153); docs canon; vision-sync rev 469; push |

## Спринти (band 117) — GSV legacy vision deactivation

| Sprint | Фокус | Acceptance (ключ) |
|--------|-------|-------------------|
| **PH-S1809** | Scope + queue | FM §5.98 band 117 (PH-S1809…S1818) + manifest sync |
| **PH-S1810** | Legacy index deactivation | `docs/vision/index.html` → minimal GSV pointer page (no `vision.js`/`vision.css` refs) |
| **PH-S1811** | Legacy JS/CSS deactivation | DEACTIVATED banner у `vision.js`/`vision.css`; deactivation note у `docs/vision/README.md` |
| **PH-S1812** | Live link retarget | `poolai-vision-sync` feed links → `http://127.0.0.1:8891/#b-sprint-board`; GSV `vision.rs` sample links; RUN_LOCAL/GSV_SERVER/gsv README/SPEED_INDEX/RUST_DIAGNOSTICS → GSV |
| **PH-S1813** | Legacy test retirement | `poolai_vision_sync.rs` unit ×4 + `galaxy_horizon_s1011/s1019/s1039` → deactivated pointer state; e2e pointer assertions |
| **PH-S1814** | GSV parity docs | `LEGACY_PARITY.md` + `GSV_MIGRATION.md` band 117 |
| **PH-S1815** | GSV vision docs canon | `VISION.md`/`MEMORY.md`/HANDOFF/NEXT band 117 |
| **PH-S1816** | poolAI vision parity | `docs/vision/README.md`; FM §5.12 §5.98; цей файл band 117; poolAI HANDOFF/NEXT band 117 |
| **PH-S1817** | Ratio + rev prep | ratio hold advisory (**95.26%**); vision-sync rev 470 (poolai + gsv + --check) |
| **PH-S1818** | Band close | ratio hold; fmt/clippy/test (poolAI test-ci + GSV 153); docs canon; vision-sync rev 470; push |

## Спринти (band 118) — GSV sprint UI migration (theme + focus map)

| Sprint | Фокус | Acceptance (ключ) |
|--------|-------|-------------------|
| **PH-S1819** | Scope + queue | FM §5.99 band 118 (PH-S1819…S1828) + §5.12 header (master horizon) |
| **PH-S1820** | Sprint theme wire | `SprintThemeReport`/`SprintPillTheme`/`SprintChipTheme`/`SprintQueueStateTheme`/`SprintLayerColor`/`SprintEdgeKindColor` + `sprint_theme_report`/`wire_sprint_theme` → `GET /api/vision/sprint-theme` (sprint `#a78bfa`/next `#c4b5fd`, pill/chip/queue colors, layers L0–L5, edge kinds) |
| **PH-S1821** | Sprint focus SVG | `sprint_token_matches`/`path_matches_glob`/`nodes_for_sprint` + `sprint_focus_svg` → `GET /api/vision/sprint-focus.svg?sprint=` (sprint-dim: in-scope accent, out-of-scope opacity 0.22/text 0.28, edges tinted; default active sprint; empty-state) |
| **PH-S1822** | Contracts | `gsv_vision_contracts` (theme real-workspace + wire shapes + focus svg highlight/dim/empty) + `gsv_server_contracts` (theme + focus endpoints) |
| **PH-S1823** | UI sprint colors | `GSV/ui/index.html`: `--sprint*` CSS-змінні + sprint-pill/queue chips у Sprint Queue/Board cards; Sprint Focus card (`<img id="i-sprint-focus">`) + `loadSprintTheme`/`loadSprintFocus` |
| **PH-S1824** | GSV vision docs canon | `VISION.md` +band 118 (theme + focus endpoints/section); MEMORY band 118; GSV HANDOFF/NEXT band 118 |
| **PH-S1825** | poolAI vision parity | `docs/vision/README.md`; FM §5.12 §5.99; цей файл band 118 |
| **PH-S1826** | Ratio hold advisory | `gsv-loc-audit --min-ratio 0.95 --advisory` ≥95% (**95.35%**) + poolAI ratio96 advisory hold |
| **PH-S1827** | poolai-vision-sync close | `poolai-vision-sync` rev **471** (band 118); `--check` ok; sprint-queue/feed updated |
| **PH-S1828** | Band close | ratio hold; fmt/clippy/test (**163**); docs canon; vision-sync rev 471; push |

## Ключові UX-вимоги (узагальнення ТЗ)

1. Оновлюємо/дебажимо vision Rust-кодбазу, запущена **bin-версія** → сервер приймає **повідомлення про апдейт**.
2. Перекомпіляція на новий бінарник → у UI **«Update» замість reload**.
3. Вебсторінка **не падає** при офлайн — просто переходить в offline.
4. Після реконекту — **всі метрики синхронізуються** (resync).
5. Tracker показує технічні параметри воркфлоу, що виконувалось.
6. SLI console показує команди + усі SLI-функції з наявних скриптів (+ нові).
7. Toolchain показує, які тули використовуються.
8. IDE — портовані opencode + cursor чати; вибір, з чим працювати.
9. Box preview — Rust-кольори відповідно до синтаксису.
10. SLI terminal — щоб AI міг посилати команди.
11. Rust tests/benchmarks — хук **без перекомпіляції**.

## Посилання

- Бокси: [`GSV_BOXES.md`](./GSV_BOXES.md)
- Сервер: [`GSV_SERVER.md`](./GSV_SERVER.md)
- Архітектура: [`GSV_ARCHITECTURE.md`](./GSV_ARCHITECTURE.md)
- Міграція: [`GSV_MIGRATION.md`](./GSV_MIGRATION.md)
- FM §5.12 band 102: [`../catalog/FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md)
