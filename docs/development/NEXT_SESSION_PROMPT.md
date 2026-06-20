# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-20 (master backlog **351** PH-S660…S1010 · active **PH-S660…S669** · vision **rev 265** · rust_ratio **94.76%**)

| **← наступний** | **`абракадабра`** (drain **10** активних → promote наступні 10 з master backlog) |
| **Master backlog** | **351** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM **§5.14** |
| **Активних §5.12** | **10** (PH-S660…S669, band 1) |
| **Після band 1** | promote PH-S670…S679 (band 2) |
| **Сесій drain** | **36** (`351÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (зараз PH-S660…S669) — деталі в master backlog band 1
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 1: **PH-S670…S679**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 351 (PH-S660…S1010)

Повний реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s660s1010-351-pending-2026-06-20).

| Band | Sprints | Статус |
|------|---------|--------|
| 1 | PH-S660…S669 | **активна §5.12** `[ ]` |
| 2 | PH-S670…S679 | queued |
| 3–35 | PH-S680…S1009 | queued |
| 36 | PH-S1010 | tail / replenish |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK (BLOCKED/Deferred).

---

## Активна смуга (band 1 — drain зараз)

| Sprint | Фокус |
|--------|--------|
| **PH-S660** | ui-core UTC timestamp fix |
| **PH-S661** | ui-core ML URL encode fix |
| **PH-S662** | `cargo test -p poolai-ui-core` green |
| **PH-S663** | wasm-only datetime `src/ui/mod.rs` |
| **PH-S664** | Galaxy §8 network_profile persist stub |
| **PH-S665** | loc-audit → `rust_ratio.json` |
| **PH-S666** | INDEX canon sync |
| **PH-S667** | `poolai-vision-sync --check` |
| **PH-S668** | ratio advisory `--min-ratio 0.95` |
| **PH-S669** | horizon close band + docs sync |

---

## Закрито (PH-S650…S659)

PH-S650…S659 ✅ maintenance close band (2026-06-20).

**rust_ratio:** **94.76%** · **BLOCKED:** FM-003 LAN · FM-041 Cloud SDK.
