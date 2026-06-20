# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-20 (master backlog **331** PH-S680…S1010 · active **PH-S680…S689** · vision **rev 267** · rust_ratio **94.72%**)

| **← наступний** | **`абракадабра`** (drain **10** активних → promote наступні 10 з master backlog) |
| **Master backlog** | **331** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM **§5.14** |
| **Активних §5.12** | **10** (PH-S680…S689, band 3) |
| **Після band 3** | promote PH-S690…S699 (band 4) |
| **Сесій drain** | **34** (`331÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (зараз PH-S680…S689) — деталі в master backlog band 3
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 3: **PH-S690…S699**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 331 (PH-S680…S1010)

Повний реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s660s1010-351-pending-2026-06-20).

| Band | Sprints | Статус |
|------|---------|--------|
| 1 | PH-S660…S669 | ✅ drained |
| 2 | PH-S670…S679 | ✅ drained |
| 3 | PH-S680…S689 | **активна §5.12** `[ ]` |
| 4–35 | PH-S690…S1009 | queued |
| 36 | PH-S1010 | tail / replenish |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK (BLOCKED/Deferred).

---

## Активна смуга (band 3 — drain зараз)

| Sprint | Фокус |
|--------|--------|
| **PH-S680** | Galaxy settlement metric HTTP wire |
| **PH-S681** | Galaxy trust metric HTTP wire |
| **PH-S682** | Admin panel wasm glue |
| **PH-S683** | Stand smoke /metrics export shape |
| **PH-S684** | Galaxy concept helper stub |
| **PH-S685** | loc-audit → `rust_ratio.json` |
| **PH-S686** | INDEX canon sync |
| **PH-S687** | `poolai-vision-sync --check` |
| **PH-S688** | ratio advisory `--min-ratio 0.95` |
| **PH-S689** | horizon close band + docs sync |

---

## Закрито (PH-S670…S679)

PH-S670…S679 ✅ band 2 drain (2026-06-20): verification/replay metrics HTTP; `verification_replay_depth_stub`; wasm `parsePrometheusGauge`; stand smoke API shape; `galaxy_horizon_s670_integration`; rust_ratio **94.72%** hold advisory.

**rust_ratio:** **94.72%** · **BLOCKED:** FM-003 LAN · FM-041 Cloud SDK.
