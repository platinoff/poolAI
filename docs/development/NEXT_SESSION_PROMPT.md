# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-25 (band 81 **PH-S1449…S1458, 2026-07-24) ·** ✅ · horizon band 82)

Maintenance mode (FM §5.15) · band 81 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 82) |
| **§5.12 active** | **10** (band 81 ✅) |
| **P0 open** | **PH-SVC34** re-verify GH · **PH-SVC35** OWNER |
| **Completion pending** | **820** sprints PH-S1459…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon** | band 82 → **PH-S1459…S1468** |
| **Vision** | rev **393** |
| **Cursor / GH** | local **3.13.10** · Auto-review · Actions `GITHUB_TOKEN` opaque/JWT · [`GITHUB_APP_INSTALLATION_TOKENS_2026-07-25.md`](./GITHUB_APP_INSTALLATION_TOKENS_2026-07-25.md) |

---

## Тригер

```
абракадабра
```

**Порядок:** drain band 82 (Policies store wire). Після push — PH-SVC34 GH re-verify (JWT-format `GITHUB_TOKEN` ok). **Не** комітити `certs/*.pem`, `.env`, `data/audit/*`. **Не** валідувати довжину `ghs_*`.

---

## Band 82 (очікуваний фокус — project scan)

| Sprint | Фокус |
|--------|--------|
| **PH-S1459** | `policy_store_depth` scaffold |
| **PH-S1460** | `policy_store_wire` durable path |
| **PH-S1461** | store wire contracts |
| **PH-S1462** | `VERIFY_POLICY_STORE` + `--policy-store` |
| **PH-S1463** | Stand smoke `policy_store` export |
| **PH-S1464** | poolai-loc-audit `--policy-store` |
| **PH-S1465** | Docs `POLICIES_STORE.md` + canon |
| **PH-S1466** | vision-sync --check |
| **PH-S1467** | Ratio hold advisory |
| **PH-S1468** | Band close → `galaxy_horizon_s1459_integration` |

Канон: FM **§5.63** · [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md)

---

## Не повторювати

PH-SVC65…74 ✅ (GH App / Actions token opaque) · PH-SVC55…64 ✅ · band 81 ✅ · band 80 ✅ · band 79 ✅ · PH-SVC45…54 ✅ · PH-SVC41…43 ✅ · band 74 ✅ · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без OWNER · staging `certs/*.pem` / `data/audit/*` / `.env` · token length-checks на `ghs_*`.
