# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S239 ✅ · vision **rev 188** · **3** відкриті (PH-S240…S242) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S240** — Admin i18n slim table toolbar |
| **Відкритих** | **3** (PH-S240…S242) |

---

## PH-S240 — scope

- `admin.table.*` → `poolai-ui-core` + `admin_table_patch` + wire into default admin layout (or dedicated helper)
- Remove `admin.table.*` from `i18n_core.js`
- Pattern: PH-S236…S239 slim panels; table keys shared across many admin pages
- Acceptance: targeted `cargo test`; FM/HANDOFF/NEXT; push

---

## Copy-paste — PH-S240

```
PH-S240 — Admin i18n slim table toolbar (code)
Scope: admin.table.* → poolai-ui-core; slim patch on admin layout; cargo test; FM/HANDOFF/NEXT; commit+push
```
