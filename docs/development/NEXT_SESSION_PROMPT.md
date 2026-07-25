# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-25 (band 84 **PH-S1479…S1488** ✅ · horizon band 85)

Maintenance mode (FM §5.15) · band 84 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 85) |
| **§5.12 active** | **10** (band 84 ✅) |
| **P0 open** | **PH-SVC34** re-verify GH · **PH-SVC35** OWNER |
| **Completion pending** | **790** sprints PH-S1489…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon** | band 85 → **PH-S1489…S1498** |
| **Vision** | rev **399** |
| **Cursor / GH** | local **3.13.10** · Auto-review · Actions `GITHUB_TOKEN` opaque/JWT · [`GITHUB_APP_INSTALLATION_TOKENS_2026-07-25.md`](./GITHUB_APP_INSTALLATION_TOKENS_2026-07-25.md) |

---

## Тригер

```
абракадабра
```

**Порядок:** drain band 85 (Policies stand smoke). Після push — PH-SVC34 GH re-verify (JWT-format `GITHUB_TOKEN` ok). **Не** комітити `certs/*.pem`, `.env`, `data/audit/*`. **Не** валідувати довжину `ghs_*`.

---

## Band 85 (очікуваний фокус — project scan)

| Sprint | Фокус |
|--------|--------|
| **PH-S1489** | `policy_stand_smoke_depth` scaffold |
| **PH-S1490** | Live store wire smoke `GET /policy/store` |
| **PH-S1491** | Live security policies query smoke |
| **PH-S1492** | Live policy-field fixture smoke |
| **PH-S1493** | CLI `--policy-stand-smoke` |
| **PH-S1494** | `poolai-loc-audit --policy-stand-smoke` |
| **PH-S1495** | `VERIFY_POLICY_STAND_SMOKE` |
| **PH-S1496** | Docs `POLICIES_STAND_SMOKE.md` + canon |
| **PH-S1497** | Ratio hold advisory |
| **PH-S1498** | Band close → `galaxy_horizon_s1489_integration` |

Канон: FM **§5.66** · [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md)

---

## Не повторювати

PH-SVC65…74 ✅ (GH App / Actions token opaque) · PH-SVC55…64 ✅ · band 84 ✅ · band 83 ✅ · band 82 ✅ · band 81 ✅ · band 80 ✅ · band 79 ✅ · PH-SVC45…54 ✅ · PH-SVC41…43 ✅ · band 74 ✅ · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без OWNER · staging `certs/*.pem` / `data/audit/*` / `.env` · token length-checks на `ghs_*`.
