# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-27 (band 93 **PH-S1569…S1578** ✅ · horizon band 94)

Maintenance mode (FM §5.15) · band 93 drained.

Enterprise horizon v2 (FM §5.14b / §5.17) · Project close extension (FM §5.18 @ S2278).

| **← наступний** | **`абракадабра`** (project scan → band 94) |
| **§5.12 active** | **0** (band 93 ✅) |
| **P0 open** | **PH-SVC34** re-verify GH · **PH-SVC35** OWNER |
| **Completion pending** | **700** sprints PH-S1579…S2278 · [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md) |
| **Horizon** | band 94 → **PH-S1579…S1588** |
| **Vision** | rev **422** |
| **Cursor / GH** | local **3.13.21** · Auto-review · Router Balance/Intelligence · Actions `GITHUB_TOKEN` opaque/JWT · [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md) |

---

## Тригер

```
абракадабра
```

**Порядок:** **S0 диск** (`df -h /s` + `check_target_disk.sh` → `cargo clean` за потреби) → project scan (**спочатку** `rust_diagnostics` / clippy warnings → виправлення в топ смуги) → drain band 94 (Monitoring admin/ops glue … / close; **без** mid-push) → Speeds (`bash bin/record-test-ci-speed.sh`) · Rust panel (`bash bin/record-rust-diagnostics.sh`) → vision-sync → **один** commit → **`git push` + самарі (завжди кінець сесії)**. Після push — PH-SVC34 GH re-verify (JWT-format `GITHUB_TOKEN` ok). **Не** комітити `certs/*.pem`, `.env`, `data/audit/*`. **Не** валідувати довжину `ghs_*`.

**Cursor:** desktop **3.13.21** · Run Mode **Auto-review** · drain = Agent mode · Router Auto → Balance/Intelligence · research [`CURSOR_UPDATE_RESEARCH_2026-07-27.md`](./CURSOR_UPDATE_RESEARCH_2026-07-27.md).

---

## Band 94 (очікуваний фокус — project scan)

| Sprint | Фокус |
|--------|--------|
| **PH-S1579** | `monitoring` admin/ops glue scaffold |
| **PH-S1580** | HTTP/admin strip |
| **PH-S1581** | API contracts tests |
| **PH-S1582** | verify-dev-stand / admin strip |
| **PH-S1583** | Stand smoke export |
| **PH-S1584** | `poolai-loc-audit` monitoring-admin zriz |
| **PH-S1585** | Docs canon sync |
| **PH-S1586** | vision-sync --check |
| **PH-S1587** | Ratio hold advisory |
| **PH-S1588** | Band close → `galaxy_horizon_s1579_integration` |

Канон: next project-scan band from [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md) (band 94 `PH-S1579…S1588`)

---

## Не повторювати

PH-SVC85 ✅ (Rust diagnostics panel / CI) · PH-SVC75…84 ✅ (Cursor 3.13.21 / vision tools) · PH-SVC65…74 ✅ (GH App / Actions token opaque) · PH-SVC55…64 ✅ · band 93 ✅ · band 92 ✅ · band 91 ✅ · band 90 ✅ · band 89 ✅ · band 88 ✅ · band 87 ✅ · band 86 ✅ · band 85 ✅ · band 84 ✅ · band 83 ✅ · band 82 ✅ · band 81 ✅ · band 80 ✅ · band 79 ✅ · PH-SVC45…54 ✅ · PH-SVC41…43 ✅ · band 74 ✅ · FM-003 LAN · FM-041 Cloud SDK · mandatory ZK/TEE · history rewrite без OWNER · staging `certs/*.pem` / `data/audit/*` / `.env` · token length-checks на `ghs_*`.
