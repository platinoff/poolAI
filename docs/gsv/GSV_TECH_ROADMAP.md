# GSV TechPreroadMap — Galaxy StarWalker Vision

**TechPreroadMap**: логічний порядок реалізації проєкту GSV → future sprints.

Дата: 2026-08-05 · **Стан:** band 102 **реалізовано** + band 108 (roles/ratio canon) **✅** ·
**Спринти:** `PH-S1659…S1668` (FM §5.12 §5.83 ✅) · `PH-S1719…S1728` (FM §5.12 §5.89 ✅).

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
