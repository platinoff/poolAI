# GSV Server — exe/bin «Galaxy StarWalker Vision»

Специфікація бінарного сервера проєкту GSV. Окремий Rust bin `gsv-server`, що віддає UI та реалізує бокси.

## Призначення

- **Bin/exe** «Galaxy StarWalker Vision» — `cargo run --bin gsv-server` (або зібраний `.exe` на Windows).
- Віддає static UI (наслідник `docs/vision/index.html`) + REST API боксів + події (SSE).
- Працює як **self-contained server**: доки + метрики + бокs — все в одному Rust бінарнику.

## Endpoints (план)

| Метод | Шлях | Опис |
|-------|------|------|
| GET | `/` | GSV UI (index.html) |
| GET | `/docs/gsv/…` | docs проєкту |
| GET | `/api/tracker` | параметри виконаного workflow (Tracker box) |
| GET | `/api/sli` | SLI-каталог (команди + функції) |
| GET | `/api/toolchain` | інвентар тулів |
| GET | `/api/ide/sessions` | список сесій (opencode/cursor) |
| POST | `/api/ide/select` | вибір сесії, з чим працювати |
| GET | `/api/update` | статус оновлення (Update box) |
| GET | `/api/preview` | превʼю з Rust-синтаксис-кольорами |
| POST | `/api/terminal` | SLI terminal — виконати команду (AI) |
| GET | `/api/hooks/tests` | результати тестів (read-only, без build) |
| GET | `/api/hooks/bench` | Criterion medians (read-only) |
| GET | `/api/omni` | OmniRouter overview (providers, models, recommended, routing) |
| GET | `/api/omni/config` | OmniRouter конфіг (redacted: лише `key_set`) |
| POST | `/api/omni/config` | тюнінг провайдерів (base_url/api_key/enabled/priority/routing) |
| GET | `/api/omni/v1/models` | OpenAI-сумісний список моделей |
| POST | `/api/omni/v1/chat/completions` | OpenAI-сумісний proxy (dry-run через `X-Omni-Dry-Run: 1`) |
| POST | `/api/omni/test` | connectivity check провайдера (`GET {base}/models`) |
| GET | `/api/health` | health-чек |
| GET | `/events` | SSE: update · offline/online · metrics resync |

## Update-повідомлення (Update box)

Ключова вимога: **якщо запущено bin-версію, сервер приймає повідомлення про апдейт.**

Сценарій (з ТЗ):
1. Йде перекомпіляція vision Rust-кодбази на **новий бінарник**.
2. Замість «reload» у UI з’являється **«Update»**.
3. Вебсторінка **не падає** при офлайн — переходить у стан «offline».
4. Після відновлення зв’язку **всі метрики синхронізуються** (resync).

Реалізація (Rust):
- Сервер тримає `update_flag` (AtomicBool) + версію бінарника.
- Під час заміни бінарника (hot-swap файлу/процесу) сервер надсилає SSE подію `update_available`.
- UI показує кнопку/бейдж **Update** замість auto-reload.
- Клієнтський JS тримає стан offline в `navigator.onLine` / heartbeat SSE; при реконекті робить `GET /api/...` full-resync та оновлює метрики (Tracker/SLI/toolchain/speed/rust diagnostics).

## Offline-стійкість

- Static assets (UI) кешуються у Service Worker / localStorage → сторінка відкривається офлайн.
- Жодних повних reload-ів без потреби: зміна даних → SSE подія → частковий re-render.
- Якщо сервер недоступний: UI показує статус «offline», дані лишаються на екрані, при реконекті — resync.

## Залежності (план)

`tokio`, `axum` (або `rocket`), `serde`, `serde_json`, `tracing`, `tower-http` (static). Все — Rust, у `GSV/Cargo.toml`.

## Тести (Rust)

- `tests/gsv_server_contracts.rs` — API-контракти (HTTP/4xx/JSON), Rust-інтеграційні тести.
- `tests/gsv_omni_contracts.rs` — контракти OmniRouter (catalog, redacted config, dry-run proxy, v1/models).
- `tests/gsv_update_flow.rs` — сценарій update/offline/resync (state machine + SSE).
- Playwright — лише для браузерного UI (DOM), не для API-дублювання.

## Хук оновлення (без перекомпіляції — Tests/bench box)

Окремо в [`GSV_BOXES.md`](./GSV_BOXES.md): сервер читає `target/…/deps` результати тестів/бенчмарків **без перекомпіляції** (read-only запуск `cargo test` / `criterion` через `/api/hooks`).
