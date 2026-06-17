# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S221 ✅ · vision **rev 168** · **6** відкритих (PH-S222…S227) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S222** — Admin i18n slim workers panel |
| **Відкритих** | **6** (PH-S222…S227) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (6 відкритих: PH-S222…S227)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S221 | Admin i18n slim updates-compat panel | `admin.updatesCompat.*` → slim patch; default layout jobs-only |
| PH-S220 | Admin i18n slim monitoring panel | `admin.mon.*` → `admin_monitoring_patch` |

### Відкрито (PH-S222…S227)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S222** | Admin i18n slim workers panel |
| 2 | PH-S223 | Admin i18n slim libs panel |
| 3 | PH-S224 | Galaxy pricing cache age metrics smoke |
| 4 | PH-S225 | Galaxy verification sample metrics smoke |
| 5 | PH-S226 | Vision sprint-queue → map focus |
| 6 | PH-S227 | Vision VDT rules docs autosync audit |

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

## PH-S222 — scope

- `admin.wrk.*` → `admin_workers_patch` + `admin_layout_workers`; remove from `i18n_core.js`
- Acceptance: FM/HANDOFF/NEXT; `cargo test -p poolai-ui-core i18n`; push

---

## Copy-paste — PH-S222

```
PoolAI VDT · один PH-S* · main · MSYS2 PATH · git-push.md

S0: git fetch · HANDOFF · FM §5.12 · df -h /s

PH-S222 — Admin i18n slim workers panel (code/ui)
Scope: admin.wrk.* Rust i18n patch; slim i18n_core.js; cargo test; FM/HANDOFF/NEXT; vision-sync; commit+push
```
