# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-20 (completion v2 · master backlog **281** · active **PH-S730…S739** · vision **rev 273** · rust_ratio **94.55%**)

| **← наступний** | **`абракадабра`** (drain band 8 PH-S730…S739) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **281** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S730…S739, band 8 Galaxy **§8.1** network_profile persist) |
| **Після band 8** | promote PH-S740…S749 (band 9 Galaxy **§6.6** signed capability admission) |
| **Сесій drain** | **29** (`281÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S730…S739) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 8
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 8: **PH-S740…S749**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 281 → product-complete (PH-S730…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–7 | PH-S660…S729 | ✅ drained |
| 8 | PH-S730…S739 | **активна §5.12** — Galaxy **§8.1** network_profile persist |
| 9–14 | PH-S740…S789 | Galaxy depth (caps, prefetch, payout, fees, governance) |
| 15–19 | PH-S800…S849 | Admin wasm slim + stand smoke v2 + OpenAPI |
| 20–26 | PH-S850…S919 | Job/Memory/Solana + production gates |
| 27–29 | PH-S920…S949 | Ratio **95–96%** |
| 30–33 | PH-S950…S989 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 8 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S730** | `network_profile_store` persist read | GET profile survives restart stub + integration test |
| **PH-S731** | `network_profile_store` persist write | PUT + heartbeat merge persist + test |
| **PH-S732** | Admin network-profile panel wasm glue | ui-core; fetch `/grid/network-profiles` |
| **PH-S733** | Stand smoke network-profiles list/put | live runner cases green |
| **PH-S734** | `network_profile_depth_stub` | Galaxy §8.1 classification + unit test |
| **PH-S735** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S735 |
| **PH-S736** | INDEX / HANDOFF / NEXT / STABLE / GALAXY sync | docs canon + completion roadmap |
| **PH-S737** | `poolai-vision-sync --check` | drift gate green |
| **PH-S738** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S739** | `galaxy_horizon_s730_integration` | §8.1 profile persist band close + docs |

**Після band 8 promote:** PH-S740…S749 (band 9 — Galaxy **§6.6** signed capability admission).

---

## Не повторювати

PH-S720…S729 ✅ band 7 drain (2026-06-20): `re_migrate_policy_depth_stub` + dispatch hook; `routing_policy_locality_gate`; admin payout-batch settlement/trust wasm strip; stand smoke settlement/trust parity; band7 depth stub extend; `galaxy_horizon_s720_integration`; rust_ratio **94.55%** hold advisory.

PH-S710…S719 ✅ band 6 drain (2026-06-20): `stand_smoke_metrics_parity` JSON↔Prometheus; ui-core `stand_smoke_metrics` + verification admin wasm strip; stand smoke band6 parity runner; `galaxy_horizon_s710_integration`; rust_ratio **94.76%** hold advisory.

PH-S700…S709 ✅ band 5 drain (2026-06-20): ui-core `render_grid_replication_pricing_panel_html` + wasm; admin_charts wasm-only canvas glue; `admin_wasm_slim_depth_stub`; stand smoke export shape; `galaxy_horizon_s700_integration`; rust_ratio **94.75%** hold advisory.

PH-S690…S699 ✅ band 4 drain (2026-06-20): replication/pricing metrics HTTP; `replication_pricing_depth_stub`; wasm `parsePrometheusGauge` on grid-replication-pricing; stand smoke replication/pricing API; `galaxy_horizon_s690_integration`; rust_ratio **94.67%** hold advisory.
