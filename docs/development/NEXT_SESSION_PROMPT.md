# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-13 · PH-S143 ✅ · vision **rev 74** · **rust_ratio 91.48%** · PH-S144…S150

| **← наступний** | **PH-S144** — Playwright API → Rust migration |
| **Відкритих** | **10** (PH-S144…S150) |
| **Канон ratio** | [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md) · [`rust_ratio.json`](./rust_ratio.json) |

## Черга (1 PH-S* = 1 сесія = 1 commit)

| # | Sprint | Scope | Acceptance (коротко) |
|---|--------|-------|----------------------|
| 1 | **PH-S144** | `tests/` migration | legacy API Playwright → Rust integration; `cargo test-ci` |
| 2 | **PH-S145** | `src/bin/poolai-http-stand-smoke` | Rust HTTP stand smoke; RUN_LOCAL doc |
| 3 | **PH-S146** | `crates/poolai-ui-core` | shared validators/formatters + unit tests |
| 4 | **PH-S147** | wasm32 POC | one admin helper module + portability docs |
| 5 | **PH-S148** | `e2e/` slim | `test:ci` browser-only; ratio ≥90% |
| 6 | **PH-S150** | CI ops | ratio advisory if Rust share <88% |

**VDT:** `cargo fmt` → `cargo test-ci` → FM/HANDOFF/NEXT_SESSION/vision → MSYS2 push.

---

## Copy-paste — наступна сесія (PH-S144)

```
PoolAI — спринт PH-S144 (одin PH-S*, VDT).
PH-S144: legacy Playwright API specs → tests/*_integration.rs; cargo test-ci
Без нового Playwright API spec. FM/HANDOFF/NEXT_SESSION/vision revision++
Канон: docs/development/RUST_RATIO_STRATEGY_2026-06-13.md (90–95% Rust)
```
