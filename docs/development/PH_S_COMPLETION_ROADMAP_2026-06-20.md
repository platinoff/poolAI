# PH-S completion roadmap v2 (PH-S720…S1010)

**Оновлено:** 2026-06-20 · **Мета:** повне закриття розробки PoolAI (code + docs + ratio) за **291** спринтами · **30** сесій `абракадабра` (10 PH-S* / сесія)

**Канон drain:** FM **§5.12** (max 10 відкритих) · реєстр — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · regen: `bash scripts/generate-ph-s-master-backlog-351.sh`

**Поза scope (не в backlog):** FM-003 LAN 2-host (**BLOCKED**) · FM-041 Cloud SDK prod (**Deferred**) · ZK/TEE (Galaxy §6.6 roadmap only)

---

## Фази до product-complete

| Фаза | Bands | Sprints | Фокус |
|------|-------|---------|--------|
| **A — Galaxy horizon depth** | 7–14 | S720–S789 | §4 routing, §8 profile persist, §6.6 caps, §5 prefetch/locality, §8.2 payout, §1 fees, §9 governance |
| **B — Admin wasm + wire hardening** | 15–19 | S800–S849 | wasm slim panels, stand smoke v2, OpenAPI gap 0 |
| **C — Job/Memory/Solana depth** | 20–22 | S850–S879 | raid job store, memory persist, on-chain cleared |
| **D — Production gates** | 23–26 | S880–S919 | verification lifecycle, replication quorum, pricing live, trust persist |
| **E — Rust ratio 95–96%** | 27–29 | S920–S949 | admin_charts → wasm, JS glue removal, ratio gates |
| **F — Docs product-complete** | 30–33 | S950–S989 | DIGEST, DOCS_LEGACY, Galaxy ✅ markers, STABLE |
| **G — Final verification** | 34–35 | S990–S1009 | integration expansion, multi-module horizon close |
| **H — Closure** | 36 | S1010 | FM §5.15 product-complete declaration |

**Після S1010 ✅:** лише maintenance / BLOCKED ops (LAN) / явний FM-horizon v2 за запитом власника.

---

## Band 7 — PH-S720…S729 (Galaxy §4 routing / re-migrate) · **✅ drained**

| Sprint | Focus | Acceptance |
|--------|-------|------------|
| **PH-S720** | `re_migrate_policy_depth_stub` | Galaxy §4.3; unit test; dispatch/scheduler hook |
| **PH-S721** | `routing_policy_locality_gate` | Galaxy §4.1 strict routing helper + unit test |
| **PH-S722** | Admin settlement/trust metrics wasm strip | ui-core strip; fetch JSON metrics + wasm render |
| **PH-S723** | Stand smoke settlement/trust JSON↔Prometheus parity | unit tests in `poolai-http-stand-smoke` |
| **PH-S724** | `stand_smoke_metrics_parity_depth_stub` band extend | concept stub + unit test (§4–§8) |
| **PH-S725** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S725 |
| **PH-S726** | INDEX/HANDOFF/NEXT/STABLE/GALAXY sync | active band pointers |
| **PH-S727** | `poolai-vision-sync --check` | drift gate green |
| **PH-S728** | Ratio hold advisory | `--min-ratio 0.95 --advisory` |
| **PH-S729** | `galaxy_horizon_s720_integration` | §4 routing band close + docs |

---

## Band 8 — PH-S730…S739 (Galaxy §8.1 network_profile full persist) · **✅ drained 2026-06-20**

| Sprint | Focus | Acceptance |
|--------|-------|------------|
| **PH-S730** | `network_profile_store` persist read API | GET profile survives restart stub + test |
| **PH-S731** | `network_profile_store` persist write path | PUT + heartbeat merge persist + test |
| **PH-S732** | Admin network-profile panel wasm glue | ui-core + fetch `/grid/network-profiles` |
| **PH-S733** | Stand smoke network-profiles list/put | live runner cases green |
| **PH-S734** | `network_profile_depth_stub` | Galaxy §8.1 egress/locality classification + unit test |
| **PH-S735** | loc-audit → `rust_ratio.json` | PH-S735 zriz |
| **PH-S736** | docs canon sync | INDEX/HANDOFF/NEXT/STABLE/GALAXY |
| **PH-S737** | vision-sync `--check` | green |
| **PH-S738** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S739** | `galaxy_horizon_s730_integration` | profile persist band close |

---

## Band 9 — PH-S740…S749 (Galaxy §6.6 signed capability admission) · **✅ drained 2026-06-20**

| Sprint | Focus | Acceptance |
|--------|-------|------------|
| **PH-S740** | `galaxy_capability_admission` strict signed gate | unsigned telegram_edge → 403 + metric |
| **PH-S741** | Dev fixture signed capability pass path | integration test register-remote OK |
| **PH-S742** | Admin capability doc pointer panel | `/ui/admin/updates-compat` capability section |
| **PH-S743** | Stand smoke signed-cap reject shape | export shape unit test |
| **PH-S744** | `capability_admission_depth_stub` | unit test + concept §6.6 link |
| **PH-S745** | loc-audit | PH-S745 zriz |
| **PH-S746** | docs canon | SECURITY_HARDENING ↔ Galaxy §6.6 cross-link |
| **PH-S747** | vision-sync `--check` | green |
| **PH-S748** | ratio advisory | hold snapshot |
| **PH-S749** | `galaxy_horizon_s740_integration` | signed cap band close |

---

## Band 10 — PH-S750…S759 (Galaxy §5.5 prefetch live pull depth) · **✅ drained**

## Band 11 — PH-S760…S769 (Galaxy §5.2–5.4 locality / hot-tier depth) · **✅ drained 2026-06-21**

| Sprint | Focus | Acceptance |
|--------|-------|------------|
| **PH-S760** | GET `/grid/locality-metrics` HTTP wire | integration test |
| **PH-S761** | Hot-tier promote/evict metrics parity | JSON + Prometheus parity test |
| **PH-S762** | Admin locality wasm glue | ui-core metrics strip |
| **PH-S763** | Stand smoke locality/prefetch band | runner extend |
| **PH-S764** | `locality_hot_tier_depth_stub` | unit test |
| **PH-S765** | loc-audit | zriz |
| **PH-S766** | docs canon | INDEX §7 |
| **PH-S767** | vision-sync | green |
| **PH-S768** | ratio advisory | hold |
| **PH-S769** | `galaxy_horizon_s760_integration` | locality band close |

---

## Band 12 — PH-S770…S779 (Galaxy §8.2 payout / settlement batch) · ✅ drained

| Sprint | Focus | Acceptance |
|--------|-------|------------|
| **PH-S770** | Offline payout batch settlement wire depth | cleared → batch queue stub + metric |
| **PH-S771** | Payout batch history admin wasm panel | ui-core render + fetch |
| **PH-S772** | Stand smoke payout-batch/history API | runner green |
| **PH-S773** | `settlement_payout_depth_stub` | Galaxy §8.2 unit test |
| **PH-S774** | On-chain vs offline mode gate doc stub | `galaxy_settlement_mode` test extend |
| **PH-S775** | loc-audit | zriz |
| **PH-S776** | docs canon | Galaxy §8.2 payout row ✅ |
| **PH-S777** | vision-sync | green |
| **PH-S778** | ratio advisory | hold |
| **PH-S779** | `galaxy_horizon_s770_integration` | payout band close |

---

## Band 13 — PH-S780…S789 (Galaxy §1.2 fee split production) · **✅ drained**

| Sprint | Focus | Acceptance |
|--------|-------|------------|
| **PH-S780** | Fee split applied metric production parity | JSON/Prom parity |
| **PH-S781** | Primary/secondary fee hint admin read-only strip | ui-core or grid-pricing extend |
| **PH-S782** | Stand smoke fee-split metrics export | unit test |
| **PH-S783** | `galaxy_fee_split_depth_stub` | unit test |
| **PH-S784** | Bench gate `galaxy_fee_split_benchmarks` in CI note | docs/BENCHMARKS pointer |
| **PH-S785** | loc-audit | zriz |
| **PH-S786** | docs canon | concept §1.2 implemented |
| **PH-S787** | vision-sync | green |
| **PH-S788** | ratio advisory | hold |
| **PH-S789** | `galaxy_horizon_s780_integration` | fee band close |

---

## Band 14 — PH-S790…S799 (Galaxy §9.5–9.6 governance ops) · ✅ drained

| Sprint | Focus | Acceptance |
|--------|-------|------------|
| **PH-S790** | Update policy env stub wire | `galaxy_update_policy` HTTP read + test |
| **PH-S791** | Security advisory metric/export shape | stand smoke or unit test |
| **PH-S792** | Admin updates-compat governance extend | wasm panel |
| **PH-S793** | Stand smoke governance metrics | runner |
| **PH-S794** | `governance_depth_stub` | unit test |
| **PH-S795** | loc-audit | zriz |
| **PH-S796** | docs canon | SECURITY_HARDENING hub sync |
| **PH-S797** | vision-sync | green |
| **PH-S798** | ratio advisory | hold |
| **PH-S799** | `galaxy_horizon_s790_integration` | governance band close |

---

## Band 15 — PH-S800…S809 (Admin wasm slim: monitoring + payout-batch) · **✅ drained 2026-06-21**

| Sprint | Focus | Acceptance |
|--------|-------|------------|
| **PH-S800** | wasm slim monitoring ML panel | `poolaiRenderMlPipelineMetricsPanel` wasm-only |
| **PH-S801** | wasm slim payout-batch panel | ui-core → wasm export |
| **PH-S802** | admin/mod.rs regression PH-S800/S801 | `parsePrometheusGauge` tests |
| **PH-S803** | stand smoke monitoring/payout APIs | runner shape tests |
| **PH-S804** | admin wasm slim depth stub extend | unit test |
| **PH-S805** | loc-audit | zriz |
| **PH-S806** | docs canon | HANDOFF/NEXT/STABLE |
| **PH-S807** | vision-sync | green |
| **PH-S808** | ratio advisory | hold |
| **PH-S809** | `galaxy_horizon_s800_integration` | wasm monitoring band close |

---

## Band 16 — PH-S810…S819 (Admin wasm slim: security + topology) · **✅ drained 2026-06-21**

| Sprint | Focus | Acceptance |
|--------|-------|------------|
| **PH-S810** | wasm slim security panel glue | secret rotation strip wasm |
| **PH-S811** | wasm slim topology panel glue | topology timestamp wasm |
| **PH-S812** | admin/mod.rs regression PH-S810/S811 | wasm glue tests |
| **PH-S813** | stand smoke security/topology APIs | export shape if applicable |
| **PH-S814** | concept stub security/topology | unit test |
| **PH-S815** | loc-audit | zriz |
| **PH-S816** | docs canon | HANDOFF/NEXT/STABLE |
| **PH-S817** | vision-sync | green |
| **PH-S818** | ratio advisory | hold |
| **PH-S819** | `galaxy_horizon_s810_integration` | security/topology wasm band close |

---

## Band 17 — PH-S820…S829 (Admin wasm slim: vm + workers + libs) · **✅ drained 2026-06-21**

| Sprint | Focus | Acceptance |
|--------|-------|------------|
| **PH-S820** | wasm slim vm panel glue | vm admin wasm render |
| **PH-S821** | wasm slim workers/libs panels | ui-core → wasm |
| **PH-S822** | admin/mod.rs regression PH-S820/S821 | wasm glue tests |
| **PH-S823** | stand smoke vm/workers API shape | runner tests |
| **PH-S824** | concept stub vm/workers DTO | Galaxy §2.3 unit test |
| **PH-S825** | loc-audit | zriz |
| **PH-S826** | docs canon | HANDOFF/NEXT/STABLE |
| **PH-S827** | vision-sync | green |
| **PH-S828** | ratio advisory | hold |
| **PH-S829** | `galaxy_horizon_s820_integration` | vm/workers wasm band close |

---

## Bands 18–35 (summary — деталі в master backlog)

| Band | Sprints | Theme |
|------|---------|-------|
| 15 | S800–S809 | Admin wasm slim: monitoring + payout-batch ✅ |
| 16 | S810–S819 | Admin wasm slim: security + topology ✅ |
| 17 | S820–S829 | Admin wasm slim: vm + workers + libs **active** |
| 18 | S830–S839 | Stand smoke v2: all grid JSON↔Prom parity + runner |
| 19 | S840–S849 | OpenAPI gap audit 0 + contract test band |
| 20 | S850–S859 | Job store RAID production path + restart test |
| 21 | S860–S869 | Memory shard persist + seed-inventory depth |
| 22 | S870–S879 | Solana on-chain cleared depth (mock RPC) |
| 23 | S880–S889 | Verification checker full lifecycle wire |
| 24 | S890–S899 | Replication quorum + rate cap production gates |
| 25 | S900–S909 | Pricing oracle live provider hardening |
| 26 | S910–S919 | Trust score SQLite persist + payout gate |
| 27 | S920–S929 | wasm: admin_charts ML/sparkline full migration |
| 28 | S930–S939 | Ratio **95%** gate: admin_common.js slim |
| 29 | S940–S949 | Ratio **96%** stretch: e2e scope audit |
| 30 | S950–S959 | FUNCTIONALITY_DIGEST full module sync |
| 31 | S960–S969 | DOCS_LEGACY audit close + archive banners |
| 32 | S970–S979 | Galaxy concept ✅ markers (all § implemented) |
| 33 | S980–S989 | STABLE_STATE + INDEX product-complete |
| 34 | S990–S999 | Integration test gap fill (top 10 FM gaps) |
| 35 | S1000–S1009 | Final multi-module `galaxy_horizon_s1000_integration` |

---

## Band 36 — PH-S1010 (product-complete closure)

| Sprint | Focus | Acceptance |
|--------|-------|------------|
| **PH-S1010** | **Product-complete declaration** | FM **§5.15** ✅; STABLE «development complete»; HANDOFF maintenance mode; `poolai-vision-sync`; post-scan template only for BLOCKED/Deferred |

---

## Звірка concept ↔ completion roadmap

| Concept gap | Closing band |
|-------------|--------------|
| §8.1 full network_profile persist | Band 8 |
| §6.6 signed capabilities | Band 9 |
| §5.5 prefetch live pull | Band 10 |
| §5.2–5.4 locality/hot-tier | Band 11 |
| §8.2 payout batch | Band 12 |
| §1.2 fee split prod | Band 13 |
| §9.5–9.6 governance ops | Band 14 |
| wasm slim 95%+ | Bands 15–17, 27–29 |
| Job/Memory/Solana depth | Bands 20–22 |
| Docs product-complete | Bands 30–33, S1010 |
