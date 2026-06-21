# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-21 (completion v2 · master backlog **101** · active **PH-S910…S919** · vision **rev 291** · rust_ratio **94.77%**)

| **← наступний** | **`абракадабра`** (drain band 26 PH-S910…S919) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **101** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S910…S919, band 26 Trust score SQLite persist) |
| **Після band 26** | promote PH-S920…S929 (band 27 wasm admin_charts migration) |
| **Сесій drain** | **11** (`101÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S910…S919) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 26
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 26: **PH-S920…S929**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 101 → product-complete (PH-S920…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–25 | PH-S660…S909 | ✅ drained |
| 26 | PH-S910…S919 | **активна §5.12** — Trust score SQLite persist |
| 27–29 | PH-S920…S959 | Ratio **95–96%** |
| 30–33 | PH-S960…S999 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 26 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S910** | trust score SQLite persist | galaxy_trust_score_store wire |
| **PH-S911** | trust payout gate integration | low trust → held metric |
| **PH-S912** | admin trust metrics wasm strip | ui-core |
| **PH-S913** | stand smoke trust-metrics parity | JSON↔Prom |
| **PH-S914** | trust persist depth stub | unit test |
| **PH-S915** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S915 |
| **PH-S916** | Galaxy §6.5 trust persist ✅ docs | docs |
| **PH-S917** | `poolai-vision-sync --check` | drift gate green |
| **PH-S918** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S919** | `galaxy_horizon_s910_integration` | trust persist band close |

**Після band 26 promote:** PH-S920…S929 (band 27 — wasm admin_charts migration).

---

## Не повторювати

Закриті PH-S900…S909 acceptance (pricing timeout hardening, forced-fallback stand smoke, admin wasm freshness strip, pricing-metrics JSON↔Prom parity, `pricing_depth_stub`, `galaxy_horizon_s900_integration`) — див. FM §5.12 ✅.
