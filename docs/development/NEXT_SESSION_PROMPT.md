# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-29 (band 101 **PH-S1649…S1658, 2026-08-01** ✅ · horizon band 102)

Maintenance mode (FM §5.15) · band 101 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 102) |
| **§5.12 active** | **10** (band 101 ✅) |
| **P0 open** | **PH-SVC34** re-verify GH · **PH-SVC35** OWNER |
| **Completion pending** | **610** sprints PH-S1669…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon** | band 102 → **PH-S1659…S1668** |
| **Vision** | rev **439** |
| **GSV** | окремий проєкт Rust-first · [`GSV/README.md`](../../GSV/README.md) · [`docs/gsv/`](../../gsv/README.md) · **TechPreroadMap** [`GSV_TECH_ROADMAP.md`](../../gsv/GSV_TECH_ROADMAP.md) |
| **Cursor / GH** | local **3.13.21** · Auto-review · Router Balance/Intelligence · Actions `GITHUB_TOKEN` opaque/JWT · [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md) |

---

## Тригер

```
абракадабра
```

**Порядок:** **S0 диск** (`df -h /s` + `check_target_disk.sh` → `cargo clean` за потреби) → project scan (**спочатку** `rust_diagnostics` / clippy warnings → виправлення в топ смуги) → drain band 102 **GSV migration** (PH-S1659…S1668; **без** mid-push) → Speeds (`bash bin/record-test-ci-speed.sh`) · Rust panel (`bash bin/record-rust-diagnostics.sh`) → vision-sync → **один** commit → **`git push` + самарі (завжди кінець сесії)**. Після push — PH-SVC34 GH re-verify (JWT-format `GITHUB_TOKEN` ok). **Не** комітити `certs/*.pem`, `.env`, `data/audit/*`. **Не** валідувати довжину `ghs_*`.

**Cursor:** desktop **3.13.21** · Run Mode **Auto-review** · drain = Agent mode · Router Auto → Balance/Intelligence · research [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md).

---

## Band 102 (очікуваний фокус — project scan)

Окремий проєкт **Galaxy StarWalker Vision**: vision migration у Rust-first bin-сервер `gsv-server` з боксами (Tracker · SLI console · Toolchain · IDE · Update/offline · Box preview · SLI terminal · Tests/bench hooks). Канон: **TechPreroadMap** [`GSV_TECH_ROADMAP.md`](../../gsv/GSV_TECH_ROADMAP.md).

| Sprint | Фокус |
|--------|--------|
| **PH-S1659** | GSV docs/architecture + Cargo scaffold |
| **PH-S1660** | gsv-server bin scaffold (`GET /`, `/api/health`) |
| **PH-S1661** | Tracker box (`/api/tracker`) |
| **PH-S1662** | SLI console box (`/api/sli`) |
| **PH-S1663** | Toolchain box (`/api/toolchain`) |
| **PH-S1664** | IDE box (opencode + cursor чати) |
| **PH-S1665** | Update box (SSE `update_available`; «Update» замість reload) |
| **PH-S1666** | Box preview (Rust-кольори) + SLI terminal |
| **PH-S1667** | Tests/bench hooks (без перекомпіляції) |
| **PH-S1668** | Band close (offline-стійкість + metrics resync; tests; docs canon) |

Канон: [`GSV/README.md`](../../GSV/README.md) · [`docs/gsv/GSV_BOXES.md`](../../gsv/GSV_BOXES.md) · [`docs/gsv/GSV_SERVER.md`](../../gsv/GSV_SERVER.md) · FM §5.12 §5.83

---

## Не повторювати

PH-SVC85 ✅ (Rust diagnostics panel / CI) · PH-SVC75…84 ✅ (Cursor 3.13.21 / vision tools) · PH-SVC65…74 ✅ (GH App / Actions token opaque) · PH-SVC55…64 ✅ · band 101 ✅ · band 100 ✅ · band 99 ✅ · band 98 ✅ · band 97 ✅ · band 96 ✅ · band 95 ✅ · band 94 ✅ · band 93 ✅ · band 92 ✅ · band 91 ✅ · band 90 ✅ · band 89 ✅ · band 88 ✅ · band 87 ✅ · band 86 ✅ · band 85 ✅ · band 84 ✅ · band 83 ✅ · band 82 ✅ · band 81 ✅ · band 80 ✅ · band 79 ✅ · PH-SVC45…54 ✅ · PH-SVC41…43 ✅ · band 74 ✅ · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без OWNER · staging `certs/*.pem` / `data/audit/*` / `.env` · token length-checks на `ghs_*`.
