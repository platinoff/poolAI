# Galaxy Grid roadmap (зріз)

**Оновлено:** 2026-06-15 · **Канон черги:** [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12 (**9** відкритих PH-S192…S200)

| Зріз | Значення |
|------|----------|
| **Wire band** | PH-S65…S187 ✅ |
| **Vision UX** | PH-S188…S191 ✅ |
| **Відкрито** | **9** — PH-S192…S200 (vision + code-first) |
| **Після S200** | replenish §5.12 (≤10) |

---

## 1. Закрито (концепт → wire)

Див. FM §5.12 рядки PH-S55…S187 · [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md) §4–§6 implemented tables.

---

## 2. Vision UX (PH-S188…S191) ✅

| Sprint | Тема |
|--------|------|
| PH-S188 ✅ | Map filters — independent layer/type toggles |
| PH-S189 ✅ | Eco/FX/Ms hover trace |
| PH-S190 ✅ | Filter dropdowns + panel collapse strip |
| PH-S191 ✅ | Sprint queue panel (FM §5.12 parse) |

---

## 3. Черга §5.12 (PH-S192…S200)

| # | Sprint | Тема | Acceptance |
|---|--------|------|------------|
| 1 | **PH-S192** | Overview LOD + minimap | hub labels; viewport inset |
| 2 | **PH-S193** | Dashboard wasm formatters | `poolai-ui-core` + wasm |
| 3 | **PH-S194** | Fee split result counter | grid result path stub |
| 4 | **PH-S195** | seed_inventory GET | OpenAPI + integration test |
| 5 | **PH-S196** | Stand smoke lease renew | `poolai-http-stand-smoke` |
| 6 | **PH-S197** | updates-compat wasm | admin panel wiring |
| 7 | **PH-S198** | Topology Rust labels | slim `topology_graph.js` |
| 8 | **PH-S199** | feed.json RSS ticker | vision panel |
| 9 | **PH-S200** | Cursor post-push hook | `.cursor/hooks` |

Повна таблиця — FM **§5.12** · [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md).

---

## 4. Rust ratio (§5.13)

Hold **95%** · spirit **96%** · baseline **92.78%** · replenish після S200.
