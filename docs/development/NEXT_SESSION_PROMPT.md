# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-20 (PH-S650…S659 ✅ · черга **PH-S660…S669** · vision **rev 265** · rust_ratio **94.76%**)

| **← наступний** | **`абракадабра`** (drain **10** відкритих PH-S660…S669) |
| **Відкритих** | **10** (PH-S660…S669) |
| **Scan** | **не потрібен** — §5.12 уже заповнено project scan 2026-06-20 |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 (**10** відкритих PH-S660…S669); `poolai-vision-sync --check`; `df -h /s`
2. **Drain** усіх відкритих PH-S660…S669 (код → scope-тести)
3. Vision close: FM §5.12 ✅ + HANDOFF + NEXT → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Один commit (`git-commit-tree-msg.sh`) + `git push origin main` + самарі

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Черга drain (PH-S660…S669)

| Sprint | Фокус | Acceptance |
|--------|--------|------------|
| **PH-S660** | ui-core format timestamp UTC fix | `format_unix_timestamp_display_ph_s628` green |
| **PH-S661** | ui-core ML metric URL encode fix | `build_metric_history_url_ph_s314/s334` green |
| **PH-S662** | ui-core full test gate | `cargo test -p poolai-ui-core` 0 failed |
| **PH-S663** | Shared layout datetime wasm-only | drop `toLocaleString` in `src/ui/mod.rs` |
| **PH-S664** | network_profile persist stub | Galaxy §8 L916 stub + unit test |
| **PH-S665** | Rust ratio loc-audit refresh | `rust_ratio.json` sprint zriz |
| **PH-S666** | Docs INDEX canon sync | INDEX §7 + ratio + vision rev |
| **PH-S667** | poolai-vision-sync drift gate | `--check` green |
| **PH-S668** | Ratio hold advisory snapshot | `--min-ratio 0.95 --advisory` (шаблон **PH-S351** ✅) |
| **PH-S669** | Horizon close band S660–S668 | integration + FM/HANDOFF/NEXT/STABLE/GALAXY |

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

**Не повторювати:** PH-S351 ✅ ratio hold advisory (дублювати лише як PH-S668 у смузі drain).
