# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-24 · **PH-S23:** ✅ · **PH-S24:** черга legacy

---

```
PoolAI — PH-S24 (перший Planned) або ops FM-003 / FM-041 за запитом.

## S0
MSYS2 bash · HANDOFF · FM §5.1 · §5.9

## Стан
- **PH-S03…S23:** ✅ (PH-S23: Playwright admin flows — dashboard, users, config, instances, topology refresh)
- **Наступний:** **PH-S24** Security ops (rotation, pen-test checklist)

## Черга PH-S15…S24
| 1 | **PH-S24** | Security ops (rotation, pen-test doc) | **Planned** |

## Мета
**PH-S24** — secret rotation hooks + pen-test checklist doc (`docs/security/`).

## Перевірки
cargo fmt --all
cargo test-ci
cargo test-raft-ci
bash bin/e2e-playwright.sh --start
```
