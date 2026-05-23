# План покращення UI, автотестів і сталого стану (2026-04-06)

## Проблема

Частина сторінок покладається на поля JSON з API, які **не поверталися** бекендом (наприклад, адмін **Workers** очікував `is_healthy`, `total_requests_processed`, тоді як `/api/v1/workers` віддавав лише `id`, `status`, `current_task`). Після створення воркера таблиці виглядали «порожніми» або некоректними.

## Зроблено в цьому інкременті

- Розширено **`WorkerInfo`** у `src/network/api/workers.rs`: додано `is_healthy`, `total_requests_processed`, `queue_size`, `active_connections`, `average_response_time_ms` (з `pool::worker::WorkerStatus`); оновлено mock-відповідь.
- Оновлено таблицю **Workers** у `src/ui/mod.rs`: колонки Health, State, Current task, Requests, Queue (узгоджено з API).
- Інтеграційний тест форми JSON для `GET /api/v1/workers` (див. `tests/network_api_integration.rs`).
- **`VmStatus` JSON для UI:** серіалізація як рядок (`Creating` / `Running` / `Stopped` / `Failed: …`); десеріалізація з рядка та legacy-об’єкта `{"Failed":"…"}` (`src/vm/mod.rs`, юніт-тести).
- **Admin VM** (`src/ui/admin/vm.rs`): видалення через **`DELETE /api/v1/vm/instances/{id}`** (раніше помилково `POST …/delete`); бейдж статусу та fallback для `resources`.
- OpenAPI: оновлено опис `VmInstance.status` під рядковий формат.

## S24 (2026-05-19)

- **`DELETE /ui/dashboards/{id}`** — реалізовано: `MonitoringManager.delete_dashboard()`, `UiService::delete_dashboard`, HTTP **204**; OpenAPI оновлено; тест `test_delete_dashboard` у `enterprise_monitoring_integration.rs`.

## S25 (2026-05-19)

- **Admin JSON contracts (P1):** +3 тести — tenants, OAuth2, dashboards.

## S26 (2026-05-19)

- **Admin JSON contracts (P1 закрито):** +4 тести — metrics, alert-rules, SAML providers, security policies (**27** tests total).
- Таблиця покриття — [`ADMIN_UI_JSON_CONTRACTS.md`](./ADMIN_UI_JSON_CONTRACTS.md). **P1 ✅**

## Наступні кроки за пріоритетом

### P1 — Узгодження API ↔ UI (1–2 тижні)

- Пройти **admin**-сторінки (`src/ui/admin/*.rs`) і для кожної перевірити: які поля читає JS / шаблон, що реально повертає відповідний handler у `src/network/api/`.
- Зведення по адмінці: [`ADMIN_UI_JSON_CONTRACTS.md`](./ADMIN_UI_JSON_CONTRACTS.md); тести — `tests/admin_ui_api_contracts.rs`.
- Ввести легкі **DTO-огляди** в документації або коментарях поруч із handler’ом: «поля для UI: …».
- Для критичних сутей (VM, RAID artifacts, libraries) — той самий підхід, що для workers.

### P2 — Автотести функціональності

| Рівень | Що робити | Інструменти |
|--------|-----------|-------------|
| **A. API контракт** | Перевірка наявності ключів у JSON після змін handler’ів | `tests/*_integration.rs`, `serde_json::Value` |
| **B. З авторизацією** | POST/PATCH/DELETE під Bearer (dev JWT або тестовий user) | Існуючі патерни `network_auth_integration` |
| **C. E2E браузер** | Сценарії «відкрити /ui → логін → дія»; **visual regression** | **✅ FM-039 + PH-S11:** Playwright у `e2e/`, [`E2E_PLAYWRIGHT.md`](./E2E_PLAYWRIGHT.md), [`VISUAL_REGRESSION_E2E.md`](./VISUAL_REGRESSION_E2E.md) |

Рекомендація: тримати **A** в обов’язковому CI; **B** розширювати точково; **C** — коли стабілізується P1.

### P3 — Сталий стан документації

- Оновлювати [`docs/status/STABLE_STATE_SUMMARY.md`](../status/STABLE_STATE_SUMMARY.md) після значних UI/API змін.
- Посилання на цей план — у [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) або кореневому README (*Next Focus*).

### P4 — UX polish

- Єдині **повідомлення про помилки** (вже є `apiErrorMessageFromBody` для `{ error: { message } }`).
- **Завантаження / порожній стан** для таблиць після мутацій (debounce refresh).
- Доступність: зберегти `aria-*` при зміні колонок.

## Критерії «готово» для фази workers

- [x] `GET /api/v1/workers` містить поля, які читає dashboard і admin workers.
- [x] Таблиця на `/ui/.../workers` показує health і лічильники.
- [x] Тест на форму JSON у CI.

## Посилання

- План архітектора: [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](./NEXT_STEPS_ARCHITECT_2026-03-17.md)
- API workers: `src/network/api/workers.rs`
- UI: `src/ui/mod.rs` (dashboard workers), `src/ui/admin/workers.rs` (admin)
