# Open docs vision (browser / Simple Browser)

Cursor **не** відкриває `S:/rust/poolAI/...` у Simple Browser (`Unable to resolve resource`). Використовуй **HTTP URL** або **відносний шлях**.

## Спосіб 1 — локальний сервер (рекомендовано)

PowerShell у корені репо:

```powershell
.\bin\open-docs-vision.ps1
```

У **Simple Browser** (`Ctrl+Shift+P` → *Simple Browser: Show*) вставте:

```text
http://127.0.0.1:8765/docs/vision/index.html
```

Не `S:/rust/...` — лише цей URL.

Сервер тримайте у терміналі; зупинка — `Ctrl+C`.

## Спосіб 2 — відкрити файл у редакторі, потім preview

1. У Explorer відкрийте `docs/vision/index.html` (подвійний клік — таб у редакторі).
2. `Ctrl+Shift+P` → **Simple Browser: Show**.
3. Якщо запитає URL — введіть відносно workspace:

```text
./docs/vision/index.html
```

або

```text
docs/vision/index.html
```

## Спосіб 3 — зовнішній браузер

```powershell
.\bin\open-docs-vision.ps1
```

Відкриється системний браузер на той самий URL.

## Спосіб 4 — лише SVG

Відкрийте в IDE: `docs/vision/vision.svg` (без сервера).

## Після оновлення manifest

У вікні vision натисніть **Reload manifest** або перезавантажте сторінку (`F5`).
