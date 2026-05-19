# Прогрес розробки PoolAI (менеджер функціоналу)

**Оновлено:** 2026-05-19 (після **S33**) · **Гілка:** `main`  
**Метод:** FM-001…019 у scope автопрогону; BLOCKED/Deferred/Concept — поза чисельником шару A.

---

## Зведені показники (0–100%)

| Шар | % | Що вимірює |
|-----|---|------------|
| **A. Продукт у scope автопрогону** | **100%** | FM-001…019 (dev stand + CI + OpenAPI + E2E + axe); §4 LAN — ops BLOCKED, не в чисельнику |
| **B. Architect P1–P5 (інженерія)** | **97%** | 1 чекбокс LAN+TQ01 (**BLOCKED**, 2 хости) |
| **C. Повна візія (з P6 + concept)** | **79%** | Grid/Solana, SIMD, cloud-sdk deep, LAN sign-off |

**Офіційний зріз HANDOFF / README:** **100%** (шар A).

---

## Методика шару A (FM-*) — S33

**Чисельник (15 пунктів):** FM-001…019.  
**Поза чисельником:** FM-004, FM-006 (Deferred), FM-009, FM-010 (Concept-only).  
**FM-003 §4 LAN sign-off:** ops **BLOCKED** (2 хости) — не знижує % шару A; dev stand + wire ✅ = **100%** для продуктового scope.

| FM | Стан | % | Примітка |
|----|------|---|----------|
| FM-001–002, 005, 007–018 | Implemented | 100 | — |
| FM-003 | Implemented (scope A) | 100 | dev stand ✅; §4 LAN — ops BLOCKED |
| FM-019 | Implemented (scope A) | 100 | pa11y CI ✅; axe Playwright ✅ S33 |

**Розрахунок:** 15 × 100 / 15 = **100%**.

---

## Закрито S31–S33 (не повторювати)

| Спринт | Результат |
|--------|-----------|
| S31 | OpenAPI `/raid/distributed/*`, ML ops, Playwright raid/topology |
| S32 | `run-poolai` + `RUN_LOCAL.md` |
| S33 | OpenAPI distributed DTO schemas; axe Playwright; E2E vm/workers |

---

## Залишок поза шаром A (свідомо)

| ID | Пункт |
|----|--------|
| FM-003 §4 | Реальний LAN sign-off — **2 фізичні хости** |
| FM-004, FM-006 | Deferred |
| FM-009, FM-010, P6 | Concept-only |
| BurstRAID v0.2+, VM Windows deep, Raft log membership | Опційно / v0.3+ |

---

## Посилання

- [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.1, §5.3, §5.5  
- [`AUTO_RUN_SESSION_2026-07-01.md`](../development/AUTO_RUN_SESSION_2026-07-01.md)  
- [`RUN_LOCAL.md`](../development/RUN_LOCAL.md)
