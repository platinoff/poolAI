# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S212 ✅ · vision **rev 154** · **7** відкритих (PH-S213…S219) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S213** — Galaxy prefetch metrics stand smoke |
| **Відкритих** | **7** (PH-S213…S219) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (7 відкритих: PH-S213…S219)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S212 | Vision reduced-motion map FX | `prefers-reduced-motion` → skip glow/animation; `map-fx-off` |
| PH-S211 | Admin i18n slim jobs panel | `admin.jobs.*` → `poolai-ui-core`; slim `i18n_core.js` |
| PH-S210 | Stand smoke seed_inventory GET | `grid_seed_inventory` on live stand |

### Відкрито — code-first + vision a11y band (PH-S213…S219)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S213** | Galaxy prefetch metrics stand smoke |
| 2 | **PH-S214** | Admin i18n slim raid panel |
| 3 | **PH-S215** | Vision panel collapse focus restore |
| 4 | **PH-S216** | Galaxy pricing fallback metrics smoke |
| 5 | **PH-S217** | Admin i18n slim grid-pricing panel |
| 6 | **PH-S218** | Vision map aria-live selection |
| 7 | **PH-S219** | Galaxy trust payout metrics smoke |

---

## S0

```bash
git fetch origin
df -h /s
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export CARGO_TARGET_DIR=/s/rust/poolAI/target
export K8S_OPENAPI_ENABLED_VERSION=1.28
```

---

## PH-S213 — scope

- `src/bin/poolai_http_stand_smoke.rs` — stand smoke checks prefetch counters on `/metrics`
- Acceptance: FM/HANDOFF/NEXT; `cargo test-ci`; push

---

## Copy-paste — PH-S213

```
PoolAI VDT · один PH-S* · main · MSYS2 PATH · git-push.md

S0: git fetch · HANDOFF · FM §5.12 · df -h /s

PH-S213 — Galaxy prefetch metrics stand smoke (tests)
Scope: poolai-http-stand-smoke prefetch counters on /metrics; cargo test-ci; FM/HANDOFF/NEXT; vision-sync if FM rev changes; commit+push
```
