# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-17 · PH-S245 ✅ · vision **rev 197** (after sync) · **7** відкритих (PH-S246…S252) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S246** — Admin err hint keys slim patch |
| **Відкритих** | **7** (PH-S246…S252) |

---

## Черга §5.12 (канон)

| Sprint | Scope | Type |
|--------|-------|------|
| **PH-S246** ← | `err.hint*` + access keys slim | code |
| PH-S247 | pricing provider metrics stand smoke | tests |
| PH-S248 | `vm.*` modal i18n slim | code |
| PH-S249 | settlement metrics stand smoke | tests |
| PH-S250 | shard locality metrics stand smoke | tests |
| PH-S251 | GALAXY_GRID_ROADMAP + README sync | docs |
| PH-S252 | `ui.confirm*` modal slim patch | code |

---

## PH-S246 — scope

- `err.hint403`, `err.hint503.*`, `err.hint404.enterprise`, `err.insufficientAdmin`, `admin.accessRequired` → slim patch
- Remove from `i18n_core.js`; extend admin layout tests
- Pattern: PH-S245 status slim
- Acceptance: `cargo test ph_s246`; FM/HANDOFF/NEXT; `poolai-vision-sync --check`; commit+push

---

## Copy-paste — PH-S246

```
PH-S246 — Admin err hint keys slim patch (code)
Scope: err.hint* + err.insufficientAdmin + admin.accessRequired slim patch; cargo test ph_s246; FM/HANDOFF/NEXT; commit+push
```

---

## VDT ітерація (один PH-S*)

1. S0: `git fetch`; HANDOFF; FM §5.12; `df -h /s`
2. Один спринт з таблиці вище
3. `cargo fmt --all` → targeted `cargo test`
4. FM ✅ + HANDOFF + NEXT → `cargo run --bin poolai-vision-sync -- --check`
5. MSYS2 commit+push ([`git-push.md`](../../.cursor/commands/git-push.md))
