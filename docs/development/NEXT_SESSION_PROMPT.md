# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-25 (band 82 **PH-S1459…S1468** ✅ · horizon band 83)

Maintenance mode (FM §5.15) · band 82 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 83) |
| **§5.12 active** | **10** (band 82 ✅) |
| **P0 open** | **PH-SVC34** re-verify GH · **PH-SVC35** OWNER |
| **Completion pending** | **810** sprints PH-S1469…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon** | band 83 → **PH-S1469…S1478** |
| **Vision** | rev **395** |
| **Cursor / GH** | local **3.13.10** · Auto-review · Actions `GITHUB_TOKEN` opaque/JWT · [`GITHUB_APP_INSTALLATION_TOKENS_2026-07-25.md`](./GITHUB_APP_INSTALLATION_TOKENS_2026-07-25.md) |

---

## Тригер

```
абракадабра
```

**Порядок:** drain band 83 (Policies API contracts). Після push — PH-SVC34 GH re-verify (JWT-format `GITHUB_TOKEN` ok). **Не** комітити `certs/*.pem`, `.env`, `data/audit/*`. **Не** валідувати довжину `ghs_*`.

---

## Band 83 (очікуваний фокус — project scan)

| Sprint | Фокус |
|--------|--------|
| **PH-S1469** | `policy_api_contracts_depth` scaffold |
| **PH-S1470** | Policy query HTTP lifecycle |
| **PH-S1471** | `GET /policy/store` wire status |
| **PH-S1472** | OpenAPI `PolicyStoreWire` |
| **PH-S1473** | Policy field validation fixtures |
| **PH-S1474** | `VERIFY_POLICY_API` + `--policy-api` |
| **PH-S1475** | Stand smoke + loc-audit `--policy-api` |
| **PH-S1476** | Docs `POLICIES_API.md` + canon |
| **PH-S1477** | vision-sync + ratio hold |
| **PH-S1478** | Band close → `galaxy_horizon_s1469_integration` |

Канон: FM **§5.64** · [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md)

---

## Не повторювати

PH-SVC65…74 ✅ (GH App / Actions token opaque) · PH-SVC55…64 ✅ · band 82 ✅ · band 81 ✅ · band 80 ✅ · band 79 ✅ · PH-SVC45…54 ✅ · PH-SVC41…43 ✅ · band 74 ✅ · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без OWNER · staging `certs/*.pem` / `data/audit/*` / `.env` · token length-checks на `ghs_*`.
