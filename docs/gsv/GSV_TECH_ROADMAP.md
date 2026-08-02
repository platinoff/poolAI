# GSV TechPreroadMap — Galaxy StarWalker Vision

**TechPreroadMap**: логічний порядок реалізації проєкту GSV → future sprints.

Дата: 2026-08-01 · **Стан:** planned · **Реалізація:** band 102 (`PH-S1659…S1668`, FM §5.12).

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
