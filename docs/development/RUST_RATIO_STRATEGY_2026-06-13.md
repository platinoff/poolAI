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

**Черга §5.12 (10 відкритих):** PH-S210…S219 code-first + vision a11y band.

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
| 23 | **PH-S166** | Design tokens CSS → Rust | `design_tokens.rs`; slim CSS `:root` | **✅** |
| 24 | **PH-S167** | Galaxy prefetch metrics stub | Prometheus on `plan_prefetch` | **✅** |
| 25 | **PH-S168** | Galaxy pricing cache age /metrics | `galaxy_pricing_cache_age_seconds` gauge | **✅** |
| 26 | **PH-S169** | Locality stale profile penalty | `stale_network_profile_penalty` stub | **✅** |
| 27 | **PH-S170** | Galaxy settlement pending_verification stub | `SettlementStatus` on grid result | **✅** |
| 28 | **PH-S171** | Galaxy replication strict tier stub | §6.3 `replication_strict` config | **✅** |
| 29 | **PH-S172** | Galaxy pricing provider catalog metrics stub | §4.2 catalog hits | **✅** |
| 30 | **PH-S173** | Galaxy pricing provider errors metrics stub | §4.2 provider fetch fail | **✅** |
| 31 | **PH-S174** | Galaxy pricing quote usd_micro metrics stub | §4.2 last quote gauge | **✅** |
| 32 | **PH-S175** | Galaxy verification mismatch metrics stub | §6.2 mismatch counter | **✅** |
| 33 | **PH-S176** | Galaxy replay pending metrics stub | §6.3 replay pending gauge | **✅** |
| 34 | **PH-S177** | Galaxy verification sample total metrics stub | §6.2 sample counter | **✅** |
| 35 | **PH-S178** | Galaxy settlement pending_verification metrics stub | §6.4 grid result path | **✅** |
| 36 | **PH-S179** | Galaxy replication strict tier metrics stub | §6.3 grid job ingest | **✅** |
| 37 | **PH-S180** | Galaxy verification match metrics stub | §6.2 match counter | **✅** |
| 38 | **PH-S181** | Galaxy pricing market min usd_micro metrics stub | §4.2 market min gauge | **✅** |
| 39 | **PH-S182** | Galaxy trust score metrics stub | §6.2 trust score gauge | **✅** |
| 40 | **PH-S183** | Galaxy shard local hit ratio metrics stub | §5.3 locality gauge | **✅** |
| 41 | **PH-S184** | Galaxy prefetch bytes total metrics stub | §5.5 prefetch path | **✅** |
| 42 | **PH-S185** | Galaxy cross region egress mb metrics stub | §5.3 rank/prefetch | **✅** |
| 43 | **PH-S186** | Galaxy verification sample scheduled /metrics export | §6.2 PH-S164 export | **✅** |
| 44 | **PH-S187** | Galaxy settlement cleared total metrics stub | §6.4 Cleared path | **✅** |
| 45 | **PH-S188** | Vision map filters UX | docs/vision filters | **✅** |
| 46 | **PH-S189** | Vision Eco/FX/Ms hover trace | docs/vision Ms mode | **✅** |
| 47 | **PH-S190** | Vision filter dropdowns + panel collapse | docs/vision layout | **✅** |
| 48 | **PH-S191** | Vision sprint queue panel | FM §5.12 parse | **✅** |
| 49 | **PH-S192** | Vision overview LOD + minimap | docs/vision minimap | **✅** |
| 50 | **PH-S193** | Dashboard shell formatters → wasm | poolai-ui-core | **✅** |
| 51 | **PH-S194** | Galaxy fee split result counter | §4.1 stub | **✅** |
| 52 | **PH-S195** | Galaxy seed_inventory GET | §5.5 wire | **✅** |
| 53 | **PH-S196** | Stand smoke lease renew | poolai-http-stand-smoke | **✅** |
| 54 | **PH-S197** | updates-compat wasm | admin UI | **✅** |
| 55 | **PH-S198** | Topology Rust labels slim | PH-S157 | **✅** |
| 56 | **PH-S199** | Vision map Ms hit-test + focus nav | docs/vision | **✅** |
| 57 | **PH-S200** | Vision feed.json RSS | docs/vision | відкрито |
| 58 | **PH-S201** | Cursor post-push hook | VDT ops | відкрито |
| 59 | **PH-S202** | Vision sprint-queue → map focus | docs/vision | відкрито |
| 60 | **PH-S203** | Vision keyboard nav nodes | docs/vision | відкрито |
| 61 | **PH-S204** | Vision edge click select | docs/vision | відкрито |
| 62 | **PH-S205** | poolai-vision-sync drift gate | ops | відкрито |
| 63 | **PH-S206** | Vision minimap selection ring | docs/vision | відкрито |
| 64 | **PH-S207** | Admin i18n slim next panel | code | відкрито |
| 65 | **PH-S208** | Stand smoke vision rev parity | tests | відкрито |
| 66 | **PH-S209** | Vision map a11y focus ring | docs/vision | **✅** |
| 67 | **PH-S210** | Stand smoke seed_inventory GET | tests | **✅** |
| 68 | **PH-S211** | Admin i18n slim jobs panel | code | **✅** |
| 69 | **PH-S212** | Vision reduced-motion map FX | docs/vision | **✅** |
| 70 | **PH-S213** | Galaxy prefetch metrics stand smoke | tests | **✅** |
| 71 | **PH-S214** | Admin i18n slim raid panel | code | **✅** |
| 72 | **PH-S215** | Vision panel collapse focus restore | docs/vision | **✅** |
| 73 | **PH-S216** | Galaxy pricing fallback metrics smoke | tests | **✅** |
| 74 | **PH-S217** | Admin i18n slim grid-pricing panel | code | **✅** |
| 75 | **PH-S218** | Vision map aria-live selection | docs/vision | **✅** |
| 76 | **PH-S219** | Galaxy trust payout metrics smoke | tests | **✅** |
| 77 | **PH-S220** | Admin i18n slim monitoring panel | code | **✅** |
| 78 | **PH-S221** | Admin i18n slim updates-compat panel | code | **✅** |
| 79 | **PH-S222** | Admin i18n slim workers panel | code | **✅** |
| 80 | **PH-S223** | Admin i18n slim libs panel | code | **✅** |
| 81 | **PH-S224** | Galaxy pricing cache age metrics smoke | tests | **✅** |
| 82 | **PH-S225** | Galaxy verification sample metrics smoke | tests | **✅** |
| 83 | **PH-S226** | Vision sprint-queue → map focus | docs/vision | **✅** |
| 84 | **PH-S227** | Vision VDT rules docs autosync audit | docs/vision | **✅** |
| 85 | **PH-S228** | Admin i18n slim dashboard panel | code | **✅** |
| 86 | **PH-S229** | Admin i18n slim audit panel | code | **✅** |
| 87 | **PH-S230** | Admin i18n slim tenants panel | code | **✅** |
| 88 | **PH-S231** | Admin i18n slim security panel | code | **✅** |
| 89 | **PH-S232** | Galaxy replication metrics stand smoke | tests | **✅** |
| 90 | **PH-S233** | Vision map sprint chips a11y | docs/vision | **✅** |
| 91 | **PH-S234** | Admin i18n slim topology panel | code | **✅** |
| 92 | **PH-S235** | Stand smoke vision rev parity | tests | **✅** |
| 93 | **PH-S236** | Admin i18n slim instances panel | code | **✅** |
| 94 | **PH-S237** | Admin i18n slim vm panel | code | **✅** |
| 95 | **PH-S238** | Admin i18n slim users panel | code | **✅** |
| 96 | **PH-S239** | Admin i18n slim config panel | code | **✅** |
| 97 | **PH-S240** | Admin i18n slim table toolbar | code | **✅** |
| 98 | **PH-S241** | Galaxy pricing fresh served metrics stand smoke | tests | **✅** |
| 99 | **PH-S242** | Admin i18n nav shell key audit | code | **✅** |
| 100 | **PH-S243** | Admin i18n slim admin chrome shell | code | **✅** |
| 101 | **PH-S244** | Galaxy pricing stale served metrics stand smoke | tests | відкрито |
| 102 | **PH-S245** | Admin shared status keys slim patch | code | відкрито |
| 103 | **PH-S246** | Admin err hint keys slim patch | code | відкрито |
| 104 | **PH-S247** | Galaxy pricing provider metrics stand smoke | tests | відкрито |
| 105 | **PH-S248** | Admin vm modal i18n slim | code | відкрито |
| 106 | **PH-S249** | Galaxy settlement metrics stand smoke | tests | відкрито |
| 107 | **PH-S250** | Galaxy shard locality metrics stand smoke | tests | відкрито |
| 108 | **PH-S251** | Docs roadmap sync band | docs | відкрито |
| 109 | **PH-S252** | Admin shared ui.confirm slim patch | code | відкрито |

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
