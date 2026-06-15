# PoolAI documentation vision

Ізометрична карта зв’язків **доків ↔ код ↔ спринти** для ітераційної розробки (VDT). Оновлюється разом із закриттям PH-S* / змінами FM §5.11.

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

**Auto-reload:** кнопка **Auto** у шапці; сервер `GET /docs/vision/__watch` — зміни `manifest.json` / `extensions.json` оновлюють граф без F5; **`git_head`** (короткий hash) оновлює cyan pill у шапці без F5; зміни `index.html` / `vision.css` / `vision.js` — повне перезавантаження сторінки.

**Auto-sync (інкрементальна карта):** `cargo run --bin poolai-vision-sync` або `GET /docs/vision/__sync` — додає нові/змінені git-tracked файли (`docs/`, `src/`, `e2e/`, …) у `manifest.json` з ребрами до hub-вузлів (`galaxy_grid`, `fm`, `handoff`, …). `open-docs-vision.ps1` викликає sync при старті; **Reload manifest** у UI — теж через `__sync`.

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
- Кнопки **+ / − / ⌂** — zoom in/out/reset.
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
| **Galaxy wallpaper** | `vision2.png` у корені репо (`manifest.galaxy_background`, 15% opacity). **Не** `PoolAIGalaxy.png` — це схема шарів |
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
| **PH-S192** ✅ | Overview LOD + minimap | `vision.js`, `vision.css`, `index.html` |
| **PH-S199** | `feed.json` RSS ticker | `docs/vision/` |
| **PH-S200** | Cursor post-push hook | `.cursor/hooks` |

Канон черги: [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12 · наступний спринт **PH-S193** (dashboard wasm formatters).

**Overview LOD (PH-S192):** при low zoom (scale ≤1.05 на dense map) — hub-only nodes/labels; **minimap** inset (правий нижній кут, click-to-pan).
