# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-18 (PH-S514…S523 ✅ · vision **rev 240** · **0** відкритих · rust_ratio **94.47%**)

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
| Drain | Усі відкриті PH-S* (код + scope-тести + docs **без** Vision rev footer) |
| Vision close | FM §5.12 ✅ + HANDOFF + NEXT → **один** `poolai-vision-sync` → rev з `manifest.json` → `--check` |
| Test | `cargo fmt` → **один** `cargo test-ci` (після vision close) |
| Git | **один** commit: код + `docs/vision/*` + FM/HANDOFF/NEXT → `git-commit-tree-msg.sh` + push + **самарі** |

**Vision:** не sync mid-drain; не другий commit «sync rev» — див. `.cursor/rules/poolai-session-iteration.mdc` § Vision close band.

**Наступна сесія:** знову **`абракадабра`** → S0 → project scan → drain → push.

---

## Закрито (смуга PH-S514…S523)

PH-S514 ✅ — tgbot `/status` → coordinator `GET /api/v1/grid/telegram-seats` snapshot.
PH-S515 ✅ — tgbot `/stop` → `DELETE /api/v1/virtual-nodes/telegram/bindings/{telegram_user_id}`.
PH-S516 ✅ — Galaxy DTO `capabilities` + `seed_inventory` on virtual-nodes list.
PH-S517 ✅ — `/ui/admin/telegram-seats` read-only panel (wasm renderer).
PH-S518 ✅ — lease failover retry budget (`POOLAI_JOB_MAX_MIGRATIONS_PER_JOB`) + `fail_reason` codes.
PH-S519 ✅ — heartbeat-remote refreshes `network_profile.last_measured_at`.
PH-S520 ✅ — optional `POOLAI_ALLOWED_BUILD_IDS` gate on register-remote (`403 build_id_rejected`).
PH-S521 ✅ — payout batch ledger carries fee-split lamports fields.
PH-S522 ✅ — consecutive heartbeat misses → `galaxy_worker_unhealthy_total`.
PH-S523 ✅ — horizon S514 integration + stand smoke + loc-audit + vision-sync.

**rust_ratio:** **94.47%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
