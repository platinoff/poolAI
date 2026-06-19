# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-19 (PH-S590…S599 ✅ · vision **rev 259** · **0** відкритих · rust_ratio **94.67%**)

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

## Закрито (PH-S590…S599)

PH-S590 ✅ vision orbit UX (~30% WASD, pause/layers, `vision.spec.ts` rotY).  
PH-S591/S592 ✅ prefetch profile bandwidth/egress gates.  
PH-S593/S594 ✅ GPU passthrough + TEE attestation HTTP integration.  
PH-S595 ✅ wallet rebind admin override.  
PH-S596 ✅ network-profiles admin PUT UI.  
PH-S597 ✅ on-chain grid complete HTTP.  
PH-S598 ✅ a11y matrix (network-profiles, seed-inventory, security-advisories).  
PH-S599 ✅ `galaxy_horizon_s591_integration` close band.

**rust_ratio:** **94.67%** · **BLOCKED:** FM-003 LAN · FM-041 Cloud SDK.
