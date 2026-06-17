# Galaxy Grid roadmap (зріз)

**Оновлено:** 2026-06-17 · **Канон черги:** [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12 (**8** відкритих PH-S245…S252)

| Зріз | Значення |
|------|----------|
| **Wire band** | PH-S65…S187 ✅ |
| **Vision UX** | PH-S188…S233 ✅ |
| **Ops / stand smoke** | PH-S196…S244 ✅ · band S247…S250 — **відкрито** |
| **Admin i18n slim** | PH-S207…S243 ✅ · band S245…S246, S248, S252 — **відкрито** |
| **Docs sync** | PH-S251 — roadmap + README + INDEX zriz |
| **Rust ratio** | **92.78%** hold **95%** · spirit **96%** |

---

## 3. Черга §5.12 (PH-S245…S252)

| # | Sprint | Тема | Acceptance |
|---|--------|------|------------|
| 1 | **PH-S245** | Admin status keys slim | `admin.status.*` + `admin.na` + `admin.btn.edit` patch |
| 2 | **PH-S246** | Admin err hint keys slim | `err.hint*` + insufficientAdmin + accessRequired |
| 3 | **PH-S247** | Pricing provider metrics smoke | catalog lookups/hits + provider errors on `/metrics` |
| 4 | **PH-S248** | VM modal i18n slim | `vm.*` modal keys out of `i18n_core.js` |
| 5 | **PH-S249** | Settlement metrics smoke | settlement pending + cleared on `/metrics` |
| 6 | **PH-S250** | Shard locality metrics smoke | `galaxy_shard_local_hit_ratio` on `/metrics` |
| 7 | **PH-S251** | Docs roadmap sync | GALAXY_GRID_ROADMAP + README + INDEX sprint zriz |
| 8 | **PH-S252** | UI confirm modal slim | `ui.confirm*` + modal glue keys patch |

Повна таблиця — FM **§5.12** · [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md) · [`NEXT_SESSION_PROMPT.md`](./NEXT_SESSION_PROMPT.md).

---

## 4. Концепт ↔ код (не зроблено / horizon)

| Тема | Джерело | Статус |
|------|---------|--------|
| LAN replication benchmarks | Architect P4 / FM-003 §4 | **BLOCKED** (2 хости) |
| Cloud SDK live (AWS/Azure/GCP) | FM-041 Deferred | infra only |
| ZK / TEE attestation | Galaxy §6.6 | roadmap only |
| Telegram wallet wire extension | Galaxy §3.2 | post-MVP |

Stand smoke band закриває **Prometheus export parity** для Galaxy pricing / settlement / locality — див. [`PROMETHEUS_METRICS.md`](./PROMETHEUS_METRICS.md), [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md) §4.2 / §5.3 / §6.4.
