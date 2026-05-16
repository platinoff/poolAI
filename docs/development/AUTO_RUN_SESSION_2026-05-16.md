# Автономний прогін розробки (PoolAI) — 2026-05-16

**Призначення:** інструкція для агента Cursor у **наступній сесії** — виконувати спринти по черзі до **git push** з Summary, без зупинок між спринтами, поки не виконано всі пункти **«в обсязі 100% продукту»** або явний **BLOCKED** (див. нижче).

**Канон:** кроки 1–12 → [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md); тікети → [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md); git → [`.cursor/commands/git-push.md`](../../.cursor/commands/git-push.md).

---

## 1. Визначення «100% робочого стану» (реалістичне)

| Категорія | FM / область | Ціль автопрогону |
|-----------|--------------|------------------|
| **Закрити в коді** | FM-012 (Telegram/OAuth hardening), FM-007/008 (добити wire/тести якщо є прогалини), FM-002 (аудит service/handler) | `Implemented` у таблиці FM-* |
| **Закрити доками + baseline** | FM-003 / P4 (локальний ref-host), FM-011 (clippy матриці + `test-ci`) | Рядки в `BENCHMARKS.md` + запис у HANDOFF |
| **BLOCKED (не чекати в сесії)** | FM-003 LAN-стенд (немає двох хостів у мережі) | `docs/performance/LAN_BENCHMARK_RUNBOOK.md` + статус *Planned (ops)* |
| **Поза обсягом «100%»** | FM-004, FM-006 (Deferred), FM-009, FM-010 (Concept-only) | Не реалізовувати без явного запиту користувача |

**Не плутати:** маркетингове «PROJECT 100%» у `poolAI_concept_root.txt` — історичний заголовок; операційний «100%» = усі **Partial/Planned** у FM-таблиці закриті або формалізовані як BLOCKED з runbook.

---

## 2. Стартовий промпт для наступної сесії (copy-paste)

```text
Продовж автономний прогін PoolAI за docs/development/AUTO_RUN_SESSION_2026-05-16.md:
- Читай HANDOFF + FUNCTION_MANAGEMENT §5.2 + цей файл.
- Виконуй спринти S1→S6 по черзі; після кожного спринту: cargo fmt, cargo test-ci (K8S_OPENAPI_ENABLED_VERSION=1.28), за змін у src/ — clippy як у CI.
- Не стаджити data/audit/*.log.gz; git push через MSYS2 bash за .cursor/commands/git-push.md з Summary.
- FM-004/006/009/010 не чіпати. LAN без стенду — лише runbook (S4 BLOCKED).
- Ціль: закрити всі FM у обсязі §1; оновити STABLE_STATE, FUNCTION_MANAGEMENT, CHANGELOG.
```

---

## 3. Середовище (кожен спринт)

```bash
export PATH="$HOME/.cargo/bin:/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI   # або cd "S:/rust/poolAI"
```

| Крок | Команда | Коли |
|------|---------|------|
| Формат | `cargo fmt --all` | після змін у `src/` / `tests/` |
| Тести (канон) | `cargo test-ci` | після кожного спринту з кодом |
| Clippy | `cargo clippy --all-targets --no-default-features -- -D warnings` | якщо змінювався Rust |
| | `cargo clippy --all-targets --features jwt,https -- -D warnings` | |
| | `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo clippy --all-targets --features cloud,cloud-sdk -- -D warnings` | |
| Enterprise UI/auth | `cargo clippy -p poolai --features enterprise -- -D warnings` | зміни в `src/ui/`, `enterprise_api/` |
| Clean (за потреби) | `cargo clean` | лише при link OOM / os error 1455 |

**Git push:** зовнішній **MSYS2 UCRT64**; один коміт на спринт або один фінальний — за розміром diff; body з **Summary** (див. git-push.md).

---

## 4. Черга спринтів (виконувати по порядку)

### S0 — Зріз (15 хв)

- [ ] `git fetch && git status -sb` на `main`
- [ ] Прочитати: HANDOFF §3–4, FUNCTION_MANAGEMENT §5.1–5.2, цей файл
- [ ] `rg "TODO|FIXME" src/ --glob "!target" -c` → зафіксувати в коміті docs якщо нові критичні

**Вихід:** короткий список відкритих FM у коментарі до першого коміта (опційно).

---

### S1 — FM-012: закрити Telegram / OAuth (продукт)

**Файли:** `src/network/enterprise_api/oauth.rs`, `src/enterprise/security.rs`, `src/ui/i18n_core.js`, `src/ui/mod.rs` (auth alerts).

**Задачі:**

- [ ] Перевірити `verify_telegram_oauth_callback`: HMAC, `auth_date` window, `telegram_allow_user_ids`
- [ ] Узгодити audit-події для success/fail/allowlist deny
- [ ] Прибрати hardcoded user-facing рядки в Telegram HTML/widget flow → `poolaiT` / ключі `auth.*` де доречно
- [ ] RBAC: Telegram-юзер не обходить `check_permission`
- [ ] Тест(и): розширити `tests/network_*` або enterprise auth integration за наявним патерном

**Критерій готовності:** FM-012 → **Implemented** (окрім опційного E2E Playwright — не блокує).

**Доки:** оновити рядок FM-012 у FUNCTION_MANAGEMENT, HANDOFF зріз, STABLE_STATE.

---

### S2 — FM-007 / FM-008: distributed RAID

**Файли:** `src/services/raid_distributed_protocol_service.rs`, `tests/distributed_raid_wire_integration.rs`.

**Задачі:**

- [ ] Прогнати `cargo test --test distributed_raid_wire_integration --features test-utils`
- [ ] Якщо тести зелені і conflicts/leave покриті — FM-007/008 → **Implemented**
- [ ] Якщо є прогалина в wire — мінімальний фікс + тест

**Не робити в цій сесії:** multi-hop реплікація по LAN (це FM-003 ops).

---

### S3 — FM-002: service layer (аудит)

**Задачі:**

- [ ] `rg get_global_ src/network/api` → має бути **0** (виняток документувати в ARCHITECTURE_REVIEW якщо лишиться в `discovery.rs`)
- [ ] `rg get_global_ src/services` → лише задокументовані винятки; інакше — тонкий рефактор
- [ ] Оновити FM-002 → **Implemented** або залишити **Partial** з переліком файлів у FUNCTION_MANAGEMENT

---

### S4 — FM-003 / P4: baseline

**Задачі:**

- [ ] `cargo run --release --bin poolai_health_load -- --json http://127.0.0.1:8080/api/v1/health` (якщо сервер піднято) **або** оновити існуючий рядок у `BENCHMARKS.md` з датою 2026-05-16
- [ ] **LAN:** якщо немає двох вузлів — створити `docs/performance/LAN_BENCHMARK_RUNBOOK.md` (кроки, env, очікувані метрики TQ01); FM-003 залишити **Planned (ops)** — **не BLOCKED сесію**

---

### S5 — FM-011: ops / CI parity

**Задачі:**

- [ ] `cargo test-ci` — фінальний зелений прогін
- [ ] Clippy три матриці (див. §3) — записати «pass» і дату в HANDOFF
- [ ] FM-011 → **Implemented** (alias + профіль тестів + задокументована межа doctest на Windows)

---

### S6 — Закриття документації + релізний зріз

- [ ] `docs/CHANGELOG.md` — секція `[Unreleased]` актуальна
- [ ] `FUNCTIONALITY_DIGEST`, `STABLE_STATE_SUMMARY` — дати **2026-05-16** (або дата сесії)
- [ ] `poolAI_concept_root.txt` — FM-012/005 узгоджені (без «залишку FM-005»)
- [ ] §5.1 FUNCTION_MANAGEMENT — таблиця порядку відображає закриті FM

**Фінальний push:** subject `docs(status): close FM sprint YYYY-MM-DD` або окремі `feat`/`docs` коміти з агрегованим Summary.

---

## 5. Шаблон Summary для git push

```
type(scope): short subject

Summary:
- Sprint: S1–S6 per AUTO_RUN_SESSION_2026-05-16.md
- FM closed: FM-012, FM-007/008, … (list)
- FM ops/doc: FM-003 baseline, FM-011 clippy/test-ci
- Out of scope: FM-004, FM-006, FM-009, FM-010
- Checks: cargo fmt; cargo test-ci; clippy (matrices …)
- Docs: HANDOFF, FUNCTION_MANAGEMENT §5.2, STABLE_STATE, CHANGELOG
```

---

## 6. Якщо агент застряг

| Симптом | Дія |
|---------|-----|
| `link.exe not found` | GNU toolchain у PATH; або `cargo clean` + `-j 1` |
| `os error 1455` | лише `cargo test-ci`, не повний `cargo test` з doctests |
| Немає LAN | S4 runbook, не блокувати S1–S3 |
| Pre-push fmt fail | `cargo fmt --all`, amend/new commit |
| Сумнів у scope | FM-009/010/004/006 — **skip** |

---

## 7. Після завершення автопрогону

Користувач перевіряє: `git log -3 --oneline`, CI на GitHub, опційно ручний smoke `/ui` + `/ui/admin` (UA/EN).

**Наступний горизонт (не автопрогін):** FM-006 cloud-sdk, FM-004 SIMD, FM-009 Grid, FM-010 Solana — окремі епіки.
