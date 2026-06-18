# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-18 (PH-S464…S473 ✅ · vision **rev 235** · **0** відкритих · rust_ratio **94.39%**)

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

## Закрито (смуга PH-S464…S473)

PH-S464…S469 ✅ — Galaxy horizon: prefetch backpressure, RAID fetch stub, capability sig verify, payout-batch GET, protocol accepted metric, co-access env JSON.
PH-S470 ✅ — monitoring dashboards panel wasm (`renderMonitoringDashboardsPanel`).
PH-S471…S472 ✅ — horizon S464 integration + stand smoke band.
PH-S473 ✅ — loc-audit **94.39%**, vision-sync + `--check`.

**rust_ratio:** **94.39%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
