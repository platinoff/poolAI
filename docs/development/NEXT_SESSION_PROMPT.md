# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-18 (PH-S504…S513 ✅ · vision **rev 239** · **0** відкритих · rust_ratio **94.42%**)

| **← наступний** | **`абракадабра`** (project scan + drain) **або** один PH-S* |
| **Відкритих** | **0** |

---

## Тригер «абракадабра» (drain **всього проєкту**)

Одне слово в **новій сесії** → project scan → drain → push (без підтвердження кожного PH-S*):

```
абракадабра
```

| Крок | Дія |
|------|-----|
| S0 | `git fetch`; HANDOFF; FM **§1–§5.1**; STABLE_STATE; **`poolai-vision-sync --check` ok**; `df -h /s` |
| Project scan | Якщо §5.12 **< 10** → scan **всього репо** → **10 PH-S*** у §5.12 |
| Drain | Усі відкриті PH-S* (код + scope-тестs + docs **без** Vision rev footer) |
| Vision close | FM §5.12 ✅ + HANDOFF + NEXT → **один** `poolai-vision-sync` → rev з `manifest.json` → `--check` |
| Test | `cargo fmt` → **один** `cargo test-ci` (після vision close) |
| Git | **один** commit: код + `docs/vision/*` + FM/HANDOFF/NEXT → `git-commit-tree-msg.sh` + push + **самарі** |

**Vision:** не sync mid-drain; не другий commit «sync rev» — див. `.cursor/rules/poolai-session-iteration.mdc` § Vision close band.

**Наступна сесія:** знову **`абракадабра`** → S0 → project scan → drain → push.

---

## Закрито (смуга PH-S504…S513)

PH-S504 ✅ — mandatory signed `capability_document` for `telegram_edge` register-remote.
PH-S505 ✅ — `GET /api/v1/grid/telegram-seats` coordinator snapshot.
PH-S506 ✅ — `PUT /api/v1/grid/network-profiles/{peer_id}` upsert API.
PH-S507 ✅ — unified `galaxy` DTO on `GET /api/v1/discovery/virtual-nodes`.
PH-S508 ✅ — workers admin virtual-nodes panel (origin badges + latency sort).
PH-S509 ✅ — tgbot `/wallet` → coordinator wallet bind.
PH-S510 ✅ — wallet rebind cooldown (`409 wallet_rebind_cooldown`).
PH-S511 ✅ — non-deterministic `semantic_hash` verification stub.
PH-S512 ✅ — `/ui/admin/grid-verification` read-only checker panel.
PH-S513 ✅ — horizon S504 integration + stand smoke + loc-audit + vision-sync.

**rust_ratio:** **94.42%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
