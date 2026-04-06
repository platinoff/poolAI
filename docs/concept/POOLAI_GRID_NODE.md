# PoolAI Grid Node — концепт вузла (v1, 2026-03-17)

## 1. Що таке PoolAI Node

**PoolAI Node** — це повноцінний інстанс PoolAI, який включає всі 15 модулів:

- Core, Pool, Monitoring, Network, Platform, Runtime, Rewards, TGBot
- Security, Enterprise, Cloud, UI, Libs, RAID, VM

Нода вже сьогодні є:

- **AI runtime** (VM + Runtime + Libs + Cloud)
- **Storage/consensus** вузлом (RAID + SmallWorld + Event Sourcing + Admin Control Plane)
- **Enterprise‑entrypoint** (Network + UI + Enterprise + Security)

Цей документ не додає нових модулів, а **описує існуючу архітектуру як building block Grid‑мережі**.

## 2. Ролі ноди в Grid

- **Miner Node**:
  - Виконує AI‑таски (інференс, тренування, індексація, обробка даних) на CPU/GPU/DPU/ASIC.
  - Піднімає локальний VM/Runtime/Libs стек.
  - Отримує винагороду через Rewards Module (і в майбутньому — через Solana‑adapter).

- **Hub Node**:
  - Агрегує задачі та результати, виступає “routing/coordination” центром.
  - Використовує Network/Enterprise/Monitoring/Cloud/RAID для координації.

- **Hybrid Node**:
  - Поєднує обидві ролі (typical default).

Роль — це **конфігурація поверх уже реалізованих модулів**, а не окремий тип інстансу.

## 3. Grid Layer поверх існуючих механізмів

Grid‑рівень спирається на вже реалізовані частини:

- **RAID + SmallWorld**:
  - Доведена масштабованість до десятків/сотень вузлів (grid‑тести RAID до 120 нод).
  - Топологічно усвідомлена реплікація (latency, clustering coefficient).

- **Discovery / Peer API**:
  - API для реєстрації peers та отримання інформації про них.
  - Інтеграція з Worker Pool (peers як workers).

Grid Layer додає **протокол між нодами**, не змінюючи існуючі модулі:

- **Типи повідомлень (на концептуальному рівні)**:
  - `Job`: опис задачі (ресурси, дедлайн, тип, політика верифікації).
  - `Result`: результат виконання (output + метрики).
  - `MemoryShard`: інформація про shard памʼяті/артефакту.
  - `PeerStatus`: поточне навантаження/здоровʼя/репутація ноди.

## 4. Від “кластеру” до “грида”

Історично PoolAI розвивався як:

1. **Продакшн‑кластер** з Kubernetes/Cloud/Enterprise/RAID/VM.
2. **Grid‑масштабування RAID** (тести до 120 нод).
3. **ML/Enterprise надбудови** (AutoML, Federated, Context Memory, Admin Panel).

Цей документ робить наступний крок:

- **кластер** = багато PoolAI‑нод, керованих одним Cloud/K8s середовищем;
- **grid** = набір нод, які можуть:
  - працювати як окремі інстанси,
  - або збиратися в децентралізовану мережу через Discovery + Grid Protocol.

## 5. Звʼязок із існуючими доками

- `docs/concept/poolAI_concept.txt` — базова концепція “AI Mining Pool Management System”.
- `docs/status/*` — підтверджують 100% готовність 15 модулів.
- `docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md` — цей файл розширює план на Grid‑рівень.

Пов’язані development‑доки:

- `POOLAI_MEMORY_LAYER.md` — опис AGI‑памʼяті та seeds.
- `development/JOB_LAYER_CONCEPT_2026-03-17.md` — Job / Mining Layer.
- `development/GRID_PROTOCOL_CONCEPT_2026-04-06.md` — **Grid Protocol** (типи повідомлень Job / Result / MemoryShard / PeerStatus і мапінг на Discovery/RAID/тести).

