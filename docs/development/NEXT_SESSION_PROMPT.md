# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-08-02 (band 110 **PH-S1739…S1748, 2026-08-05** ✅ · horizon band 111)

Maintenance mode (FM §5.15) · band 110 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 111) |
| **§5.12 active** | **0** (band 110 ✅) |
| **P0 open** | **PH-SVC34** re-verify GH · **PH-SVC35** OWNER |
| **Completion pending** | **560** sprints PH-S1739…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon** | band 111 → **PH-S1749…S1758** |
| **Vision** | rev **461** |
| **GSV** | окремий проєкт Rust-first (bands 102+108+109+110 ✅, ratio 96.01%, tests 106) · [`GSV/README.md`](../../GSV/README.md) · [`docs/gsv/`](../../gsv/README.md) · ролі [`GSV/docs/GSV_ROLES.md`](../../GSV/docs/GSV_ROLES.md) · Vision box [`GSV/docs/VISION.md`](../../GSV/docs/VISION.md) |
| **Cursor / GH** | local **3.13.21** · Auto-review · Router Balance/Intelligence · Actions `GITHUB_TOKEN` opaque/JWT · [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md) |

---

## Тригер

```
абракадабра
```

**Порядок:** **S0 диск** (`df -h /s` + `check_target_disk.sh` → `cargo clean` за потреби) → project scan (**спочатку** `rust_diagnostics` / clippy warnings → виправлення в топ смуги) → drain наступного band (черга — FM §5.12 / completion roadmap; наступний за каноном — band 111 **PH-S1749…S1758**; **без** mid-push) → Speeds (`bash bin/record-test-ci-speed.sh`) · Rust panel (`bash bin/record-rust-diagnostics.sh`) → vision-sync → **один** commit → **`git push` + самарі (завжди кінець сесії)**. Після push — PH-SVC34 GH re-verify (JWT-format `GITHUB_TOKEN` ok). **Не** комітити `certs/*.pem`, `.env`, `data/audit/*`. **Не** валідувати довжину `ghs_*`.

**Cursor:** desktop **3.13.21** · Run Mode **Auto-review** · drain = Agent mode · Router Auto → Balance/Intelligence · research [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md).

---

## Band 111 (очікуваний фокус — project scan)

Окремий проєкт **Galaxy StarWalker Vision** — **bands 102 + 108 + 109 + 110 ✅**: Rust-first bin-сервер `gsv-server`
з боксами (Tracker · SLI console · Toolchain · IDE · Update/offline · Box preview · SLI terminal ·
Tests/bench hooks · Ratio · **Vision** · **Vision Map** · OmniRouter), **106 tests green**, clippy 0, **ratio 96.01%**.
Канон: ролі [`GSV/docs/GSV_ROLES.md`](../../GSV/docs/GSV_ROLES.md) · пам'ять [`GSV/docs/MEMORY.md`](../../GSV/docs/MEMORY.md) · Vision box [`GSV/docs/VISION.md`](../../GSV/docs/VISION.md) · TechPreroadMap [`GSV_TECH_ROADMAP.md`](../../gsv/GSV_TECH_ROADMAP.md).

Band 110 (**PH-S1739…S1748**, GSV vision map UI) — **drained** ✅: Vision map wire
`map_report`/`wire_map` → `GET /api/vision/map` (rev 461, 1218 nodes, 535 edges, layers L0..L5
z-sorted: L0:4 L1:253 L2:156 L3:679 L4:124 L5:2; edge kinds: catalog/concept-ref/crate/implements/
mirror/queue/session-tracks/sprint-scope/wire/workspace); `GSV/ui/vision.svg` порт +
`GET /assets/vision.svg` (`image/svg+xml`, audit Ignored, ratio-neutral); Vision Map card
(layer chips + edge kinds + svg link); `GET /api/vision/feed?status=` фільтр;
`tests/gsv_vision_contracts.rs` **10**; `GSV/docs/VISION.md` + `GSV_MIGRATION.md` rows ✅ + MEMORY;
poolAI vision README parity; ratio holds GSV **96.01%** + poolAI **95.02%** advisory;
FM §5.12 §5.91 ✅; vision rev **461**.
Далі band 111 (`PH-S1749…S1758`) — за пріоритетом власника: master backlog Ratio96 phase F або GSV
future — sprint-queue/doc-preview map логіка (`GSV_MIGRATION.md` ⏳ rows). Оновлювати FM §5.12
(черга, ≥10 відкритих), HANDOFF, NEXT_SESSION.

Канон GSV: [`GSV/README.md`](../../GSV/README.md) · [`docs/gsv/GSV_BOXES.md`](../../gsv/GSV_BOXES.md) · [`docs/gsv/GSV_SERVER.md`](../../gsv/GSV_SERVER.md) · FM §5.12 §5.83 · §5.89 · §5.90 · §5.91

---

## Не повторювати

PH-SVC85 ✅ (Rust diagnostics panel / CI) · PH-SVC75…84 ✅ (Cursor 3.13.21 / vision tools) · PH-SVC65…74 ✅ (GH App / Actions token opaque) · PH-SVC55…64 ✅ · band 110 ✅ (GSV vision map UI) · band 109 ✅ (GSV vision sync/migration) · band 108 ✅ (GSV roles/ratio canon) · band 107 ✅ (Ratio96 docs canon) · band 106 ✅ (Ratio96 loc-audit) · band 105 ✅ (Ratio96 stand smoke) · band 104 ✅ (Ratio96 admin/ops glue) · band 103 ✅ · band 102 ✅ (GSV migration) · band 101 ✅ · band 100 ✅ · band 99 ✅ · band 98 ✅ · band 97 ✅ · band 96 ✅ · band 95 ✅ · band 94 ✅ · band 93 ✅ · band 92 ✅ · band 91 ✅ · band 90 ✅ · band 89 ✅ · band 88 ✅ · band 87 ✅ · band 86 ✅ · band 85 ✅ · band 84 ✅ · band 83 ✅ · band 82 ✅ · band 81 ✅ · band 80 ✅ · band 79 ✅ · PH-SVC45…54 ✅ · PH-SVC41…43 ✅ · band 74 ✅ · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без OWNER · staging `certs/*.pem` / `data/audit/*` / `.env` · token length-checks на `ghs_*` · build/test GSV при запущеному `gsv-server` · обхід GSV ratio-смуги Rust-кодом замість compact UI · перенесення legacy `vision.js`/`vision.css` у `GSV/ui/`.
