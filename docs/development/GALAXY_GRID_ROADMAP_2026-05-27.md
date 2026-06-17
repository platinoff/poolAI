# Galaxy Grid roadmap (зріз)

**Оновлено:** 2026-06-17 · **Канон черги:** [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12 (**10** відкритих · PH-S283…S292)

| Зріз | Значення |
|------|----------|
| **Wire band** | PH-S65…S187 ✅ |
| **Vision UX** | PH-S188…S233 ✅ |
| **Ops / stand smoke** | PH-S196…S256 ✅ |
| **Admin i18n slim** | PH-S207…S266 ✅ · `i18n_core.js` STRINGS core **empty** |
| **Wasm / admin slim** | PH-S273…S275 ✅ |
| **Docs / ratio** | PH-S278…S282 ✅ |
| **Rust ratio** | **94.36%** hold **95%** advisory · spirit **96%** |

---

## 3. Черга §5.12 (активна смуга)

| Sprint | Scope | Стан |
|--------|-------|------|
| **PH-S283** | Galaxy prefetch **enqueue** wire stub | відкрито |
| **PH-S284** | admin line chart wasm HTML | відкрито |
| **PH-S285** | Locality rank job ingest stub | відкрито |
| **PH-S286** | Stand smoke prefetch enqueue | відкрито |
| **PH-S287** | admin_charts metric group wasm | відкрито |
| **PH-S288…S292** | loc-audit + docs + vision maintain | відкрито |

Повна таблиця — FM **§5.12** · [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md).

---

## 4. Концепт ↔ код (не зроблено / horizon)

| Тема | Джерело | Статус |
|------|---------|--------|
| LAN replication benchmarks | Architect P4 / FM-003 §4 | **BLOCKED** (2 хости) |
| Cloud SDK live (AWS/Azure/GCP) | FM-041 Deferred | infra only |
| ZK / TEE attestation | Galaxy §6.6 | roadmap only |
| Telegram wallet wire extension | Galaxy §3.2 | **wire ✅**; stand smoke `telegram_wallet` ✅ |
| **Prefetch enqueue (live pull)** | Galaxy **§5.5** | **ingest stub ✅** (PH-S276); **enqueue stub** — PH-S283 |
| Signed capability documents | Galaxy §6.6 / §9 | post-S292 horizon |

**§5.5 prefetch (PH-S268, PH-S276, PH-S283):** metrics + ingest `plan_prefetch`; наступний крок — **enqueue hook stub** (без live seed pull); повний wire — після S292.

---

## 5. Пов’язані документи

- [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md) — концепт v1
- [`PROMETHEUS_METRICS.md`](./PROMETHEUS_METRICS.md) — stand smoke band PH-S244…S256
- [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md) — ratio **94.36%**
