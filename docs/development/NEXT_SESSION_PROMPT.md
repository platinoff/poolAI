# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-13 · PH-S142 ✅ · vision **rev 73** · **10-спринтова смуга** PH-S143…S150

| **← наступний** | **PH-S143** — LOC ratio baseline audit |
| **Відкритих** | **10** (PH-S143…S150) |
| **Канон ratio** | [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md) · FM §5.12 |

## Черга (1 PH-S* = 1 сесія = 1 commit)

| # | Sprint | Scope | Acceptance (коротко) |
|---|--------|-------|----------------------|
| 1 | **PH-S143** | audit bin/script | LOC ratio baseline report; FM sync |
| 2 | **PH-S144** | `tests/` migration | legacy API Playwright → Rust integration; `cargo test-ci` |
| 3 | **PH-S145** | `src/bin/poolai-http-stand-smoke` | Rust HTTP stand smoke; RUN_LOCAL doc |
| 4 | **PH-S146** | `crates/poolai-ui-core` | shared validators/formatters + unit tests |
| 5 | **PH-S147** | wasm32 POC | one admin helper module + portability docs |
| 6 | **PH-S148** | `e2e/` slim | `test:ci` browser-only; ratio ≥90% |
| 7 | **PH-S150** | CI ops | ratio advisory if Rust share <88% |

**VDT:** `cargo fmt` → `cargo test-ci` (+ openapi-gap / e2e за scope) → FM/HANDOFF/NEXT_SESSION/vision → MSYS2 push.

---

## Copy-paste — наступна сесія (PH-S143)

```
PoolAI — спринт PH-S143 (одin PH-S*, VDT).
PH-S143: LOC ratio baseline audit bin/script; FM sync; cargo test-ci
FM/HANDOFF/NEXT_SESSION/vision revision++
Канон: docs/development/RUST_RATIO_STRATEGY_2026-06-13.md (90–95% Rust)
```

## Шаблон для PH-S144…S150 (замінити NNN і scope з таблиці)

```
PoolAI — спринт PH-SNNN (одin PH-S*, VDT).
PH-SNNN: <acceptance з FM §5.12>
cargo test-ci (+ e2e лише якщо src/ui/ або axe/visual у scope)
FM/HANDOFF/NEXT_SESSION/vision revision++
```
