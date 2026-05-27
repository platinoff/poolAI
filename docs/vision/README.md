# PoolAI documentation vision

Ізометрична карта зв’язків **доків ↔ код ↔ спринти** для ітераційної розробки (VDT). Оновлюється разом із закриттям PH-S* / змінами FM §5.11.

## Файли

| Файл | Призначення |
|------|-------------|
| [`vision.svg`](./vision.svg) | Статична ізометрична схема шарів L0–L3 (concept → ops → catalog → code) |
| [`manifest.json`](./manifest.json) | Граф вузлів і ребер (machine-readable); джерело для HTML |
| [`extensions.json`](./extensions.json) | Актуальність розширень і scope спринту (що синхронізувати) |
| [`index.html`](./index.html) | **Sidebar explorer** + layer/map/links + preview; клік по файлу підсвічує шар і звʼязки |

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
4. Перевірити `index.html` → Reload manifest.

## Шари (канон)

| ID | Шар | Приклади |
|----|-----|----------|
| L0 | Concept | `POOLAI_GALAXY_GRID.md`, Memory, Grid Node |
| L1 | Operations | HANDOFF, NEXT_SESSION_PROMPT |
| L2 | Catalog | FUNCTION_MANAGEMENT, DIGEST |
| L3 | Code | `src/grid/`, virtual_nodes API |
