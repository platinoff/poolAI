# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-18 (PH-S1010 ✅ band 36 product-complete · FM **§5.15** ✅ · vision **rev 306** · rust_ratio **≥95%** · **maintenance mode**)

| **Режим** | **Maintenance mode** (FM §5.15 ✅) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** ✅ |
| **Master backlog** | **0** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **0** |
| **Нові PH-S*** | Лише за явним scan власника (BLOCKED/Deferred / FM-horizon v2) |

---

## Maintenance mode (PH-S1010)

Після **PH-S1010** / FM **§5.15** ✅ сесії працюють у **maintenance mode**:

| Крок | Дія |
|------|-----|
| S0 | `git fetch`; HANDOFF; FM **§5.15**; `poolai-vision-sync --check`; `df -h /s` |
| Scope | Лише BLOCKED/Deferred (FM-003 LAN, FM-041 Cloud SDK) або явний FM-horizon v2 за запитом власника |
| Тести | `cargo fmt --all` → `cargo test-ci` перед push |
| Docs | STABLE «development complete»; INDEX/DIGEST без нових PH-S* у §5.12 |
| **Не** | Автоматичний project scan / replenish §5.12 без запиту власника |

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Закрито (band 36 — PH-S1010 ✅)

| Sprint | Focus | Result |
|--------|--------|--------|
| **PH-S1010** | FM §5.15 product-complete declaration | STABLE final; HANDOFF maintenance; `galaxy_horizon_s1010_integration`; admin_charts wasm-only zriz; vision rev **306** |

---

## Не повторювати

Bands 1–36 (PH-S660…S1010) ✅ — master backlog drained. **`абракадабра`** project scan — лише за явним запитом власника після maintenance triage.
