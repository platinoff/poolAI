# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-23 (band 74 **PH-S1379…S1388, 2026-07-23) ·** ✅ · horizon band 75)

Maintenance mode (FM §5.15) · band 74 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 75) |
| **§5.12 active** | **10** (band 74 ✅) |
| **P0 open** | **PH-SVC34** GH CI verify · **PH-SVC35** OWNER Atlassian revoke |
| **Completion pending** | **890** sprints PH-S1389…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon** | band 75 → **PH-S1389…S1398** |
| **Vision** | rev **377** |
| **Cursor** | local **3.12.30** · [`CURSOR_UPDATE_RESEARCH_2026-07-22.md`](./CURSOR_UPDATE_RESEARCH_2026-07-22.md) |

---

## Тригер

```
абракадабра
```

---

## P0 (перша черга — скріншоти CI / Security)

| Sprint | Фокус | Acceptance |
|--------|--------|------------|
| **PH-SVC34** | GitHub Actions green | Check (no features), Test Suite, OpenAPI gap, Pa11y contract, LOC ratio, Vision drift — green після CI-fix push |
| **PH-SVC35** | Secret scanning #1 | **OWNER:** revoke Atlassian API Token; see [`SECRETS_MANAGEMENT.md`](../security/SECRETS_MANAGEMENT.md) §4 |

`gh` CLI not on this host PATH — verify via GitHub UI or install `gh`.

---

## Band 75 (очікуваний фокус — project scan)

| Sprint | Фокус |
|--------|--------|
| **PH-S1389** | `audit_stand_smoke_depth` ui-core |
| **PH-S1390** | Live store wire smoke |
| **PH-S1391** | Live audit events query smoke |
| **PH-S1392** | Live event-field fixture smoke |
| **PH-S1393** | CLI `--audit-stand-smoke` |
| **PH-S1394** | `poolai-loc-audit --audit-stand-smoke` |
| **PH-S1395** | `VERIFY_AUDIT_STAND_SMOKE` |
| **PH-S1396** | `AUDIT_STAND_SMOKE.md` + canon |
| **PH-S1397** | Ratio hold advisory |
| **PH-S1398** | Band close → `galaxy_horizon_s1389_integration` |

Канон: FM **§5.56** · [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md) · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) · mirror [`SSO_STAND_SMOKE.md`](./SSO_STAND_SMOKE.md)

---

## Не повторювати

band 74 ✅ · band 73 ✅ · band 72 ✅ · band 71 ✅ · band 70 ✅ · band 69 ✅ · band 68 ✅ · band 67 ✅ · band 66 ✅ · band 65 ✅ · band 64 ✅ · band 63 ✅ · band 62 ✅ · band 61 ✅ · band 60 ✅ · product-complete S1010 ✅ · PH-SVC31…33 ✅ · PH-SVC36…40 ✅ (крім 34/35 open) · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без явного OWNER.
