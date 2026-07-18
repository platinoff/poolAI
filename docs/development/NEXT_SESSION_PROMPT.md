# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-18 (band 37 **PH-S1011…S1018** ✅ · vision **rev 308**)

Maintenance mode active (FM §5.15) — owner ops band 37 drained; next **`абракадабра`** = project scan replenish.

| **← наступний** | **`абракадабра`** (project scan → replenish §5.12) |
| **§5.12 active** | **0** |
| **FM band** | maintenance + owner queue drained |
| **Сесій drain** | band 37 closed |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + project scan

1. `git fetch`; HANDOFF; FM **§5.12**; `poolai-vision-sync --check`; `df -h /s`
2. **Project scan** (§5.12 < 10) → top 10 PH-S* → drain
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Не повторювати

PH-S1018 ✅ band 37 · PH-S1010 ✅ product-complete · bands 1–36 drained. FM-003 LAN · FM-041 Cloud SDK — поза scope.
