# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-25 (band 83 **PH-S1469…S1478** ✅ · horizon band 84)

Maintenance mode (FM §5.15) · band 83 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 84) |
| **§5.12 active** | **10** (band 83 ✅) |
| **P0 open** | **PH-SVC34** re-verify GH · **PH-SVC35** OWNER |
| **Completion pending** | **800** sprints PH-S1479…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon** | band 84 → **PH-S1479…S1488** |
| **Vision** | rev **398** |
| **Cursor / GH** | local **3.13.10** · Auto-review · Actions `GITHUB_TOKEN` opaque/JWT · [`GITHUB_APP_INSTALLATION_TOKENS_2026-07-25.md`](./GITHUB_APP_INSTALLATION_TOKENS_2026-07-25.md) |

---

## Тригер

```
абракадабра
```

**Порядок:** drain band 84 (Policies admin/ops glue). Після push — PH-SVC34 GH re-verify (JWT-format `GITHUB_TOKEN` ok). **Не** комітити `certs/*.pem`, `.env`, `data/audit/*`. **Не** валідувати довжину `ghs_*`.

---

## Band 84 (очікуваний фокус — project scan)

| Sprint | Фокус |
|--------|--------|
| **PH-S1479** | `policy_admin_ops_depth` scaffold |
| **PH-S1480** | Admin policy store-wire status strip |
| **PH-S1481** | Admin policy query ops glue |
| **PH-S1482** | Admin policy ops HTML contracts |
| **PH-S1483** | i18n Policies admin ops keys |
| **PH-S1484** | `VERIFY_POLICY_ADMIN_OPS` + `--policy-admin-ops` |
| **PH-S1485** | Stand smoke + loc-audit `--policy-admin-ops` |
| **PH-S1486** | Docs `POLICIES_ADMIN_OPS.md` + canon |
| **PH-S1487** | vision-sync + ratio hold |
| **PH-S1488** | Band close → `galaxy_horizon_s1479_integration` |

Канон: FM **§5.65** · [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md)

---

## Не повторювати

PH-SVC65…74 ✅ (GH App / Actions token opaque) · PH-SVC55…64 ✅ · band 83 ✅ · band 82 ✅ · band 81 ✅ · band 80 ✅ · band 79 ✅ · PH-SVC45…54 ✅ · PH-SVC41…43 ✅ · band 74 ✅ · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без OWNER · staging `certs/*.pem` / `data/audit/*` / `.env` · token length-checks на `ghs_*`.
