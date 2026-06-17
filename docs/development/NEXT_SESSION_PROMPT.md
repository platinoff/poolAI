# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-17 · PH-S254…S262 ✅ · vision **rev 203** · **0** відкритих у §5.12 · rust_ratio **94.23%** · hold **95%** advisory · stretch **96%**

| **← наступний** | **replenish** з §5.13 (≤10 PH-S*) |
| **Відкритих** | **0** |

---

## Copy-paste — ітераційна сесія (VDT)

```
S0: git fetch; HANDOFF; FM §5.12 (0 відкритих); df -h /s

Replenish ≤10 з §5.13 / rg "TODO|FIXME" src/ / Galaxy §8 horizon → один PH-S* за сесію:
- scope лише файли спринту
- cargo fmt --all → targeted tests → poolai-vision-sync --check
- FM/HANDOFF/NEXT + MSYS2 commit+push
```

---

## Закрито (смуга 2026-06-17)

PH-S128…S262 ✅ — Galaxy stand smoke S253…S256 + admin i18n slim S257…S260 + docs S261 + loc-audit S262.

**rust_ratio:** **94.23%** (formal 90–95% ✅; hold 95% advisory; stretch spirit 96%).

**BLOCKED / Deferred:** FM-003 LAN (2 хости) · FM-041 Cloud SDK live.

---

## Acceptance по типах (для replenish)

| Тип | Канон |
|-----|-------|
| **stand smoke** | `poolai-http-stand-smoke` live `/metrics` + unit `ph_sNNN` |
| **i18n slim** | `poolai-ui-core` patch + inject; keys out of `i18n_core.js` |
| **docs** | FM/HANDOFF/NEXT/INDEX/README/STABLE_STATE sync; vision `--check` |
| **ops** | `cargo run --bin poolai-loc-audit`; `rust_ratio.json` + FM footer |
