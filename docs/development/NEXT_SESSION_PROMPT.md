# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-08-02 (band 106 **PH-S1699…S1708** ✅ · horizon band 107)

Maintenance mode (FM §5.15) · band 106 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 107) |
| **§5.12 active** | **0** (band 106 ✅) |
| **P0 open** | **PH-SVC34** re-verify GH · **PH-SVC35** OWNER |
| **Completion pending** | **580** sprints PH-S1709…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon** | band 107 → **PH-S1709…S1718** |
| **Vision** | rev **456** |
| **GSV** | окремий проєкт Rust-first · [`GSV/README.md`](../../GSV/README.md) · [`docs/gsv/`](../../gsv/README.md) · **TechPreroadMap** [`GSV_TECH_ROADMAP.md`](../../gsv/GSV_TECH_ROADMAP.md) |
| **Cursor / GH** | local **3.13.21** · Auto-review · Router Balance/Intelligence · Actions `GITHUB_TOKEN` opaque/JWT · [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md) |

---

## Тригер

```
абракадабра
```

**Порядок:** **S0 диск** (`df -h /s` + `check_target_disk.sh` → `cargo clean` за потреби) → project scan (**спочатку** `rust_diagnostics` / clippy warnings → виправлення в топ смуги) → drain наступного band (черга — FM §5.12 / completion roadmap; наступний за каноном — band 107 **PH-S1709…S1718**; **без** mid-push) → Speeds (`bash bin/record-test-ci-speed.sh`) · Rust panel (`bash bin/record-rust-diagnostics.sh`) → vision-sync → **один** commit → **`git push` + самарі (завжди кінець сесії)**. Після push — PH-SVC34 GH re-verify (JWT-format `GITHUB_TOKEN` ok). **Не** комітити `certs/*.pem`, `.env`, `data/audit/*`. **Не** валідувати довжину `ghs_*`.

**Cursor:** desktop **3.13.21** · Run Mode **Auto-review** · drain = Agent mode · Router Auto → Balance/Intelligence · research [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md).

---

## Band 107 (очікуваний фокус — project scan)

Окремий проєкт **Galaxy StarWalker Vision** **завершено** (band 102 ✅): Rust-first bin-сервер `gsv-server` з боксами (Tracker · SLI console · Toolchain · IDE · Update/offline · Box preview · SLI terminal · Tests/bench hooks), 52 tests green, clippy 0. Канон: **TechPreroadMap** [`GSV_TECH_ROADMAP.md`](../../gsv/GSV_TECH_ROADMAP.md).

Band 106 (**PH-S1699…S1708**, F Ratio96 loc-audit) — **drained** ✅: `ratio96_loc_audit_depth.rs`, `--ratio96-loc-audit`, `RATIO96_LOC_AUDIT.md`, `VERIFY_RATIO96_LOC_AUDIT`, `galaxy_horizon_s1699_integration`; FM §5.12 §5.87 ✅. Band 105 (`PH-S1689…S1698`, F Ratio96 · stand smoke) — **drained** ✅: `ratio96_stand_smoke_depth.rs`, `--ratio96-stand-smoke`, `RATIO96_STAND_SMOKE.md`, `VERIFY_RATIO96_STAND_SMOKE`, `galaxy_horizon_s1689_integration`; FM §5.12 §5.86 ✅. Далі band 107 (`PH-S1709…S1718`) — project scan warnings-first (AGENTS.md «warnings first») → 10 PH-S* з `rust_diagnostics` / FM §5.1 / architect rows / completion roadmap. Оновлювати FM §5.12 (черга, ≥10 відкритих), HANDOFF, NEXT_SESSION.

Канон GSV: [`GSV/README.md`](../../GSV/README.md) · [`docs/gsv/GSV_BOXES.md`](../../gsv/GSV_BOXES.md) · [`docs/gsv/GSV_SERVER.md`](../../gsv/GSV_SERVER.md) · FM §5.12 §5.83

---

## Не повторювати

PH-SVC85 ✅ (Rust diagnostics panel / CI) · PH-SVC75…84 ✅ (Cursor 3.13.21 / vision tools) · PH-SVC65…74 ✅ (GH App / Actions token opaque) · PH-SVC55…64 ✅ · band 106 ✅ (Ratio96 loc-audit) · band 105 ✅ (Ratio96 stand smoke) · band 104 ✅ (Ratio96 admin/ops glue) · band 103 ✅ · band 102 ✅ (GSV migration) · band 101 ✅ · band 100 ✅ · band 99 ✅ · band 98 ✅ · band 97 ✅ · band 96 ✅ · band 95 ✅ · band 94 ✅ · band 93 ✅ · band 92 ✅ · band 91 ✅ · band 90 ✅ · band 89 ✅ · band 88 ✅ · band 87 ✅ · band 86 ✅ · band 85 ✅ · band 84 ✅ · band 83 ✅ · band 82 ✅ · band 81 ✅ · band 80 ✅ · band 79 ✅ · PH-SVC45…54 ✅ · PH-SVC41…43 ✅ · band 74 ✅ · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без OWNER · staging `certs/*.pem` / `data/audit/*` / `.env` · token length-checks на `ghs_*`.
