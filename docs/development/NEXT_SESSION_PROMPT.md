# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-23 (band 75 **PH-S1389…S1398, 2026-07-23) ·** ✅ · horizon band 76)

Maintenance mode (FM §5.15) · band 75 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 76) |
| **§5.12 active** | **10** (band 75 ✅) |
| **P0 open** | **PH-SVC34** re-verify GH · **PH-SVC35** OWNER |
| **Completion pending** | **880** sprints PH-S1399…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon** | band 76 → **PH-S1399…S1408** |
| **Vision** | rev **380** |
| **Cursor** | local **3.13.10** · Auto-review · [`CURSOR_UPDATE_RESEARCH_2026-07-24.md`](./CURSOR_UPDATE_RESEARCH_2026-07-24.md) |

---

## Тригер

```
абракадабра
```

**Порядок:** drain band 76 (Audit loc-audit aggregate). Після push — PH-SVC34 GH re-verify.

---

## Band 76 (очікуваний фокус — project scan)

| Sprint | Фокус |
|--------|--------|
| **PH-S1399** | `audit_loc_audit_depth` ui-core |
| **PH-S1400** | Slice aggregate (audit→stand-smoke) |
| **PH-S1401** | Criteria contracts |
| **PH-S1402** | `VERIFY_AUDIT_LOC_AUDIT` + quick |
| **PH-S1403** | Stand smoke export shape |
| **PH-S1404** | `poolai-loc-audit --audit-loc-audit` |
| **PH-S1405** | `AUDIT_LOC_AUDIT.md` + canon |
| **PH-S1406** | vision-sync --check |
| **PH-S1407** | Ratio hold advisory |
| **PH-S1408** | Band close → `galaxy_horizon_s1399_integration` |

Канон: FM **§5.57** · [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md) · mirror [`SSO_LOC_AUDIT.md`](./SSO_LOC_AUDIT.md)

---

## Не повторювати

band 75 ✅ · PH-SVC45…54 ✅ (Cursor 3.13.10 / Auto-review / vision eye+prune) · PH-SVC41…43 ✅ (`cargo build --debug` invalid; TLS reload needs `await`) · band 74 ✅ · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без OWNER.
