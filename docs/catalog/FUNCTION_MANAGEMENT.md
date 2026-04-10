# Керування функціоналом PoolAI (індекс, прогалини, тікети)

**Оновлено:** 2026-04-10 (FM-005 узгоджено в доках з кодом: `HttpAppError` + `RestError` для стабільних `error.code`; зроблено: `ui`, `users`, `ai_ml`, `workers`, `instances`, `system`, `completions` та ін.; залишок — `raid*`, `enterprise_api/`, login/refresh)  
**Роль документа:** операційна інструкція для людини й агента («менеджер функціоналу»): звірка з **сталевим станом**, пошук **недоробленого**, пріоритизація, **чернетки тікетів** для передачі в розробку.

**Пов’язані кроки канону:** [крок 11 — витяг](./FUNCTIONALITY_DIGEST_2026-04-06.md) · **крок 12 — цей файл** (керування та беклог).

---

## 1. Джерела правди (порядок звірки)

| Порядок | Джерело | Що дає |
|--------|---------|--------|
| 1 | [`docs/status/STABLE_STATE_SUMMARY.md`](../status/STABLE_STATE_SUMMARY.md) | Що задекларовано як стабільне / 100% / CI. |
| 2 | [`docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`](./FUNCTIONALITY_DIGEST_2026-04-06.md) | Структурований перелік підсистем, features, HTTP-шару. |
| 3 | Код | `src/network/api/`, `src/network/mod.rs`, `src/services/`, `Cargo.toml` (`[features]`, `[[test]]`) — фактичні маршрути й опції збірки. |
| 4 | [`docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md`](../development/NEXT_STEPS_ARCHITECT_2026-03-17.md) | Архітектурний беклог P1–P6 (відкриті чекбокси = офіційні наступні кроки). |
| 5 | [`docs/development/HANDOFF_NEW_SESSION.md`](../development/HANDOFF_NEW_SESSION.md) | Короткий операційний фокус сесії. |
| 6 | Концепти | [`GRID_PROTOCOL_CONCEPT_2026-04-06.md`](../development/GRID_PROTOCOL_CONCEPT_2026-04-06.md), [`SOLANA_ADAPTER_CONCEPT_2026-04-06.md`](../development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md), [`concept/poolAI_concept_root.txt`](../concept/poolAI_concept_root.txt) — **наміри без гарантії коду**. |
| 7 | Історія | `docs/archive/`, `git log`, старі плани в `docs/development/` — якщо пункт зник із актуального плану, але згадується в концепті. |

**OpenAPI** (`docs/openapi.yaml`) — довідково; може відставати від роутерів.

---

## 2. Класифікація пунктів

| Тег | Значення |
|-----|----------|
| **Implemented** | Є в коді + згадка в DIGEST / STABLE; покрито тестами або явно позначено як MVP. |
| **Partial** | Є API; частина логіки спрощена або без повного wire (наприклад sync без remote metadata для conflicts). |
| **Planned** | Відкритий пункт у `NEXT_STEPS_ARCHITECT` або README Next Focus. |
| **Concept-only** | Описано в концепті / протоколі; немає цільової реалізації в `src/`. |
| **Deferred** | `cloud-sdk`, SIMD, on-chain — явно optional у планах. |

---

## 3. Процедура для агента / PM (крок за кроком)

1. Прочитати **STABLE_STATE** і **FUNCTIONALITY_DIGEST** — побудувати список «оголошених» можливостей.
2. За потреби вибірково звірити **критичні маршрути** у `src/network/` (не вичитувати весь репозиторій).
3. Пройти **відкриті чекбокси** в `NEXT_STEPS_ARCHITECT` — перенести в таблицю беклогу або оновити існуючі `FM-*`.
4. Для кожної **прогалини** між концептом і кодом — записати: джерело концепту, поточний код (файл/модуль), ризик, пріоритет.
5. Якщо невідомо, чи колись було зроблено — **`git log -S"keyword"`** або пошук по `docs/archive/`.
6. Додати **тікет** (нижче) з посиланнями на доки та (за можливості) шлях у коді.
7. Після реалізації фічі — оновити **FUNCTIONALITY_DIGEST** і за потреби один рядок у **STABLE_STATE** / HANDOFF.

---

## 4. Шаблон тікета (GitHub / внутрішній)

Скопіюй і заповни:

```markdown
## Title
[Area] Коротка дія (наприклад: RAID: wire-реплікація TurboQuant на стенді)

## Type
Planned | Partial | Concept → Code | Tech-debt | Docs

## Priority
P1 / P2 / P2b / P3 / P4 / Deferred

## Sources
- Plan: …
- Concept: …
- Code: `path/to/file.rs`

## Current behavior
…

## Desired behavior
…

## Acceptance criteria
- [ ] …
- [ ] Тести / бенч / док оновлено

## FM-ID
FM-xxx (з таблиці нижче)
```

---

## 5. Чернетка беклогу (FM-*)

Приклади з **актуальних планів і коду**; переносити в Issues при плануванні спринту. Статус періодично звіряти з `NEXT_STEPS_ARCHITECT`.

| ID | Область | Короткий опис | Стан за каноном | Джерело |
|----|---------|---------------|-----------------|---------|
| FM-001 | P1 / тести | Інтеграційні тести: повний nest `/api/v1` + інжектований `AppState` (`attach_*_for_test`), без `raid::`/`vm::` globals | Implemented | `tests/appstate_http_injection_integration.rs`, CI `--features …,test-utils` |
| FM-002 | P2 | Доробити service layer: тонкі handler’и, логіка в `services/*` для решти доменів | Partial / Planned | NEXT_STEPS P2 |
| FM-003 | P2b / RAID | Повні заміри реплікації артефактів по мережі; порівняння розміру до/після TQ01 на стенді | Planned | NEXT_STEPS P2b, `BENCHMARKS.md` |
| FM-004 | ML | SIMD / прискорений шлях TurboQuant у Rust | Deferred | NEXT_STEPS P2b |
| FM-005 | P3 | Спрощення handler’ів до `Result<T, AppError>` / `HttpAppError` де доречно (без зміни стабільних `error.code`) | Partial | NEXT_STEPS P3; зроблено також: **`ui`** (dashboards, themes/components), **`users`**, **`ai_ml`** (pipeline), **`workers`**, **`instances`**, **`system`**, **`completions`**; лишаються **`raid`**, **`raid_admin`**, **`raid_http`**, решта **`enterprise`** HTTP поза `ui`, login/refresh `(status, Json)` з auth |
| FM-006 | Cloud | Реалізація відкладених гілок Azure/GCP під `cloud-sdk` (credential/compute/location тощо) | Partial / Deferred | P5, `src/cloud/providers/azure.rs`, `gcp.rs` |
| FM-007 | Distributed RAID | Sync: порівняння локального каталогу з peer `artifact_ids` за напрямком (Pull/Push/Bidirectional); `conflicts` лишаються порожніми без remote timestamps у payload | Partial | `RaidDistributedProtocolService::sync_artifacts`, `diff_sync_catalog`; wire-тест **`tests/distributed_raid_wire_integration.rs`** (`wire_sync_artifacts_push_*`) |
| FM-008 | Distributed RAID | LeaveCluster: `graceful` — `replicate_stored_artifact` по всіх локальних артефактах, далі `delete_worker`; помилки membership / невалідний `node_id` | Partial | Якщо `list_nodes` непорожній — `node_id` має бути членом кластера (інакше `InvalidRequest` до replication); wire-тести leave у **`tests/distributed_raid_wire_integration.rs`** |
| FM-009 | Grid | Єдиний wire envelope для Grid protocol (згадано як залишок P6) | Concept-only | GRID_PROTOCOL_CONCEPT |
| FM-010 | Tokenization | On-chain прототип / crate Solana за адаптер-концептом | Concept-only | SOLANA_ADAPTER_CONCEPT |
| FM-011 | Ops | MSVC: **`[profile.test] debug = 1`** у `Cargo.toml` (менший PDB, обхід LNK1318); збірка `cargo test -j1 --all-features --no-run` (+ опційно `CARGO_INCREMENTAL=0`) — перевірено; GNU / дроблення features — за потреби | Partial | `Cargo.toml`, NEXT_STEPS |

### 5.1 Пріоритезовані наступні кроки (зведення FM-* і Architect-плану)

**Якість збірки (не змінює порядок FM):** репозиторій проходить **`cargo clippy --all-targets … -- -D warnings`** для тих самих feature-матриць, що в [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml) (`--no-default-features`, `jwt,https`, `cloud,cloud-sdk` + `K8S_OPENAPI_ENABLED_VERSION`) — станом на **2026-04-10**. Далі за пріоритетом лишаються продуктові пункти нижче, не «полювання на clippy».

**Відкриті чекбокси** у [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](../development/NEXT_STEPS_ARCHITECT_2026-03-17.md): LAN-заміри реплікації + TQ01 на стенді (P2b/P4); опційно **cloud-sdk** (Azure/GCP). Усе інше в таблиці FM-* нижче — дорожня карта без обов’язкового чекбокса.

| Порядок | Фокус | FM / план | Дія |
|--------|--------|-----------|-----|
| 1 | Baseline і мережа | **FM-003**, P4, P2b чекбокс | Референс-хост: Criterion + **`poolai_health_load --json`** → рядки [`BENCHMARKS.md`](../performance/BENCHMARKS.md); на LAN-стенді — повні заміри реплікації та порівняння розміру до/після TQ01. |
| 2 | HTTP-шар | **FM-005** (Partial) | Довести міграцію: **`Result<_, HttpAppError>`** / **`impl IntoResponse`** + **`AppError::RestError { code, message }`** для стабільних кодів (як у `users` / `ui`). Залишок без зміни контракту: **`raid.rs`**, **`raid_admin.rs`**, **`raid_http.rs`**, **`network/enterprise_api/`**; **`login` / `refresh`** і **`check_permission`** — окремо. |
| 3 | Distributed RAID | **FM-007**, **FM-008** | Код: каталог sync + leave з replication/membership; далі — LAN, conflicts у протоколі, поглиблена реплікація. |
| 4 | Ops | **FM-011** | Тримати збірку `--all-features` стабільною (профіль тестів, `-j 1`, incremental, GNU за потреби). |
| 5 | Відкладено | **FM-006**, **FM-004** | **cloud-sdk** (Azure/GCP); SIMD TurboQuant. |
| 6 | Концепт → код (поза спринтом) | **FM-009**, **FM-010** | Grid wire envelope; Solana / on-chain прототип. |

**Канонічний порядок читання доків** (кроки 1–12) — кореневий [`README.md`](../../README.md); цей файл — **крок 12**.

Нові рядки додавати **вниз таблиці** з наступним вільним `FM-0xx`; дублікати з Issues закривати посиланням на PR.

---

## 6. Швидкий пошук по репозиторію

```bash
# Відкриті архітектурні чекбокси (ручна звірка)
rg "\- \[ \]" docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md

# TODO у виконуваному коді (зведення в NEXT_STEPS P5)
rg "TODO|FIXME" src/

# Історія за ключовим словом
git log --oneline --all --grep="RAID"
git log -S "PutArtifact" --oneline -n 20 -- src/
```

---

## 7. Підтримка цього файлу

- Оновлювати **дату** у шапці, таблицю **FM-*** і **§5.1** після значних змін плану Architect, витягу функціоналу або релізу.
- **§5.1** — коротка агрегація; довгі чекбокси й верифікації лишаються в `NEXT_STEPS`; HANDOFF посилається на **§5.1** як на операційний порядок.
- Правило для Cursor: [`.cursor/rules/functionality-management.mdc`](../../.cursor/rules/functionality-management.mdc).

---

## Див. також

- [FUNCTIONALITY_DIGEST_2026-04-06.md](./FUNCTIONALITY_DIGEST_2026-04-06.md)  
- [STABLE_STATE_SUMMARY.md](../status/STABLE_STATE_SUMMARY.md)  
- [STRUCTURE.md](../STRUCTURE.md) (таксономія `docs/`)
