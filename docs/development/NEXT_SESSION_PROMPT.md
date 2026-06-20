# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-20 (master backlog **321** PH-S690…S1010 · active **PH-S690…S699** · vision **rev 268** · rust_ratio **94.73%** · Cursor **3.8.11** research ✅)

| **← наступний** | **`абракадабра`** (drain **10** активних → promote наступні 10 з master backlog) |
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

---

## Активна смуга (band 4 — drain зараз)

| Sprint | Фокус |
|--------|--------|
| **PH-S690** | Galaxy replication metric HTTP wire |
| **PH-S691** | Galaxy pricing metric HTTP wire |
| **PH-S692** | Admin panel wasm glue |
| **PH-S693** | Stand smoke /metrics export shape |
| **PH-S694** | Galaxy concept helper stub |
| **PH-S695** | loc-audit → `rust_ratio.json` |
| **PH-S696** | INDEX canon sync |
| **PH-S697** | `poolai-vision-sync --check` |
| **PH-S698** | ratio advisory `--min-ratio 0.95` |
| **PH-S699** | horizon close band + docs sync |

---

## Закрито (PH-S680…S689)

PH-S680…S689 ✅ band 3 drain (2026-06-20): settlement/trust metrics HTTP; `settlement_gate_depth_stub`; wasm `parsePrometheusGauge` on payout-batch; stand smoke settlement/trust API; `galaxy_horizon_s680_integration`; rust_ratio **94.73%** hold advisory.

**rust_ratio:** **94.73%** · **BLOCKED:** FM-003 LAN · FM-041 Cloud SDK.
