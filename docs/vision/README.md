# PoolAI documentation vision

Ізометрична карта зв’язків **доків ↔ код ↔ спринти** для ітераційної розробки (VDT). Оновлюється разом із закриттям PH-S* / змінами FM §5.11.

## Файли

| Файл | Призначення |
|------|-------------|
| [`vision.svg`](./vision.svg) | Статична ізометрична схема шарів L0–L3 (concept → ops → catalog → code) |
| [`manifest.json`](./manifest.json) | Граф вузлів і ребер (machine-readable); джерело для HTML |
| [`extensions.json`](./extensions.json) | Актуальність розширень і scope спринту (що синхронізувати) |
| [`index.html`](./index.html) | **4 панелі** у браузері: SVG, 3D-стек, sprint scope, preview doc |

## Як відкрити в Cursor / браузері

1. **Cursor Simple Browser:** `Ctrl+Shift+P` → *Simple Browser: Show* → шлях до файлу:
   `S:/rust/poolAI/docs/vision/index.html`
2. **Зовнішній браузер:** відкрити `index.html` подвійним кліком (file://). Кнопка **Reload manifest** підтягує свіжі `manifest.json` / `extensions.json` після коміту.
3. **Лише SVG:** `docs/vision/vision.svg` — вставка в README або перегляд у IDE.

Панель 4 намагається завантажити `.md` у iframe; якщо браузер блокує file:// — відкривайте doc через посилання в списку вузлів (Cursor file link).

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
