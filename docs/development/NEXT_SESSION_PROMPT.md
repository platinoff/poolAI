# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-25 (band 85 **PH-S1489…S1498** ✅ · horizon band 86)

Maintenance mode (FM §5.15) · band 85 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 86) |
| **§5.12 active** | **10** (band 85 ✅) |
| **P0 open** | **PH-SVC34** re-verify GH · **PH-SVC35** OWNER |
| **Completion pending** | **780** sprints PH-S1499…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon** | band 86 → **PH-S1499…S1508** |
| **Vision** | rev **402** |
| **Cursor / GH** | local **3.13.10** · Auto-review · Actions `GITHUB_TOKEN` opaque/JWT · [`GITHUB_APP_INSTALLATION_TOKENS_2026-07-25.md`](./GITHUB_APP_INSTALLATION_TOKENS_2026-07-25.md) |

---

## Тригер

```
абракадабра
```

**Порядок:** drain band 86 (Policies loc-audit aggregate). Після push — PH-SVC34 GH re-verify (JWT-format `GITHUB_TOKEN` ok). **Не** комітити `certs/*.pem`, `.env`, `data/audit/*`. **Не** валідувати довжину `ghs_*`.

---

## Band 86 (очікуваний фокус — project scan)

| Sprint | Фокус |
|--------|--------|
| **PH-S1499** | `policy_loc_audit_depth` scaffold |
| **PH-S1500** | Slice aggregate (phase-D `policy*`) |
| **PH-S1501** | Criteria contracts |
| **PH-S1502** | `VERIFY_POLICY_LOC_AUDIT` + quick `--policy-loc-audit` |
| **PH-S1503** | Stand smoke export shape band 86 |
| **PH-S1504** | `poolai-loc-audit --policy-loc-audit` |
| **PH-S1505** | Docs `POLICIES_LOC_AUDIT.md` + canon |
| **PH-S1506** | vision-sync --check |
| **PH-S1507** | Ratio hold advisory |
| **PH-S1508** | Band close → `galaxy_horizon_s1499_integration` |

Канон: FM **§5.67** · [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md)

---

## Не повторювати

PH-SVC65…74 ✅ (GH App / Actions token opaque) · PH-SVC55…64 ✅ · band 85 ✅ · band 84 ✅ · band 83 ✅ · band 82 ✅ · band 81 ✅ · band 80 ✅ · band 79 ✅ · PH-SVC45…54 ✅ · PH-SVC41…43 ✅ · band 74 ✅ · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без OWNER · staging `certs/*.pem` / `data/audit/*` / `.env` · token length-checks на `ghs_*`.
