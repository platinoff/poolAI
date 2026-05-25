# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-25 · **HEAD** `68340ece` · **PH-S03…S34:** ✅ · **PH-S37:** workflow готовий, PNG — після Linux CI

---

```
PoolAI — закрити PH-S37 (Linux PNG) → PH-S44; §5.10 PH-S35…S44.

## S0
MSYS2 bash · HANDOFF · FM §5.1 · §5.10

## Стан
- **PH-S03…S34:** ✅
- **PH-S37:** workflow `update-visual-baselines.yml` + docs; **PNG ще не в main**
- **PH-S35/S16, FM-003 LAN §4:** BLOCKED (2 хости)
- **PH-S36/S15, FM-041:** Deferred

## PH-S37 — закрити
1. Push (якщо ще не на origin): `.github/workflows/update-visual-baselines.yml`, `bin/update-visual-baselines.sh`, `VISUAL_REGRESSION_E2E.md`, prometheus `record_secret_rotation`
2. GitHub → **Actions** → **Update visual baselines (PH-S37)** → Run workflow
3. Artifact `visual-spec-snapshots-linux` → `e2e/tests/visual.spec.ts-snapshots/`
4. Commit: `test(e2e): refresh Linux visual baselines (PH-S37)` — **лише** Linux PNG
5. FM §5.10 PH-S37 → ✅; HANDOFF → **PH-S44**

## Не повторювати
PH-S03…S34; workflow/docs PH-S37 (без PNG); `record_secret_rotation` wiring

## Наступний (§5.1 / §5.10)
1. **PH-S44** — visual + axe gate у `ci.yml` / e2e policy
2. **PH-S39** — VM Windows resource limits
3. **PH-S42** — admin tables UX
4. **PH-S43** — ML/monitoring metrics UI
5. **PH-S38** — job scheduler / on-chain (за запитом)
6. **PH-S41** — macvlan (Linux)
7. **PH-S40** — hardware VM isolation
8. **PH-S35** — LAN §4 (2 хости)
9. **PH-S36** — FM-041 (явний запит)

## Перевірки
cargo fmt --all
cargo test-ci
bash bin/e2e-playwright.sh --start
# Windows: visual drift очікуваний до Linux PNG commit
```
