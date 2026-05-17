# Автономний прогін розробки (PoolAI) — 2026-05-17

**Попередній прогін:** [`AUTO_RUN_SESSION_2026-05-16.md`](./AUTO_RUN_SESSION_2026-05-16.md) (S1–S6 закрито: FM-012, 007/008, 002, 011; FM-003 ops).

**Призначення:** наступна сесія — **оркестратор** + **збір точних патернів** + пріоритет **§5.1** (FM-003 LAN, далі deferred/concept).

**Правила Cursor:** [`.cursor/rules/autonomous-orchestrator.mdc`](../../.cursor/rules/autonomous-orchestrator.mdc), [`.cursor/rules/functionality-management.mdc`](../../.cursor/rules/functionality-management.mdc).

**Канон:** кроки 1–12 → [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md); тікети → [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md); патерни → [`AUTO_DEV_PATTERNS.md`](./AUTO_DEV_PATTERNS.md).

---

## 1. Ціль сесії

| Фаза | Ціль |
|------|------|
| **P0** | Зібрати **≥15** конкретних патернів у `AUTO_DEV_PATTERNS.md` (код + docs), subagent `explore` за таксономією `docs/STRUCTURE.md` |
| **S1** | **FM-003 (ops):** LAN-стенд **або** оновити runbook + рядок у `BENCHMARKS.md` з датою сесії |
| **S2** | Architect: `rg "\- \[ \]" docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md` — закрити документовано або новий FM-* |
| **S3** | `cargo test-ci` + clippy-матриці (якщо були зміни в Rust) → FM-011 зріз дати |
| **S4** | HANDOFF, FUNCTION_MANAGEMENT §5.1, STABLE_STATE, CHANGELOG, DIGEST (якщо API змінювався) |
| **Push** | MSYS2, Summary за шаблоном §5 |

**Поза обсягом:** FM-004, FM-006, FM-009, FM-010 без явного запиту.

---

## 2. Стартовий промпт (copy-paste)

```text
Запусти автономний прогін PoolAI за docs/development/AUTO_RUN_SESSION_2026-05-17.md.
Ти — оркестратор (.cursor/rules/autonomous-orchestrator.mdc):
- S0: HANDOFF + FUNCTION_MANAGEMENT §5.1 + git status.
- P0: делегуй explore для збору патернів; заповни docs/development/AUTO_DEV_PATTERNS.md (≥15 записів з шляхами в коді).
- S1→S4 по черзі; після коду — cargo fmt + cargo test-ci (K8S_OPENAPI_ENABLED_VERSION=1.28).
- Не стаджити data/audit/*.log.gz; push через MSYS2 за .cursor/commands/git-push.md з Summary.
- FM-004/006/009/010 не чіпати.
```

---

## 3. P0 — збір патернів (оркестратор)

**Subagent `explore` (prompt-зразок):**

> Пройди `docs/STRUCTURE.md` + `src/services/`, `src/network/api/`, `tests/*integration*`. Знайди повторювані патерни: thin handlers, HttpAppError, raid wire, oauth/i18n, test-ci. Поверни таблицю: область | файл:рядок | патерн | команда перевірки.

**Оркестратор сам:**

- [ ] Доповнити секції в `AUTO_DEV_PATTERNS.md` (не дублювати загальні поради з README).
- [ ] `rg "get_global_" src/network/api` → має залишатися 0 (регресія).
- [ ] `rg "HttpAppError|AppError::RestError" src/network` — зразок для нових handlers.

**Вихід P0:** коміт `docs(development): collect auto-dev patterns YYYY-MM-DD` (опційно окремо від коду).

---

## 4. S1 — FM-003 (LAN / perf)

- [ ] Якщо є 2+ хости: виконати [`LAN_BENCHMARK_RUNBOOK.md`](../performance/LAN_BENCHMARK_RUNBOOK.md), додати рядки в `BENCHMARKS.md`.
- [ ] Якщо немає стенду: оновити runbook (кроки/env), FM-003 лишається **Planned (ops)** — **не BLOCKED**.

---

## 5. S2 — Architect залишки

- [ ] Відкриті `- [ ]` у `NEXT_STEPS_ARCHITECT_2026-03-17.md` → або закрити в коді/доках, або новий рядок FM-* + посилання.
- [ ] `rg "TODO|FIXME" src/` — критичні перенести в FM або Architect (не масовий рефактор).

---

## 6. S3 — якість

```bash
export PATH="$HOME/.cargo/bin:/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI
cargo fmt --all
cargo test-ci
# clippy — лише якщо змінювався Rust (три матриці як у AUTO_RUN 2026-05-16 §3)
```

---

## 7. S4 — документація

- [ ] `HANDOFF_NEW_SESSION.md` — дата, зріз P0/S1–S3.
- [ ] `FUNCTION_MANAGEMENT.md` — §5.1, таблиця FM-*.
- [ ] `AUTO_DEV_PATTERNS.md` — дата в шапці.
- [ ] `CHANGELOG.md` — `[Unreleased]`.

---

## 8. Шаблон Summary (git push)

```
docs(development): auto-run YYYY-MM-DD patterns + FM-003 slice

Summary:
- Orchestrator: P0 patterns → AUTO_DEV_PATTERNS.md (N entries)
- Sprint: S1 FM-003 (ops/doc), S2 Architect checkboxes, S3 test-ci
- FM touched: FM-003, …
- Out of scope: FM-004, FM-006, FM-009, FM-010
- Checks: cargo fmt; cargo test-ci; …
- Docs: HANDOFF, FUNCTION_MANAGEMENT §5.1, AUTO_DEV_PATTERNS
```

---

## 9. Застрягання

| Симптом | Дія |
|---------|-----|
| Занадто багато `docs/` | P0 лише канон 1–12 + `STRUCTURE.md` + `catalog/` + `development/HANDOFF*` / `AUTO_RUN*` |
| Subagent повернув загальності | Перезапит з вимогою `path:line` |
| link.exe / 1455 | AUTO_RUN 2026-05-16 §6 |
