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

### P0 — Невірний префікс RAID admin (404)

У `src/ui/admin/raid.rs` використовується **`/api/raid/admin/...`**, тоді як роутери зареєстровані під **`/api/v1`**:

- Фактичні шляхи: `/api/v1/raid/admin/status`, `/api/v1/raid/admin/rebalance`, `/api/v1/raid/admin/metrics/burst`, `/api/v1/raid/admin/metrics/smallworld` (`raid_admin.rs`).

**Виправлення:** замінити всі `/api/raid/admin` на `/api/v1/raid/admin`.

### P0 — Відсутній endpoint оновлення токена

Головний UI (`src/ui/mod.rs`): `refreshToken()` викликає **`POST /api/v1/refresh`**.

У `src/network` **немає** такого маршруту → при простроченому JWT оновлення завжди неуспішне, далі 401 і редірект на логін.

**Варіанти:** додати `POST /api/v1/refresh` (узгодити з `login_handler`) **або** прибрати виклик і документувати «перелогін»; **або** валідувати тільки локально без refresh.

### P1 — Вкладена відповідь RAID admin strategy / metrics

Бекенд повертає обгортки:

- `StrategyStatusResponse` → JSON **`{ "status": { "mode", "initialized", ... } }`** (`raid_admin.rs`).
- `BurstRaidMetricsResponse` / `SmallWorldMetricsResponse` → **`{ "metrics": { ... } }`**.

У `renderRaidAdmin(status, ...)` зараз очікуються поля **`status.mode`** тощо на верхньому рівні — для реальної відповіді треба **`status.status.*`** та **`burstMetrics.metrics.*`** (або змінити серіалізацію API на плоский DTO для UI).

### P1 — Distributed «Sync artifacts» з тіла `{}`

`POST /api/v1/raid/distributed/artifacts/sync` очікує **`Json<ProtocolMessage>`** (`raid_distributed_handlers.rs`). Тіло **`{}`** не відповідає контракту → помилка десеріалізації / 422.

**Виправлення:** або зібрати коректний `ProtocolMessage` у UI (і документувати поля), або додати окремий «admin-friendly» wrapper endpoint з мінімальним тілом.

### P2 — Авторизація

- `admin_common.js` додає `Authorization: Bearer` лише якщо є `poolai_token`.
- Багато мутацій у `/api/v1` і майже всі чутливі enterprise-операції вимагають JWT; без логіну кнопки «мовчки» падають з 401/403.
- Адмін-панель: при 401 — одразу редірект на `/ui/auth` **без** retry refresh (на відміну від головного `fetchJson` у `mod.rs`).

**План UX:** єдиний потік логіну, явне «потрібен вхід» на сторінках з мутаціями, опційно узгодити retry з `refresh` після появи endpoint.

### P2 — 503 «manager / pool unavailable»

RAID, worker pool тощо можуть повертати **503** з кодом на кшталт `RAID_MANAGER_UNAVAILABLE`. UI варто показувати **стійкий банер** з текстом з `error.message`, а не загальне «Error loading».

---

## 3. Головний UI (`/ui`, `src/ui/mod.rs`)

| Область | Endpoint | Статус |
|---------|----------|--------|
| Статус / health / metrics | `GET /api/v1/status`, `/health`, `/metrics` | Є в `system.rs`. |
| Workers / libraries / VM / RAID | `/api/v1/...` | Збігаються з роутерами. |
| Логін | `POST /api/v1/login` | Є. |
| OAuth redirect | `/api/enterprise/auth/{provider}` | За enterprise. |
| **Refresh токена** | `POST /api/v1/refresh` | **Відсутній** — див. §2. |

---

## 4. План апгрейду UI (фази)

### Фаза A — Швидкі виправлення «поламаного» (1–2 дні)

1. Виправити префікси **`/api/v1/raid/admin/*`** у `admin/raid.rs`.
2. Узгодити **парсинг JSON** для RAID admin (витяг `status` / `metrics` або змінити відповідь API під плоский DTO — обрати один підхід і тримати його в доках).
3. Рішення по **`/api/v1/refresh`**: реалізувати або прибрати виклики з `validateToken` / `refreshToken`.

### Фаза B — Контракти та тести (1–2 тижні)

1. Для кожної адмін-сторінки: короткий коментар або рядок у докі «поля JSON → колонки таблиці» (як у [UI_QUALITY](./UI_QUALITY_AND_E2E_PLAN_2026-04-06.md) для workers).
2. Інтеграційні тести **ключів JSON** для критичних GET (RAID admin, enterprise dashboard slices) — патерн `tests/network_api_integration.rs`.
3. Окремий сценарій або док для **SyncArtifacts**: валідне тіло або новий endpoint.

### Фаза C — UX і стабільність (паралельно з B)

1. Обробка **401 / 403 / 503** з читабельним текстом і підказкою (логін / enterprise feature / ініціалізація сервісу).
2. Стани завантаження / порожньо / помилка для всіх таблиць адмінки.
3. Узгодити **README**: URL адмінки **`http://localhost:8080/ui/admin`**, не плутати з `/admin`.

### Фаза D — E2E (опційно)

Після стабілізації A–B: Playwright/Cypress «відкрити /ui/admin → логін → smoke по розділах» (див. колонку C в UI_QUALITY).

---

## 5. Критерії готовності (checkpoint)

- [ ] Немає звернень до **`/api/raid/admin/*`** (лише `/api/v1/raid/admin/*`).
- [ ] RAID admin картки показують реальні дані при активному RAID manager.
- [ ] Немає «мертвого» **`POST /api/v1/refresh`** без реалізації або маршрут додано і задокументовано.
- [ ] Sync artifacts або прибрано з UI до появи контракту, або виклик відповідає `ProtocolMessage`.
- [ ] README / troubleshooting вказують **`/ui/admin`** і залежність від **`enterprise`** для enterprise-розділів.

---

## Посилання на код

- Монтування API: `src/network/mod.rs`.
- RAID user + distributed: `src/network/api/raid.rs`.
- RAID admin: `src/network/api/raid_admin.rs`.
- Адмін RAID UI: `src/ui/admin/raid.rs`.
- Головний UI + refresh: `src/ui/mod.rs`.
- Спільні fetch / токен для адмінки: `src/ui/admin_common.js`.
