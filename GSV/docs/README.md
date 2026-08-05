# GSV docs — індекс

Канонічна документація проєкту **Galaxy StarWalker Vision** (окремий Rust-first проєкт
у `GSV/` репо PoolAI). Архітектурні файли `docs/gsv/` живуть у репо PoolAI; внутрішні
канон-файли GSV — тут, у `GSV/docs/`.

| Файл | Призначення |
|------|-------------|
| [`GSV_ROLES.md`](GSV_ROLES.md) | Ролі GSV VDT (Власник/Оркестратор/Субагенти), канон сесії, Rust ratio gate |
| [`MEMORY.md`](MEMORY.md) | **Memory mark** — стан проєкту (bands 102 · 108 · 109), ключові факти, what/why |
| [`VISION.md`](VISION.md) | **Vision box** — дзеркало poolAI vision canon (manifest/feed/sync) |
| [`HANDOFF_NEW_SESSION.md`](HANDOFF_NEW_SESSION.md) | Операційний зріз для наступної сесії (S0, щоденники, тести) |
| [`NEXT_SESSION_PROMPT.md`](NEXT_SESSION_PROMPT.md) | Copy-paste промпт наступної сесії GSV |

Зовнішні посилання (репо PoolAI):

| Файл | Призначення |
|------|-------------|
| [`docs/gsv/README.md`](../../docs/gsv/README.md) | Індекс docs проєкту GSV (зовнішній) |
| [`docs/gsv/GSV_ARCHITECTURE.md`](../../docs/gsv/GSV_ARCHITECTURE.md) | Архітектура сервера + боксів |
| [`docs/gsv/GSV_SERVER.md`](../../docs/gsv/GSV_SERVER.md) | exe/bin сервер (endpoints, update, offline) |
| [`docs/gsv/GSV_BOXES.md`](../../docs/gsv/GSV_BOXES.md) | Специфікація боксів |
| [`docs/gsv/GSV_MIGRATION.md`](../../docs/gsv/GSV_MIGRATION.md) | Що мігруємо з `docs/vision/` / `src/` у GSV |
| [`docs/gsv/GSV_TECH_ROADMAP.md`](../../docs/gsv/GSV_TECH_ROADMAP.md) | TechPreroadMap → future sprints |

Канон поведінки (roles/ratio/session) — [`GSV_ROLES.md`](GSV_ROLES.md); лічильники
(кількість тестів, ratio) завжди вимірювати командами, не з пам'яті.
