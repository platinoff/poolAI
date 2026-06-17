# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S242 ✅ · replenish §5.12 · vision **rev 192** (after sync) · **10** відкритих (PH-S243…S252) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S243** — Admin i18n slim admin chrome shell |
| **Відкритих** | **10** (PH-S243…S252) |

---

## Черга §5.12 (канон)

| Sprint | Scope | Type |
|--------|-------|------|
| **PH-S243** ← | admin chrome → `auth_dash_shell_patch` | code |
| PH-S244 | `galaxy_pricing_stale_served` stand smoke | tests |
| PH-S245 | `admin.status.*` slim patch | code |
| PH-S246 | `err.hint*` + access keys slim | code |
| PH-S247 | pricing provider metrics stand smoke | tests |
| PH-S248 | `vm.*` modal i18n slim | code |
| PH-S249 | settlement metrics stand smoke | tests |
| PH-S250 | shard locality metrics stand smoke | tests |
| PH-S251 | GALAXY_GRID_ROADMAP + README sync | docs |
| PH-S252 | `ui.confirm*` modal slim patch | code |

---

## PH-S243 — scope

- `admin.brand`, `admin.skipMain`, `admin.skipNav`, `admin.lang.label`, `admin.logout`, `admin.browserSuffix` → `auth_dash_shell_patch` (`ADMIN_CHROME_*`)
- Remove from `i18n_core.js`; extend auth_dash tests
- Pattern: PH-S242 nav shell audit
- Acceptance: `cargo test ph_s243`; FM/HANDOFF/NEXT; `poolai-vision-sync --check`; commit+push

---

## Copy-paste — PH-S243

```
PH-S243 — Admin i18n slim admin chrome shell (code)
Scope: admin chrome keys → auth_dash patch; cargo test ph_s243; FM/HANDOFF/NEXT; commit+push
```

---

## VDT ітерація (один PH-S*)

1. S0: `git fetch`; HANDOFF; FM §5.12; `df -h /s`
2. Один спринт з таблиці вище
3. `cargo fmt --all` → targeted `cargo test`
4. FM ✅ + HANDOFF + NEXT → `cargo run --bin poolai-vision-sync -- --check`
5. MSYS2 commit+push ([`git-push.md`](../../.cursor/commands/git-push.md))
