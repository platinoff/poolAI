# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-23 (band 74 **PH-S1379…S1388, 2026-07-23) ·** ✅ · horizon band 75)

Maintenance mode (FM §5.15) · band 74 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 75) |
| **§5.12 active** | **10** (band 74 ✅) |
| **P0 open** | **PH-SVC41…43** CI red · **PH-SVC34** re-verify · **PH-SVC35** OWNER |
| **Completion pending** | **890** sprints PH-S1389…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon** | band 75 → **PH-S1389…S1398** |
| **Vision** | rev **377** |
| **Cursor** | local **3.12.30** · [`CURSOR_UPDATE_RESEARCH_2026-07-22.md`](./CURSOR_UPDATE_RESEARCH_2026-07-22.md) |

---

## Тригер

```
абракадабра
```

**Порядок:** спочатку drain **P0 CI** (PH-SVC41…43 + verify PH-SVC34), потім band 75.

---

## P0 (перша черга — скріншоти CI 2026-07-24)

З push після band 74: **3 failing / 13 successful**. Check/Test/OpenAPI/LOC/Vision — green; червоні нижче.

| Sprint | Фокус | Acceptance |
|--------|--------|------------|
| **PH-SVC41** | Pa11y WCAG 2.2 | `bin/pa11y-ci.sh`: у CI не викликати `cargo build --debug` (cargo не приймає `--debug`); `cargo build` (dev) або `--release`; job **Pa11y WCAG 2.2** green |
| **PH-SVC42** | Playwright admin E2E | той самий баг у `bin/e2e-playwright.sh` (`cargo build "--${E2E_PROFILE}"` → `--debug`); job **Playwright admin suite** green |
| **PH-SVC43** | Documentation / Generate Documentation | діагноз логу `docs.yml` (`cargo doc --no-deps --features jwt,https`); фікс rustdoc/workflow; job green |
| **PH-SVC34** | GH Actions re-verify | усі колишні + Pa11y live + Playwright admin + Docs — green після CI-fix |
| **PH-SVC35** | Secret scanning #1 | **OWNER:** revoke Atlassian API Token; [`SECRETS_MANAGEMENT.md`](../security/SECRETS_MANAGEMENT.md) §4 |

**Root cause (Pa11y log):** `error: unexpected argument '--debug' found` · tip: `'--debug' is the default for 'cargo build'`.

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

Канон: FM **§5.56** · [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md) · mirror [`SSO_STAND_SMOKE.md`](./SSO_STAND_SMOKE.md)

---

## Не повторювати

band 74 ✅ · PH-SVC31…33 ✅ · PH-SVC36…40 ✅ · не плутати `cargo build --debug` з dev profile · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без OWNER.
