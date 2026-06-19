# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-19 (PH-S600…S609 ✅ · vision **rev 260** · **0** відкритих · rust_ratio **94.69%**)

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

## Закрито (PH-S600…S609)

PH-S600 ✅ strict-locality HTTP (`locality_unsatisfied` / `prefetch-timeout` 409).  
PH-S601 ✅ semantic_hash human-review settlement HTTP.  
PH-S602 ✅ wallet rebind cooldown HTTP integration.  
PH-S603 ✅ `latency_ms_p95` tail-latency locality penalty + metric.  
PH-S604 ✅ topology ring / white-IP prefetch admission guard.  
PH-S605 ✅ RAID prefetch fetch on grid job HTTP.  
PH-S606 ✅ re-migrate prefetch on Migrating→Leased PATCH HTTP.  
PH-S607 ✅ fraud-proof hold via grid envelope HTTP.  
PH-S608 ✅ admin dashboard wasm-first formatter slim.  
PH-S609 ✅ `galaxy_horizon_s600_integration` close band.

**rust_ratio:** **94.69%** · **BLOCKED:** FM-003 LAN · FM-041 Cloud SDK.
