# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-20 (PH-S620…S629 ✅ · vision **rev 262** · **0** відкритих · rust_ratio **94.73%**)

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

## Закрито (PH-S620…S629)

PH-S620 ✅ verification verdict trust delta persists to store (+10 match).  
PH-S621 ✅ telegram_edge low-trust payout-held HTTP `/metrics`.  
PH-S622 ✅ post-mismatch elevated sampling HTTP.  
PH-S623 ✅ lease-acquired prefetch HTTP (`POST /jobs/{id}/lease`).  
PH-S624 ✅ hot-tier promote/evict HTTP integration.  
PH-S625 ✅ prefetch ingest/wait/complete HTTP metric band.  
PH-S626 ✅ `galaxy_shard_fetch_latency_ms_p50` gauge.  
PH-S627 ✅ admin raid `formatBytes` wasm-only (drop JS dup).  
PH-S628 ✅ admin security `formatUnixTimestamp` / `formatRotationKind` wasm.  
PH-S629 ✅ `galaxy_horizon_s620_integration` close band.

**rust_ratio:** **94.73%** · **BLOCKED:** FM-003 LAN · FM-041 Cloud SDK.
