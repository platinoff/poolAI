# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-20 (master backlog **321** · theme-aware slots · active **PH-S690…S699** · vision **rev 269** · rust_ratio **94.73%**)

| **← наступний** | **`абракадабра`** (drain band 4 PH-S690…S699) |
| **Master backlog** | **321** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM **§5.14** |
| **Активних §5.12** | **10** (PH-S690…S699, band 4) |
| **Після band 4** | promote PH-S700…S709 (band 5) |
| **Сесій drain** | **33** (`321÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (зараз PH-S690…S699) — деталі в master backlog band 4
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 4: **PH-S700…S709**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 321 (PH-S690…S1010)

Повний реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s660s1010-351-pending-2026-06-20).

| Band | Sprints | Статус |
|------|---------|--------|
| 1 | PH-S660…S669 | ✅ drained |
| 2 | PH-S670…S679 | ✅ drained |
| 3 | PH-S680…S689 | ✅ drained |
| 4 | PH-S690…S699 | **активна §5.12** `[ ]` |
| 5–35 | PH-S700…S1009 | queued |
| 36 | PH-S1010 | tail / replenish |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK (BLOCKED/Deferred).

**Band slot map:** slots 1–2 = theme (Galaxy JSON APIs / wasm / stand smoke / …); regen `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 4 — drain зараз)

| Sprint | Фокус | Acceptance |
|--------|--------|------------|
| **PH-S690** | `GET /api/v1/grid/replication-metrics` | Galaxy §6.4 · integration test |
| **PH-S691** | `GET /api/v1/grid/pricing-metrics` | Galaxy §4.2 oracle snapshot · integration test |
| **PH-S692** | Admin wasm glue (replication/pricing panels) | JSON metrics fetch / parsePrometheusGauge |
| **PH-S693** | Stand smoke replication + pricing API | poolai-http-stand-smoke export shape |
| **PH-S694** | Concept stub (replication/pricing depth) | unit test · Galaxy §4.2 / §6.4 |
| **PH-S695** | loc-audit → `rust_ratio.json` | sprint zriz PH-S695 |
| **PH-S696** | INDEX / HANDOFF / NEXT / STABLE / GALAXY sync | docs canon |
| **PH-S697** | `poolai-vision-sync --check` | drift gate green |
| **PH-S698** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S699** | `galaxy_horizon_s690_integration` | horizon close + docs |

**Після band 4 promote:** PH-S700…S709 (band 5 — **Admin wasm slim**, slots 1–2 = wasm panels, не generic metric HTTP).

---

## Закрито (PH-S680…S689)

PH-S680…S689 ✅ band 3 drain (2026-06-20): settlement/trust metrics HTTP; `settlement_gate_depth_stub`; wasm `parsePrometheusGauge` on payout-batch; stand smoke settlement/trust API; `galaxy_horizon_s680_integration`; rust_ratio **94.73%** hold advisory.

**rust_ratio:** **94.73%** · **BLOCKED:** FM-003 LAN · FM-041 Cloud SDK.
