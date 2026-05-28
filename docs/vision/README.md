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

**Шапка:** `rev N · PH-S*` (закритий спринт) · **git HEAD** pill · `→ PH-S*` (наступний з `manifest.next_sprint`).

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
| **⊟ Folders** (увімкнено за замовч.) | папки з **≥5** файлами згортаються в один hub; **клік** — розгорнути, **dblclick** — розгорнути + zoom |
| **◎ Sprint** | тьмяні вузли/ребра поза `active_sprint` з `extensions.json` |
| **Layers (3D stack)** | клік по шару L0–L5 — підсвітка tier на Galaxy map; інші тьмяні; повторний клік або Esc — скинути |
| **Вузол на map** | клік — повна назва у callout; pipeline-ребра до hub-файлів (`galaxy_grid`, `fm`, …) як сузір’я |
| **Constellation layout** | файли в дузі/спіралі (не ряд); криві ребра між зірками |
| **Galaxy wallpaper** | `vision2.png` у корені репо (`manifest.galaxy_background`, 15% opacity). **Не** `PoolAIGalaxy.png` — це схема шарів |
| Кластер + сітка | ≤4 файли на шар — ряд; більше — сітка 2–3 колонки |
| pan/zoom | навігація по великій карті |

Налаштування зберігаються в `localStorage` (`poolai-vision-map-prefs`).

**Після кожного спринту:** `revision++`, нові `nodes`/`edges`, `extensions.json` → `active_sprint`. Див. `docs-vision.mdc`.
