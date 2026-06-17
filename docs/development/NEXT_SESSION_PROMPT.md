# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S241 ✅ · vision **rev 190** · **1** відкритий (PH-S242) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S242** — Admin i18n nav shell key audit |
| **Відкритих** | **1** (PH-S242) |

---

## PH-S242 — scope

- Audit `admin.nav.*` keys — ensure only in `auth_dash_shell_patch` (PH-S162), not duplicated in `i18n_core.js`
- Add/extend tests in `poolai-ui-core` + admin layout parity
- Replenish §5.12 when queue closes (≤10 open from §5.13)
- Acceptance: targeted `cargo test`; FM/HANDOFF/NEXT; push

---

## Copy-paste — PH-S242

```
PH-S242 — Admin i18n nav shell key audit (code)
Scope: verify admin.nav.* only in auth_dash patch; cargo test; FM/HANDOFF/NEXT; replenish §5.12; commit+push
```
