# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-20 (PH-S610…S619 ✅ · vision **rev 261** · **0** відкритих · rust_ratio **94.72%**)

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

## Закрито (PH-S610…S619)

PH-S610 ✅ stale-epoch grid result trust delta −50.  
PH-S611 ✅ worker-unhealthy streak trust delta −30.  
PH-S612 ✅ hot-tier scheduling gate (`hot_tier_hit_ratio > 0.8`).  
PH-S613 ✅ re-migrate delta-fetch missing shards on PATCH.  
PH-S614 ✅ prefetch order by shard access weight.  
PH-S615 ✅ replication hourly cap HTTP integration.  
PH-S616 ✅ payout-batch primary/secondary/worker lamports wire.  
PH-S617 ✅ checker-timeout grid result HTTP integration.  
PH-S618 ✅ admin raid `formatBytes` wasm-first slim.  
PH-S619 ✅ `galaxy_horizon_s610_integration` close band.

**rust_ratio:** **94.72%** · **BLOCKED:** FM-003 LAN · FM-041 Cloud SDK.
