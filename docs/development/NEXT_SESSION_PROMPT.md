# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-27 (band 88 **PH-S1519…S1528, 2026-07-27** ✅ · horizon band 89)

Maintenance mode (FM §5.15) · band 88 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 89) |
| **§5.12 active** | **10** (band 88 ✅) |
| **P0 open** | **PH-SVC34** re-verify GH · **PH-SVC35** OWNER |
| **Completion pending** | **750** sprints PH-S1529…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon** | band 89 → **PH-S1529…S1538** |
| **Vision** | rev **411** |
| **Cursor / GH** | local **3.13.21** · Auto-review · Router Balance/Intelligence · Actions `GITHUB_TOKEN` opaque/JWT · [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md) |

---

## Тригер

```
абракадабра
```

**Порядок:** drain band 89 (Policies ratio-advisory aggregate). Після drain — **`bash bin/record-test-ci-speed.sh`** (або timed `cargo test-ci` + `poolai-speed-index --record-test-ci`) → Speeds panel у vision ([`SPEED_INDEX.md`](./SPEED_INDEX.md)). Потім vision-sync / push. Після push — PH-SVC34 GH re-verify (JWT-format `GITHUB_TOKEN` ok). **Не** комітити `certs/*.pem`, `.env`, `data/audit/*`. **Не** валідувати довжину `ghs_*`.

**Cursor:** desktop **3.13.21** · Run Mode **Auto-review** · drain = Agent mode · Router Auto → Balance/Intelligence · research [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md).

---

## Band 89 (очікуваний фокус — project scan)

| Sprint | Фокус |
|--------|--------|
| **PH-S1529** | `policy_ratio_advisory_depth` scaffold |
| **PH-S1530** | Slice aggregate (prior `--policy*` + vision-sync) |
| **PH-S1531** | Criteria contracts |
| **PH-S1532** | `VERIFY_POLICY_RATIO_ADVISORY` + quick `--policy-ratio-advisory` |
| **PH-S1533** | Stand smoke export shape band 89 |
| **PH-S1534** | `poolai-loc-audit --policy-ratio-advisory` |
| **PH-S1535** | Docs `POLICIES_RATIO_ADVISORY.md` + canon |
| **PH-S1536** | vision-sync --check |
| **PH-S1537** | Ratio hold advisory |
| **PH-S1538** | Band close → `galaxy_horizon_s1529_integration` |

Канон: FM **§5.70** · [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md)

---

## Не повторювати

PH-SVC75…84 ✅ (Cursor 3.13.21 / vision tools) · PH-SVC65…74 ✅ (GH App / Actions token opaque) · PH-SVC55…64 ✅ · band 88 ✅ · band 87 ✅ · band 86 ✅ · band 85 ✅ · band 84 ✅ · band 83 ✅ · band 82 ✅ · band 81 ✅ · band 80 ✅ · band 79 ✅ · PH-SVC45…54 ✅ · PH-SVC41…43 ✅ · band 74 ✅ · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без OWNER · staging `certs/*.pem` / `data/audit/*` / `.env` · token length-checks на `ghs_*`.
