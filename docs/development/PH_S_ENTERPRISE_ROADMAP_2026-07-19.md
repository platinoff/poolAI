# PH-S enterprise roadmap v2 (PH-S1149…S2148)

**Оновлено:** 2026-07-21 · **Мета:** durable single-host **enterprise 100%** (FM §5.17) за **1000** спринтами · **100** сесій `абракадабра` (10 PH-S* / сесія)

**Канон drain:** FM **§5.12** (max 10 відкритих) · реєстр — [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md) · regen: `bash scripts/generate-ph-s-master-backlog-1000.sh`

**Попередній горизонт:** product-complete PH-S1010 / FM §5.15 ✅ · maintenance bands 37–50 ✅ · enterprise bands 51–56 ✅ (persist + store wire + API contracts + admin/ops glue + stand smoke + loc-audit aggregate)

**Активна смуга:** band 57 **PH-S1209…S1218** (A Tenants · docs canon)

**Поза scope (не в backlog):** FM-003 LAN 2-host (**BLOCKED**) · FM-041 Cloud SDK prod (**Deferred**) · mandatory ZK/TEE

---

## Enterprise 100% acceptance (FM §5.17)

| # | Критерій | Перевірка |
|---|----------|-----------|
| 1 | Tenants + quotas durable | restart-safe store + cross-tenant isolation tests |
| 2 | SSO production path | SAML verify + OAuth fixtures under `cargo test-ci` |
| 3 | Durable queryable audit | retention + admin/OpenAPI |
| 4 | Security policies persist | secret-rotation wire (dev-safe) |
| 5 | Monitoring durable | alert_rules + dashboards when env set |
| 6 | OpenAPI enterprise | `poolai-openapi-gap-audit` → **0** for `/api/enterprise/*` |
| 7 | Rust ratio | ≥95% hold; 96% met or formal advisory |
| 8 | Galaxy single-host | capability + network_profile + offline settlement green |
| 9 | Governance | signed-release verify; no remote root-admin |
| 10 | Gates | `cargo test-ci` + vision `--check`; FM §5.17 closed at **PH-S2148** |

---

## Фази A–J (по ~100 спринтів / 10 bands)

| Фаза | Bands | Sprints | Фокус |
|------|-------|---------|--------|
| **A — Tenants** | 51–60 | S1149–S1248 | Tenant SQLite/durable store, quotas, isolation |
| **B — SSO** | 61–70 | S1249–S1348 | SAML sig/time/audience; OAuth provider persist |
| **C — Audit** | 71–80 | S1349–S1448 | Queryable audit + retention + SIEM export |
| **D — Policies** | 81–90 | S1449–S1548 | Persisted policies; RBAC cross-tenant; secrets |
| **E — Monitoring** | 91–100 | S1549–S1648 | Persist alert_rules + dashboards |
| **F — Ratio 96%** | 101–110 | S1649–S1748 | ui_js→wasm; e2e hold; stretch gate |
| **G — Galaxy edge** | 111–120 | S1749–S1848 | Capability/fraud-proof beyond stub |
| **H — GPU limits** | 121–130 | S1849–S1948 | GPU admission + worker limits (single-host) |
| **I — Settlement** | 131–140 | S1949–S2048 | Offline payout + billing ops surface |
| **J — Governance close** | 141–150 | S2049–S2148 | Signed release; DIGEST/STABLE truth; §5.17 ✅ |

---

## Workflow promote

1. Закрити band у §5.12 → ✅
2. Взяти **наступні 10** з [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md) → §5.12 `[ ]`
3. Оновити HANDOFF + NEXT + GALAXY + цей roadmap zriz
4. Vision close → `cargo test-ci` → один commit → push
5. Повторити **`абракадабра`** до PH-S2148

**Після PH-S2148 ✅:** FM §5.17 enterprise-complete; новий scan лише за запитом власника або BLOCKED ops.
