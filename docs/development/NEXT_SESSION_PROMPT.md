# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-08-02 (band 124 **PH-S1879…S1888, 2026-08-11** ✅ · horizon band 126)

Maintenance mode (FM §5.15) · band 124 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 126) |
| **§5.12 active** | **0** (band 124 ✅) |
| **P0 open** | **PH-SVC34** re-verify GH · **PH-SVC35** OWNER |
| **Completion pending** | **560** sprints PH-S1739…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon band 127 PH-S1909.S1918** | band 125 → **PH-S1889…S1898** |
| **Vision** | rev **484** |
| **GSV** | окремий проєкт Rust-first (bands 102+108+109+110+111+112+113+114+115+116+117+118 ✅, ratio 95.35%, tests 163) · [`GSV/README.md`](../../GSV/README.md) · [`docs/gsv/`](../../gsv/README.md) · ролі [`GSV/docs/GSV_ROLES.md`](../../GSV/docs/GSV_ROLES.md) · Vision box [`GSV/docs/VISION.md`](../../GSV/docs/VISION.md) |
| **Cursor / GH** | local **3.13.21** · Auto-review · Router Balance/Intelligence · Actions `GITHUB_TOKEN` opaque/JWT · [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md) |

---

## Тригер

```
абракадабра
```

**Порядок:** **S0 диск** (`df -h /s` + `check_target_disk.sh` → `cargo clean` за потреби) → project scan (**спочатку** `rust_diagnostics` / clippy warnings → виправлення в топ смуги) → drain наступного band (черга — FM §5.12 / completion roadmap; наступний за каноном — band 119 **PH-S1829…S1838**; **без** mid-push) → Speeds (`bash bin/record-test-ci-speed.sh`) · Rust panel (`bash bin/record-rust-diagnostics.sh`) → vision-sync → **один** commit → **`git push` + самарі (завжди кінець сесії)**. Після push — PH-SVC34 GH re-verify (JWT-format `GITHUB_TOKEN` ok). **Не** комітити `certs/*.pem`, `.env`, `data/audit/*`. **Не** валідувати довжину `ghs_*`.

**Cursor:** desktop **3.13.21** · Run Mode **Auto-review** · drain = Agent mode · Router Auto → Balance/Intelligence · research [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md).

---

## Band 126 (очікуваний фокус — project scan)

**PH-S1879…S1888 🔜 (2026-08-11)** (enterprise phase H «GPU admission + worker limits, single-host»).
Pattern mirror: band 104 `ratio96_admin_ops_depth` admin/ops glue. Смуга: `gpu_limits_admin_ops_depth`
ui-core module (depth enum + criteria registry, 10) → `#gpu-limits-store-badge` dashboard
store strip (`src/ui/admin/dashboard.rs`, reads `GET /api/v1/gpu-limits`) + i18n
`admin.gpuLimits.*` → HTML contracts (`gpu_limits_admin_ops_integration`) → admin/ops glue
(`VERIFY_GPU_LIMITS_ADMIN_OPS` + quick `--gpu-limits-admin-ops`) → stand smoke export shape
(`gpu_limits_admin_ops_band124_export_shape_ph_s1883`) → loc-audit `--gpu-limits-admin-ops` →
docs `GPU_LIMITS_ADMIN_OPS.md` canon → vision-sync → ratio hold advisory → band close
(`galaxy_horizon_s1879_integration`, FM §5.105). Канон:
[`GPU_LIMITS_ADMIN_OPS.md`](./GPU_LIMITS_ADMIN_OPS.md) · [`FUNCTION_MANAGEMENT.md §5.105`](../catalog/FUNCTION_MANAGEMENT.md).

## Band 126 (очікуваний фокус — project scan)

**PH-S1869…S1878 ✅ (2026-08-11)** (enterprise phase H «GPU admission + worker limits, single-host»).
Pattern mirror: band 122 `gpu_limits_depth` docs canon. Смуга: `gpu_limits_api_depth`
ui-core module (depth enum + criteria registry, 10) → `GET /api/v1/gpu-limits` HTTP
route (`src/network/api/system.rs`; durable store wire shape) → API contracts
(`gpu_limits_api_contracts_integration`) → admin/ops glue (`VERIFY_GPU_LIMITS_API` +
quick `--gpu-limits-api`) → stand smoke export shape
(`gpu_limits_api_band123_export_shape_ph_s1869`) → loc-audit `--gpu-limits-api` →
docs `GPU_LIMITS.md` band-123 canon → vision-sync → ratio hold advisory → band close
(`galaxy_horizon_s1869_integration`, FM §5.104). Канон:
[`GPU_LIMITS.md`](./GPU_LIMITS.md) · [`FUNCTION_MANAGEMENT.md §5.104`](../catalog/FUNCTION_MANAGEMENT.md).

## Band 126 (очікуваний фокус — project scan)

**PH-S1859…S1868 ✅ (2026-08-11)** (enterprise phase H «GPU admission + worker limits, single-host»).
Pattern mirror: band 107 `RATIO96_DOCS_CANON` docs canon. Смуга: `gpu_limits_depth`
ui-core module (depth enum + criteria registry, 10) → `gpu_limits_store` store/wire
slice (`docs/development/gpu_limits.json` durable store) → API contracts
(`gpu_limits_integration`) → admin/ops glue (`VERIFY_GPU_LIMITS` + quick
`--gpu-limits`) → stand smoke export shape (`gpu_limits_band122_export_shape_ph_s1859`)
→ loc-audit `--gpu-limits` → docs `GPU_LIMITS.md` canon → vision-sync → ratio hold
advisory → band close (`galaxy_horizon_s1859_integration`, FM §5.103). Канон:
[`GPU_LIMITS.md`](./GPU_LIMITS.md) · [`FUNCTION_MANAGEMENT.md §5.103`](../catalog/FUNCTION_MANAGEMENT.md).

## Band 126 (очікуваний фокус — project scan)

Окремий проєкт **Galaxy StarWalker Vision** — **bands 102 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116 + 117 + 118 ✅**: Rust-first bin-сервер `gsv-server`
з боксами (Tracker · SLI console · Toolchain · IDE · Update/offline · Box preview · SLI terminal ·
Tests/bench hooks · Ratio · **Vision** · **Vision Map** · **Sprint Map** · **Doc Preview** · **Vision Sync** · **Sprint Queue** · **Sprint Board** · **Sprint Progress** · **Sprint Focus** · **Node Search** · **Speed Index** · **Rust Diagnostics** · OmniRouter), **163 tests green**, clippy 0, **ratio 95.35%**.
Канон: ролі [`GSV/docs/GSV_ROLES.md`](../../GSV/docs/GSV_ROLES.md) · пам'ять [`GSV/docs/MEMORY.md`](../../GSV/docs/MEMORY.md) · Vision box [`GSV/docs/VISION.md`](../../GSV/docs/VISION.md) · TechPreroadMap [`GSV_TECH_ROADMAP.md`](../../gsv/GSV_TECH_ROADMAP.md).

Band 116 (**PH-S1799…S1808**, GSV history charts) — **drained** ✅:
`GET /api/vision/speeds.svg` + `GET /api/vision/rust-diagnostics.svg` (Rust-rendered SVG history
charts: test-CI wall bars green ok / red fail ≤24 + bench footer; warnings orange + errors red
grouped bars + command footer); typed history (`SpeedTestCiRecord`/`SpeedBenchRecord`/
`RustDiagRecord` + `test_ci_history`/`bench_history`/`history`); `tests/gsv_vision_contracts.rs`
**43** (загалом **153**); `<img>` charts у Speed Index/Rust Diagnostics cards; `poolai-ui-wasm`
defer (`GSV_MIGRATION.md`); `GSV/docs/VISION.md` + MEMORY/HANDOFF/NEXT/LEGACY_PARITY band 116;
poolAI vision README/GSV_MIGRATION/TECH_ROADMAP parity; ratio holds GSV **95.26%**
(rust 7663 / product 8044); FM §5.12 §5.97 ✅; vision rev **469**.
Band 117 (**PH-S1809…S1818**, GSV legacy vision deactivation) — **drained** ✅:
`docs/vision/index.html` → GSV pointer page (no `vision.js`/`vision.css` refs); `vision.js`/
`vision.css` DEACTIVATED banner (band 117, архів — не видаляємо); `docs/vision/README.md`
deactivation note; live link retarget: poolai-vision-sync feed + GSV `vision.rs` sample links →
`http://127.0.0.1:8891/#b-sprint-board`; RUN_LOCAL/GSV_SERVER/docs-gsv README/SPEED_INDEX/
RUST_DIAGNOSTICS → GSV; legacy test retirement (`poolai_vision_sync.rs` unit ×4,
`galaxy_horizon_s1011/s1019/s1039`, e2e `vision.spec.ts`/`a11y.spec.ts` → deactivated pointer
state; `VISION_MAP_BAND40_ROWS`); `LEGACY_PARITY.md`/`GSV_MIGRATION.md` band 117; GSV ratio
**95.26%**; FM §5.12 §5.98 ✅; vision rev **470**.
Band 118 (**PH-S1819…S1828**, GSV sprint UI migration) — **drained** ✅:
`GET /api/vision/sprint-theme` (`SprintThemeReport` + `sprint_theme_report`/
`wire_sprint_theme`: sprint `#a78bfa`/next `#c4b5fd`, pill/chip/queue colors, layers L0–L5,
edge-kind palettes) та `GET /api/vision/sprint-focus.svg?sprint=` (`sprint_focus_svg`:
sprint-dim — in-scope accent, out-of-scope opacity 0.22/text 0.28, edges tinted, default
active sprint, empty-state); `--sprint*` CSS-змінні + sprint-pill/queue chips у Sprint
Queue/Board cards + Sprint Focus card (`<img id="i-sprint-focus">`); contracts **163**
(44 vision + 25 server); GSV ratio **95.35%** (rust 8328 / product 8734); FM §5.12 §5.99 ✅;
vision rev **471**.
Далі band 119 — за пріоритетом власника: master backlog (Ratio96 phase F) або GSV future
(scope за project scan).
Оновлювати FM §5.12 (черга, ≥10 відкритих), HANDOFF, NEXT_SESSION.

Канон GSV: [`GSV/README.md`](../../GSV/README.md) · [`docs/gsv/GSV_BOXES.md`](../../gsv/GSV_BOXES.md) · [`docs/gsv/GSV_SERVER.md`](../../gsv/GSV_SERVER.md) · FM §5.12 §5.83 · §5.89 · §5.90 · §5.91 · §5.92 · §5.93 · §5.94 · §5.95 · §5.96 · §5.97 · §5.98 · §5.99

---

## Не повторювати

PH-SVC85 ✅ (Rust diagnostics panel / CI) · PH-SVC75…84 ✅ (Cursor 3.13.21 / vision tools) · PH-SVC65…74 ✅ (GH App / Actions token opaque) · PH-SVC55…64 ✅ · band 118 ✅ (GSV sprint UI migration) · band 117 ✅ (GSV legacy vision deactivation) · band 116 ✅ (GSV history charts) · band 115 ✅ (GSV migration completion) · band 114 ✅ (GSV sprint-board + progress UI) · band 113 ✅ (GSV node search + interactive map) · band 112 ✅ (GSV vision auto-sync + sprint-queue) · band 111 ✅ (GSV sprint-map + doc-preview) · band 110 ✅ (GSV vision map UI) · band 109 ✅ (GSV vision sync/migration) · band 108 ✅ (GSV roles/ratio canon) · band 107 ✅ (Ratio96 docs canon) · band 106 ✅ (Ratio96 loc-audit) · band 105 ✅ (Ratio96 stand smoke) · band 104 ✅ (Ratio96 admin/ops glue) · band 103 ✅ · band 102 ✅ (GSV migration) · band 101 ✅ · band 100 ✅ · band 99 ✅ · band 98 ✅ · band 97 ✅ · band 96 ✅ · band 95 ✅ · band 94 ✅ · band 93 ✅ · band 92 ✅ · band 91 ✅ · band 90 ✅ · band 89 ✅ · band 88 ✅ · band 87 ✅ · band 86 ✅ · band 85 ✅ · band 84 ✅ · band 83 ✅ · band 82 ✅ · band 81 ✅ · band 80 ✅ · band 79 ✅ · PH-SVC45…54 ✅ · PH-SVC41…43 ✅ · band 74 ✅ · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без OWNER · staging `certs/*.pem` / `data/audit/*` / `.env` · token length-checks на `ghs_*` · build/test GSV при запущеному `gsv-server` · обхід GSV ratio-смуги Rust-кодом замість compact UI · перенесення legacy `vision.js`/`vision.css` у `GSV/ui/`.
## Band 126 (очікуваний фокус — project scan)
admin.gpuLimits.* HTML contracts (gpu_limits_debug1_integration) admin/ops glue
