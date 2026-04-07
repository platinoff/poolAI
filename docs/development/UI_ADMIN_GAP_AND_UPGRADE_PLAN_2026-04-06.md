# Порівняння UI / Admin з API та план апгрейду (2026-04-06)

## Мета

Зафіксувати **що вже покрито** маршрутами бекенду, **де UI викликає неіснуючі або неправильні URL**, **де форма JSON не збігається** з тим, що малює JS, і **які обмеження** (JWT, enterprise feature, 503 «manager unavailable») ламають сценарії «з коробки».

Базові префікси (з `src/network/mod.rs`):

- Публічний REST: **`/api/v1/...`** (`create_api_routes`).
- Enterprise: **`/api/enterprise/...`** (лише з `--features enterprise`).
- UI: **`/ui`**, адмінка: **`/ui/admin`** (вкладені маршрути з `src/ui/admin/mod.rs`).

Пов’язаний документ: [UI_QUALITY_AND_E2E_PLAN_2026-04-06.md](./UI_QUALITY_AND_E2E_PLAN_2026-04-06.md) (вже зроблені фікси workers / VM).

---

## 1. Матриця: сторінка адмінки → виклики → бекенд

| Сторінка (`/ui/admin/...`) | Основні виклики в JS | Маршрут у коді | Примітки |
|----------------------------|----------------------|----------------|----------|
| Dashboard | `GET /api/v1/admin/overview` | `admin.rs` → `/admin/overview` | OK під `/api/v1`. |
| | `GET /api/enterprise/monitoring/alerts`, `audit/events`, `monitoring/metrics` | `enterprise_api` | Потрібен **enterprise** + зазвичай **Bearer** для мутацій; GET часто доступні залежно від handler. |
| Tenants | `/api/enterprise/tenants` CRUD | enterprise | Як вище. |
| Security | `/api/enterprise/security/...` | enterprise | Як вище. |
| Audit | `/api/enterprise/audit/events` | enterprise | OK за feature. |
| Monitoring | `/api/enterprise/monitoring/...` | enterprise | OK за feature. |
| Workers | `/api/v1/workers` | `workers.rs` | Після недавнього узгодження полів — OK (див. UI_QUALITY doc). |
| VM | `/api/v1/vm/instances` | `vm.rs` | OK. |
| Libraries | `/api/v1/libraries` + install/uninstall/update/upload | `libraries.rs` | Мутації під `auth_middleware` — потрібен токен. |
| Instances (ML) | `/api/v1/instance`, previews, `/{id}` | `instances.rs` | Шляхи **`/instance`** (однина) — збігаються з UI. |
| Topology | `/api/v1/topology`, `.../nodes`, `.../latency`, `.../nodes/{id}` | `topology.rs` | OK. |
| Users | `/api/v1/users` | `users.rs` | Мутації зазвичай з auth. |
| Config | `GET/PUT /api/v1/config` | `config.rs` | PUT з auth. |
| RAID | Див. §2 — **є критичні помилки** | `raid.rs` + `raid_admin.rs` | Частина URL і форм JSON. |

---

## 2. Підтверджені розбіжності (пріоритет виправлення)

### P0 — Невірний префікс RAID admin (404) — **виправлено**

У `src/ui/admin/raid.rs` усі виклики переведені на **`/api/v1/raid/admin/...`** (узгоджено з `raid_admin.rs`).

### P0 — Відсутній endpoint оновлення токена — **виправлено**

Реалізовано **`POST /api/v1/refresh`** (`system.rs`): приймає `Authorization: Bearer`, декодує claims **без відхилення прострочених** токенів, перевіряє наявність користувача, повертає той самий JSON, що й логін (`AuthResponse`). Див. також `decode_token_claims_allow_expired` у `auth.rs`.

### P1 — Вкладена відповідь RAID admin strategy / metrics — **виправлено в UI**

У `admin/raid.rs` додано розгортання **`status`** / **`metrics`**; картки Burst/SmallWorld показують поля з реальних DTO (`artifacts_in_burst`, `base_replication_factor`, тощо).

### P1 — Distributed «Sync artifacts» з тіла `{}` — **виправлено в UI**

Кнопка «Sync Artifacts» надсилає валідний **`ProtocolMessage`** (`type: sync_artifacts`, `node_id: ui-admin`, `payload.direction: bidirectional`). Окремий wrapper endpoint не потрібен для базового сценарію.

### P2 — Авторизація — **частково закрито**

- `admin_common.js`: **`fetchJson`** спочатку пробує **`POST /api/v1/refresh`** при **401** (як головний UI), потім редірект на `/ui/auth`.
- Підказки для **403** / **404** на `/api/enterprise` залишаються в тексті помилки.

### P2 — 503 «manager / pool unavailable» — **частково закрито**

- Повідомлення з тіла API (**`error.message`**, **`context.hint`**, код **`RAID_MANAGER_UNAVAILABLE`**) додаються до викинутого `Error` у **`fetchJson`**; таблиці показують **inline**-блок **`.admin-fetch-error`** після невдалого завантаження.

---

## 3. Головний UI (`/ui`, `src/ui/mod.rs`)

| Область | Endpoint | Статус |
|---------|----------|--------|
| Статус / health / metrics | `GET /api/v1/status`, `/health`, `/metrics` | Є в `system.rs`. |
| Workers / libraries / VM / RAID | `/api/v1/...` | Збігаються з роутерами. |
| Логін | `POST /api/v1/login` | Є. |
| OAuth redirect | `/api/enterprise/auth/{provider}` | За enterprise. |
| **Refresh токена** | `POST /api/v1/refresh` | Є (`system.rs`); адмінський **`fetchJson`** робить refresh при 401. |

---

## 4. План апгрейду UI (фази)

### Фаза A — Швидкі виправлення «поламаного» (1–2 дні)

1. Виправити префікси **`/api/v1/raid/admin/*`** у `admin/raid.rs`.
2. Узгодити **парсинг JSON** для RAID admin (витяг `status` / `metrics` або змінити відповідь API під плоский DTO — обрати один підхід і тримати його в доках).
3. Рішення по **`/api/v1/refresh`**: реалізувати або прибрати виклики з `validateToken` / `refreshToken`.

### Фаза B — Контракти та тести (1–2 тижні) — **інкремент зроблено**

1. Зведена таблиця: **[ADMIN_UI_JSON_CONTRACTS.md](./ADMIN_UI_JSON_CONTRACTS.md)** (поля JSON → UI + **SyncArtifacts**).
2. Тести: **`tests/admin_ui_api_contracts.rs`** (`/admin/overview`, RAID admin status/burst; з **`--features enterprise`** — alerts + audit events).
3. **SyncArtifacts** задокументовано в тому ж файлі (приклад тіла `ProtocolMessage`).

### Фаза C — UX і стабільність (паралельно з B) — **інкремент зроблено**

1. **`admin_common.js`**: refresh при 401, розбір **`apiErrorDetailFromBody`**, підказки для 403 / 503 / enterprise 404.
2. **Loading / inline error** для основних списків (dashboard, workers, VM, libs, users, tenants, RAID, audit, monitoring, config, security tabs, instances).
3. **README** — раніше (фаза A).

### Фаза D — E2E (опційно)

Після стабілізації A–B: Playwright/Cypress «відкрити /ui/admin → логін → smoke по розділах» (див. колонку C в UI_QUALITY).

---

## 5. Критерії готовності (checkpoint)

- [x] Немає звернень до **`/api/raid/admin/*`** (лише `/api/v1/raid/admin/*`).
- [x] RAID admin картки показують реальні дані при активному RAID manager (парсинг відповіді узгоджено).
- [x] **`POST /api/v1/refresh`** реалізовано; OpenAPI оновлено.
- [x] Sync artifacts: виклик відповідає `ProtocolMessage`.
- [x] README / troubleshooting вказують **`/ui/admin`** і залежність від **`enterprise`** для enterprise-розділів (оновлено `README.md` у блоці Usage).
- [x] Фаза B: **`ADMIN_UI_JSON_CONTRACTS.md`**, **`tests/admin_ui_api_contracts.rs`** (+ enterprise slices за `--features enterprise`).
- [x] Фаза C: покращений **`admin_common.js`** (401→refresh, 403/503/enterprise 404), loading + **`.admin-fetch-error`** на основних адмін-сторінках.

---

## Посилання на код

- Контракти UI ↔ JSON: [ADMIN_UI_JSON_CONTRACTS.md](./ADMIN_UI_JSON_CONTRACTS.md).
- Тести форми JSON: `tests/admin_ui_api_contracts.rs`.
- Монтування API: `src/network/mod.rs`.
- RAID user + distributed: `src/network/api/raid.rs`.
- RAID admin: `src/network/api/raid_admin.rs`.
- Адмін RAID UI: `src/ui/admin/raid.rs`.
- Головний UI + refresh: `src/ui/mod.rs`.
- Спільні fetch / токен для адмінки: `src/ui/admin_common.js`.
