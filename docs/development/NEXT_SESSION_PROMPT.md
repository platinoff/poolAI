# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-20 (PH-S630…S639 ✅ · vision **rev 263** · **0** відкритих · rust_ratio **94.75%**)

| **← наступний** | **`абракадабра`** (project scan → +10 PH-S* → drain) |
| **Відкритих** | **0** |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12; `poolai-vision-sync --check`; `df -h /s`
2. **Project scan** всього репо → **10 PH-S*** у FM §5.12
3. Drain усіх відкритих (код → scope-тести)
4. Vision close: FM §5.12 ✅ + HANDOFF + NEXT → `poolai-vision-sync` → `--check`
5. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
6. Один commit (`git-commit-tree-msg.sh`) + `git push origin main` + самарі

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

Vision e2e (після `docs/vision/` scope):

```bash
bash bin/e2e-playwright.sh --start
```

---

## Закрито (PH-S630…S639)

PH-S630 ✅ verification mismatch trust delta persist (−100).  
PH-S631 ✅ cleared settlement payout-batch HTTP `/metrics`.  
PH-S632 ✅ prefetch seed-pull hot-tier fallback HTTP.  
PH-S633 ✅ replication executor enqueue HTTP.  
PH-S634 ✅ replay verification enqueue on mismatch HTTP.  
PH-S635 ✅ worker-unhealthy heartbeat-remote HTTP.  
PH-S636 ✅ admin topology formatters wasm-only.  
PH-S637 ✅ admin security datetime wasm-only slim.  
PH-S638 ✅ admin grid-pricing formatters wasm-only.  
PH-S639 ✅ `galaxy_horizon_s630_integration` close band.

**rust_ratio:** **94.75%** · **BLOCKED:** FM-003 LAN · FM-041 Cloud SDK.
