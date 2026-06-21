# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-21 (completion v2 · master backlog **121** · active **PH-S890…S899** · vision **rev 289** · rust_ratio **94.73%**)

| **← наступний** | **`абракадабра`** (drain band 24 PH-S890…S899) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **121** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S890…S899, band 24 Replication quorum production gates) |
| **Після band 24** | promote PH-S900…S909 (band 25 Pricing oracle live fetch hardening) |
| **Сесій drain** | **13** (`121÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S890…S899) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 24
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 24: **PH-S900…S909**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 121 → product-complete (PH-S900…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–23 | PH-S660…S889 | ✅ drained |
| 24 | PH-S890…S899 | **активна §5.12** — Replication quorum production gates |
| 25–26 | PH-S900…S919 | Pricing/trust gates |
| 27–29 | PH-S920…S959 | Ratio **95–96%** |
| 30–33 | PH-S960…S999 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 24 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S890** | replication quorum gate production | strict tier integration |
| **PH-S891** | replication rate cap HTTP wire | integration test |
| **PH-S892** | admin replication-pricing wasm polish | ui-core regression |
| **PH-S893** | stand smoke replication metrics parity | JSON↔Prom |
| **PH-S894** | replication depth stub | unit test |
| **PH-S895** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S895 |
| **PH-S896** | Galaxy §6.4 implemented table | docs |
| **PH-S897** | `poolai-vision-sync --check` | drift gate green |
| **PH-S898** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S899** | `galaxy_horizon_s890_integration` | replication band close |

**Після band 24 promote:** PH-S900…S909 (band 25 — Pricing oracle live fetch hardening).

---

## Не повторювати

Закриті PH-S880…S889 acceptance (verification lifecycle depth, checker drain/shadow submit, admin wasm strip, stand smoke lifecycle_depth) — див. FM §5.12 ✅.
