# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S225 ✅ · vision **rev 172** · **2** відкритих (PH-S226…S227) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S226** — Vision sprint-queue → map focus |
| **Відкритих** | **2** (PH-S226…S227) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (2 відкритих: PH-S226…S227)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S225 | Galaxy verification sample metrics smoke | verification gauges on `/metrics` |
| PH-S224 | Galaxy pricing cache age metrics smoke | `galaxy_pricing_cache_age_seconds` |

### Відкрито

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S226** | Vision sprint-queue → map focus |
| 2 | PH-S227 | Vision VDT rules docs autosync audit |

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

## PH-S226 — scope

- `docs/vision/vision.js` — click sprint-queue item focuses/selects map node
- Acceptance: FM/HANDOFF/NEXT; rev++; `poolai-vision-sync --check`; push

---

## Copy-paste — PH-S226

```
PoolAI VDT · один PH-S* · main · MSYS2 PATH · git-push.md

S0: git fetch · HANDOFF · FM §5.12 · df -h /s

PH-S226 — Vision sprint-queue → map focus (docs/vision)
Scope: queue click → map node select; rev++; poolai-vision-sync --check; FM/HANDOFF/NEXT; commit+push
```
