# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-21 (completion v2 · master backlog **111** · active **PH-S900…S909** · vision **rev 290** · rust_ratio **94.74%**)

| **← наступний** | **`абракадабра`** (drain band 25 PH-S900…S909) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **111** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S900…S909, band 25 Pricing oracle live fetch hardening) |
| **Після band 25** | promote PH-S910…S919 (band 26 Trust score SQLite persist) |
| **Сесій drain** | **12** (`111÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S900…S909) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 25
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 25: **PH-S910…S919**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 111 → product-complete (PH-S910…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–24 | PH-S660…S899 | ✅ drained |
| 25 | PH-S900…S909 | **активна §5.12** — Pricing oracle live fetch hardening |
| 26–27 | PH-S910…S929 | Trust/trust gates |
| 28–29 | PH-S930…S959 | Ratio **95–96%** |
| 30–33 | PH-S960…S999 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 25 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S900** | pricing live provider timeout hardening | oracle unit + integration |
| **PH-S901** | pricing forced-fallback stand smoke | PH-S123 pattern |
| **PH-S902** | admin grid-pricing wasm polish | freshness metadata display |
| **PH-S903** | stand smoke pricing-metrics parity | JSON↔Prom |
| **PH-S904** | pricing depth stub | unit test |
| **PH-S905** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S905 |
| **PH-S906** | Galaxy §4.2 live fetch ✅ docs | docs |
| **PH-S907** | `poolai-vision-sync --check` | drift gate green |
| **PH-S908** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S909** | `galaxy_horizon_s900_integration` | pricing band close |

**Після band 25 promote:** PH-S910…S919 (band 26 — Trust score SQLite persist).

---

## Не повторювати

Закриті PH-S890…S899 acceptance (replication quorum production, rate cap HTTP wire, admin wasm rate cap strip, stand smoke replication_depth, `galaxy_horizon_s890_integration`) — див. FM §5.12 ✅.
