# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-25 · **HEAD** `b34bb90d` · **PH-S03…S34:** ✅ · **Черга:** **PH-S37** (§5.10)

---

```
PoolAI — PH-S37 visual baselines (Linux CI); §5.10 PH-S35…S44.

## S0
MSYS2 bash · HANDOFF · FM §5.1 · §5.10

## Стан
- **PH-S03…S34:** ✅
- **PH-S35/S16, FM-003 LAN §4:** BLOCKED (2 хости)
- **PH-S36/S15, FM-041:** Deferred
- **E2E login (Windows):** ✅ helpers Promise.all + localStorage

## Останнє (push цієї сесії)
- `e2e/tests/helpers.ts` — стабільний loginAsAdmin (Windows)
- `bin/e2e-playwright.sh`, `bin/update-visual-baselines.sh` — STAND_ROOT=/tmp
- FM **§5.10** — 10 спринтів PH-S35…S44 з legacy audit
- HANDOFF + FUNCTION_MANAGEMENT синхрон

## Не повторювати
PH-S03…S34; E2E login fix; §5.10 audit (доки)

## Наступний (§5.1 / §5.10)
1. **PH-S37** — `bash bin/update-visual-baselines.sh` на **Linux** (CI snapshots); не комітити Windows PNG
2. **PH-S44** — visual + axe gate у `ci.yml` / e2e policy (після S37)
3. **PH-S39** — VM Windows resource limits (`vm/resources.rs`)
4. **PH-S42** — admin tables UX (sort/filter/export)
5. **PH-S43** — ML/monitoring metrics UI
6. **PH-S38** — job scheduler / on-chain epics (за запитом)
7. **PH-S41** — macvlan (Linux)
8. **PH-S40** — hardware VM isolation (великий scope)
9. **PH-S35** — LAN §4 (2 хости)
10. **PH-S36** — FM-041 (явний запит)

## Перевірки
cargo fmt --all
cargo test-ci
bash bin/e2e-playwright.sh --start
# visual only: cd e2e && npm run test:visual
```
