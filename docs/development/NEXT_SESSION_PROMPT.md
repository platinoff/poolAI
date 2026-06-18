# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-18 (PH-S434…S443 ✅ · vision **rev 230** · **0** відкритих · rust_ratio **94.36%**)

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
| S0 | `git fetch`; HANDOFF; FM **§1–§5.1**; STABLE_STATE; `df -h /s` |
| Project scan | Якщо §5.12 **< 10** → scan **всього репо** (concept · FM §5.1 · roadmaps · architect · code) → **10 PH-S*** у §5.12 (**журнал**, не scope) |
| Drain | Усі відкриті PH-S* → `cargo fmt` → **один** `cargo test-ci` |
| Docs | FM §5.12 ✅ + HANDOFF + NEXT + vision rev |
| Git | `git-commit-tree-msg.sh` + `git push origin main` + **самарі** |

**Наступна сесія:** знову **`абракадабра`** → S0 → project scan → drain → push.

Канон: [`.cursor/rules/poolai-session-iteration.mdc`](../.cursor/rules/poolai-session-iteration.mdc) § «Тригер абракадабра».

---

## Copy-paste — одна PH-S* (звичайна ітерація)

```
S0: git fetch; HANDOFF; FM §5.12; df -h /s
Один PH-S* з §5.12 → fmt → test-ci → FM/HANDOFF/NEXT → vision-sync --check → push
```

---

## Закрито (смуга PH-S434…S443)

PH-S434…S438 ✅ — Galaxy horizon wire: seed-pull resolver, replication executor, payout ledger, verify/replay enqueue stubs, capability doc parse.
PH-S439…S441 ✅ — capability doc module, network_profile heartbeat persistence, metric history query wasm.
PH-S442…S443 ✅ — stand smoke horizon metrics, loc-audit **94.36%**, docs canon, vision-sync.

**rust_ratio:** **94.36%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
