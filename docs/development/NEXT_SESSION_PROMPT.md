# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-13 · PH-S144 ✅ · vision **rev 75** · **rust_ratio 91.91%** · PH-S145…S150

| **← наступний** | **PH-S145** — `poolai-http-stand-smoke` bin |
| **Відкритих** | **9** (PH-S145…S150) |
| **Канон ratio** | [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md) · [`rust_ratio.json`](./rust_ratio.json) |

## Черга (1 PH-S* = 1 сесія = 1 commit)

| # | Sprint | Scope | Acceptance (коротко) |
|---|--------|-------|----------------------|
| 1 | **PH-S145** | `src/bin/poolai-http-stand-smoke` | Rust HTTP stand smoke; RUN_LOCAL doc |
| 2 | **PH-S146** | `crates/poolai-ui-core` | shared validators/formatters + unit tests |
| 3 | **PH-S147** | wasm32 POC | one admin helper module + portability docs |
| 4 | **PH-S148** | `e2e/` slim | `test:ci` browser-only; ratio ≥90% |
| 5 | **PH-S150** | CI ops | ratio advisory if Rust share <88% |

**VDT:** `cargo fmt` → `cargo test-ci` → FM/HANDOFF/NEXT_SESSION/vision → MSYS2 push.

---

## Copy-paste — наступна сесія (PH-S145)

```
PoolAI — спринт PH-S145 (одin PH-S*, VDT).
PH-S145: poolai-http-stand-smoke bin (reqwest + stand env); RUN_LOCAL doc
FM/HANDOFF/NEXT_SESSION/vision revision++
Канон: docs/development/RUST_RATIO_STRATEGY_2026-06-13.md (90–95% Rust)
```
