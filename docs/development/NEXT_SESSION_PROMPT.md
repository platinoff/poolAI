# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-17 · PH-S243 ✅ · vision **rev 193** (after sync) · **9** відкритих (PH-S244…S252) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S244** — Galaxy pricing stale served metrics stand smoke |
| **Відкритих** | **9** (PH-S244…S252) |

---

## Черга §5.12 (канон)

| Sprint | Scope | Type |
|--------|-------|------|
| **PH-S244** ← | `galaxy_pricing_stale_served` stand smoke | tests |
| PH-S245 | `admin.status.*` slim patch | code |
| PH-S246 | `err.hint*` + access keys slim | code |
| PH-S247 | pricing provider metrics stand smoke | tests |
| PH-S248 | `vm.*` modal i18n slim | code |
| PH-S249 | settlement metrics stand smoke | tests |
| PH-S250 | shard locality metrics stand smoke | tests |
| PH-S251 | GALAXY_GRID_ROADMAP + README sync | docs |
| PH-S252 | `ui.confirm*` modal slim patch | code |

---

## PH-S244 — scope

- `poolai-http-stand-smoke` — `galaxy_pricing_stale_served` on live `/metrics` (PH-S127 gauge export)
- Pattern: PH-S241 fresh served stand smoke
- Acceptance: `cargo test ph_s244`; FM/HANDOFF/NEXT; `poolai-vision-sync --check`; commit+push

---

## Copy-paste — PH-S244

```
PH-S244 — Galaxy pricing stale served metrics stand smoke (tests)
Scope: galaxy_pricing_stale_served on live /metrics; cargo test ph_s244; FM/HANDOFF/NEXT; commit+push
```

---

## VDT ітерація (один PH-S*)

1. S0: `git fetch`; HANDOFF; FM §5.12; `df -h /s`
2. Один спринт з таблиці вище
3. `cargo fmt --all` → targeted `cargo test`
4. FM ✅ + HANDOFF + NEXT → `cargo run --bin poolai-vision-sync -- --check`
5. MSYS2 commit+push ([`git-push.md`](../../.cursor/commands/git-push.md))
