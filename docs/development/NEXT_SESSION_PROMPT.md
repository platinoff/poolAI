# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-08-02 (band 108 **PH-S1719…S1728** ✅ · horizon band 109)

Maintenance mode (FM §5.15) · band 108 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 109) |
| **§5.12 active** | **0** (band 108 ✅) |
| **P0 open** | **PH-SVC34** re-verify GH · **PH-SVC35** OWNER |
| **Completion pending** | **560** sprints PH-S1729…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon** | band 109 → **PH-S1729…S1738** |
| **Vision** | rev **458** |
| **GSV** | окремий проєкт Rust-first (bands 102+108 ✅, ratio 95.52%) · [`GSV/README.md`](../../GSV/README.md) · [`docs/gsv/`](../../gsv/README.md) · ролі [`GSV/docs/GSV_ROLES.md`](../../GSV/docs/GSV_ROLES.md) |
| **Cursor / GH** | local **3.13.21** · Auto-review · Router Balance/Intelligence · Actions `GITHUB_TOKEN` opaque/JWT · [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md) |

---

## Тригер

```
абракадабра
```

**Порядок:** **S0 диск** (`df -h /s` + `check_target_disk.sh` → `cargo clean` за потреби) → project scan (**спочатку** `rust_diagnostics` / clippy warnings → виправлення в топ смуги) → drain наступного band (черга — FM §5.12 / completion roadmap; наступний за каноном — band 109 **PH-S1729…S1738**; **без** mid-push) → Speeds (`bash bin/record-test-ci-speed.sh`) · Rust panel (`bash bin/record-rust-diagnostics.sh`) → vision-sync → **один** commit → **`git push` + самарі (завжди кінець сесії)**. Після push — PH-SVC34 GH re-verify (JWT-format `GITHUB_TOKEN` ok). **Не** комітити `certs/*.pem`, `.env`, `data/audit/*`. **Не** валідувати довжину `ghs_*`.

**Cursor:** desktop **3.13.21** · Run Mode **Auto-review** · drain = Agent mode · Router Auto → Balance/Intelligence · research [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md).

---

## Band 109 (очікуваний фокус — project scan)

Окремий проєкт **Galaxy StarWalker Vision** — **bands 102 + 108 ✅**: Rust-first bin-сервер `gsv-server`
з боксами (Tracker · SLI console · Toolchain · IDE · Update/offline · Box preview · SLI terminal ·
Tests/bench hooks · Ratio · OmniRouter), **87 tests green**, clippy 0, **ratio 95.52%**.
Канон: ролі [`GSV/docs/GSV_ROLES.md`](../../GSV/docs/GSV_ROLES.md) · пам'ять [`GSV/docs/MEMORY.md`](../../GSV/docs/MEMORY.md) · TechPreroadMap [`GSV_TECH_ROADMAP.md`](../../gsv/GSV_TECH_ROADMAP.md).

Band 108 (**PH-S1719…S1728**, GSV roles/ratio canon) — **drained** ✅: `GSV/docs/GSV_ROLES.md`;
`GSV/src/bin/gsv_loc_audit.rs` + `GSV/src/boxes/ratio.rs` (git-tracked LOC audit, gate ≥95%);
`tests/gsv_ratio_contracts.rs` (7); Ratio box + `GET /api/ratio` + UI Ratio card;
`GSV/docs/{README,MEMORY,HANDOFF_NEW_SESSION,NEXT_SESSION_PROMPT}`; FM §5.12 §5.89 ✅;
poolAI docs parity; vision rev **458**. Далі band 109 (`PH-S1729…S1738`) — project scan
warnings-first (AGENTS.md «warnings first») → 10 PH-S* з `rust_diagnostics` / FM §5.1 /
architect rows / completion roadmap. Оновлювати FM §5.12 (черга, ≥10 відкритих), HANDOFF, NEXT_SESSION.

Канон GSV: [`GSV/README.md`](../../GSV/README.md) · [`docs/gsv/GSV_BOXES.md`](../../gsv/GSV_BOXES.md) · [`docs/gsv/GSV_SERVER.md`](../../gsv/GSV_SERVER.md) · FM §5.12 §5.83 · §5.89

---

## Не повторювати

PH-SVC85 ✅ (Rust diagnostics panel / CI) · PH-SVC75…84 ✅ (Cursor 3.13.21 / vision tools) · PH-SVC65…74 ✅ (GH App / Actions token opaque) · PH-SVC55…64 ✅ · band 108 ✅ (GSV roles/ratio canon) · band 107 ✅ (Ratio96 docs canon) · band 106 ✅ (Ratio96 loc-audit) · band 105 ✅ (Ratio96 stand smoke) · band 104 ✅ (Ratio96 admin/ops glue) · band 103 ✅ · band 102 ✅ (GSV migration) · band 101 ✅ · band 100 ✅ · band 99 ✅ · band 98 ✅ · band 97 ✅ · band 96 ✅ · band 95 ✅ · band 94 ✅ · band 93 ✅ · band 92 ✅ · band 91 ✅ · band 90 ✅ · band 89 ✅ · band 88 ✅ · band 87 ✅ · band 86 ✅ · band 85 ✅ · band 84 ✅ · band 83 ✅ · band 82 ✅ · band 81 ✅ · band 80 ✅ · band 79 ✅ · PH-SVC45…54 ✅ · PH-SVC41…43 ✅ · band 74 ✅ · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без OWNER · staging `certs/*.pem` / `data/audit/*` / `.env` · token length-checks на `ghs_*` · build/test GSV при запущеному `gsv-server` · обхід GSV ratio-смуги Rust-кодом замість compact UI.
