# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-20 (master backlog **311** · theme-aware slots · active **PH-S700…S709** · vision **rev 270** · rust_ratio **94.67%**)

| **← наступний** | **`абракадабра`** (drain band 5 PH-S700…S709) |
| **Master backlog** | **311** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM **§5.14** |
| **Активних §5.12** | **10** (PH-S700…S709, band 5 wasm slim) |
| **Після band 5** | promote PH-S710…S719 (band 6 stand smoke) |
| **Сесій drain** | **32** (`311÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (зараз PH-S700…S709) — деталі в master backlog band 5
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 5: **PH-S710…S719**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 311 (PH-S700…S1010)

Повний реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s660s1010-351-pending-2026-06-20).

| Band | Sprints | Статус |
|------|---------|--------|
| 1 | PH-S660…S669 | ✅ drained |
| 2 | PH-S670…S679 | ✅ drained |
| 3 | PH-S680…S689 | ✅ drained |
| 4 | PH-S690…S699 | ✅ drained |
| 5 | PH-S700…S709 | **активна §5.12** `[ ]` |
| 6–35 | PH-S710…S1009 | queued |
| 36 | PH-S1010 | tail / replenish |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK (BLOCKED/Deferred).

**Band slot map:** slots 1–2 = theme (Admin wasm slim / stand smoke / …); regen `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 5 — drain зараз)

| Sprint | Фокус | Acceptance |
|--------|--------|------------|
| **PH-S700** | Admin wasm slim panel #1 | poolai-ui-core → poolai-ui-wasm |
| **PH-S701** | Admin wasm slim panel #2 | admin_charts.js canvas glue → wasm |
| **PH-S702** | Admin wasm glue regression | admin/mod.rs wasm render/parsePrometheusGauge |
| **PH-S703** | Stand smoke /metrics export shape | poolai-http-stand-smoke |
| **PH-S704** | Concept stub (Galaxy §4–§8) | unit test |
| **PH-S705** | loc-audit → `rust_ratio.json` | sprint zriz PH-S705 |
| **PH-S706** | INDEX / HANDOFF / NEXT / STABLE / GALAXY sync | docs canon |
| **PH-S707** | `poolai-vision-sync --check` | drift gate green |
| **PH-S708** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S709** | `galaxy_horizon_s700_integration` | horizon close + docs |

**Після band 5 promote:** PH-S710…S719 (band 6 — **Stand smoke /metrics parity**).

---

## Закрито (PH-S690…S699)

PH-S690…S699 ✅ band 4 drain (2026-06-20): replication/pricing metrics HTTP; `replication_pricing_depth_stub`; wasm `parsePrometheusGauge` on grid-replication-pricing; stand smoke replication/pricing API; `galaxy_horizon_s690_integration`; rust_ratio **94.67%** hold advisory.
