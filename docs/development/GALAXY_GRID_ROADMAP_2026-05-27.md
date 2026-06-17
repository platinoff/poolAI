# Galaxy Grid roadmap (зріз)

**Оновлено:** 2026-06-17 · **Канон черги:** [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12 (**0** відкритих · PH-S128…S272 ✅)

| Зріз | Значення |
|------|----------|
| **Wire band** | PH-S65…S187 ✅ |
| **Vision UX** | PH-S188…S233 ✅ |
| **Ops / stand smoke** | PH-S196…S256 ✅ |
| **Admin i18n slim** | PH-S207…S266 ✅ · `i18n_core.js` STRINGS core **empty** |
| **Docs / ratio** | S261…S272 ✅ |
| **Rust ratio** | **94.34%** hold **95%** advisory · spirit **96%** |

---

## 3. Черга §5.12 (активна смуга)

| Sprint | Scope | Стан |
|--------|-------|------|
| **PH-S253…S256** | Galaxy metrics stand smoke band | **✅** |
| **PH-S257…S260** | Admin i18n slim toolbar/home/form | **✅** |
| **PH-S261…S262** | Docs + loc-audit | **✅** |
| **PH-S263…S266** | i18n finish (`ui.*`/`common.*`/`libs.*`/`raid.*` → Rust) | **✅** |
| **PH-S267…S272** | Docs/vision/ratio maintain | **✅** |

Повна таблиця — FM **§5.12** · [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md).

---

## 4. Концепт ↔ код (не зроблено / horizon)

| Тема | Джерело | Статус |
|------|---------|--------|
| LAN replication benchmarks | Architect P4 / FM-003 §4 | **BLOCKED** (2 хости) |
| Cloud SDK live (AWS/Azure/GCP) | FM-041 Deferred | infra only |
| ZK / TEE attestation | Galaxy §6.6 | roadmap only |
| Telegram wallet wire extension | Galaxy §3.2 | **wire ✅**; stand smoke `telegram_wallet` ✅ |
| **Prefetch wire beyond stub** | Galaxy **§5.5** | **metrics ✅** (`galaxy_prefetch_*`, PH-S167/S213); **live prefetch wire** — horizon (PH-S268 doc pointer); replenish §5.13 |
| Signed capability documents | Galaxy §6.6 / §9 | post-S272 horizon |

**§5.5 prefetch (PH-S268):** Prometheus stubs + stand smoke покривають `plan_prefetch` / bytes / shard locality; повний live prefetch ingest (task-driven seed pull) — наступна code-first смуга з §5.13, не дублювати metrics-only спринти.

---

## 5. Пов’язані документи

- [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md) — концепт v1
- [`PROMETHEUS_METRICS.md`](./PROMETHEUS_METRICS.md) — stand smoke band PH-S244…S256
- [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md) — ratio **94.34%**
