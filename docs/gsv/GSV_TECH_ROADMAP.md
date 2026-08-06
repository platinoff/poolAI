# GSV TechPreroadMap — Galaxy StarWalker Vision

**TechPreroadMap**: логічний порядок реалізації проєкту GSV → future sprints.

Дата: 2026-08-05 · **Стан:** band 102 **реалізовано** + band 108 (roles/ratio canon) **✅** +
band 109 (Vision sync/migration) **✅** + band 110 (Vision map UI) **✅** + band 111 (Sprint map + doc-preview) **✅** +
band 112 (Vision auto-sync + sprint-queue planning) **✅** ·
**Спринти:** `PH-S1659…S1668` (FM §5.12 §5.83 ✅) · `PH-S1719…S1728` (FM §5.12 §5.89 ✅) ·
`PH-S1729…S1738` (FM §5.12 §5.90 ✅) · `PH-S1739…S1748` (FM §5.12 §5.91 ✅) ·
`PH-S1749…S1758` (FM §5.12 §5.92 ✅) · `PH-S1759…S1768` (FM §5.12 §5.93 ✅).

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

## Спринти (band 109) — Vision box (poolAI vision canon mirror)

| Sprint | Фокус | Acceptance (ключ) |
|--------|-------|-------------------|
| **PH-S1729** | Vision box scaffold | `GSV/src/boxes/vision.rs` (manifest/feed serde) + `Cargo.toml` bin |
| **PH-S1730** | Manifest wire | `gsv_manifest.json`; `GET /api/vision/manifest` |
| **PH-S1731** | Feed wire | `gsv_feed.json`; `GET /api/vision/feed` |
| **PH-S1732** | `gsv-vision-sync` bin | mirror + `--check` drift gate |
| **PH-S1733** | Vision UI card | summary + sprint ticker |
| **PH-S1734** | Vision contracts | `tests/gsv_vision_contracts.rs` (7) |
| **PH-S1735** | GSV vision docs | `VISION.md` + `GSV_MIGRATION.md` rows ✅ + MEMORY mark |
| **PH-S1736** | poolAI vision parity | `docs/vision/README.md` + cross-check |
| **PH-S1737** | Ratio hold advisory | `gsv-loc-audit --min-ratio 0.95 --advisory` |
| **PH-S1738** | Band close | ratio hold (≥95%); fmt/clippy/test; docs canon; vision-sync rev 459 |

## Спринти (band 110) — Vision map UI

| Sprint | Фокус | Acceptance (ключ) |
|--------|-------|-------------------|
| **PH-S1739** | Vision map wire | `map_report`/`wire_map`; `GET /api/vision/map` (layers L0..L5 z-sorted + edge kinds) |
| **PH-S1740** | vision.svg port | `GSV/ui/vision.svg` + `GET /assets/vision.svg` (audit Ignored, ratio-neutral) |
| **PH-S1741** | Vision Map UI card | layer chips + edge kinds + svg link у `ui/index.html` |
| **PH-S1742** | Vision map contracts | `tests/gsv_vision_contracts.rs` (10) |
| **PH-S1743** | Feed status filter | `GET /api/vision/feed?status=closed\|open\|all` |
| **PH-S1744** | GSV vision docs | `VISION.md` map/feed-filter/svg; `GSV_MIGRATION.md` rows ✅; MEMORY band 110 |
| **PH-S1745** | poolAI vision parity | `docs/vision/README.md` band 110; roadmap band 110 |
| **PH-S1746** | Ratio hold advisory | `gsv-loc-audit --min-ratio 0.95 --advisory` |
| **PH-S1747** | vision-sync close | `gsv-vision-sync` refresh + poolAI vision rev **461** |
| **PH-S1748** | Band close | ratio hold (≥95%); fmt/clippy/test; docs canon; push |

## Спринти (band 111) — Sprint map + doc-preview (Vision UI логіка)

| Sprint | Фокус | Acceptance (ключ) |
|--------|-------|-------------------|
| **PH-S1749** | Sprint-map wire | `sprint_map_report`/`wire_sprint_map`; `GET /api/vision/sprint-map` (sprint-scope/queue/session-tracks links + modules + kinds) |
| **PH-S1750** | Doc-preview wire | `doc_preview`/`wire_doc_preview`; `GET /api/vision/doc-preview?id=` (node + 1-hop neighbors) |
| **PH-S1751** | Sprint-map contracts | `tests/gsv_vision_contracts.rs` (12) |
| **PH-S1752** | Doc-preview contracts | `tests/gsv_vision_contracts.rs` (**14**) |
| **PH-S1753** | Sprint Map UI card | modules/kinds/links у `ui/index.html` |
| **PH-S1754** | Doc Preview UI card | node id input + out/in links + sections у `ui/index.html` |
| **PH-S1755** | GSV vision docs | `VISION.md` sprint-map/doc-preview; MEMORY band 111; HANDOFF/NEXT band 111 |
| **PH-S1756** | poolAI vision parity | `GSV_MIGRATION.md` row 21 ✅; `docs/vision/README.md`; roadmap band 111 |
| **PH-S1757** | Ratio hold advisory | `gsv-loc-audit --min-ratio 0.95 --advisory`; poolAI parity hold |
| **PH-S1758** | Band close | ratio hold (≥95%); fmt/clippy/test; docs canon; vision-sync; push |

## Спринти (band 112) — Vision auto-sync + sprint-queue planning

| Sprint | Фокус | Acceptance (ключ) |
|--------|-------|-------------------|
| **PH-S1759** | Extensions mirror | `Extensions` struct + read/save/load/source; `gsv_extensions.json` snapshot; `wire_extensions` → `GET /api/vision/extensions`; `sync()`/`collect_drift`/bin include extensions |
| **PH-S1760** | Vision auto-sync wire | `wire_sync` → `GET /api/vision/sync` (re-mirror + drift gate) |
| **PH-S1761** | Sprint-queue planning wire | `SprintQueueReport`/`wire_sprint_queue` → `GET /api/vision/sprint-queue` (entries ∪ active) |
| **PH-S1762** | Extensions contracts | `tests/gsv_vision_contracts.rs` extensions (17) |
| **PH-S1763** | Sprint-queue contracts | sync + sprint-queue endpoints + real-workspace report (**19**) |
| **PH-S1764** | Vision Sync + Sprint Queue UI cards | Resync button + drift status; next/active/open + planned у `ui/index.html` |
| **PH-S1765** | GSV vision docs | `VISION.md` sync/extensions/sprint-queue; MEMORY band 112; HANDOFF/NEXT band 112 |
| **PH-S1766** | poolAI vision parity | `GSV_MIGRATION.md` rows ✅; `docs/vision/README.md`; roadmap band 112 |
| **PH-S1767** | Ratio hold advisory | `gsv-loc-audit --min-ratio 0.95 --advisory` (95.56%) |
| **PH-S1768** | Band close | ratio hold (≥95%); fmt/clippy/test (118); docs canon; vision-sync rev 463; push |

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
