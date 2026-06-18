# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-18 (PH-S414…S423 ✅ · vision **rev 228** · **0** відкритих · rust_ratio **94.35%**)

| **← наступний** | **`абракадабра`** (research + drain) **або** один PH-S* |
| **Відкритих** | **0** |

---

## Тригер «абракадабра» (ітераційний drain проєкту)

Одне слово в **новій сесії** → повний цикл без підтвердження кожного PH-S*:

```
абракадабра
```

| Крок | Дія |
|------|-----|
| S0 | `git fetch`; HANDOFF; FM **§5.1** + **§5.12**; `df -h /s` |
| Research | Якщо §5.12 **< 10** → replenish **10** PH-S* з **concept/roadmap/FM §5.1/architect/code** (не лише ratio band) |
| Drain | Усі відкриті PH-S* → `cargo fmt` → **один** `cargo test-ci` |
| Docs | FM §5.12 ✅ + HANDOFF + NEXT + vision rev |
| Git | `git-commit-tree-msg.sh` + `git push origin main` + **самарі** |

**Наступна сесія:** знову **`абракадабра`** → S0 → replenish (якщо 0) → drain → push.

Канон: [`.cursor/rules/poolai-session-iteration.mdc`](../.cursor/rules/poolai-session-iteration.mdc) § «Тригер абракадабра».

---

## Copy-paste — одна PH-S* (звичайна ітерація)

```
S0: git fetch; HANDOFF; FM §5.12; df -h /s
Один PH-S* з §5.12 → fmt → test-ci → FM/HANDOFF/NEXT → vision-sync --check → push
```

---

## Закрито (смуга PH-S414…S423)

PH-S414…S418 ✅ — verify/replay metrics, dashboard refresh wasm, stand smoke.
PH-S419…S423 ✅ — loc-audit **94.35%**, docs canon, vision **rev 228**.

**rust_ratio:** **94.35%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
