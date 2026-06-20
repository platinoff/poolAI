# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-20 (master backlog **341** PH-S670…S1010 · active **PH-S670…S679** · vision **rev 266** · rust_ratio **94.70%**)

| **← наступний** | **`абракадабра`** (drain **10** активних → promote наступні 10 з master backlog) |
| **Master backlog** | **341** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM **§5.14** |
| **Активних §5.12** | **10** (PH-S670…S679, band 2) |
| **Після band 2** | promote PH-S680…S689 (band 3) |
| **Сесій drain** | **35** (`341÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (зараз PH-S670…S679) — деталі в master backlog band 2
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 2: **PH-S680…S689**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 341 (PH-S670…S1010)

Повний реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s660s1010-351-pending-2026-06-20).

| Band | Sprints | Статус |
|------|---------|--------|
| 1 | PH-S660…S669 | ✅ drained |
| 2 | PH-S670…S679 | **активна §5.12** `[ ]` |
| 3–35 | PH-S680…S1009 | queued |
| 36 | PH-S1010 | tail / replenish |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK (BLOCKED/Deferred).

---

## Активна смуга (band 2 — drain зараз)

| Sprint | Фокус |
|--------|--------|
| **PH-S670** | Galaxy verification metric HTTP wire |
| **PH-S671** | Galaxy replay metric HTTP wire |
| **PH-S672** | Admin panel wasm glue |
| **PH-S673** | Stand smoke /metrics export shape |
| **PH-S674** | Galaxy concept helper stub |
| **PH-S675** | loc-audit → `rust_ratio.json` |
| **PH-S676** | INDEX canon sync |
| **PH-S677** | `poolai-vision-sync --check` |
| **PH-S678** | ratio advisory `--min-ratio 0.95` |
| **PH-S679** | horizon close band + docs sync |

---

## Закрито (PH-S660…S669)

PH-S660…S669 ✅ band 1 drain (2026-06-20): ui-core UTC timestamp + ML URL encode; `poolai-ui-core` green; wasm-only `formatIsoDatetime`; network_profile heartbeat persist; `galaxy_horizon_s660_integration`; rust_ratio **94.70%** hold advisory.

**rust_ratio:** **94.70%** · **BLOCKED:** FM-003 LAN · FM-041 Cloud SDK.
