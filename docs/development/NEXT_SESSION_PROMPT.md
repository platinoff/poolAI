# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-18 (PH-S494…S503 ✅ · vision **rev 238** · **0** відкритих · rust_ratio **94.41%**)

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

## Закрито (смуга PH-S494…S503)

PH-S494 ✅ — `GET /api/v1/grid/verification-checker/tasks` read API.
PH-S495 ✅ — drain checker task on grid result verdict.
PH-S496 ✅ — `galaxy_verification_checker_pending_total` Prometheus gauge.
PH-S497 ✅ — `GET /api/v1/grid/network-profiles/{peer_id}` read API.
PH-S498 ✅ — register-remote hydrates persisted `network_profile`.
PH-S499 ✅ — VM admin panel wasm (`renderVmPanel`).
PH-S500…S501 ✅ — horizon S494 integration + stand smoke band.
PH-S502…S503 ✅ — loc-audit **94.41%**, vision-sync + `--check`.

**rust_ratio:** **94.41%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
