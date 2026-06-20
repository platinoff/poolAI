# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-20 (PH-S650…S659 ✅ · vision **rev 264** · **0** відкритих · rust_ratio **94.76%**)

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

## Закрито (PH-S650…S659)

PH-S650 ✅ `poolai-ui-core` warning cleanup (`table.rs` unused import).  
PH-S651 ✅ `GALAXY_GRID_ROADMAP` sync to S640…S649 and rust_ratio 94.76%.  
PH-S652 ✅ cursor sandbox temp cache cleanup (disk pressure unblock).  
PH-S653 ✅ sandbox path restore + `poolai-vision-sync --check` (ok, rev 264).  
PH-S654 ✅ `cargo fmt --all` gate.  
PH-S655 ✅ `cargo test -p poolai-ui-core` rerun (3 pre-existing failing tests captured).  
PH-S656 ✅ FM §5.12 maintenance close-band sync.  
PH-S657 ✅ HANDOFF maintenance snapshot sync.  
PH-S658 ✅ NEXT prompt refresh for next `абракадабра`.  
PH-S659 ✅ STABLE_STATE header refresh.

**rust_ratio:** **94.76%** · **BLOCKED:** FM-003 LAN · FM-041 Cloud SDK.
