# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-17 · replenish **PH-S283…S292** · vision **rev 208** · **10** відкритих у §5.12 · rust_ratio **94.36%** · hold **95%** advisory

| **← наступний** | **PH-S283** — Galaxy prefetch enqueue wire stub |
| **Відкритих** | **10** (S283…S292) |

---

## Copy-paste — ітераційна сесія (VDT)

```
S0: git fetch; HANDOFF; FM §5.12 (10 відкритих); df -h /s

PH-S283: Galaxy prefetch enqueue wire stub
- scope: src/grid/dispatch.rs, galaxy_prefetch_metrics
- cargo fmt --all → cargo test grid::dispatch::tests
- FM/HANDOFF/NEXT + poolai-vision-sync --check
```

---

## Replenish band (post-S282)

| Sprint | Фокус |
|--------|--------|
| **PH-S283** | `enqueue_prefetch_hook` wire stub (Galaxy §5.5) |
| **PH-S284** | `render_line_chart_html` wasm — slim line chart JS |
| **PH-S285** | Locality rank on grid job ingest |
| **PH-S286** | Stand smoke prefetch enqueue |
| **PH-S287** | `poolaiGroupMetricsByName` wasm |
| **PH-S288…S292** | loc-audit, docs sync, vision, hold advisory, INDEX |

**rust_ratio:** **94.36%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
