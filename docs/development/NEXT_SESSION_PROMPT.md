# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-24 (band 78 **PH-S1419…S1428, 2026-07-24) ·** ✅ · horizon band 79)

Maintenance mode (FM §5.15) · band 78 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 79) |
| **§5.12 active** | **10** (band 78 ✅) |
| **P0 open** | **PH-SVC34** re-verify GH · **PH-SVC35** OWNER |
| **Completion pending** | **850** sprints PH-S1429…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon** | band 79 → **PH-S1429…S1438** |
| **Vision** | rev **386** |
| **Cursor** | local **3.13.10** · Auto-review · security PEMs untracked |

---

## Тригер

```
абракадабра
```

**Порядок:** drain band 79 (Audit ratio-advisory). Після push — PH-SVC34 GH re-verify. **Не** комітити `certs/*.pem`, `.env`, `data/audit/*`.

---

## Band 79 (очікуваний фокус — project scan)

| Sprint | Фокус |
|--------|--------|
| **PH-S1429** | `audit_ratio_advisory_depth` ui-core |
| **PH-S1430** | Slice aggregate (`AUDIT_RATIO_ADVISORY_SLICES`) |
| **PH-S1431** | Criteria contracts |
| **PH-S1432** | `VERIFY_AUDIT_RATIO_ADVISORY` + quick |
| **PH-S1433** | Stand smoke export shape |
| **PH-S1434** | `poolai-loc-audit --audit-ratio-advisory` |
| **PH-S1435** | `AUDIT_RATIO_ADVISORY.md` + canon |
| **PH-S1436** | vision-sync --check |
| **PH-S1437** | Ratio hold advisory |
| **PH-S1438** | Band close → `galaxy_horizon_s1429_integration` |

Канон: FM **§5.60** · [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md) · mirror [`SSO_RATIO_ADVISORY.md`](./SSO_RATIO_ADVISORY.md)

---

## Не повторювати

PH-SVC55…64 ✅ (PEMs/audit untrack + gitignore) · band 78 ✅ · band 77 ✅ · PH-SVC45…54 ✅ · PH-SVC41…43 ✅ · band 74 ✅ · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без OWNER · staging `certs/*.pem` / `data/audit/*` / `.env`.
