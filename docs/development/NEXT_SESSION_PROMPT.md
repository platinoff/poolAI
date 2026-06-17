# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S230 ✅ · vision **rev 179** · **5** відкритих (PH-S231…S235) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S231** — Admin i18n slim security panel |
| **Відкритих** | **5** (PH-S231…S235) |

---

## PH-S231 — scope

- `admin.sec.*` → `poolai-ui-core` + `admin_security_patch` + slim layout
- Remove `admin.tenants.col.name` shim from jobs patch once security owns headers
- Acceptance: `cargo test` targeted; FM/HANDOFF/NEXT; push

---

## Copy-paste — PH-S231

```
PH-S231 — Admin i18n slim security panel (code)
Scope: admin.sec.* → poolai-ui-core; slim layout; cargo test; FM/HANDOFF/NEXT; commit+push
```
