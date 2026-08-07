# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-08-02 (band 115 **PH-S1789…S1798, 2026-08-05** ✅ · horizon band 116)

Maintenance mode (FM §5.15) · band 115 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 116) |
| **§5.12 active** | **0** (band 115 ✅) |
| **P0 open** | **PH-SVC34** re-verify GH · **PH-SVC35** OWNER |
| **Completion pending** | **560** sprints PH-S1739…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon** | band 116 → **PH-S1799…S1808** |
| **Vision** | rev **468** |
| **GSV** | окремий проєкт Rust-first (bands 102+108+109+110+111+112+113+114+115 ✅, ratio 95.04%, tests 150) · [`GSV/README.md`](../../GSV/README.md) · [`docs/gsv/`](../../gsv/README.md) · ролі [`GSV/docs/GSV_ROLES.md`](../../GSV/docs/GSV_ROLES.md) · Vision box [`GSV/docs/VISION.md`](../../GSV/docs/VISION.md) |
| **Cursor / GH** | local **3.13.21** · Auto-review · Router Balance/Intelligence · Actions `GITHUB_TOKEN` opaque/JWT · [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md) |

---

## Тригер

```
абракадабра
```

**Порядок:** **S0 диск** (`df -h /s` + `check_target_disk.sh` → `cargo clean` за потреби) → project scan (**спочатку** `rust_diagnostics` / clippy warnings → виправлення в топ смуги) → drain наступного band (черга — FM §5.12 / completion roadmap; наступний за каноном — band 113 **PH-S1769…S1778**; **без** mid-push) → Speeds (`bash bin/record-test-ci-speed.sh`) · Rust panel (`bash bin/record-rust-diagnostics.sh`) → vision-sync → **один** commit → **`git push` + самарі (завжди кінець сесії)**. Після push — PH-SVC34 GH re-verify (JWT-format `GITHUB_TOKEN` ok). **Не** комітити `certs/*.pem`, `.env`, `data/audit/*`. **Не** валідувати довжину `ghs_*`.

**Cursor:** desktop **3.13.21** · Run Mode **Auto-review** · drain = Agent mode · Router Auto → Balance/Intelligence · research [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md).

---

## Band 116 (очікуваний фокус — project scan)

Окремий проєкт **Galaxy StarWalker Vision** — **bands 102 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 ✅**: Rust-first bin-сервер `gsv-server`
з боксами (Tracker · SLI console · Toolchain · IDE · Update/offline · Box preview · SLI terminal ·
Tests/bench hooks · Ratio · **Vision** · **Vision Map** · **Sprint Map** · **Doc Preview** · **Vision Sync** · **Sprint Queue** · **Sprint Board** · **Sprint Progress** · **Node Search** · **Speed Index** · **Rust Diagnostics** · OmniRouter), **150 tests green**, clippy 0, **ratio 95.04%**.
Канон: ролі [`GSV/docs/GSV_ROLES.md`](../../GSV/docs/GSV_ROLES.md) · пам'ять [`GSV/docs/MEMORY.md`](../../GSV/docs/MEMORY.md) · Vision box [`GSV/docs/VISION.md`](../../GSV/docs/VISION.md) · TechPreroadMap [`GSV_TECH_ROADMAP.md`](../../gsv/GSV_TECH_ROADMAP.md).

Band 115 (**PH-S1789…S1798**, GSV migration completion) — **drained** ✅:
`GET /api/vision/speeds` (`SpeedIndexReport` — latest test-CI wall/ok + bench median + history
counts, mirror `gsv_speed_index.json`, empty-tolerant); `GET /api/vision/rust-diagnostics`
(`RustDiagnosticsReport` — latest warnings/errors/top_codes + history count, mirror
`gsv_rust_diagnostics.json`, empty-tolerant); `tests/gsv_vision_contracts.rs` **40** +
`gsv_server_contracts.rs` **23** (загалом **150**); Speed Index + Rust Diagnostics UI cards у
`GSV/ui/index.html`; parity audit `GSV/docs/LEGACY_PARITY.md` (Speeds + Rust — єдині прогалини,
закрито; `vision.js`/`vision.css` superseded); `GSV/docs/VISION.md` + MEMORY/HANDOFF/NEXT
band 115; poolAI vision README/GSV_MIGRATION/TECH_ROADMAP parity; ratio holds GSV **95.04%**
(rust 7303 / product 7684) + poolAI **95.04%** advisory; FM §5.12 §5.96 ✅; vision rev **468**.
Далі band 116 (`PH-S1799…S1808`) — за пріоритетом власника: master backlog Ratio96 phase F або GSV
future (legacy `vision.js`/`vision.css` superseded — дезактивація після stand smoke,
`poolai-ui-wasm` defer, speed-index history UI, rust-diagnostics history chart).
Оновлювати FM §5.12 (черга, ≥10 відкритих), HANDOFF, NEXT_SESSION.

Канон GSV: [`GSV/README.md`](../../GSV/README.md) · [`docs/gsv/GSV_BOXES.md`](../../gsv/GSV_BOXES.md) · [`docs/gsv/GSV_SERVER.md`](../../gsv/GSV_SERVER.md) · FM §5.12 §5.83 · §5.89 · §5.90 · §5.91 · §5.92 · §5.93 · §5.94 · §5.95 · §5.96

---

## Не повторювати

PH-SVC85 ✅ (Rust diagnostics panel / CI) · PH-SVC75…84 ✅ (Cursor 3.13.21 / vision tools) · PH-SVC65…74 ✅ (GH App / Actions token opaque) · PH-SVC55…64 ✅ · band 115 ✅ (GSV migration completion) · band 114 ✅ (GSV sprint-board + progress UI) · band 113 ✅ (GSV node search + interactive map) · band 112 ✅ (GSV vision auto-sync + sprint-queue) · band 111 ✅ (GSV sprint-map + doc-preview) · band 110 ✅ (GSV vision map UI) · band 109 ✅ (GSV vision sync/migration) · band 108 ✅ (GSV roles/ratio canon) · band 107 ✅ (Ratio96 docs canon) · band 106 ✅ (Ratio96 loc-audit) · band 105 ✅ (Ratio96 stand smoke) · band 104 ✅ (Ratio96 admin/ops glue) · band 103 ✅ · band 102 ✅ (GSV migration) · band 101 ✅ · band 100 ✅ · band 99 ✅ · band 98 ✅ · band 97 ✅ · band 96 ✅ · band 95 ✅ · band 94 ✅ · band 93 ✅ · band 92 ✅ · band 91 ✅ · band 90 ✅ · band 89 ✅ · band 88 ✅ · band 87 ✅ · band 86 ✅ · band 85 ✅ · band 84 ✅ · band 83 ✅ · band 82 ✅ · band 81 ✅ · band 80 ✅ · band 79 ✅ · PH-SVC45…54 ✅ · PH-SVC41…43 ✅ · band 74 ✅ · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без OWNER · staging `certs/*.pem` / `data/audit/*` / `.env` · token length-checks на `ghs_*` · build/test GSV при запущеному `gsv-server` · обхід GSV ratio-смуги Rust-кодом замість compact UI · перенесення legacy `vision.js`/`vision.css` у `GSV/ui/`.
