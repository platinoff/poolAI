# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S236 ✅ · vision **rev 185** · **3** відкритих (PH-S237…S239) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S237** — Admin i18n slim vm panel |
| **Відкритих** | **3** (PH-S237…S239) |

---

## PH-S237 — scope

- `admin.vmadm.*` → `poolai-ui-core` + `admin_vm_patch` + `admin_layout_vm`
- Remove `admin.vmadm.*` + `admin.page.vm` from `i18n_core.js`
- Pattern: PH-S236 instances slim panel
- Acceptance: targeted `cargo test`; FM/HANDOFF/NEXT; push

---

## Copy-paste — PH-S237

```
PH-S237 — Admin i18n slim vm panel (code)
Scope: admin.vmadm.* → poolai-ui-core; slim layout; cargo test; FM/HANDOFF/NEXT; commit+push
```
