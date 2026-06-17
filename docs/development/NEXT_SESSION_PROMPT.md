# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S222 ✅ · vision **rev 169** · **5** відкритих (PH-S223…S227) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S223** — Admin i18n slim libs panel |
| **Відкритих** | **5** (PH-S223…S227) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (5 відкритих: PH-S223…S227)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S222 | Admin i18n slim workers panel | `admin.wrk.*` → `admin_workers_patch`; slim `i18n_core.js` |
| PH-S221 | Admin i18n slim updates-compat panel | slim patch; default layout jobs-only |

### Відкрито (PH-S223…S227)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S223** | Admin i18n slim libs panel |
| 2 | PH-S224 | Galaxy pricing cache age metrics smoke |
| 3 | PH-S225 | Galaxy verification sample metrics smoke |
| 4 | PH-S226 | Vision sprint-queue → map focus |
| 5 | PH-S227 | Vision VDT rules docs autosync audit |

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

## PH-S223 — scope

- `admin.lib.*` → `admin_libs_patch` + `admin_layout_libs`; remove from `i18n_core.js`
- Acceptance: FM/HANDOFF/NEXT; cargo test; vision-sync; push

---

## Copy-paste — PH-S223

```
PoolAI VDT · один PH-S* · main · MSYS2 PATH · git-push.md

S0: git fetch · HANDOFF · FM §5.12 · df -h /s

PH-S223 — Admin i18n slim libs panel (code/ui)
Scope: admin.lib.* Rust i18n patch; slim i18n_core.js; cargo test; FM/HANDOFF/NEXT; vision-sync; commit+push
```
