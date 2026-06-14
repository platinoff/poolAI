# Rust codebase ratio — стратегія 90–95% (PoolAI)

**Оновлено:** 2026-06-13 · **Канон:** FM **§5.13** · правила [`.cursor/rules/runtime-stack-policy.mdc`](../../.cursor/rules/runtime-stack-policy.mdc), [`.cursor/rules/poolai-testing-policy.mdc`](../../.cursor/rules/poolai-testing-policy.mdc)

**Мета:** зростання частки **Rust** у виконуваному коді репозиторію до **90–95%** (формально), **96% stretch spirit** (орієнтир replenish PH-S150…S159) — платформа збирається і перевіряється через **`cargo`** без обов'язкового Node на edge.

---

## 1. Ціль і вимір

| Показник | Ціль | Коментар |
|----------|------|----------|
| **Rust LOC share** | **90–95%** (formal) · **96% stretch** | `src/`, `tests/`, `crates/`, `src/bin/` |
| **Non-Rust (допустимо)** | **5–10%** | `src/ui/*.js` (glue), `e2e/*.ts` (лише browser), `bin/*.sh` (ops) |
| **Поза ratio** | docs, `.md`, CI yaml, snapshots PNG | не входять у знаменник «коду продукту» |

**Орієнтовний зріз (2026-06-13, PH-S150):** **`92.00%`** Rust LOC (`cargo run --bin poolai-loc-audit` → [`rust_ratio.json`](./rust_ratio.json)). Non-Rust «шум»: **`i18n_core.js`** (~2k LOC), **`admin_common.js`**, **`admin_charts.js`**, browser-only `e2e/tests/`, ops shell.

**Audit:** `cargo run --bin poolai-loc-audit` — звіт `docs/development/rust_ratio.json` для FM §5.13 / PH-S151…S170 gates. CI hold advisory (PH-S165): `cargo run --bin poolai-loc-audit -- --warn-below 0.93 --target 0.95 --stretch 0.96 --min-ratio 0.95 --advisory`.

---

## 2. Портативність «працює де завгодно»

| Шар | Технологія | Пристрої |
|-----|------------|----------|
| **Coordinator / worker / tools** | Rust binaries | Linux, Windows, macOS; ARM/x86; без JVM/Python |
| **HTTP API + business rules** | Rust `src/` | будь-який клієнт (curl, mobile, bot) |
| **Admin UI (поточний)** | HTML + thin JS | будь-який браузер; Node **не** на production host |
| **Admin UI (wasm POC PH-S147 ✅)** | **`crates/poolai-ui-wasm`** → `src/ui/wasm/` via `bash bin/build-ui-wasm.sh` | grid-pricing + lease helpers; shared logic з `poolai-ui-core` |
| **Admin UI (горизонт)** | повне підключення wasm у admin panels | post-POC; thin JS лишається DOM glue |
| **Перевірка якості** | **`cargo test-ci`** | dev/CI на будь-якій машині з Rust toolchain |
| **Browser regression** | Playwright (мінімум) | лише CI/dev з Node; не на edge nodes |

**Принцип:** нові можливості — **Rust-first**; JS/TS — лише те, що браузер не отримує з WASM/DOM glue; Node — лише harness для axe/visual/admin smoke.

**Portable deploy (PH-S149 sync, коротко):**

| Surface | Coordinator / worker | Admin UI | Перевірка без Node |
|---------|----------------------|----------|-------------------|
| Linux / Windows dev | `cargo build --release` | HTML + JS; wasm via `bash bin/build-ui-wasm.sh` | `cargo test-ci`, `poolai-http-stand-smoke` |
| Edge node | Rust binary only | static UI (optional wasm) | curl / Rust smoke bins |

---

## 3. Піраміда розробки (узгоджено з testing policy)

```mermaid
flowchart TB
  subgraph rust [Rust 90-95%]
    SRC[src/ domains]
    TESTS[tests/ integration + contracts]
    CRATES[crates/]
    BINS[src/bin/ tools]
  end
  subgraph thin [Thin non-Rust 5-10%]
    UI[src/ui/ JS glue]
    E2E[e2e/ browser only]
    OPS[bin/*.sh ops]
  end
  SRC --> TESTS
  NEW[New PH-S* API feature] --> TESTS
  NEW --> SRC
  UI --> E2E
```

| Тип задачі | Куди писати | Не робити |
|------------|-------------|-----------|
| API, grid, job, discovery, telegram wire | `src/` + `tests/*_integration.rs` | новий `e2e/tests/*.spec.ts` для HTTP-only |
| Валідація, pricing, trust, locality | `src/grid/`, unit + integration | дубль логіки в JS |
| Admin read-only panel | `src/ui/admin/*.rs` HTML + мінімум JS | великі JS modules |
| DOM / a11y / theme / visual | `e2e/tests/smoke|admin|a11y|visual` | — |
| Shared UI↔server rules | `crates/poolai-ui-core` (PH-S146) → wasm (PH-S147) | copy-paste у TS |

---

## 4. Фази (роадмеп)

| Фаза | Коли | Дія | Ratio |
|------|------|-----|-------|
| **A — freeze** | зараз (PH-S140…S142) | нові API acceptance → Rust tests; UI спринти — thin JS | утримати ≥88% |
| **B — dedupe** | §5.13 PH-S144…S145 | перенести legacy API Playwright → Rust; HTTP stand smoke bin | +3–5% |
| **C — UI core** | PH-S146…S147 | shared Rust crate + wasm32 POC для admin helpers | +2–4% |
| **D — slim e2e** | PH-S148 | `e2e/` лише smoke/admin/a11y/visual; прибрати API specs з `test:ci` | +1–2% |
| **E — gate** | PH-S150 ✅ | CI advisory якщо ratio <88%; target 93%; stretch 96% spirit | стабільно 90–95% |
| **F — stretch 96%** | PH-S151…S159 ✅ | wasm wiring, slim JS/i18n/charts, Rust stand/e2e bins; CI warn **93%** | **→96% spirit** |

**Черга §5.12 (10 відкритих):** PH-S166…S170 — post-S165 replenish; replenish після S170.

---

## 5. FM §5.13 — черга PH-S143…S159

| # | Sprint | Фокус | Acceptance |
|---|--------|--------|------------|
| 1 | **PH-S143** | LOC ratio baseline audit | `cargo run --bin poolai-loc-audit`; [`rust_ratio.json`](./rust_ratio.json) **91.48%**; FM оновлено | **✅** |
| 2 | **PH-S144** | Playwright API → Rust migration | `jobs_lease`, `grid_*`, `protocol_middleware`, `telegram_wallet`, `jobs_migrating` покриті integration; Playwright specs archived або deleted | **✅** |
| 3 | **PH-S145** | `poolai-http-stand-smoke` bin (Rust) | `reqwest` + stand env; RUN_LOCAL doc; `--raid` + `POOLAI_E2E_STAND_ROOT` | **✅** |
| 4 | **PH-S146** | `crates/poolai-ui-core` stub | shared validators/formatters з admin JS винесені в Rust crate + unit tests | **✅** |
| 5 | **PH-S147** | wasm32 admin core POC | один panel helper compiled to wasm; docs portability §2 | **✅** |
| 6 | **PH-S148** | Slim `e2e/` | `test:ci` без API patterns; ratio ≥90% | **✅** |
| 7 | **PH-S150** | Ratio CI advisory | CI `rust-ratio-audit`; `--warn-below 0.88` `--target 0.93` `--stretch 0.96` `--advisory`; **92.00%** | **✅** |
| 8 | **PH-S151** | wasm grid-pricing wiring | `/ui/wasm/*` + grid-pricing module; Playwright smoke | **✅** |
| 9 | **PH-S152** | wasm jobs lease display | shared `POOLAI_UI_WASM_MODULE`; jobs `leaseStateLabel`; Playwright smoke | **✅** |
| 10 | **PH-S153** | admin_common slim | `table.rs` + wasm; admin_common −426 LOC | **✅** |
| 11 | **PH-S154** | Admin i18n subset Rust | slim `i18n_core.js` admin keys | **✅** |
| 12 | **PH-S155** | ML charts → wasm | admin_charts canvas-only glue | **✅** |
| 13 | **PH-S156** | jobs_raid → Rust smoke | drop `jobs_raid` from `test:ci` | **✅** |
| 14 | **PH-S157** | topology SVG Rust | slim `topology_graph.js` | **✅** |
| 15 | **PH-S158** | `poolai-e2e-stand` bin | Rust stand lifecycle; slim shell | **✅** |
| 16 | **PH-S159** | Ratio **96%** stretch gate | warn 93%; stretch 96%; replenish post-S159 | **✅** |
| 17 | **PH-S160** | Admin theme → Rust | slim `admin_theme.js` | **✅** |
| 18 | **PH-S161** | Admin modal a11y → wasm | slim `admin_modal_a11y.js` | **✅** |
| 19 | **PH-S162** | Auth i18n subset Rust | slim `i18n_core.js` auth block | ✅ |
| 20 | **PH-S163** | Galaxy trust metrics wire | Prometheus on grid result path | ✅ |
| 21 | **PH-S164** | Verify sampling env apply | `galaxy_verify_sampling` HTTP stub | ✅ |
| 22 | **PH-S165** | Ratio **96%** hold gate | CI `--min-ratio 0.95` advisory; target **95%**; replenish post-S165 | **✅** |
| 23 | **PH-S166** | Design tokens CSS → Rust | slim `design_tokens.css` / `admin_styles.css` | відкрито |
| 24 | **PH-S167** | Galaxy prefetch metrics stub | Prometheus on `plan_prefetch` | відкрито |
| 25 | **PH-S168** | Galaxy pricing cache age /metrics | `galaxy_pricing_cache_age_seconds` gauge | відкрито |
| 26 | **PH-S169** | Locality stale profile penalty | `galaxy_locality.rs` stub | відкрито |
| 27 | **PH-S170** | Galaxy settlement pending_verification stub | §6.4 grid result path | відкрито |

*(PH-S149 — portable deploy matrix docs — закрито sync у PH-S147 §2.)*

---

## 6. Правила агента / VDT (коротко)

1. **Новий PH-S* (API):** `tests/` + `cargo test-ci` — див. [`poolai-testing-policy.mdc`](../../.cursor/rules/poolai-testing-policy.mdc).
2. **Replenish §5.12:** пріоритет code-first Rust; Playwright — лише якщо acceptance явно вимагає browser.
3. **Після PH-S142:** replenish з **§5.13**, не з нових Playwright API specs.
4. **MSYS2:** `export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"` — для UI E2E; API scope — лише `cargo`.

---

## 7. Пов'язані документи

| Документ | Роль |
|----------|------|
| [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12, **§5.13** | черги PH-S* |
| [`GALAXY_GRID_ROADMAP_2026-05-27.md`](./GALAXY_GRID_ROADMAP_2026-05-27.md) | Galaxy + ratio фази |
| [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](./NEXT_STEPS_ARCHITECT_2026-03-17.md) | **P8** Rust ratio |
| [`E2E_PLAYWRIGHT.md`](./E2E_PLAYWRIGHT.md) | browser-only E2E |
| [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) | операційний зріз |
