# GSV — Memory mark (what/why)

Стан проєкту **Galaxy StarWalker Vision** — окремого Rust-first проєкту в `GSV/` репо PoolAI.
Оновлюється в кінці кожного band. Лічильники — вимірювані (`wc -l`, `cargo test`,
`cargo run --bin gsv-loc-audit`), не з пам'яті.

## Стан (2026-08-05 · band 112 ✅)

- **Канон:** Rust **95–100%** / wasm **0–5%** (завжди), без Python/Java; bins — лише `src/bin/`.
- **Ratio (виміряно):** `cargo run --bin gsv-loc-audit` → **95.56%** (rust 6162 / product 6448) — gate ≥95% ✅.
  Звіт: `GSV/data/rust_ratio.json` (gitignored).
- **Тести (виміряно):** `cargo test` → **118** (58 unit + 8 `gsv_omni_contracts` + 7 `gsv_ratio_contracts`
  + 18 `gsv_server_contracts` + 8 `gsv_update_flow` + 19 `gsv_vision_contracts`).
  `cargo clippy --all-targets` → **0** warnings. `cargo fmt` clean.
- **Бокси:** Tracker · SLI console · Toolchain · IDE · Update/offline · Box preview · SLI terminal ·
  Tests/bench hooks · **Ratio** · **Vision** · **Vision Map** · **Sprint Map** · **Doc Preview** ·
  **Vision Sync** · **Sprint Queue** · **OmniRouter** (Rust AI-проксі/роутер).

## Що зроблено

### Band 102 (PH-S1659…S1668, ✅ 2026-08-01) — GSV migration
- `docs/gsv/` канон + `GSV/Cargo.toml` (окремий workspace, `.cargo/config.toml` → `target-dir`).
- `gsv-server` bin (axum + tokio, SSE `/events`, single-page UI `ui/index.html` embedded).
- Бокси: Tracker, SLI console, Toolchain, IDE, Update/offline, Box preview, SLI terminal, Tests/bench hooks.
- 52 tests green (на той момент), clippy 0. FM §5.12 §5.83 ✅.

### Band 108 (PH-S1719…S1728, ✅ 2026-08-05) — roles/ratio/roles canon (poolAI дисципліна)
- **PH-S1719** `GSV/docs/GSV_ROLES.md` — ролі VDT (Власник/Оркестратор/Субагенти) + канон сесії
  (S0 disk-first → project scan warnings-first → drain ≤10 PH-S* → Speeds + Rust panel → vision-sync
  → один commit → `git push` + самарі).
- **PH-S1720** `GSV/src/bin/gsv_loc_audit.rs` + `GSV/src/boxes/ratio.rs` — LOC ratio audit
  (дзеркало poolAI `poolai_loc_audit.rs`): `git ls-files --full-name`, `classify_product_path`,
  `--print/--no-write/--advisory/--min-ratio/--output/--data-dir`, gate ≥95%.
- **PH-S1721** `tests/gsv_ratio_contracts.rs` — 7 integration contracts (audit/save/load/wire/API).
- **PH-S1722** Ratio box + `GET /api/ratio` + UI Ratio card.
- **PH-S1723** `GSV/docs/MEMORY.md` (цей файл) + `GSV/docs/README.md` індекс.
- **PH-S1724** `GSV/docs/HANDOFF_NEW_SESSION.md` + `NEXT_SESSION_PROMPT.md`.
- **PH-S1725** FM §5.12 §5.89 band 108 + `docs/gsv/GSV_TECH_ROADMAP.md` band 108.
- **PH-S1726** poolAI docs parity (FUNCTIONALITY_DIGEST / vision README / GSV rows).
- **PH-S1727** poolAI `docs/development/HANDOFF_NEW_SESSION.md` + `NEXT_SESSION_PROMPT.md`.
- **PH-S1728** Band close: ratio hold, fmt, clippy, cargo test, docs canon, vision-sync, push.

### Band 109 (PH-S1729…S1738, ✅ 2026-08-05) — Vision box (poolAI vision canon mirror)
- **PH-S1729** `GSV/src/boxes/vision.rs` + `boxes/mod.rs` + Cargo `[[bin]]` — serde-структури
  (manifest nodes/edges/layers + feed) та реєстрація боксу.
- **PH-S1730** manifest wire: read `docs/vision/manifest.json` → `GSV/data/gsv_manifest.json`;
  `GET /api/vision/manifest` (nodes/edges/layers).
- **PH-S1731** feed wire: `docs/vision/feed.json` → `GSV/data/gsv_feed.json`; `GET /api/vision/feed`.
- **PH-S1732** `GSV/src/bin/gsv_vision_sync.rs` — mirror + `--check` drift gate (source parse +
  revision parity). Live: rev 458, 1218 nodes, 535 edges, 12 feed items.
- **PH-S1733** Vision UI card (`ui/index.html`): summary + sprint feed ticker; ratio-safe (без 161 KB legacy JS).
- **PH-S1734** `tests/gsv_vision_contracts.rs` — 7 integration contracts.
- **PH-S1735** `GSV/docs/VISION.md` + `GSV_MIGRATION.md` rows ✅ + MEMORY mark.
- **PH-S1736** poolAI vision parity (`docs/vision/README.md` + cross-check).
- **PH-S1737** Ratio hold advisory (`gsv-loc-audit --min-ratio 0.95 --advisory`).
- **PH-S1738** Band close: ratio hold, fmt, clippy, cargo test, docs canon, vision-sync, push.

### Band 110 (PH-S1739…S1748, ✅ 2026-08-05) — Vision map UI (svg + map wire)
- **PH-S1739** `boxes/vision.rs` `map_report`/`wire_map` — легкий map-звіт (layers L0..L5 z-sorted,
  `node_count`/`edges_from`, `edge_kinds` tally, totals); `GET /api/vision/map`.
- **PH-S1740** `GSV/ui/vision.svg` (порт `docs/vision/vision.svg`, include_str!) + `GET /assets/vision.svg`
  (`image/svg+xml`). `.svg` = audit Ignored → ratio-neutral.
- **PH-S1741** Vision Map card у `ui/index.html`: per-layer chips + edge kinds + посилання на svg.
- **PH-S1742** `tests/gsv_vision_contracts.rs` +3 → **10** (map endpoint: 6 layers z-sorted, layer_sum;
  feed `?status=closed`; `/assets/vision.svg` 200 + content-type).
- **PH-S1743** `GET /api/vision/feed?status=closed|open|all` — серверний фільтр.
- **PH-S1744** `GSV/docs/VISION.md` (map/feed-filter/svg) + `GSV_MIGRATION.md` rows ✅ (svg, map, feed filter).
- **PH-S1745** poolAI vision parity + `GSV_TECH_ROADMAP.md` band 110 row.
- **PH-S1746** Ratio hold advisory (`gsv-loc-audit --min-ratio 0.95 --advisory`).
- **PH-S1747** vision-sync close: `gsv-vision-sync` refresh + poolAI vision rev **461**.
- **PH-S1748** Band close: ratio hold, fmt, clippy, cargo test, docs canon, vision-sync, push.

### Band 111 (PH-S1749…S1758, ✅ 2026-08-05) — sprint-map + doc-preview (Vision UI логіка)
- **PH-S1749** `boxes/vision.rs` `SprintMapReport`/`SprintNode` structs + `sprint_map_report`/`wire_sprint_map` —
  sprint-queue map (scoping/tracking edges: `sprint-scope`+`queue`+`session-tracks` → links з `NodeRef`,
  per-node targets tally → `modules`, kinds/layers stats); `GET /api/vision/sprint-map`.
- **PH-S1750** `DocPreviewReport`/`LinkTarget` structs + `doc_preview`/`wire_doc_preview` — node + 1-hop
  neighbors (`links_out`/`links_in`); `GET /api/vision/doc-preview?id=<node>` (missing → `ok:false` + error).
- **PH-S1751** `tests/gsv_vision_contracts.rs` sprint-map contracts (endpoint kinds ⊆
  {sprint-scope,queue,session-tracks}, real-workspace report + module ids) → 12.
- **PH-S1752** doc-preview contracts (endpoint node/link_count, missing+empty params, real-workspace
  1-hop read) → **14**.
- **PH-S1753** Sprint Map card у `ui/index.html`: modules/kinds/links (details), rev/next/last header.
- **PH-S1754** Doc Preview card: node id input (`galaxy_grid` default) + out/in links + sections/path.
- **PH-S1755** `GSV/docs/VISION.md` (sprint-map/doc-preview API) + `MEMORY.md` band 111 + HANDOFF/NEXT_SESSION.
- **PH-S1756** poolAI parity: `docs/gsv/GSV_MIGRATION.md` row 21 ✅, `docs/vision/README.md`,
  `GSV_TECH_ROADMAP.md` band 111.
- **PH-S1757** Ratio hold advisory (`gsv-loc-audit --min-ratio 0.95 --advisory`) + poolAI parity hold.
- **PH-S1758** Band close: ratio hold, fmt, clippy, cargo test, docs canon, vision-sync, push.

### Band 112 (PH-S1759…S1768, ✅ 2026-08-05) — vision auto-sync + sprint-queue planning
- **PH-S1759** `boxes/vision.rs` `Extensions` struct (active_sprint/revision/ui_version/updated_at +
  opaque `scopes` map) + `read_extensions`/`save_extensions`/`load_extensions`/`source_extensions` +
  `extensions_source`/`extensions_target` paths; `sync()` також мірорить `gsv_extensions.json`;
  `SyncReport` + extensions_source/target; `gsv-vision-sync` bin друкує extensions target;
  `collect_drift` парсить extensions. `wire_extensions` → `GET /api/vision/extensions`
  (active_sprint, revision, ui_version, scope_count + sorted scopes).
- **PH-S1760** `wire_sync` → `GET /api/vision/sync` — auto-sync: re-mirror canon у знімки +
  drift gate у відповіді (`drift: []` = зелено); route у `server/mod.rs`.
- **PH-S1761** `SprintQueueReport`/`sprint_queue_report`/`wire_sprint_queue` → `GET /api/vision/sprint-queue` —
  manifest.sprint_queue → `entries`/`open_count`, extensions.active_sprint → `active_sprint`,
  `planned` = entries ∪ активний спринт.
- **PH-S1762** `tests/gsv_vision_contracts.rs` — extensions contracts (real-workspace read:
  revision>0, scopes present, active == manifest.next; sync snapshot now also asserts
  `gsv_extensions.json` + extensions revision parity) → 17.
- **PH-S1763** sprint-queue + sync contracts (`/api/vision/sync` ok + empty drift + synced_at;
  `/api/vision/sprint-queue` ok + active == next + planned includes active; real-workspace
  `sprint_queue_report`) → **19**.
- **PH-S1764** UI cards у `ui/index.html`: **Vision Sync card** (Resync snapshot button + drift status)
  та **Sprint Queue card** (rev/next/last + active/open + planned details).
- **PH-S1765** `GSV/docs/VISION.md` (sync/extensions/sprint-queue API + sync док-секції) +
  `MEMORY.md` band 112 + HANDOFF/NEXT_SESSION.
- **PH-S1766** poolAI parity: `docs/gsv/GSV_MIGRATION.md` rows ✅, `docs/vision/README.md`,
  `GSV_TECH_ROADMAP.md` band 112, FM §5.93, poolAI HANDOFF/NEXT.
- **PH-S1767** Ratio hold advisory (`gsv-loc-audit --min-ratio 0.95 --advisory` → 95.56%) +
  poolAI parity hold.
- **PH-S1768** Band close: ratio hold, fmt, clippy, cargo test (118), docs canon, vision-sync, push.

## Важливі факти (не забувати)

1. **GSV — окремий Rust-проєкт** у `S:\rust\poolAI\GSV` (own workspace, own `target/`).
2. **Ratio аудит іде по git-tracked файлах** репо poolAI під префіксом `GSV/` (не `GSV/target/`, не `data/`).
   git-топ має MSYS-стиль `/s/rust/poolAI` — нормалізуємо в `S:/rust/poolAI` (`normalize_git_root`).
3. **Запущений `gsv-server` блокує `target/debug/gsv-server.exe`** → `cargo test`/`build` падає
   з `Access is denied (os error 5)` → спочатку зупинити сервер.
4. **Data dir:** `GSV/data/*` gitignored (омні-конфіг, rust_ratio.json, трекер). Запуск:
   `--repo-root S:/rust/poolAI --data-dir S:/rust/poolAI/GSV/data --port 8891`.
5. **Збірка:** terminal MSYS2 bash; PATH префікс `C:\Users\plati\.cargo\bin`.
6. **OmniRouter** прокидає через OpenAI-сумісний proxy; dry-run заголовок `X-Omni-Dry-Run: 1` —
   жодного реального мережевого запиту в тестах.
7. **UI канон:** тонкий JS/DOM glue; якщо ratio падає <95% — **compact UI/CSS**, не Rust-обхід.
