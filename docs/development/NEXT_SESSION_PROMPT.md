# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S237 ✅ · vision **rev 186** · **2** відкритих (PH-S238…S239) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S238** — Admin i18n slim users panel |
| **Відкритих** | **2** (PH-S238…S239) |

---

## PH-S238 — scope

- `admin.usr.*` → `poolai-ui-core` + `admin_users_patch` + `admin_layout_users`
- Remove `admin.usr.*` + `admin.page.users` from `i18n_core.js`
- Pattern: PH-S236…S237 slim panels
- Acceptance: targeted `cargo test`; FM/HANDOFF/NEXT; push

---

## Copy-paste — PH-S238

```
PH-S238 — Admin i18n slim users panel (code)
Scope: admin.usr.* → poolai-ui-core; slim layout; cargo test; FM/HANDOFF/NEXT; commit+push
```
