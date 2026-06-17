# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-17 · PH-S246 ✅ · vision **rev 198** (after sync) · **6** відкритих (PH-S247…S252) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S247** — Galaxy pricing provider metrics stand smoke |
| **Відкритих** | **6** (PH-S247…S252) |

---

## Черга §5.12 (канон)

| Sprint | Scope | Type |
|--------|-------|------|
| **PH-S247** ← | pricing provider metrics stand smoke | tests |
| PH-S248 | `vm.*` modal i18n slim | code |
| PH-S249 | settlement metrics stand smoke | tests |
| PH-S250 | shard locality metrics stand smoke | tests |
| PH-S251 | GALAXY_GRID_ROADMAP + README sync | docs |
| PH-S252 | `ui.confirm*` modal slim patch | code |

---

## PH-S247 — scope

- `poolai-http-stand-smoke` — provider catalog lookups/hits + provider errors on live `/metrics`
- Pattern: PH-S241/PH-S244 pricing metrics stand smoke
- Acceptance: `cargo test ph_s247`; FM/HANDOFF/NEXT; `poolai-vision-sync --check`; commit+push

---

## Copy-paste — PH-S247

```
PH-S247 — Galaxy pricing provider metrics stand smoke (tests)
Scope: provider catalog metrics on live /metrics; cargo test ph_s247; FM/HANDOFF/NEXT; commit+push
```

---

## VDT ітерація (один PH-S*)

1. S0: `git fetch`; HANDOFF; FM §5.12; `df -h /s`
2. Один спринт з таблиці вище
3. `cargo fmt --all` → targeted `cargo test`
4. FM ✅ + HANDOFF + NEXT → `cargo run --bin poolai-vision-sync -- --check`
5. MSYS2 commit+push ([`git-push.md`](../../.cursor/commands/git-push.md))
