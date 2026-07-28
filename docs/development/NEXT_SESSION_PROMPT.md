# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-27 (band 94 **PH-S1579…S1588, 2026-07-28** ✅ · horizon band 95)

Maintenance mode (FM §5.15) · band 94 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 95) |
| **§5.12 active** | **0** (band 94 ✅) |
| **P0 open** | **PH-SVC34** re-verify GH · **PH-SVC35** OWNER |
| **Completion pending** | **690** sprints PH-S1589…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon** | band 95 → **PH-S1589…S1598** |
| **Vision** | rev **423** |
| **Cursor / GH** | local **3.13.21** · Auto-review · Router Balance/Intelligence · Actions `GITHUB_TOKEN` opaque/JWT · [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md) |

---

## Тригер

```
абракадабра
```

**Порядок:** **S0 диск** (`df -h /s` + `check_target_disk.sh` → `cargo clean` за потреби) → project scan (**спочатку** `rust_diagnostics` / clippy warnings → виправлення в топ смуги) → drain band 95 (Monitoring stand smoke … / close; **без** mid-push) → Speeds (`bash bin/record-test-ci-speed.sh`) · Rust panel (`bash bin/record-rust-diagnostics.sh`) → vision-sync → **один** commit → **`git push` + самарі (завжди кінець сесії)**. Після push — PH-SVC34 GH re-verify (JWT-format `GITHUB_TOKEN` ok). **Не** комітити `certs/*.pem`, `.env`, `data/audit/*`. **Не** валідувати довжину `ghs_*`.

**Cursor:** desktop **3.13.21** · Run Mode **Auto-review** · drain = Agent mode · Router Auto → Balance/Intelligence · research [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md).

---

## Band 95 (очікуваний фокус — project scan)

| Sprint | Фокус |
|--------|--------|
| **PH-S1589** | `monitoring` stand smoke scaffold |
| **PH-S1590** | Live stand smoke slice |
| **PH-S1591** | Smoke contracts / export |
| **PH-S1592** | verify-dev-stand / stand smoke hook |
| **PH-S1593** | Stand smoke export shape |
| **PH-S1594** | `poolai-loc-audit` monitoring-stand-smoke |
| **PH-S1595** | Docs canon sync |
| **PH-S1596** | vision-sync --check |
| **PH-S1597** | Ratio hold advisory |
| **PH-S1598** | Band close → `galaxy_horizon_s1589_integration` |

Канон: next project-scan band from [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md) (band 95 `PH-S1589…S1598`)

---

## Не повторювати

PH-SVC85 ✅ (Rust diagnostics panel / CI) · PH-SVC75…84 ✅ (Cursor 3.13.21 / vision tools) · PH-SVC65…74 ✅ (GH App / Actions token opaque) · PH-SVC55…64 ✅ · band 94 ✅ · band 93 ✅ · band 92 ✅ · band 91 ✅ · band 90 ✅ · band 89 ✅ · band 88 ✅ · band 87 ✅ · band 86 ✅ · band 85 ✅ · band 84 ✅ · band 83 ✅ · band 82 ✅ · band 81 ✅ · band 80 ✅ · band 79 ✅ · PH-SVC45…54 ✅ · PH-SVC41…43 ✅ · band 74 ✅ · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без OWNER · staging `certs/*.pem` / `data/audit/*` / `.env` · token length-checks на `ghs_*`.
