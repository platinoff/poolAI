# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S218 ✅ · vision **rev 157** · **1** відкритий (PH-S219) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S219** — Galaxy trust payout metrics smoke |
| **Відкритих** | **1** (PH-S219) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (1 відкритий: PH-S219)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S218 | Vision map aria-live selection | `#map-selection-live` on node select |
| PH-S217 | Admin i18n slim grid-pricing panel | `admin.page.gridPricing` → `poolai-ui-core` |
| PH-S216 | Galaxy pricing fallback metrics smoke | `galaxy_pricing_forced_fallback_total` on `/metrics` |

### Відкрито

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S219** | Galaxy trust payout metrics smoke |

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

## PH-S219 — scope

- `poolai-http-stand-smoke` — trust payout counters on live `/metrics` (PH-S182 pattern)
- Acceptance: FM/HANDOFF/NEXT; `cargo test --bin poolai-http-stand-smoke`; push

---

## Copy-paste — PH-S219

```
PoolAI VDT · один PH-S* · main · MSYS2 PATH · git-push.md

S0: git fetch · HANDOFF · FM §5.12 · df -h /s

PH-S219 — Galaxy trust payout metrics smoke (tests)
Scope: stand smoke trust payout counters on /metrics; cargo test; FM/HANDOFF/NEXT; commit+push
```
