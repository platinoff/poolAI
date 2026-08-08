# Структура та таксономія документації PoolAI

**Оновлено:** 2026-05-27 (PH-S64 — Galaxy Grid у canonical pointers; §5.11 S55–S64 закрито)  
**Джерело правди для порядку читання:** кроки **1–12** у кореневому [`README.md`](../README.md), [`docs/README.md`](./README.md) та [`INDEX_2026-03-17.md`](./INDEX_2026-03-17.md). **Legacy / stale плани:** [`development/DOCS_LEGACY_AUDIT_2026-05-19.md`](./development/DOCS_LEGACY_AUDIT_2026-05-19.md).

---

## 1. Канонічні точки входу

| Шар | Призначення |
|-----|-------------|
| **Кореневий README** | Збірка, CI, карта доків 1–12, Next Focus. |
| **`docs/README.md`** | Короткий індекс + посилання на таксономію (цей файл). |
| **`docs/INDEX_2026-03-17.md`** | Повна навігація по `docs/` за темами. |
| **`docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md`** | Покровий план Architect (P1–P6). |
| **`docs/development/HANDOFF_NEW_SESSION.md`** | Старт нової сесії: гілка, тести, git-push. |
| **`docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`** | Витяг функціоналу (крок 11). |
| **`docs/catalog/FUNCTION_MANAGEMENT.md`** | Керування функціоналом, беклог FM-* та PH-S* (крок 12; **§5.11** — наступні 10 спринтів). |
| **`.cursor/rules/functionality-management.mdc`** | Менеджер функціоналу: FM-*, охоплення docs за `STRUCTURE.md`. |
| **`.cursor/rules/autonomous-orchestrator.mdc`** | Оркестратор авторозробки: субагенти, AUTO_RUN, push. |
| **`.cursor/rules/documentation.mdc`** | Правила для агента: куди класти нові `.md`. |
| **`.cursor/skills/poolai-documentation/SKILL.md`** | Складений skill з тим самим порядком 1–12 + AUTO_RUN / патерни. |

---

## 2. Каталоги під `docs/` (що куди класти)

```
docs/
├── README.md              # Індекс + canonical steps
├── INDEX_2026-03-17.md    # Повна карта
├── STRUCTURE.md           # Цей файл — таксономія
├── openapi.yaml           # OpenAPI (може відставати від коду)
│
├── catalog/               # Зведення / дайджести / керування функціями (не плани й не статус)
├── concept/               # Бачення продукту; **Galaxy Grid** — `POOLAI_GALAXY_GRID.md` (федерація srvN, PH-S55–S63)
├── development/           # Плани, Architect, концепти протоколів (розробка)
├── status/               # Зрізи стану, стабільність, відсотки
├── performance/          # BENCHMARKS, PROFILING, TUNING
├── ml/                   # ML-специфіка (TurboQuant, pipeline, …)
├── cloud/                # Хмарні інтеграції
├── deployment/           # Helm, K8s, bare metal
├── troubleshooting/      # Git, toolchain, MSYS2, push
├── archive/              # Архівні й одноразові нотатки
└── …                     # monitoring/, security/, vm/, runtime/ тощо
```

**Правило:** нові **плани** → `docs/development/`; **статус** → `docs/status/`; **концепт продукту** → `docs/concept/` або `development/` якщо це саме протокол/шар; **історія** → `docs/archive/`.

---

## 3. Плоскі файли в `docs/*.md` (спадщина)

У корені `docs/` лишаються десятки історичних файлів (`EXECUTE_NOW.md`, `PUSH_*`, session summaries тощо). Вони **не** входять до канонічних кроків 1–12; користуйся ними як архівом усного контексту або шукай через `INDEX` / `rg`.

**Політика на майбутнє:** нові архівні нотатки додавати в **`docs/archive/`**; масове прибирання плоских дублів — окремими інкрементами, щоб не ламати зовнішні посилання.

**Аудит stale (FM):** таблиця файлів січень–квітень 2026 і backlog «не зроблено» — [`development/DOCS_LEGACY_AUDIT_2026-05-19.md`](./development/DOCS_LEGACY_AUDIT_2026-05-19.md) (після S29).

---

## 4. Правила Cursor (не в `docs/`)

| Шлях | Роль |
|------|------|
| `.cursor/rules/documentation.mdc` | Куди писати доки, кроки 11–12, каталог. |
| `.cursor/rules/functionality-management.mdc` | Беклог функцій, прогалини, тікети FM-* (агент). |
| `.cursor/rules/autonomous-orchestrator.mdc` | Автономний прогін: оркестратор, Task subagents, спринти. |
| `.cursor/rules/project-structure.mdc` | Організація `src/`, скриптів. |
| `.cursor/commands/git-push.md` | Push, MSYS2, змінні середовища. |

---

## 5. Інвентар репозиторію

- **`file_list.csv`** (корінь) — ручний зріз ключових шляхів; оновлюй при змінах у `src/services/`, `src/network/`, `.github/workflows/`, `.cursor/`, `docs/catalog/`.
- Повний перелік: `git ls-files`.

---

## 6. Тести та документація

- Рекомендований локальний прогін (узгоджено з CI-матрицею):  
  `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test --lib --tests --features ml,enterprise,cloud,test-utils`
- Інтеграційні таргети з **`required-features`** (наприклад `test-utils`, `ml`) не збираються без відповідних `--features`; див. кореневий `Cargo.toml`, секція `[[test]]`.
- Повний `cargo test` з **doctests** на деяких Windows-конфігураціях може давати помилки лінкера; це відомий клас проблем середовища, не «канонічний» обов’язок для щоденної розробки.

---

## 7. Runtime stack (мови та заборони)

| Шар | Канон | Шлях |
|-----|--------|------|
| Продукт | **Rust** | `src/`, `tests/`, `crates/` |
| Admin UI | HTML/CSS/JS | `src/ui/` |
| E2E | TypeScript (Playwright) | `e2e/` |
| Ops (dev launch) | Bash / PowerShell | **`bin/`** — run, LAN, verify, e2e |
| Ops (toolchain) | Bash (MSYS2) | **`scripts/`** — PATH, gcc, deploy |
| Cargo binaries | Rust | **`src/bin/`** — `cargo run --bin …` |
| Dev audit | Rust bin | `poolai-openapi-gap-audit` (`src/bin/poolai_openapi_gap_audit.rs`) |

**Заборонено в репозиторії:** будь-які `.py`, Python sidecar, `requirements.txt`, PyPI runtime.

**Агент Cursor:** `.cursor/rules/runtime-stack-policy.mdc` (**alwaysApply**). Архівні `docs/archive/*` з Python — історія, не план.

**Java:** у репозиторії немає `.java`; допоміжні JVM-артефакти поза scope, якщо не додано окремим епіком.

---

## 8. Код репозиторію (не `docs/`)

Людська карта шляхів (`src/` vs `src/bin/` vs `bin/` vs `scripts/` vs `tests/` vs `crates/`): **[`development/REPOSITORY_LAYOUT.md`](./development/REPOSITORY_LAYOUT.md)**.

**Версія опису структури:** 2.2 (repository layout clarity, 2026-05-20).
