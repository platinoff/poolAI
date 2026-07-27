# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-27 (band 86 **PH-S1499…S1508, 2026-07-27** ✅ · horizon band 87)

Maintenance mode (FM §5.15) · band 86 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 87) |
| **§5.12 active** | **10** (band 86 ✅) |
| **P0 open** | **PH-SVC34** re-verify GH · **PH-SVC35** OWNER |
| **Completion pending** | **770** sprints PH-S1509…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon** | band 87 → **PH-S1509…S1518** |
| **Vision** | rev **406** |
| **Cursor / GH** | local **3.13.21** · Auto-review · Router Balance/Intelligence · Actions `GITHUB_TOKEN` opaque/JWT · [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md) |

---

## Тригер

```
абракадабра
```

**Порядок:** drain band 87 (Policies docs-canon aggregate). Після drain — **`bash bin/record-test-ci-speed.sh`** (або timed `cargo test-ci` + `poolai-speed-index --record-test-ci`) → Speeds panel у vision ([`SPEED_INDEX.md`](./SPEED_INDEX.md)). Потім vision-sync / push. Після push — PH-SVC34 GH re-verify (JWT-format `GITHUB_TOKEN` ok). **Не** комітити `certs/*.pem`, `.env`, `data/audit/*`. **Не** валідувати довжину `ghs_*`.

**Cursor:** desktop **3.13.21** · Run Mode **Auto-review** · drain = Agent mode · Router Auto → Balance/Intelligence · research [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md).

**Cursor:** desktop **3.13.21** · Run Mode **Auto-review** · drain = Agent mode · Router Auto → Balance/Intelligence.

---

## Band 87 (очікуваний фокус — project scan)

| Sprint | Фокус |
|--------|--------|
| **PH-S1509** | `policy_docs_canon_depth` scaffold |
| **PH-S1510** | Slice aggregate (`POLICIES_*.md`) |
| **PH-S1511** | Criteria contracts |
| **PH-S1512** | `VERIFY_POLICY_DOCS_CANON` + quick `--policy-docs-canon` |
| **PH-S1513** | Stand smoke export shape band 87 |
| **PH-S1514** | `poolai-loc-audit --policy-docs-canon` |
| **PH-S1515** | Docs `POLICIES_DOCS_CANON.md` + canon |
| **PH-S1516** | vision-sync --check |
| **PH-S1517** | Ratio hold advisory |
| **PH-S1518** | Band close → `galaxy_horizon_s1509_integration` |

Канон: FM **§5.68** · [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md)

---

## Не повторювати

PH-SVC75…84 ✅ (Cursor 3.13.21 / vision tools) · PH-SVC65…74 ✅ (GH App / Actions token opaque) · PH-SVC55…64 ✅ · band 86 ✅ · band 85 ✅ · band 84 ✅ · band 83 ✅ · band 82 ✅ · band 81 ✅ · band 80 ✅ · band 79 ✅ · PH-SVC45…54 ✅ · PH-SVC41…43 ✅ · band 74 ✅ · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без OWNER · staging `certs/*.pem` / `data/audit/*` / `.env` · token length-checks на `ghs_*`.
