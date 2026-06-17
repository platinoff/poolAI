# Galaxy Grid roadmap (зріз)

**Оновлено:** 2026-06-17 · **Канон черги:** [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12 (**9** відкритих PH-S254…S262)

| Зріз | Значення |
|------|----------|
| **Wire band** | PH-S65…S187 ✅ |
| **Vision UX** | PH-S188…S233 ✅ |
| **Ops / stand smoke** | PH-S196…S250 ✅ · **S253…S256** відкрито |
| **Admin i18n slim** | PH-S207…S252 ✅ · **S257…S260** відкрито |
| **Docs / ratio** | S261…S262 відкрито |
| **Rust ratio** | **92.78%** hold **95%** · spirit **96%** |

---

## 3. Черга §5.12 (активна смуга)

| Sprint | Scope | Джерело |
|--------|-------|---------|
| **PH-S253** | `galaxy_pricing_quote_usd_micro` + `galaxy_pricing_market_min_usd_micro` stand smoke | §4.2, PH-S174/S181 | **✅** |
| **PH-S254** | `galaxy_fee_split_applied_total` stand smoke | §4.1, PH-S194 |
| **PH-S255** | `galaxy_cross_region_egress_mb` stand smoke | §5.3, PH-S185 |
| **PH-S256** | `galaxy_replay_pending` stand smoke | §6.3, PH-S176 |
| **PH-S257** | `workers.*` → `workers_patch` | ratio / admin UI |
| **PH-S258** | `home.*` → `home_patch` | ratio / public shell |
| **PH-S259** | `form.*` + residual `err.*` slim | ratio / shared forms |
| **PH-S260** | shared `ui.*` toolbar glue slim | ratio / admin_common |
| **PH-S261** | Docs canon sync (INDEX, STABLE_STATE, README) | FM hygiene |
| **PH-S262** | `poolai-loc-audit` refresh + hold gate doc | §5.13, P8 |

Повна таблиця — FM **§5.12** · [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md).

---

## 4. Концепт ↔ код (не зроблено / horizon)

| Тема | Джерело | Статус |
|------|---------|--------|
| LAN replication benchmarks | Architect P4 / FM-003 §4 | **BLOCKED** (2 хости) |
| Cloud SDK live (AWS/Azure/GCP) | FM-041 Deferred | infra only |
| ZK / TEE attestation | Galaxy §6.6 | roadmap only |
| Telegram wallet wire extension | Galaxy §3.2 | **wire ✅** (PH-S prior); stand smoke `telegram_wallet` ✅ |
| Prefetch wire beyond stub | Galaxy §5.5 | metrics ✅; live prefetch TBD |
| Signed capability documents | Galaxy §6.6 / §9 | post-S262 horizon |
