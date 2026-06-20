# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-20 (PH-S640…S649 ✅ · vision **rev 264** · **0** відкритих · rust_ratio **94.76%**)

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

---

## Закрито (PH-S640…S649)

PH-S640 ✅ replay pending resolved HTTP.  
PH-S641 ✅ verification replay record + history API.  
PH-S642 ✅ verification checker enqueue HTTP.  
PH-S643 ✅ trust payout-eligible HTTP.  
PH-S644 ✅ settlement resolved HTTP.  
PH-S645 ✅ prefetch strict-mode HTTP.  
PH-S646 ✅ admin dashboard datetime wasm-only.  
PH-S647 ✅ admin updates-compat wasm-only.  
PH-S648 ✅ admin jobs lease badge wasm-only.  
PH-S649 ✅ `galaxy_horizon_s640_integration` close band.

**rust_ratio:** **94.76%** · **BLOCKED:** FM-003 LAN · FM-041 Cloud SDK.
