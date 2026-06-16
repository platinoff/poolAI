# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S213 ✅ · vision **rev 155** · **6** відкритих (PH-S214…S219) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S214** — Admin i18n slim raid panel |
| **Відкритих** | **6** (PH-S214…S219) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (6 відкритих: PH-S214…S219)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S213 | Galaxy prefetch metrics stand smoke | `galaxy_prefetch_*` on live `/metrics` |
| PH-S212 | Vision reduced-motion map FX | `prefers-reduced-motion` → skip glow/animation |
| PH-S211 | Admin i18n slim jobs panel | `admin.jobs.*` → `poolai-ui-core` |

### Відкрито — code-first + vision a11y band (PH-S214…S219)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S214** | Admin i18n slim raid panel |
| 2 | **PH-S215** | Vision panel collapse focus restore |
| 3 | **PH-S216** | Galaxy pricing fallback metrics smoke |
| 4 | **PH-S217** | Admin i18n slim grid-pricing panel |
| 5 | **PH-S218** | Vision map aria-live selection |
| 6 | **PH-S219** | Galaxy trust payout metrics smoke |

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

## PH-S214 — scope

- `admin.raid.*` → `poolai-ui-core`; slim `i18n_core.js`; `admin_layout_raid` (PH-S211 jobs pattern)
- Acceptance: FM/HANDOFF/NEXT; `cargo test-ci`; push

---

## Copy-paste — PH-S214

```
PoolAI VDT · один PH-S* · main · MSYS2 PATH · git-push.md

S0: git fetch · HANDOFF · FM §5.12 · df -h /s

PH-S214 — Admin i18n slim raid panel (code/ui)
Scope: admin.raid.* Rust i18n patch; slim i18n_core.js; cargo test-ci; FM/HANDOFF/NEXT; vision-sync; commit+push
```
