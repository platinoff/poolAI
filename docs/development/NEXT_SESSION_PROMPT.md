# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S226 ✅ · vision **rev 173** · **1** відкритий (PH-S227) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S227** — Vision VDT rules docs autosync audit |
| **Відкритих** | **1** (PH-S227) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (1 відкритий: PH-S227)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S226 | Vision sprint-queue → map focus | queue/ticker → map; panel expand fix |
| PH-S225 | Galaxy verification sample metrics smoke | verification gauges on `/metrics` |

### Відкрито

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S227** | Vision VDT rules ↔ docs autosync audit |

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

## PH-S227 — scope

- `poolai-vision-sync --check` — manifest ↔ `.cursor/rules/*.mdc` cross-link drift
- Acceptance: FM/HANDOFF/NEXT; rev++; push; replenish §5.13 після закриття S227

---

## Copy-paste — PH-S227

```
PoolAI VDT · один PH-S* · main · MSYS2 PATH · git-push.md

S0: git fetch · HANDOFF · FM §5.12 · df -h /s

PH-S227 — Vision VDT rules docs autosync audit (docs/vision)
Scope: manifest ↔ .mdc drift gate; rev++; poolai-vision-sync --check; FM/HANDOFF/NEXT; commit+push
```
