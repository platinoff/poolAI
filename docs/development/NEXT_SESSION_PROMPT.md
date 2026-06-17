# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S224 ✅ · vision **rev 171** · **3** відкритих (PH-S225…S227) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S225** — Galaxy verification sample metrics smoke |
| **Відкритих** | **3** (PH-S225…S227) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (3 відкритих: PH-S225…S227)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S224 | Galaxy pricing cache age metrics smoke | `galaxy_pricing_cache_age_seconds` on `/metrics` |
| PH-S223 | Admin i18n slim libs panel | `admin.lib.*` → `admin_libs_patch` |

### Відкрито (PH-S225…S227)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S225** | Galaxy verification sample metrics smoke |
| 2 | PH-S226 | Vision sprint-queue → map focus |
| 3 | PH-S227 | Vision VDT rules docs autosync audit |

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

## PH-S225 — scope

- `poolai-http-stand-smoke` — verification counters on live `/metrics` (PH-S177 pattern)
- Acceptance: FM/HANDOFF/NEXT; `cargo test --bin poolai-http-stand-smoke ph_s225`; push

---

## Copy-paste — PH-S225

```
PoolAI VDT · один PH-S* · main · MSYS2 PATH · git-push.md

S0: git fetch · HANDOFF · FM §5.12 · df -h /s

PH-S225 — Galaxy verification sample metrics smoke (tests)
Scope: stand smoke verification counters on /metrics; cargo test; FM/HANDOFF/NEXT; vision-sync; commit+push
```
