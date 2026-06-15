# Galaxy Grid roadmap (зріз)

**Оновлено:** 2026-06-15 · **Канон черги:** [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12 (**4** відкритих PH-S197…S200)

| Зріз | Значення |
|------|----------|
| **Wire band** | PH-S65…S187 ✅ |
| **Vision UX** | PH-S188…S192 ✅ · dock bar rev **132** |
| **Code band** | PH-S193…S196 ✅ |
| **Відкрито** | **4** — PH-S197…S200 (vision + code-first) |
| **Після S200** | replenish §5.12 (≤10) |

---

## 3. Черга §5.12 (PH-S197…S200)

| # | Sprint | Тема | Acceptance |
|---|--------|------|------------|
| 1 | **PH-S197** | updates-compat wasm | admin panel wiring |
| 2 | **PH-S198** | Topology Rust labels | slim `topology_graph.js` |
| 3 | **PH-S199** | feed.json RSS ticker | vision panel |
| 4 | **PH-S200** | Cursor post-push hook | `.cursor/hooks` |

**Закрито:** PH-S196 ✅ — `poolai-http-stand-smoke --lease-renew`; PH-S195 ✅ — `GET /api/v1/grid/seed-inventory`; PH-S194 ✅ — fee split counter.

Повна таблиця — FM **§5.12** · [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md).

---

## 4. Після S200

Replenish §5.12 (≤10 відкритих) з §5.13 / code-first backlog.
