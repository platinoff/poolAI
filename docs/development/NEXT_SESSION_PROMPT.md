# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S242 ✅ · vision **rev 191** · **3** відкриті (PH-S243…S245) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S243** — Admin i18n slim admin chrome shell |
| **Відкритих** | **3** (PH-S243…S245) |

---

## PH-S243 — scope

- `admin.brand`, `admin.skipMain`, `admin.skipNav`, `admin.lang.label`, `admin.logout`, `admin.browserSuffix` → `auth_dash_shell_patch`
- Remove from `i18n_core.js`; extend auth_dash tests
- Pattern: PH-S242 nav shell audit
- Acceptance: targeted `cargo test`; FM/HANDOFF/NEXT; push

---

## Copy-paste — PH-S243

```
PH-S243 — Admin i18n slim admin chrome shell (code)
Scope: admin chrome keys → auth_dash patch; cargo test; FM/HANDOFF/NEXT; commit+push
```
