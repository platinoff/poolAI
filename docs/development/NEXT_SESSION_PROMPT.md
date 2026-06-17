# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-17 · PH-S244 ✅ · vision **rev 195** (after sync) · **8** відкритих (PH-S245…S252) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S245** — Admin shared status keys slim patch |
| **Відкритих** | **8** (PH-S245…S252) |

---

## Черга §5.12 (канон)

| Sprint | Scope | Type |
|--------|-------|------|
| **PH-S245** ← | `admin.status.*` slim patch | code |
| PH-S246 | `err.hint*` + access keys slim | code |
| PH-S247 | pricing provider metrics stand smoke | tests |
| PH-S248 | `vm.*` modal i18n slim | code |
| PH-S249 | settlement metrics stand smoke | tests |
| PH-S250 | shard locality metrics stand smoke | tests |
| PH-S251 | GALAXY_GRID_ROADMAP + README sync | docs |
| PH-S252 | `ui.confirm*` modal slim patch | code |

---

## PH-S245 — scope

- `admin.status.active` / `inactive` / `yes` / `no`, `admin.na`, `admin.btn.edit` → slim patch (`admin_status_patch` or shared admin patch)
- Remove from `i18n_core.js`; extend admin layout tests
- Pattern: PH-S240 table toolbar slim
- Acceptance: `cargo test ph_s245`; FM/HANDOFF/NEXT; `poolai-vision-sync --check`; commit+push

---

## Copy-paste — PH-S245

```
PH-S245 — Admin shared status keys slim patch (code)
Scope: admin.status.* + admin.na + admin.btn.edit slim patch; cargo test ph_s245; FM/HANDOFF/NEXT; commit+push
```

---

## VDT ітерація (один PH-S*)

1. S0: `git fetch`; HANDOFF; FM §5.12; `df -h /s`
2. Один спринт з таблиці вище
3. `cargo fmt --all` → targeted `cargo test`
4. FM ✅ + HANDOFF + NEXT → `cargo run --bin poolai-vision-sync -- --check`
5. MSYS2 commit+push ([`git-push.md`](../../.cursor/commands/git-push.md))
