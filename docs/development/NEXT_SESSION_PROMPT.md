# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-23 (band 73 **PH-S1369…S1378, 2026-07-23) ·** ✅ · horizon band 74)

Maintenance mode (FM §5.15) · band 73 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 74) |
| **§5.12 active** | **10** (band 73 ✅) |
| **P0 open** | **PH-SVC34** GH CI verify · **PH-SVC35** OWNER Atlassian revoke |
| **Completion pending** | **900** sprints PH-S1379…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon** | band 74 → **PH-S1379…S1388** |
| **Vision** | rev **375** |
| **Cursor** | local **3.12.30** · [`CURSOR_UPDATE_RESEARCH_2026-07-22.md`](./CURSOR_UPDATE_RESEARCH_2026-07-22.md) |

---

## Тригер

```
абракадабра
```

---

## P0 (перша черга — скріншоти CI / Security)

Перед drain band 74 перевірити / закрити:

| Sprint | Фокус | Acceptance |
|--------|--------|------------|
| **PH-SVC34** | GitHub Actions green | Check (no features), Test Suite, OpenAPI gap, Pa11y contract, LOC ratio, Vision drift — green після CI-fix push |
| **PH-SVC35** | Secret scanning #1 | **OWNER:** revoke Atlassian API Token; see [`SECRETS_MANAGEMENT.md`](../security/SECRETS_MANAGEMENT.md) §4 |

**Вже зроблено (ця tech-сесія):** PH-SVC31 macvlan `'static` · PH-SVC32 unused warnings · PH-SVC33 local `test-ci` · PH-SVC36 no history rewrite.

Коренева причина червоного CI: `validate_macvlan_mode` повертав `&str` з вхідного lifetime як `&'static str` → падав `Check (no features)` і каскад інших jobs.

---

## Band 74 (очікуваний фокус — project scan)

| Sprint | Фокус |
|--------|--------|
| **PH-S1379** | `audit_admin_ops_depth` ui-core |
| **PH-S1380** | Admin audit store-wire strip |
| **PH-S1381** | Admin audit query ops glue |
| **PH-S1382** | Admin audit ops HTML contracts |
| **PH-S1383** | i18n Audit admin ops keys |
| **PH-S1384** | `VERIFY_AUDIT_ADMIN_OPS` + quick `--audit-admin-ops` |
| **PH-S1385** | Stand smoke + loc-audit `--audit-admin-ops` |
| **PH-S1386** | `AUDIT_ADMIN_OPS.md` + canon |
| **PH-S1387** | Ratio hold advisory |
| **PH-S1388** | Band close → `galaxy_horizon_s1379_integration` |

Канон: FM **§5.55** · [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md) · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) · mirror [`SSO_ADMIN_OPS.md`](./SSO_ADMIN_OPS.md)

---

## Не повторювати

PH-SVC31…33 ✅ · PH-SVC36…40 ✅ (крім 34/35 open) · Service PH-SVC21…SVC30 ✅ · PH-SVC11…SVC20 ✅ · band 73 ✅ · band 72 ✅ · band 71 ✅ · band 70 ✅ · band 69 ✅ · band 68 ✅ · band 67 ✅ · band 66 ✅ · band 65 ✅ · band 64 ✅ · band 63 ✅ · band 62 ✅ · band 61 ✅ · band 60 ✅ · product-complete S1010 ✅ · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без явного OWNER.
