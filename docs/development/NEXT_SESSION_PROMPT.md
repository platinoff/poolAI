# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-27 (band 89 **PH-S1529…S1538, 2026-07-27** ✅ · horizon band 90)

Maintenance mode (FM §5.15) · band 89 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 90) |
| **§5.12 active** | **10** (band 89 ✅) |
| **P0 open** | **PH-SVC34** re-verify GH · **PH-SVC35** OWNER |
| **Completion pending** | **740** sprints PH-S1539…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon** | band 90 → **PH-S1539…S1548** |
| **Vision** | rev **414** |
| **Cursor / GH** | local **3.13.21** · Auto-review · Router Balance/Intelligence · Actions `GITHUB_TOKEN` opaque/JWT · [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md) |

---

## Тригер

```
абракадабра
```

**Порядок:** drain band 90 (Policies horizon close aggregate). Після drain — **`bash bin/record-test-ci-speed.sh`** → Speeds · **`bash bin/record-rust-diagnostics.sh`** → Rust panel ([`RUST_DIAGNOSTICS.md`](./RUST_DIAGNOSTICS.md); CI job `rust-diagnostics`). Потім vision-sync / push. Після push — PH-SVC34 GH re-verify (JWT-format `GITHUB_TOKEN` ok). **Не** комітити `certs/*.pem`, `.env`, `data/audit/*`. **Не** валідувати довжину `ghs_*`.

**Cursor:** desktop **3.13.21** · Run Mode **Auto-review** · drain = Agent mode · Router Auto → Balance/Intelligence · research [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md).

---

## Band 90 (очікуваний фокус — project scan)

| Sprint | Фокус |
|--------|--------|
| **PH-S1539** | `policy_horizon_depth` scaffold |
| **PH-S1540** | Slice aggregate (phase-D `--policy*` + ratio-advisory) |
| **PH-S1541** | Criteria contracts |
| **PH-S1542** | `VERIFY_POLICY_HORIZON` + quick `--policy-horizon` |
| **PH-S1543** | Stand smoke export shape band 90 |
| **PH-S1544** | `poolai-loc-audit --policy-horizon` |
| **PH-S1545** | Docs `POLICIES_HORIZON.md` + canon |
| **PH-S1546** | vision-sync --check |
| **PH-S1547** | Ratio hold advisory |
| **PH-S1548** | Band close → `galaxy_horizon_s1539_integration` |

Канон: FM **§5.71** · [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md)

---

## Не повторювати

PH-SVC85 ✅ (Rust diagnostics panel / CI) · PH-SVC75…84 ✅ (Cursor 3.13.21 / vision tools) · PH-SVC65…74 ✅ (GH App / Actions token opaque) · PH-SVC55…64 ✅ · band 89 ✅ · band 88 ✅ · band 87 ✅ · band 86 ✅ · band 85 ✅ · band 84 ✅ · band 83 ✅ · band 82 ✅ · band 81 ✅ · band 80 ✅ · band 79 ✅ · PH-SVC45…54 ✅ · PH-SVC41…43 ✅ · band 74 ✅ · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без OWNER · staging `certs/*.pem` / `data/audit/*` / `.env` · token length-checks на `ghs_*`.
