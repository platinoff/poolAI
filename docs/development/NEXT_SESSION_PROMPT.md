# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S235 ✅ · vision **rev 184** · **4** відкритих (PH-S236…S239) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S236** — Admin i18n slim instances panel |
| **Відкритих** | **4** (PH-S236…S239) |

---

## PH-S236 — scope

- `admin.inst.*` → `poolai-ui-core` + `admin_instances_patch` + `admin_layout_instances`
- Remove `admin.inst.*` + `admin.page.instances` from `i18n_core.js`
- Pattern: PH-S230…S234 (tenants/security/topology slim panels)
- Acceptance: targeted `cargo test`; FM/HANDOFF/NEXT; push

---

## Copy-paste — PH-S236

```
PH-S236 — Admin i18n slim instances panel (code)
Scope: admin.inst.* → poolai-ui-core; slim layout; cargo test; FM/HANDOFF/NEXT; commit+push
```
