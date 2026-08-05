# Промпт наступної сесії (GSV)

**Оновлено:** 2026-08-05 (band 111 **PH-S1749…S1758** ✅ · ratio **95.77%** · tests **113**)

```
абракадабра
```

**Порядок:** **S0 диск** (`df -h /s` + `check_target_disk.sh` → `cargo clean` за потреби) →
project scan (**warnings first** — `cargo clippy --all-targets` у GSV, `poolai-rust-diagnostics` у poolAI) →
drain наступного band (черга — FM §5.12 §5.91 / GSV_TECH_ROADMAP; **без** mid-push) →
Speeds · Rust panel → vision-sync (`poolai-vision-sync`) → **один** commit → **`git push` + самарі**.

**⚠️ Зупинити `gsv-server` перед `cargo test`/`build`** (блокує `target/debug/gsv-server.exe`);
після тестів перезапустити на порт 8891.

## Band стан

- **band 102** (PH-S1659…S1668) ✅ — GSV migration (bin, бокси, docs).
- **band 108** (PH-S1719…S1728) ✅ — roles/ratio canon: `GSV/docs/GSV_ROLES.md`; `gsv-loc-audit`
  (95.52% gate ✅); `tests/gsv_ratio_contracts.rs` (7); Ratio box + `GET /api/ratio` + UI card;
  `GSV/docs/{MEMORY,HANDOFF,NEXT,README}`; FM §5.12 §5.89; poolAI docs parity + HANDOFF/NEXT; vision-sync rev 458.
- **band 109** (PH-S1729…S1738) ✅ — Vision box: `GSV/src/boxes/vision.rs` (manifest/feed serde +
  read/save/load/wire/sync/drift); `gsv-vision-sync` bin (`--check`); `GET /api/vision*`; Vision UI card;
  `tests/gsv_vision_contracts.rs` (7); `GSV/docs/VISION.md` + `GSV_MIGRATION.md` rows ✅; poolAI vision
  README parity; FM §5.12 §5.90; GSV tests **101** (95.45% gate ✅); vision-sync rev 459.
- **band 110** (PH-S1739…S1748) ✅ — Vision map UI: `map_report`/`wire_map` → `GET /api/vision/map`
  (layers L0..L5 z-sorted + edge kinds); `GSV/ui/vision.svg` порт + `GET /assets/vision.svg` (audit
  Ignored, ratio-neutral); Vision Map card; `?status=` feed filter; contracts **106**; GSV ratio
  **96.01%** gate ✅; poolAI vision rev **461**; FM §5.12 §5.91.
- **band 111** (PH-S1749…S1758) ✅ — Sprint map + doc-preview: `sprint_map_report`/`wire_sprint_map`
  → `GET /api/vision/sprint-map` (sprint-scope/queue/session-tracks links + modules + kinds);
  `doc_preview`/`wire_doc_preview` → `GET /api/vision/doc-preview?id=` (node + 1-hop neighbors);
  Sprint Map + Doc Preview UI cards; contracts **113** (14 vision); GSV ratio **95.77%** gate ✅;
  poolAI vision rev **462**; FM §5.12 §5.92.
  **Наступний band 112**: master backlog (Ratio96 phase F) або GSV future — за пріоритетом власника
  (перші кандидати: `GSV_MIGRATION.md` ⏳ rows — vision auto-sync, sprint-queue manifest планування).

## Канон GSV

- Rust **95–100%** / wasm 0–5%, без Python/Java; bins — лише `src/bin/`. Ratio: `cargo run --bin gsv-loc-audit`.
- Ролі/сесія: [`GSV/docs/GSV_ROLES.md`](GSV_ROLES.md) · пам'ять: [`GSV/docs/MEMORY.md`](MEMORY.md).
- Архітектура: [`docs/gsv/`](../../docs/gsv/README.md) · TechPreroadMap: [`GSV_TECH_ROADMAP.md`](../../docs/gsv/GSV_TECH_ROADMAP.md).
- OmniRouter dry-run у тестах — `X-Omni-Dry-Run: 1` (жодного реального запиту).

## Не повторювати

Band 107 ✅ (poolAI Ratio96 docs canon) · band 106 ✅ (Ratio96 loc-audit) · band 105 ✅ (Ratio96 stand smoke) ·
band 104 ✅ (Ratio96 admin/ops) · band 103 ✅ · band 102 ✅ (GSV migration) · band 109 ✅ (GSV vision sync) ·
band 110 ✅ (GSV vision map UI) · band 111 ✅ (GSV sprint-map + doc-preview) ·
staging `GSV/data/*` / `certs/*.pem` /
`.env` · mid-push · build/test при запущеному `gsv-server` · обхід ratio-смуги Rust-кодом замість compact UI ·
перенесення legacy `vision.js`/`vision.css` у `GSV/ui/` (знищило б ratio canon).
