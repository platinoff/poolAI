# Передача контексту новій сесії (PoolAI)

**Оновлено:** 2026-07-18 (band 41 **PH-S1049…S1058** ✅ · horizon bands 42–47)

**Наступна сесія:** **`абракадабра`** — project scan → band 42.

## Band 41 — E2E visual/axe regression (PH-S1049…S1058, ✅)

| Sprint | Фокус |
|--------|--------|
| **PH-S1049** | Visual parity tier-1 (config, jobs) |
| **PH-S1050** | Visual parity tier-2 grid panels (updates, seed, advisories) |
| **PH-S1051** | Vision axe smoke |
| **PH-S1052** | Vision map visual snapshot |
| **PH-S1053** | High-contrast axe extend |
| **PH-S1054** | `waitForVisualSnapshotReady` helper |
| **PH-S1055** | e2e scope visual/axe parity gate |
| **PH-S1056** | rust_ratio loc-audit |
| **PH-S1057** | `e2e_visual_axe_depth` ui-core stub |
| **PH-S1058** | Band close |

**§5.12:** **0** відкритих (band 41 ✅). **Vision:** rev **313**.

**PH-S1058 ✅ (2026-07-18):** Visual snapshots for 8 admin routes; vision axe + HC extend; `galaxy_horizon_s1049_integration`; vision **rev 313**.

## Band 40 — Vision map/a11y/perf (PH-S1039…S1048, ✅)

| Sprint | Фокус |
|--------|--------|
| **PH-S1039** | Skip links + landmarks (`role="main"`) |
| **PH-S1040** | Icon control `aria-label` / `aria-pressed` parity |
| **PH-S1041** | Explorer tree keyboard (`role="tree"`, Arrow keys) |
| **PH-S1042** | Link graph neighbour focus + Enter select |
| **PH-S1043** | Sprint-dim incremental (`updateMapSprintDim`) |
| **PH-S1044** | Dense-map LOD threshold hardening (>120 nodes/layer) |
| **PH-S1045** | Background tab perf (pause starfield + orbit) |
| **PH-S1046** | `vision_map_depth` ui-core stub |
| **PH-S1047** | Vision Playwright smoke extend |
| **PH-S1048** | Band close |

**§5.12:** **0** відкритих (band 40 ✅). **Vision:** rev **312**.

**PH-S1048 ✅ (2026-07-18):** Vision skip links/landmarks; tree + link-graph a11y; sprint-dim incremental; dense LOD; tab-hidden perf; `galaxy_horizon_s1039_integration`; vision **rev 312**.

## Band 39 — Admin tables/forms polish (PH-S1029…S1038, ✅)

| Sprint | Фокус |
|--------|--------|
| **PH-S1029** | Empty-state parity (tenants, security) |
| **PH-S1030** | Security tables a11y polish |
| **PH-S1031** | Tenants + jobs table containers |
| **PH-S1032** | Instances + topology table init |
| **PH-S1033** | Grid panel tables (network/seed/advisories) |
| **PH-S1034** | Raid artifacts table polish |
| **PH-S1035** | Modal form `aria-required` parity |
| **PH-S1036** | Config + dashboard forms/empty states |
| **PH-S1037** | `admin_tables_forms_depth` stub |
| **PH-S1038** | Band close |

**§5.12:** **0** відкритих (band 39 ✅). **Vision:** rev **311**.

**PH-S1038 ✅ (2026-07-18):** FM-019 adoption — `adminEmptyStateHtml`/`adminInitTablesIn`/`aria-label` across admin tables; modal `aria-required`; `galaxy_horizon_s1029_integration`; vision **rev 311**.

## Band 38 — UI/debug polish (PH-S1019…S1028, ✅)

| Sprint | Фокус |
|--------|--------|
| **PH-S1019** | Vision power menu polish |
| **PH-S1020** | Admin power modal i18n |
| **PH-S1021** | Home UI power shortcut |
| **PH-S1022** | Clippy unused imports |
| **PH-S1023** | chrono ui-core fix |
| **PH-S1024** | admin duplicate test attr |
| **PH-S1025** | Design tokens audit |
| **PH-S1026** | Ops power feedback |
| **PH-S1027** | poolai-msys hardening |
| **PH-S1028** | Band close |

**§5.12:** **0** відкритих (band 38 ✅). **Vision:** rev **310**.

**PH-S1028 ✅ (2026-07-18):** Vision power a11y + announce; admin/home power i18n; `galaxy_horizon_s1019_integration`; `ui_debug_depth`; vision **rev 310**.

## Band 37 — owner ops UX v2 (PH-S1011…S1018, ✅)

| Sprint | Фокус |
|--------|--------|
| **PH-S1011** | Light compile profile (minimal features, швидша збірка) |
| **PH-S1012** | `run-poolai quick` — легкий запуск повного стенду |
| **PH-S1013** | Vision easy launch у README + `open-docs-vision` |
| **PH-S1014** | Збереження останніх параметрів запуску (`last_run.json`) |
| **PH-S1015** | Admin UI: кнопка power → modal виключити/перезавантажити |
| **PH-S1016** | API wire `POST /api/v1/ops/power` + integration test |
| **PH-S1017** | Vision UI: poweroff/reset + `localStorage` стану |
| **PH-S1018** | Band close: docs + `galaxy_horizon_s1011_integration` |

**§5.12:** **0** відкритих (band 37 ✅). **Vision:** rev **308**. Деталі — FM **§5.17** · [`NEXT_SESSION_PROMPT.md`](./NEXT_SESSION_PROMPT.md).

**PH-S1018 ✅ (2026-07-18):** band 37 close — `run-poolai quick`/`--light`; `last_run.json`; `POST /api/v1/ops/power`; admin + vision power UI; `galaxy_horizon_s1011_integration`; vision **rev 308**.

## Maintenance mode (PH-S1010)

Після **PH-S1010** / FM **§5.15** ✅ сесії працюють у **maintenance mode**:

| Крок | Дія |
|------|-----|
| S0 | `git fetch`; HANDOFF; FM **§5.15**; `poolai-vision-sync --check`; `df -h /s` |
| Scope | Лише BLOCKED/Deferred (FM-003 LAN, FM-041 Cloud SDK) або явний FM-horizon v2 за запитом власника |
| Тести | `cargo fmt --all` → `cargo test-ci` перед push |
| Docs | STABLE «development complete»; INDEX/DIGEST без нових PH-S* у §5.12 |
| **Не** | Автоматичний project scan / replenish §5.12 без запиту власника |

**Completion roadmap v2 (2026-06-20):** [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) — **351/351** PH-S660…S1010 ✅ · FM **§5.15** ✅. Реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md).

**PH-S1010 ✅ (2026-07-18):** STABLE «development complete» final; HANDOFF maintenance active; `admin_charts.js` wasm-only zriz; `product_complete_depth` + `galaxy_horizon_s1010_integration`; `poolai-loc-audit` → `rust_ratio.json` **≥95%**; vision **rev 306**.

**PH-S1000…S1009 ✅ (2026-07-18):** `multi_module_wire_smoke.rs` top 5 grid APIs (PH-S1000); `multi_module_admin_wasm_regression.rs` ui-core gate (PH-S1001); `multi_module_stand_smoke_audit.rs` + stand smoke `--json` (PH-S1002); cargo test-ci scope note final HANDOFF (PH-S1003); openapi-gap + test-ci dual gate FM (PH-S1004); `poolai-loc-audit` → `rust_ratio.json` **94.95%** (PH-S1005); `multi_module_depth_stub` + `galaxy_horizon_s1000_integration` (PH-S1009); vision **rev 305**.

**PH-S990…S999 ✅ (2026-07-18):** `telegram_wallet_integration.rs` (PH-S990); grid job lease canon extend (PH-S991); protocol middleware canon (PH-S992); jobs RAID restart canon (PH-S993); vm write lifecycle PH-S994; `poolai-loc-audit` → `rust_ratio.json` **94.94%** (PH-S995); `poolai-testing-policy` band 34 gap note (PH-S996); `integration_gap_depth_stub` + `galaxy_horizon_s990_integration` (PH-S999); vision **rev 304**.

**PH-S980…S989 ✅ (2026-07-18):** STABLE «Development complete (draft)» (PH-S980); INDEX product-complete zriz steps 1–12 (PH-S981); README Next Focus → maintenance prep (PH-S982); HANDOFF maintenance template (PH-S983); DEVELOPMENT_PROGRESS 100% code scope note (PH-S984); `poolai-loc-audit` → `rust_ratio.json` **94.92%** (PH-S985); FM **§5.15** draft (PH-S986); `stable_depth_stub` + `galaxy_horizon_s980_integration` (PH-S989); vision **rev 303**.

**PH-S970…S979 ✅ (2026-07-18):** Galaxy §1–3 implemented markers (PH-S970); §4–6 markers (PH-S971); §7–9 markers + §8 TBD/LAN BLOCKED (PH-S972/S973); GALAXY_GRID_ROADMAP horizon final (PH-S974); `poolai-loc-audit` → `rust_ratio.json` **94.92%** (PH-S975); INDEX concept cross-links (PH-S976); `concept_markers_depth_stub` + `galaxy_horizon_s970_integration` (PH-S979); vision **rev 302**.

**PH-S960…S969 ✅ (2026-07-17):** DOCS_LEGACY_AUDIT triage (PH-S960); flat docs stale banners (PH-S961); concept de-hype zriz (PH-S962); ARCHITECT FM §5.1 alignment (PH-S963); DOCS_LEGACY §5.3 batch (PH-S964); `poolai-loc-audit` → `rust_ratio.json` **94.92%** (PH-S965); INDEX step 12 FM pointer (PH-S966); `docs_legacy_depth_stub` + `galaxy_horizon_s960_integration` (PH-S969); vision **rev 300**.

**PH-S950…S959 ✅ (2026-07-17):** FUNCTIONALITY_DIGEST grid 57 stems (PH-S950); job/lease rows (PH-S951); ui/wasm crates (PH-S952); bins table all `src/bin/` (PH-S953); OpenAPI gap audit pointer (PH-S954); `poolai-loc-audit` → `rust_ratio.json` **94.91%** (PH-S955); `digest_depth_stub` + `galaxy_horizon_s950_integration` (PH-S959); vision **rev 298**.

**PH-S940…S949 ✅ (2026-06-22):** e2e scope audit `tests/e2e_scope_audit.rs` (PH-S940); `jobs_raid` archived + `e2e_ts_loc_reduction` **30** LOC (PH-S941); `stretch_spirit_gate_met` loc-audit (PH-S942); `ops_shell_canon_met` (PH-S943); `stretch_depth_stub` (PH-S944); `poolai-loc-audit` → `rust_ratio.json` **94.91%** (PH-S945); RUST_RATIO §5.13 band 29 row (PH-S946); `galaxy_horizon_s940_integration` (PH-S949); vision **rev 295**.

**PH-S930…S939 ✅ (2026-06-22):** admin_common table init wasm-only (PH-S930); empty state wasm-only (PH-S931); i18n_core `mergeRustI18nPatch` audit (PH-S932); `ratio_95_formal_gate_met` loc-audit test (PH-S933); `ui_js_loc_reduction` metric **131** LOC (PH-S934); `poolai-loc-audit` → `rust_ratio.json` **94.88%** (PH-S935); RUST_RATIO §5.13 band 28 row (PH-S936); `galaxy_horizon_s930_integration` (PH-S939); vision **rev 294**.

**PH-S920…S929 ✅ (2026-06-22):** admin_charts sparkline wasm-only (PH-S920); line chart wasm-only (PH-S921); regression tests mod.rs (PH-S922); build-ui-wasm.sh gate (PH-S923); `charts_depth_stub` (PH-S924); `poolai-loc-audit` → `rust_ratio.json` **94.80%** (PH-S925); RUST_RATIO §5.13 charts row (PH-S926); `galaxy_horizon_s920_integration` (PH-S929); vision **rev 293**.

**PH-S910…S919 ✅ (2026-06-22):** trust score SQLite persist + JSON migrate (PH-S910); payout gate uses persisted trust when metrics omit score (PH-S911); admin `renderGridTrustPersistStrip` wasm (PH-S912); stand smoke trust-metrics `trust_persist_depth` parity (PH-S913); `trust_persist_depth_stub` (PH-S914); `poolai-loc-audit` → `rust_ratio.json` **94.78%** (PH-S915); Galaxy §6.5 trust persist docs (PH-S916); `galaxy_horizon_s910_integration` (PH-S919); vision **rev 292**.

**PH-S900…S909 ✅ (2026-06-21):** pricing live provider timeout hardening + `galaxy_pricing_provider_timeouts_total` (PH-S900); pricing forced-fallback stand smoke (PH-S901); admin grid-pricing wasm freshness metadata strip (PH-S902); stand smoke pricing-metrics JSON↔Prom parity (PH-S903); `pricing_depth_stub` + `pricing_depth` wire (PH-S904); `poolai-loc-audit` → `rust_ratio.json` **94.77%** (PH-S905); Galaxy §4.2 live fetch implemented table (PH-S906); `galaxy_horizon_s900_integration` (PH-S909); vision **rev 291**.

**PH-S890…S899 ✅ (2026-06-21):** replication quorum gate production HTTP integration (PH-S890); replication rate cap HTTP wire (PH-S891); admin replication-pricing wasm rate cap strip (PH-S892); stand smoke replication_depth parity (PH-S893); `replication_depth_stub` + `replication_depth` wire (PH-S894); `poolai-loc-audit` → `rust_ratio.json` **94.74%** (PH-S895); Galaxy §6.4 implemented table (PH-S896); `galaxy_horizon_s890_integration` (PH-S899); vision **rev 290**.

**PH-S880…S889 ✅ (2026-06-21):** checker task drain lifecycle HTTP integration (PH-S880); shadow job submit depth (PH-S881); admin grid-verification wasm metrics+tasks strip (PH-S882); stand smoke verification-checker/lifecycle depth (PH-S883); `verification_lifecycle_depth_stub` + `lifecycle_depth` wire (PH-S884); `poolai-loc-audit` → `rust_ratio.json` **94.73%** (PH-S885); Galaxy §6.2 implemented table (PH-S886); `galaxy_horizon_s880_integration` (PH-S889); vision **rev 289**.

**PH-S870…S879 ✅ (2026-06-21):** on-chain cleared mock RPC depth + `galaxy_settlement_onchain_submit_total` (PH-S870); solana-adapter schema v1 fixture (PH-S871); domain events NDJSON persist depth (PH-S872); stand smoke on-chain payout-batch depth (PH-S873); `solana_depth_stub` (PH-S874); `poolai-loc-audit` → `rust_ratio.json` **94.71%** (PH-S875); SOLANA_ADAPTER_CONCEPT band 22 sync (PH-S876); `galaxy_horizon_s870_integration` (PH-S879); vision **rev 288**.

**PH-S860…S869 ✅ (2026-06-21):** `memory_store_depth_stub` + `tests/memory_shard_persistence` (PH-S860); seed-inventory HTTP depth fields (PH-S861); wasm `poolaiRenderMemorySeedMetaStrip` (PH-S862); stand smoke seed-inventory depth (PH-S863); `memory_layer_depth_stub` (PH-S864); `poolai-loc-audit` → `rust_ratio.json` **94.70%** (PH-S865); POOLAI_MEMORY_LAYER.md sync (PH-S866); `galaxy_horizon_s860_integration` (PH-S869); vision **rev 287**.

**PH-S850…S859 ✅ (2026-06-21):** `job_store_raid_persistence` HTTP+RAID reload (PH-S850); `verify-dev-stand` RAID jobs path PH-S851; wasm `poolaiRenderJobsStoreBadge` (PH-S852); stand smoke `jobs_store_backend` (PH-S853); `job_store_depth_stub` (PH-S854); `poolai-loc-audit` → `rust_ratio.json` **94.71%** (PH-S855); RUN_LOCAL RAID preset sync (PH-S856); `galaxy_horizon_s850_integration` (PH-S859); vision **rev 286**.

**PH-S840…S849 ✅ (2026-06-21):** openapi.yaml sync 4 missing routes + grid metrics examples (PH-S840/S844); `poolai-openapi-gap-audit` **0 missing** (PH-S841); `tests/grid_openapi_contracts.rs` (PH-S842); stand smoke OpenAPI path cases (PH-S843); `OPENAPI_GAP_AUDIT` doc sync (PH-S846); `poolai-loc-audit` → `rust_ratio.json` **94.71%** (PH-S845); `galaxy_horizon_s840_integration` (PH-S849); vision **rev 285**.

**PH-S830…S839 ✅ (2026-06-21):** `validate_band6_metrics_parity_v2` all grid `*-metrics` APIs (PH-S830); prefetch/locality JSON↔Prom bin tests (PH-S831); governance/fee parity bin tests (PH-S832); live runner `grid_metrics_json_prometheus_parity_band6_v2` (PH-S833); export shape regression suite (PH-S834); `poolai-loc-audit` → `rust_ratio.json` **94.70%** (PH-S835); PROMETHEUS_METRICS stand smoke v2 sync (PH-S836); `galaxy_horizon_s830_integration` (PH-S839); vision **rev 284**.

**PH-S820…S829 ✅ (2026-06-21):** wasm-only vm panel `poolaiRenderVmPanel` (PH-S820); workers/libs wasm `poolaiRenderLibsPanel` + ui-core `libs.rs` (PH-S821); admin/mod.rs regression PH-S822; stand smoke vm/workers API shape (PH-S823); `admin_vm_workers` Galaxy §2.3 subset + `admin_wasm_slim_depth_stub` vm/workers/libs (PH-S824); `poolai-loc-audit` → `rust_ratio.json` **94.68%** (PH-S825); FM/HANDOFF/NEXT/STABLE/GALAXY sync (PH-S826); `galaxy_horizon_s820_integration` (PH-S829); vision **rev 283**.

**PH-S810…S819 ✅ (2026-06-21):** wasm slim secret rotation `poolaiRenderSecretRotationPanel` (PH-S810); topology stats strip `poolaiRenderTopologyStatsStrip` (PH-S811); admin/mod.rs regression PH-S812; stand smoke security/topology shape (PH-S813); `admin_wasm_slim_depth_stub` Security/Topology (PH-S814); `poolai-loc-audit` → `rust_ratio.json` **94.67%** (PH-S815); FM/HANDOFF/NEXT/STABLE/GALAXY sync (PH-S816); `galaxy_horizon_s810_integration` (PH-S819); vision **rev 282**.

**PH-S800…S809 ✅ (2026-06-21):** wasm slim ML monitoring panel `poolaiRenderMlPipelineMetricsPanel` (PH-S800); payout-batch wasm-only `poolaiRenderPayoutBatchPanel` (PH-S801); admin/mod.rs regression PH-S802; stand smoke monitoring/settlement/payout shape (PH-S803); `admin_wasm_slim_depth_stub` MlPipeline/PayoutBatch (PH-S804); `poolai-loc-audit` → `rust_ratio.json` **94.68%** (PH-S805); FM/HANDOFF/NEXT/STABLE/GALAXY sync (PH-S806); `galaxy_horizon_s800_integration` (PH-S809); vision **rev 281**.

**PH-S790…S799 ✅ (2026-06-21):** `GET /api/v1/grid/update-policy` env snapshot (PH-S790); `GET /api/v1/grid/governance-metrics` + JSON↔Prom parity advisory/verify/notify (PH-S791); admin updates-compat governance wasm strip (PH-S792); stand smoke governance-metrics + update-policy API (PH-S793); `governance_depth_stub` (PH-S794); `poolai-loc-audit` → `rust_ratio.json` **94.69%** (PH-S795); SECURITY_HARDENING §9.5 hub sync (PH-S796); `galaxy_horizon_s790_integration`; FM/HANDOFF/NEXT/STABLE/GALAXY sync; vision **rev 280**.

**PH-S780…S789 ✅ (2026-06-21):** `GET /api/v1/grid/fee-split-metrics` + JSON↔Prom parity `galaxy_fee_split_applied_total` (PH-S780); grid-pricing fee hint wasm strip (PH-S781); stand smoke fee-split-metrics API (PH-S782); `galaxy_fee_split_depth_stub` (PH-S783); BENCHMARKS fee-split bench pointer (PH-S784); `poolai-loc-audit` → `rust_ratio.json` (PH-S785); GALAXY §1.2 fee split implemented table (PH-S786); `galaxy_horizon_s780_integration`; FM/HANDOFF/NEXT/STABLE/GALAXY sync; vision **rev 279**.

**PH-S770…S779 ✅ (2026-06-21):** offline payout batch queue on cleared + `galaxy_settlement_payout_batch_queue_depth` (PH-S770); `GET /api/v1/grid/payout-batch-metrics` + JSON↔Prom parity (PH-S771); admin payout-batch history wasm strip (PH-S771); stand smoke payout-batch/history/metrics API (PH-S772); `settlement_payout_depth_stub` (PH-S773); `galaxy_settlement_mode` offline vs on-chain gate (PH-S774); `poolai-loc-audit` → `rust_ratio.json` **94.65%**; hold advisory `--min-ratio 0.95`; GALAXY §8.2 payout implemented table (PH-S776); `galaxy_horizon_s770_integration`; FM/HANDOFF/NEXT/STABLE/GALAXY sync; vision **rev 278**.

**PH-S760…S769 ✅ (2026-06-21):** `GET /api/v1/grid/locality-metrics` + JSON↔Prom parity hot-tier promote/evict (PH-S760/S761); admin updates-compat locality wasm strip (PH-S762); stand smoke locality-metrics API (PH-S763); `locality_hot_tier_depth_stub` (PH-S764); `poolai-loc-audit` → `rust_ratio.json` **94.63%**; hold advisory `--min-ratio 0.95`; GALAXY §5.2–5.4 implemented table (PH-S766); `galaxy_horizon_s760_integration`; FM/HANDOFF/NEXT/STABLE/GALAXY sync; vision **rev 277**.

**PH-S750…S759 ✅ (2026-06-20):** `GET /api/v1/grid/prefetch-metrics` + JSON↔Prom parity `galaxy_prefetch_pull_bytes_total` (PH-S750); backpressure profile integration (PH-S751); admin updates-compat prefetch wasm strip (PH-S752); stand smoke prefetch-metrics API (PH-S753); `prefetch_depth_stub` (PH-S754); `poolai-loc-audit` → `rust_ratio.json` **94.62%**; hold advisory `--min-ratio 0.95`; GALAXY §5.5 implemented table (PH-S756); `galaxy_horizon_s750_integration`; FM/HANDOFF/NEXT/STABLE/GALAXY sync; vision **rev 276**.

**PH-S740…S749 ✅ (2026-06-20):** strict signed capability gate 403 + `galaxy_capability_unsigned_rejected_total` (PH-S740); dev fixture pass integration (PH-S741); admin updates-compat capability panel (PH-S742); stand smoke signed-cap reject export shape (PH-S743); `capability_admission_depth_stub` (PH-S744); `poolai-loc-audit` → `rust_ratio.json` **94.59%**; hold advisory `--min-ratio 0.95`; SECURITY_HARDENING ↔ Galaxy §6.6 cross-link (PH-S746); `galaxy_horizon_s740_integration`; FM/HANDOFF/NEXT/STABLE/GALAXY sync; vision **rev 275**.

**PH-S730…S739 ✅ (2026-06-20):** `reload_network_profile_store_from_disk` + restart integration (PH-S730); `merge_network_profile_json` + heartbeat merge persist (PH-S731); admin `renderNetworkProfilesPanel` ui-core/wasm glue (PH-S732); stand smoke network-profiles export shape (PH-S733); `network_profile_depth_stub` + parity band8 extend (PH-S734); `poolai-loc-audit` → `rust_ratio.json` **94.57%**; hold advisory `--min-ratio 0.95`; `galaxy_horizon_s730_integration`; FM/HANDOFF/NEXT/STABLE/GALAXY sync; vision **rev 274**.

**PH-S720…S729 ✅ (2026-06-20):** `re_migrate_policy_depth_stub` + dispatch hook (PH-S720); `routing_policy_locality_gate` (PH-S721); admin payout-batch `renderGridSettlementTrustMetricsStrip` wasm strip (PH-S722); stand smoke settlement/trust JSON↔Prom parity (PH-S723); `stand_smoke_metrics_parity_depth_stub` band7 extend (PH-S724); `poolai-loc-audit` → `rust_ratio.json` **94.55%**; hold advisory `--min-ratio 0.95`; `galaxy_horizon_s720_integration`; FM/HANDOFF/NEXT/STABLE/GALAXY sync; vision **rev 273**.

**Band archive (PH-S660…S879):** журнал FM §5.12 [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md#512-research-backlog-ph-s65-galaxy-wire--ops-2026-05-27) · `git log --oneline -30` · [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md).

**Autoprogon:** [`AUTO_RUN_SESSION_2026-07-01.md`](./AUTO_RUN_SESSION_2026-07-01.md) S21–S34 ✅ · **Horizon:** [`HORIZON_TO_100_PLAN.md`](./HORIZON_TO_100_PLAN.md).

**FM-003:** dev stand ✅; LAN §4 — **BLOCKED** (2 хosti). **FM-016+ / FM-012 / Post-Horizon FM-020…031** ✅ — env §2a нижче; FM §5.1.

**Зріз:** FM §5.1 [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md). **Гілка:** `main` (`git push origin main`).

**Rules:** **`абракадабра`** — drain 10 з §5.12 → vision close → push; [`.cursor/rules/poolai-session-iteration.mdc`](../../.cursor/rules/poolai-session-iteration.mdc).

**§5.12:** **0** відкритих. **Vision:** rev **308**. **Наступна:** **`абракадабра`** project scan.

**Роадмеп:** [`GALAXY_GRID_ROADMAP_2026-05-27.md`](./GALAXY_GRID_ROADMAP_2026-05-27.md) · **Промпт:** [`NEXT_SESSION_PROMPT.md`](./NEXT_SESSION_PROMPT.md).

## 1. Канонічний порядок документації та планів

Той самий список, що в кореневому [`README.md`](../../README.md) (*Documentation map*) і [`docs/README.md`](../README.md) (*Canonical reading order*), кроки **1–12**.

| Крок | Що читати |
|------|-----------|
| 0b | [`REPOSITORY_LAYOUT.md`](./REPOSITORY_LAYOUT.md) — `src/` vs `src/bin/` vs `bin/` vs `scripts/` vs `crates/`. |
| 1 | Кореневий [`README.md`](../../README.md) — швидкий старт, збірка, CI, карта доків. |
| 2 | [`INDEX_2026-03-17.md`](../INDEX_2026-03-17.md) — навігація по всьому `docs/`. |
| 3 | [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](./NEXT_STEPS_ARCHITECT_2026-03-17.md) — **головний** план Rust Architect (P1–P6, TurboQuant). |
| 4 | **Цей файл** — [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md): гілка, git-push, зріз P2/P3, next steps. |
| 5 | Концепція: [`concept/poolAI_concept_root.txt`](../concept/poolAI_concept_root.txt), Grid/Memory/Job у `docs/concept/` та [`JOB_LAYER_CONCEPT_2026-03-17.md`](./JOB_LAYER_CONCEPT_2026-03-17.md). |
| 6 | Архітектура: [`ARCHITECTURE_REVIEW.md`](../ARCHITECTURE_REVIEW.md), [`ARCHITECTURE_BEST_PRACTICES.md`](../ARCHITECTURE_BEST_PRACTICES.md). |
| 7 | Продуктивність: [`performance/BENCHMARKS.md`](../performance/BENCHMARKS.md), [`performance/PROFILING.md`](../performance/PROFILING.md); **`poolai_health_load --json`** для baseline; опційно [`benchmarks.yml`](../../.github/workflows/benchmarks.yml). |
| 8 | CI: [`ci.yml`](../../.github/workflows/ci.yml). |
| 9 | Інвентар: [`file_list.csv`](../../file_list.csv) (оновлюй також `docs/catalog/` при зміні витягу); повний список: `git ls-files`. |
| 10 | Git push (Windows): [`.cursor/commands/git-push.md`](../../.cursor/commands/git-push.md). |
| 11 | Витяг функціоналу: [`catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`](../catalog/FUNCTIONALITY_DIGEST_2026-04-06.md). |
| 12 | Керування функціоналом: [`catalog/FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) (**§5.1 — наступні кроки за FM-***); правило [`.cursor/rules/functionality-management.mdc`](../../.cursor/rules/functionality-management.mdc). |

Індекс планів у `docs/development/`: [`README.md`](./README.md). **Таксономія каталогу `docs/`:** [`../STRUCTURE.md`](../STRUCTURE.md). OpenAPI: [`docs/openapi.yaml`](../openapi.yaml). UI↔API: [`UI_QUALITY_AND_E2E_PLAN_2026-04-06.md`](./UI_QUALITY_AND_E2E_PLAN_2026-04-06.md). **Крок 11 / витяг функціоналу:** [`catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`](../catalog/FUNCTIONALITY_DIGEST_2026-04-06.md). **Крок 12 / беклог:** [`catalog/FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) (**§5.1** — наступні кроки за FM-*). **Project skill (Cursor):** [`.cursor/skills/poolai-documentation/SKILL.md`](../../.cursor/skills/poolai-documentation/SKILL.md).

## 2a. Virtual node / Telegram env (FM-016+)

| Змінна | Де | Призначення |
|--------|-----|-------------|
| `POOLAI_COORDINATOR_URL` | worker | Base URL coordinator (без trailing `/`) |
| `POOLAI_PROTOCOL_VERSION` | worker | Galaxy wire protocol на register-remote (default `1.2`) |
| `POOLAI_BUILD_ID` | worker | Build id на register-remote (default `CARGO_PKG_VERSION`) |
| `POOLAI_COORDINATOR_PROTOCOL_VERSION` | coordinator | Coordinator protocol для compat matrix (default `1.2`) |
| `POOLAI_GALAXY_PRICE_CACHE_TTL_SECS` | coordinator | Pricing oracle fresh TTL (default `300`; `galaxy_pricing_oracle`, §4.2) |
| `POOLAI_GALAXY_PRICE_MAX_STALE_SECS` | coordinator | Pricing oracle stale window (default `3600`) |
| `POOLAI_GALAXY_PRICING_FORCE_FALLBACK` | coordinator | `1` — аварійний L2-only режим (`pricing_forced_fallback` log + metric; PH-S81) |
| `POOLAI_GALAXY_PRICING_FALLBACK_JSON` | coordinator | L2 fixed quote map by unit key (usd_micro JSON); PH-S75/S78 |
| `POOLAI_GALAXY_PRICING_PROVIDERS` | coordinator | JSON allow-list provider catalog (PH-S92); no live HTTP fetch |
| `POOLAI_JOB_LEASE_TTL_SECS` | coordinator | Default lease TTL seconds (default `90`; `JobLeaseConfig`, Galaxy §4.3.1; PH-S97) |
| `POOLAI_JOB_LEASE_RENEW_INTERVAL_SECS` | coordinator | Optional renew/heartbeat interval seconds (default `lease_ttl/3`, max `lease_ttl`; PH-S111) |
| `POOLAI_TELEGRAM_ID` | worker | Telegram user id → `POST .../telegram/bind` після register |
| `POOLAI_WORKER_CACHE_DIR` | worker | Локальний кеш probe-артефактів після успішного `raid_artifact_probe` |
| `POOLAI_VIRTUAL_NODE_DATA_DIR` | coordinator | Персистентні tasks/bindings (напр. `data/virtual_nodes`) |
| `POOLAI_JOB_DATA_DIR` | coordinator | Персистентні jobs (напр. `data/jobs`; default `jobs.json`) |
| `POOLAI_ONCHAIN_EVENTS_DIR` | coordinator | NDJSON `events.ndjson` для sidecar (`JobCompleted` / memory epics; PH-S38) |
| `POOLAI_JOB_STORE` | coordinator | `sqlite` → `jobs.db` (`--features job-store-sqlite`); `raid` → snapshot у RAID (`POOLAI_RAID_BASE_PATH` **до** першого `JobStore::global()`); інакше JSON (`POOLAI_JOB_DATA_DIR` / `jobs.json`) |
| `POOLAI_RAID_BASE_PATH` | coordinator | Каталог RAID-артефактів (обов’язково для `POOLAI_JOB_STORE=raid`; той самий шлях, що для `/api/v1/raid/*`) |
| `POOLAI_MEMORY_DATA_DIR` | coordinator | Персистентні memory shards (напр. `data/memory`, `shards.json`) |
| `POOLAI_MONITORING_DATA_DIR` | coordinator | Enterprise monitoring SQLite (`monitoring.db`: metrics, dashboards, alert_rules) |
| `POOLAI_SOLANA_CONFIG` | sidecar | Шлях до TOML (default: bundled `config/devnet.toml`) |
| `POOLAI_SOLANA_CLUSTER` | sidecar | `devnet` / `localnet` (mainnet rejected) |
| `POOLAI_SOLANA_MOCK_RPC` | sidecar | `1` — mock submit у stdout ack (`rpc` block); default **off** (FM-033 real RPC) |
| `POOLAI_SOLANA_KEYPAIR_PATH` | sidecar | Solana CLI JSON keypair для devnet `sendTransaction` |
| `POOLAI_SOLANA_PROGRAM_ID` | sidecar | Deployed `poolai-events` program id (інакше Memo fallback) |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | coordinator | OTLP HTTP collector URL (feature `otel`; export off if unset) |
| `OTEL_SERVICE_NAME` | coordinator | OTel `service.name` (default `poolai`) |
| *(OTel lease spans)* | coordinator | Span attrs contract: [`OPENTELEMETRY_TRACING.md`](./OPENTELEMETRY_TRACING.md) § Job lease spans (`job.lease.*`; PH-S124 docs, PH-S126 code) |
| *(build)* `prometheus` feature | coordinator | Enables `GET /metrics` Prometheus text scrape (FM-043; included in `cargo test-ci`) |
| `HTTPS_CERT_PATH` / `HTTPS_KEY_PATH` | coordinator | PEM paths when `https.enabled` (FM-044) |
| `HTTPS_CERT_RELOAD_SECS` | coordinator | Optional hot reload interval for TLS certificates |

**Стек (агенти):** Rust-only runtime — [`.cursor/rules/runtime-stack-policy.mdc`](../../.cursor/rules/runtime-stack-policy.mdc); `docs/STRUCTURE.md` §7. **Не** пропонувати Python для ML/API.
| `POOLAI_TELEGRAM_WEBHOOK_SECRET` | coordinator | Опційно: header `X-Telegram-Webhook-Secret` для webhook |
| `POOLAI_TELEGRAM_AUTH_MAX_AGE_SECS` | enterprise OAuth | Max вік `auth_date` для Telegram Login Widget (default 86400) |
| `TELEGRAM_BOT_TOKEN` | `poolai-telegram-bot` | Token від @BotFather |

Збірка бота: `cargo build --bin poolai-telegram-bot --features tgbot`. Запуск: `TELEGRAM_BOT_TOKEN=... POOLAI_COORDINATOR_URL=http://127.0.0.1:8080 poolai-telegram-bot`.

Секрети — лише в env на хості, не в репо.

## 2b. FM-003 dev stand (одна машина)

| Скрипт | Призначення |
|--------|-------------|
| `bin/run-lan-nodes.ps1` / `.sh` | Два `poolai` на 8080+8081 |
| `bin/run-virtual-node-dev.ps1` / `.sh` | Coordinator + `poolai-worker` |
| `bin/verify-dev-stand.ps1` / `.sh` | Health + discovery + pool join + bootstrap tasks (>=4) + ML pipeline demo (PH-S17); опційно `VERIFY_RAID_JOB_STORE=1` — RAID job persist після restart (PH-S54) |
| `bin/verify-lan-prep.ps1` / `.sh` | FM-027: dual-port or `POOLAI_NODE_*_URL` health + discovery peers |
| `bin/capture-p2b-single-host-metrics.ps1` / `.sh` | FM-028: `run-lan-nodes` + health_load ×2 + TQ01 snapshot → `data/lan-stand/metrics-fm028-*.json` |

Runbook: [`LAN_BENCHMARK_RUNBOOK.md`](../performance/LAN_BENCHMARK_RUNBOOK.md) §5–5.1. **Запуск усього проєкту:** [`RUN_LOCAL.md`](./RUN_LOCAL.md) (`bin/run-poolai.sh`).

## 2. Git push (Windows / Cursor)

- **Канонічна інструкція:** [`.cursor/commands/git-push.md`](../../.cursor/commands/git-push.md) — MSYS2 UCRT64 **зовнішній** термінал, `PATH` з `~/.cargo/bin`, `K8S_OPENAPI_ENABLED_VERSION=1.28` за потреби cloud-sdk, формат коміта з Summary.
- Не робити `git add -A` без потреби; не стаджити `data/audit/*.log.gz`.
- Старі одноразові нотатки `PUSH_*.md` перенесені в [`docs/archive/`](../archive/); актуальні проблеми — [`docs/troubleshooting/`](../troubleshooting/).

## 3. Що вже зроблено (орієнтир для нової сесії)

- **`src/services/`**: `raid_service`, **`raid_distributed_protocol_service`** (distributed RAID JSON protocol; тонкий `raid_distributed_handlers.rs`), `vm_service`, `library_service`, **`instance_service`** (`/api/v1/instance/*`, `/state`), **`chat_completion_service`** (`/v1/chat/completions` — тонкий `completions.rs`), **`system_service`** (status/health/metrics/models/GPU, login, config get/update), **`ui_service`** (теми/компоненти + enterprise-дашборди через `EnterpriseService`), **`discovery_service`**, **`topology_service`**, **`worker_pool_service`**, **`rewards_service`** (`/api/v1/rewards/*`), `enterprise_service`, `cloud_service`, `admin_service` + `GET /api/v1/admin/overview` (`src/network/api/admin.rs`). HTML **`GET /api/v1/status`** — модуль **`network/api/system_status_html.rs`** (не в `SystemService`).
- **RaidService (P2)**: крім list — `put_artifact`, `delete_artifact`, `quota`, `cluster_status`; DTO квоти/статусу в `raid_service.rs`; тонкі handlers у `src/network/api/raid.rs`.
- **ML pipeline (Stage 4.4)**: детерміновані Rust-бекенди для `Preprocessing`, `Training`, `Evaluation`, `Deployment` (`src/ml/pipeline.rs`).
- **TurboQuant (P2b, фаза 1)**: `src/ml/turboquant.rs` (формат `TQ01`), інтеграція в крок `Quantization` за конфігом; див. `docs/ml/TURBOQUANT_INTEGRATION.md`.
- **Priority 3 / FM-005 (HTTP-шар)** ✅: `json_errors.rs` — **`HttpAppError`**, **`IntoResponse`**; **`AppError::RestError`**. Покриття: **`api/*`**, **`raid*`** (**`raid_api_err`**), **`enterprise_api`**, **`authenticate_user`** / **`refresh_access_token`** / **`login`/`refresh` handlers**, **`check_permission`**, **`auth_middleware`** / **`permission_middleware`**.
- **P3 (auth / WS / rate limit)**: **`auth.rs`**, **`ws.rs`**, **`rate_limit.rs`** — той самий JSON-формат помилок (`src/network/json_errors.rs`); UI читає `error.message`. **`http_status_for_app_error`**, **`IntoResponse`** для **`AppError`** / **`HttpAppError`**. Приклад змішаного стилю: **`api/rewards.rs`** — частина GET → **`Result<Json<_>, AppError>`**, **`/rewards/progress/*`** → **`Result<_, HttpAppError>`** (**`ApiNotFound`** / **`NOT_FOUND`**).
- **Перевірка тестів (як CI)**: `K8S_OPENAPI_ENABLED_VERSION=1.28` + `cargo test-ci` (alias у `.cargo/config.toml`: `ml,enterprise,cloud,test-utils,job-store-sqlite,prometheus`). **Raft (PH-S04…S06, PH-S21):** `cargo test-raft-ci` — `raft_wire_integration` + `raft_multi_node_harness` + `raft_membership_log` (`--features raft,test-utils`). Інжектований `AppState`: `tests/appstate_http_injection_integration.rs`, `vm_api_contracts.rs`, `distributed_raid_wire_integration`. На Windows при OOM: `-j 1 -- --test-threads=1`.
- **cargo test-ci scope note final (PH-S1003, band 35):** API/grid/job/telegram wire → `cargo test-ci` (+ `poolai-openapi-gap-audit` після API); Raft scope → `cargo test-raft-ci`; Admin UI / axe / visual → `bash bin/e2e-playwright.sh --start`; API-only band — Playwright skip не блокує push якщо `cargo test-ci` green. Див. [`.cursor/rules/poolai-testing-policy.mdc`](../../.cursor/rules/poolai-testing-policy.mdc) band 35.
- **Clippy (2026-04-10):** перед push доцільно прогнати ті самі команди, що в [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml): `cargo clippy --all-targets --no-default-features -- -D warnings`, `cargo clippy --all-targets --features jwt,https -- -D warnings`, і з `K8S_OPENAPI_ENABLED_VERSION=1.28` — `cargo clippy --all-targets --features cloud,cloud-sdk -- -D warnings`. Для змін у **enterprise** / UI — також `cargo clippy -p poolai --features enterprise -- -D warnings`. Код і `tests/*` вирівняні під ці матриці.
- **FM-012 ✅ (2026-05-16):** i18n UA/EN + Telegram OAuth hardening — [`oauth.rs`](../../src/network/enterprise_api/oauth.rs), [`security.rs`](../../src/enterprise/security.rs), [`i18n_core.js`](../../src/ui/i18n_core.js); unit-тести allowlist/expiry/RBAC.

## 4. Наступні кроки (канон: FM-* + Architect)

**PH-S03…S14 закрито** (лише **PH-S01/PH-S15** Deferred, **PH-S02/PH-S16** BLOCKED). **Єдине зведення FM** — [`catalog/FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) **§5.1**, **§5.9**, legacy **§5.8**, «не зроблено» **§5.3**.

| Порядок | Фокус | Стан |
|--------|--------|------|
| — | **PH-S24** Security ops | **✅** (rotation hooks + pen-test checklist) |
| — | **PH-S16** / **FM-003** LAN §4 | **BLOCKED** (2 хости) |
| — | **PH-S15** / **FM-041** Cloud SDK | **Deferred** |

**Закрито (2026-05-24–25):** **PH-S25…S34** — post-S24 maintenance (E2E, OpenAPI secrets, security/metrics, visual baseline script); **PH-S23** — Playwright admin flows; **PH-S22** topology WS; **PH-S21** Raft membership. **Не повторювати** PH-S03…S34; PH-S29 metrics test (`82d35fd3`).

**Промпт:** [`NEXT_SESSION_PROMPT.md`](./NEXT_SESSION_PROMPT.md).

## 5. Автономний режим (VDT → локальний CI → git push)

**Ролі:** людина (власник/креатив) · агент-оркестратор · субагенти `explore`/`shell`/`generalPurpose` — [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc).

1. Старт: [`NEXT_SESSION_PROMPT.md`](./NEXT_SESSION_PROMPT.md) — **`абракадабра`** (drain **PH-S770…S779**, band 12); ops LAN **BLOCKED**; FM-041 Deferred.
2. Ітерація: `poolai-session-iteration.mdc` — S0, MSYS2 bash, `df -h /s`, **один PH-S***, staging/commit/push.
3. **Локальний CI (канон):** `cargo fmt` → `cargo test-ci`; за scope — `test-raft-ci`, `poolai-openapi-gap-audit`, `e2e` `test:ci`. **GitHub CI не блокує** ітерацію.
4. Оркестратор: `autonomous-orchestrator.mdc`; бенч — лише за scope спринту (`BENCHMARKS.md`, `poolai_health_load`).
5. **Не в обсязі:** FM-003 §4 LAN (2 хости); mainnet Solana; native Azure Compute SDK crate.
6. **Push:** MSYS2 UCRT64, [`git-push.md`](../../.cursor/commands/git-push.md); код у коміті → Summary + самарі в чат.
7. **Не в git:** `data/audit/*.log*`, `data/dev/`, `comitmsg/*.txt` (чернетки commit-msg; див. `comitmsg/README.md`), `bin/commit-*.sh`, `target/`.
