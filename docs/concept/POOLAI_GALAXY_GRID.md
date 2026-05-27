# PoolAI Galaxy Grid (концепт v1)

## Коротко

**Galaxy Grid** — це глобальна, федеративна мережа інстансів PoolAI (srvN), де:

1. **Адмін srvN** підключає свої ресурси (host/VM, Cloud через API провайдера, можливі власні VM worker’и).
2. **Telegram-клієнти** вмикають mining на своїй стороні (в чаті), отримують винагороди за виконані job’и.
3. **Клієнти ШІ** споживають інференс через веб-інтерфейс гріду (через будь-який домен), який маршрутизує job на доступні worker’и.
4. **Grid** обирає “де виконувати” job з урахуванням ресурсів, locality та вартості, а якщо на srvN все зайнято — робить **re-migrate** туди, де з’явився вільний capacity.

Цей документ — продуктово-архітектурна специфікація рівня концепту/протоколу (без деталей UI/E2E), узгоджена з наявними модулями: **virtual nodes (FM-016)**, **RAID/SmallWorld**, **Jobs**, **VM isolation**, **Telegram bot** та **Solana adapter (sidecar)**.

## 1. Ролі та економіка

### 1.1 Засновник / dev (primary fee)

- У коді PoolAI є **primary dev wallet**.
- **Primary dev fee = 0.1%** (незмінна комісія для засновника).

### 1.2 Адмін srvN (secondary fee + керування)

- Адмін піднімає PoolAI srvN (hub/coordinator) і керує:
  - власними worker’ами та їх ресурсними лімітами (CPU/GPU/RAM/Disk);
  - інтеграціями з провайдерами (K8s/EC2/Azure тощо) через свій API доступ у межах свого srvN;
  - розміщенням worker’ів як VM/isolated instances.
- **Secondary fee = 1–5%** (рекомендація: чим менше — тим краща ринкова конкурентність).
- При “без Telegram-пулу” адмін заробляє собі secondary fee, віддаючи job лише primary (0.1%) dev’у.

### 1.3 Telegram-клієнт (mining worker edge)

- Telegram-користувач підключає свої гаманці для винагороди в UI/аплікації (через tgbot).
- Якщо в каналі підключені Telegram worker’и, то сукупна винагорода майнінгу розподіляється так:
  - primary dev fee = 0.1% завжди;
  - secondary fee (встановлена адміном srvN) — переходить адмінам srvN;
  - решта винагороди — Telegram edge worker’у (або власнику worker-посесії).
- Ліміт Telegram worker’ів для srvN: **не більше кількості worker-сеатів на каналі Telegram** (точну формулу seat’ів потрібно формалізувати в окремому документі).

### 1.4 Клієнт ШІ (споживач інференсу)

- Це користувач, який отримав “адресу”/endpoint від адміна srvN або через discovery гріду.
- Він:
  - заходить у web UI з **будь-якого домену** (типовий ingress + cookie/session);
  - створює job на інференс;
  - оплачує виконання (billing/settlement на стороні srvN).

> Примітка: конкретний вибір “який саме IDE/white-label UI” лишається відкритим; це не впливає на мережевий контракт job’ів.

## 2. Модель worker’ів і discovery

### 2.1 Уніфікована сутність “worker / virtual-node”

Рекомендована модель: **одна сутність** у discovery (наприклад `virtual-nodes / workers`), але з полями:

- `origin`: `local_srv` | `cloud_provider` | `telegram_edge`
- `admin_id` / `srv_id`: кому належить цей capacity
- `capabilities`: GPU/CPU/Memory/Disk, підтримка тасків, prefetch-профіль
- `network_profile`: latency/topology/white-ip/vpn+proxy параметри (для routing)
- `limits`: скільки worker реально готовий “віддати” (manual cap або auto)

У UI/API ці параметри можна показувати фільтрами/лейблами, без дублювання сутностей.

### 2.2 Auto-scale та capacity allocation

Worker’и мають:

1. **Auto-pick** (залежно від навантаження srvN та/або власної внутрішньої метрики worker’а).
2. **Manual cap**: адміністратор може задати, скільки ресурсів worker реально віддає гріду.
3. **Telegram edge cap**: обмеження кількістю seats у Telegram каналі.

Для CPU/GPU worker’ів авто-розкладка може відрізнятись, але контракт має бути однаковим: “скільки capacity доступно зараз”.

## 3. Telegram edge mining: worker як “VM-aware” isolated capacity

Telegram worker (desktop Win/Linux/Mac) має працювати як:

- worker-агент в ізоляції (VM/containment під контролем PoolAI VM/Isolation layer);
- з probros до ресурсів host’а (GPU пізніше, наразі CPU/RAM/Disk у режимі “cold mining” або без GPU passthrough).

Усі параметри mining worker’а (wallets, винагорода, primary/secondary fee split) мають керуватись **всередині Telegram чату**, а tgbot — виступає керувальним оркестратором зв’язування.

## 4. Grid scheduling, re-migration та pricing

### 4.1 Global routing policy

Grid є **глобальною мережею**: srvN можуть існувати де завгодно; адмінам надається механізм підключення своїх endpoint’ів у грід через VPN/proxy/white-IP домени.

Scheduling має враховувати:

- доступні worker capabilities;
- network_profile (latency/topology, пропускна здатність);
- locality (seed/memory shard placement та “де вже є потрібні шари”);
- pricing.

### 4.2 Pricing

- Ціна job’а визначається як **найнижча ринкова ціна** серед обраних US-провайдерів (OpenAI/Grok/xAI/Mistral тощо), але:
  - **PoolAI знижує її на 10%** від мінімуму.

Реалізацію price oracle треба робити як окремий сервіс/механізм (качує, має fallback при відсутності котирувань).

### 4.3 Re-migrate policy

Якщо srvN прийняв job, але:

- усі його worker’и зайняті,

то job не чекає безкінечно; він **re-migrate** у той srvN, де з’явився вільний capacity (потрібен механізм job lease / at-most-once гарантії).

## 5. Seeds / shards / locality-aware placement

### 5.1 Семантика seeds

Для практичного контенту “seeds” у цьому документі — це узагальнений термін для локальних даних, які найчастіше потрібні worker’у:

- memory shards / seed-proфілі;
- RAID artifacts (які інтерпретуються як потрібні дані/шари);
- hot-layer cache (RAM/VRAM) конкретного профілю тасків.

### 5.2 Placement та мережеве збереження трафіку

Мета: “не тягнути” зайве по мережі між seed’ами.

Рекомендована політика:

- placement “гарячих” шарів ближче до worker’ів, які їх використовують найчастіше;
- **авто-підхід через SmallWorld** (топологічно усвідомлена реплікація + short-path routing).

### 5.3 Prefetch: RAM vs VRAM

Prefetch політика залежить від таску та worker’а:

- CPU worker: префетч у RAM;
- GPU worker: префетч у VRAM (або GPU page-cache, якщо підтримується).

Ключ: placement і префетч повинні бути “capability-driven”, а не статичними.

## 6. Security та верифікація (edge untrusted)

Оскільки Telegram edge worker’и можуть бути “частково недовіреними”, має бути механізм:

- підтвердження capability (що worker реально може);
- мінімальна верифікація результатів job (на старті — реплікація/перевірка вибіркових підзадач; далі — розширення).

На рівні концепту: precise security model (ZK/attestation/TEEs/сигнатури) залишається TBD.

## 7. On-chain події, settlement та аудит

On-chain події потрібні, коли вони:

- фіксують settlement (комісії / винагороди),
- дають аудит trace для спорів,
- або є “обов’язковим” proof layer для безпеки routing.

У routing (швидке прийняття рішення де виконувати) основна логіка має бути off-chain.

## 8. Відкриті питання (TBD)

1. **Job lease / at-most-once**: як гарантуємо, що re-migrate не спричинить double-exec.
2. **Telegram seats**: що саме лімітує кількість worker’ів (members, wallets, concurrent sessions).
3. **Network_profile contract**: набір метрик, формат зберігання і як SmallWorld їх споживає.
4. **Primary/secondary fee settlement**: точний механізм payout (on-chain чи офлайн batch).
5. **Telegram “VM probros” на старте**: що входить у MVP для cold mining (CPU/RAM/Disk) і як мігруємо до GPU passthrough пізніше.
6. **Super-admin governance**: політика безпеки/оновлень для відкритої мережі без “root доступу” до чужих srvN.

