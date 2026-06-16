# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S209 ✅ · vision **rev 151** · **10** відкритих (PH-S210…S219) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S210** — Stand smoke seed_inventory GET |
| **Відкритих** | **10** (PH-S210…S219) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (10 відкритих: PH-S210…S219)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S209 | Vision map a11y focus ring | `:focus-visible` controls + nodes; roving tabindex |
| PH-S208 | Stand smoke vision revision parity | `X-PoolAI-Vision-Revision` + FM/manifest |
| PH-S207 | Admin i18n slim monitoring panel | `admin.mon.*` → `poolai-ui-core` |

### Відкрито — code-first + vision a11y band (PH-S210…S219)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S210** | Stand smoke seed_inventory GET |
| 2 | **PH-S211** | Admin i18n slim jobs panel |
| 3 | **PH-S212** | Vision reduced-motion map FX |
| 4 | **PH-S213** | Galaxy prefetch metrics stand smoke |
| 5 | **PH-S214** | Admin i18n slim raid panel |
| 6 | **PH-S215** | Vision panel collapse focus restore |
| 7 | **PH-S216** | Galaxy pricing fallback metrics smoke |
| 8 | **PH-S217** | Admin i18n slim grid-pricing panel |
| 9 | **PH-S218** | Vision map aria-live selection |
| 10 | **PH-S219** | Galaxy trust payout metrics smoke |

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

## PH-S210 — scope

- `src/bin/poolai_http_stand_smoke.rs` — case `grid_seed_inventory` for `GET /api/v1/grid/seed-inventory`
- Acceptance: FM/HANDOFF/NEXT; `cargo test-ci`; push

---

## Copy-paste — PH-S210

```
PoolAI VDT · один PH-S* · main · MSYS2 PATH · git-push.md

S0: git fetch · HANDOFF · FM §5.12 · df -h /s

PH-S210 — Stand smoke seed_inventory GET (tests)
Scope: poolai-http-stand-smoke grid_seed_inventory case; cargo test-ci; FM/HANDOFF/NEXT; vision-sync if FM rev changes; commit+push
```
