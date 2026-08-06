# PoolAI documentation vision

Ізометрична карта зв’язків **доків ↔ код ↔ спринти** для ітераційної розробки (VDT). Оновлюється разом із закриттям PH-S* / змінами FM §5.11.

> **GSV (2026-08-05):** vision мігрує в окремий Rust-first проєкт **Galaxy StarWalker Vision** (`GSV/` + [`docs/gsv/`](../gsv/README.md)). Цей каталог — канон-джерело до завершення GSV; архітектура міграції: [`GSV_MIGRATION.md`](../gsv/GSV_MIGRATION.md) · TechPreroadMap: [`GSV_TECH_ROADMAP.md`](../gsv/GSV_TECH_ROADMAP.md) · ролі/пам'ять: [`GSV/docs/GSV_ROLES.md`](../../GSV/docs/GSV_ROLES.md) · [`GSV/docs/MEMORY.md`](../../GSV/docs/MEMORY.md) · Vision box: [`GSV/docs/VISION.md`](../../GSV/docs/VISION.md). Bands 102 + 108 + 109 + 110 + 111 + 112 + 113 + 114 ✅ (FM §5.12 §5.83 · §5.89 · §5.90 · §5.91 · §5.92 · §5.93 · §5.94 · §5.95). Band 109: `gsv-vision-sync` mirror manifest/feed → `GSV/data/gsv_*.json` + `GET /api/vision*`. Band 110: Vision map UI — `GET /api/vision/map` (layers L0..L5 + edge kinds), `GET /assets/vision.svg` (порт `vision.svg`), `GET /api/vision/feed?status=`. Band 111: sprint-map + doc-preview — `GET /api/vision/sprint-map` (sprint-scope/queue/session-tracks links + modules) та `GET /api/vision/doc-preview?id=` (node + 1-hop neighbors) + Sprint Map/Doc Preview UI cards. Band 112: vision auto-sync + sprint-queue planning — `GET /api/vision/sync` (re-mirror + drift), `GET /api/vision/extensions` (extension mirror: `active_sprint` + planning `scopes`), `GET /api/vision/sprint-queue` (manifest `sprint_queue` ∪ `active_sprint`) + Vision Sync/Sprint Queue UI cards. Band 113: node search + interactive map — `GET /api/vision/node-search?q=&layer=` (top-N 25, layer-z-sorted) + Vision Map card inline SVG + layer filter + search → doc-preview deep-link. Band 114: GSV Sprint-board + progress UI — `GET /api/vision/sprint-board` (open/closed/planned columns + `progress_pct`) та `GET /api/vision/sprint-progress` (status counts + per-layer `node_count`/`linked_count`) + Sprint Board/Sprint Progress UI cards.

## Файли

| Файл | Призначення |
|------|-------------|
| [`vision.svg`](./vision.svg) | Статична ізометрична схема шарів L0–L5 (concept → ops → catalog → code → libs → workspace) |
| [`manifest.json`](./manifest.json) | Граф вузлів і ребер (machine-readable); джерело для HTML |
| [`extensions.json`](./extensions.json) | Актуальність розширень і scope спринту (що синхронізувати) |
| [`index.html`](./index.html) | **Galaxy UI** — starfield, інтерактивна карта з `manifest.json`, radial link graph, 3D layers, preview, fullscreen панелей |
| [`vision.css`](./vision.css) · [`vision.js`](./vision.js) | Стилі та логіка карти (клік по вузлах, підсвітка ребер, scope спринту в Explorer, скролбари теми) |

## Як відкрити в Cursor / браузері

### Помилка `Unable to resolve resource S:/rust/poolAI/...`

Cursor Simple Browser **не** приймає абсолютний диск `S:/...`. Використовуйте **HTTP** або відносний шлях workspace.

### Рекомендовано (4 панелі + manifest)

PowerShell у корені репо:

```powershell
.\bin\open-docs-vision.ps1
```

Потім **Simple Browser** → URL:

```text
http://127.0.0.1:8765/docs/vision/index.html
```

Деталі: [`.cursor/commands/open-docs-vision.md`](../../.cursor/commands/open-docs-vision.md).

### Альтернативи

1. Відкрити `docs/vision/index.html` у табі редактора → Simple Browser → `./docs/vision/index.html`
2. **Зовнішній браузер** — той самий `open-docs-vision.ps1`
3. **Лише SVG** — `vision.svg` у IDE (без JS/manifest)

Панель 4 (doc preview) працює найкраще через **localhost** (сервер вище), не через `file://`.

## Ітераційне оновлення (агент)

Правило: [`.cursor/rules/docs-vision.mdc`](../../.cursor/rules/docs-vision.mdc)

Після закриття спринту PH-S* (або зміни `POOLAI_GALAXY_GRID.md` / HANDOFF / FM §5.11):

1. Оновити `manifest.json` (`revision++`, `updated_at`, `last_sprint_closed`, вузли/ребра).
2. Оновити `extensions.json` (`active_sprint`, `scopes`).
3. За потреби — підпис у `vision.svg` (footer: last sprint / next).
4. Перевірити `index.html` → **Auto** (кожні 1.5s) або Reload.

**Speeds panel:** latest `cargo test-ci` wall-clock + Criterion medians from [`../development/speed_index.json`](../development/speed_index.json) (mirror `speed_index.json` у цьому каталозі). Canon: [`../development/SPEED_INDEX.md`](../development/SPEED_INDEX.md) · `bash bin/record-test-ci-speed.sh`. Verified service **PH-SVC78** (2026-07-27) alongside Cursor desktop **3.13.21** — [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](../development/CURSOR_UPDATE_RESEARCH_2026-07-27.md).

**Rust panel:** Clippy warning/error counts from [`../development/rust_diagnostics.json`](../development/rust_diagnostics.json) (mirror `rust_diagnostics.json`). Canon: [`../development/RUST_DIAGNOSTICS.md`](../development/RUST_DIAGNOSTICS.md) · `bash bin/record-rust-diagnostics.sh` · CI job `rust-diagnostics` (artifact). Service **PH-SVC85**.

**Auto-reload:** кнопка **Auto** у шапці; сервер `GET /docs/vision/__watch` — зміни `manifest.json` / `extensions.json` / **`speed_index.json`** / **`rust_diagnostics.json`** оновлюють граф без F5; **`git_head`** (короткий hash) оновлює cyan pill у шапці без F5; зміни `index.html` / `vision.css` / `vision.js` — повне перезавантаження сторінки.

**Auto-sync (інкрементальна карта):** `cargo run --bin poolai-vision-sync` або `GET /docs/vision/__sync` — додає нові/змінені git-tracked файли (`docs/`, `src/`, `e2e/`, …) у `manifest.json` з ребрами до hub-вузлів (`galaxy_grid`, `fm`, `handoff`, …). **Drift gate:** `cargo run --bin poolai-vision-sync -- --check` (FM §5.12 vs manifest + **PH-S227** `.mdc` cross-links; CI job `vision-manifest-drift`). `open-docs-vision.ps1` викликає sync при старті; **Reload manifest** у UI — теж через `__sync`.

**Шапка:** `rev N · PH-S*` (закритий спринт) · **git HEAD** pill · `→ PH-S*` (наступний з `manifest.next_sprint`).

**Next-scope ring (UI rev 54+):** вузли з `sprints[]`, що містять `next_sprint`, — бірюзове кільце на карті + accent у дереві (разом з **◎ Sprint** dim/focus).

## Шари (канон)

| ID | Шар | Приклади |
|----|-----|----------|
| L0 | Concept | `POOLAI_GALAXY_GRID.md`, Memory, Grid Node |
| L1 | Operations | HANDOFF, NEXT_SESSION_PROMPT |
| L2 | Catalog | FUNCTION_MANAGEMENT, DIGEST |
| L3 | Code | `src/grid/`, virtual_nodes API |
| L4 | Lib roots | `src/lib.rs`, `crates/poolai-solana-adapter/` (PH-S120: events, sidecar, `poolai-events` BPF) |
| L5 | Workspace | `Cargo.toml`, `.cargo/config.toml` (найнижчий шар на карті) |

## Galaxy map (навігація)

- **Колесо миші** — zoom у межах SVG (не масштаб сторінки браузера).
- **Drag** по порожньому фону карти — pan.
- **Подвійний клік** на вузол — focus/zoom на об’єкт.
- Кнопки **+ / − / ⌂** — zoom in/out; **⌂** = fit all + resume auto-orbit.
- **▶ / ⏸** — auto-orbit (90% WASD) з auto fit-all zoom (PH-S579).
- Ребра: **зелений** docs, **рожевий** code, **бірюзовий** toml, **фіолетовий** mixed — ортогональний маршрут через «folder hub».
- **Щільні шари (L3+):** вузли групуються за папкою (`src/grid/`, `src/job/`, …) у міні-сітку 2–3 ряди, а не в одну лінію.

## Масштабування (великий репо)

Карта **росте ітераційно** разом із PH-S*: у `manifest.json` додаються лише вузли/ребра поточного scope (доки + код спринту), а не весь `src/` одразу.

| Механізм | Дія |
|----------|-----|
| **⊟ Folders** (увімкнено за замовч.) | папки з **≥3** файлами (≥2 при >120 вузлів на шар) згортаються в hub; **клік** — розгорнути |
| **Galaxy map filters** | **Layers** (L0–L5) + **Types** (md/rs/ts/…) chips — незалежні toggle на map; **All**/**None** для кожної групи; **Shift+layer** = solo; **Esc** = скинути map filters (окремо від 3D stack) |
| **Fullscreen map** | dock Layers/Types розгортається зверху; Sprint/Folders — знизу ліворуч |
| **◎ Sprint** | тьмяні вузли/ребра поза `active_sprint` з `extensions.json` |
| **Eco / FX / Ms** | кнопка у шапці циклічно **Eco→FX→Ms** — low GPU / glow / **1-hop hover trace**; `localStorage` `visionMode` |
| **Layers / Types** | **dropdown** меню (клік → chips + All/None); не перекриває карту (PH-S190 ✅) |
| **Panel collapse** | **−** у заголовку або біля zoom — панель → смуга з назвою; решта auto-fill (PH-S190 ✅) |
| **Клік по вузлу** | миттєва підсвітка (без повного re-render); preview файлу — async fetch |
| **Layers (3D stack)** | клік по шару L0–L5 — фокус tier у stack/legend (PH-S188: окремо від map filters); повторний клік або Esc — скинути stack focus |
| **Вузол на map** | клік — повна назва у callout; pipeline-ребра до hub-файлів (`galaxy_grid`, `fm`, …) як сузір’я |
| **Constellation layout** | файли в дузі/спіралі (не ряд); криві ребра між зірками |
| **Galaxy wallpaper** | `vision2.webp` у корені репо (`manifest.galaxy_background`, 15–17% opacity; PNG fallback). **Не** `PoolAIGalaxy.png` — це схема шарів |
| **Fit-all zoom** | дефолт і **⌂** — zoom щоб усі вузли вміщались; auto-orbit підтримує fit (PH-S579) |
| **Auto-orbit** | **▶/⏸** біля zoom — обертання Y на 90% швидкості held WASD; пауза при pan/zoom/WASD |
| Кластер + сітка | ≤4 файли на шар — ряд; більше — сітка 2–3 колонки |
| pan/zoom | навігація по великій карті |

Налаштування зберігаються в `localStorage` (`poolai-vision-map-prefs`).

**Після кожного спринту:** `revision++`, нові `nodes`/`edges`, `extensions.json` → `active_sprint`. Див. `docs-vision.mdc`.

## Черга Vision UX (FM §5.12)

| Sprint | Scope | Файли |
|--------|-------|-------|
| **PH-S188** ✅ | Map filters — independent layer/type toggles; **LAYERS**/**TYPES** All/None; decouple 3D stack ↔ chips | `vision.js`, `vision.css`, `index.html` |
| **PH-S190** ✅ | Filter **dropdowns** + panel **−** collapse strip | `vision.js`, `vision.css`, `index.html` |
| **PH-S191** ✅ | Sprint queue panel (Rust parse FM §5.12) | `poolai-vision-sync`, `manifest.json`, `vision.js` |
| **PH-SVC48** ✅ | Queue eye filter + prune closed ≤2000 | `vision.js` 👁 · `poolai-vision-sync` |
| **PH-S192** ✅ | Overview LOD + minimap | `vision.js`, `vision.css`, `index.html` |
| **PH-S193** ✅ | Dashboard wasm formatters | `poolai-ui-core`, `poolai-ui-wasm`, `src/ui/mod.rs` |
| **PH-S194** ✅ (vision UX rev 132) | Panel dock bar + map bottom bar | `vision.js`, `vision.css`, `index.html` v61 |
| **PH-S199** | `feed.json` RSS ticker | `docs/vision/` |
| **PH-S200** ✅ | `feed.json` RSS ticker panel | `poolai-vision-sync`, `feed.json`, `index.html` |
| **PH-S201** ✅ | Cursor post-push hook | `.cursor/hooks/post-push-ph-s-notify.sh` |
| **PH-S202** ✅ | Sprint queue → map focus | `vision.js` `focusSprintOnMap` |
| **PH-S203** ✅ | Keyboard nav linked nodes | `linkedMapNeighbors` + Arrow keys |
| **PH-S204** ✅ | Edge click neighbor select | `handleMapEdgeClick` + trace |
| **PH-S205** ✅ | Manifest drift gate | `poolai-vision-sync --check` |
| **PH-S227** ✅ | VDT `.mdc` cross-link drift | `poolai-vision-sync --check` |
| **PH-S206** ✅ | Minimap selection ring | `#minimap-selection-ring` + viewport fill |

Канон черги: [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12 · наступний спринт **PH-S195** (seed_inventory GET).

**Overview LOD (PH-S192):** при low zoom (scale ≤1.05 на dense map) — hub-only nodes/labels; **minimap** inset (правий нижній кут, click-to-pan).
