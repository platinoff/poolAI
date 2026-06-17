# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S217 ✅ · vision **rev 156** · **2** відкритих (PH-S218…S219) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S218** — Vision map aria-live selection |
| **Відкритих** | **2** (PH-S218…S219) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (2 відкритих: PH-S218…S219)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S217 | Admin i18n slim grid-pricing panel | `admin.page.gridPricing` → `poolai-ui-core` |
| PH-S216 | Galaxy pricing fallback metrics smoke | `galaxy_pricing_forced_fallback_total` on `/metrics` |
| PH-S215 | Vision panel collapse focus restore | `focusPanelToggle`; UI cache v71 |

### Відкрито — code-first + vision a11y band (PH-S218…S219)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S218** | Vision map aria-live selection |
| 2 | **PH-S219** | Galaxy trust payout metrics smoke |

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

## PH-S218 — scope

- `docs/vision/vision.js` — `aria-live` region for selected node label on map selection
- Acceptance: FM/HANDOFF/NEXT; rev++; `poolai-vision-sync --check`; push

---

## Copy-paste — PH-S218

```
PoolAI VDT · один PH-S* · main · MSYS2 PATH · git-push.md

S0: git fetch · HANDOFF · FM §5.12 · df -h /s

PH-S218 — Vision map aria-live selection (docs/vision)
Scope: aria-live region for selected node; rev++; poolai-vision-sync --check; FM/HANDOFF/NEXT; commit+push
```
