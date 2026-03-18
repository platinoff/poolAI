# 📁 Структура документації PoolAI

Цей документ описує організацію документації проекту PoolAI.

## 🎯 Принципи організації

1. **Чистий корінь проекту**: Тільки `README.md` та `README.uk.md` залишаються в корені
2. **Логічна структура**: Документи організовані за категоріями та призначенням
3. **Відповідність GitHub clean structure**: Відповідає best practices для open source проектів
4. **Легка навігація**: Чіткі каталоги та індексний файл `docs/README.md`

## 📂 Структура каталогів

```
docs/
├── README.md                    # Індекс документації
├── STRUCTURE.md                 # Цей файл - опис структури
│
├── status/                      # Поточний стан проекту
│   ├── CURRENT_STATUS.md        # Детальний поточний стан
│   └── STABLE_STATE_SUMMARY.md  # Стабільний стан розробки
│
├── development/                 # Плани розробки
│   ├── NEXT_STEPS_2026-01-19.md           # Актуальні наступні кроки (оновлений Rust Architect план)
│   ├── NEXT_STEPS_ARCHITECT_2026-01-22.md # Останній детальний план Architect
│   ├── FUTURE_DEVELOPMENT_ROADMAP.md      # Довгостроковий roadmap
│   └── PERFORMANCE_OPTIMIZATION_PLAN_2026-03-17.md # План оптимізації продуктивності (bottleneck-и, бенчмарки, глибока логіка)
│
├── concept/                     # Концепція проекту
│   ├── poolAI_concept_root.txt  # PRIMARY концепція проекту (оновлена, v6/v7)
│   └── CONCEPT_UPDATE_2026-01-19.md # Оновлення концепції (розширені модулі, RAID/Enterprise/Cloud/ML)
│
├── archive/                     # Архівні документи
│   ├── PHASE*.md                # Статуси фаз розробки
│   ├── WEEK*.md                 # Тижневі звіти
│   ├── GIT_PUSH*.md             # Git push звіти
│   └── ...                      # Інші архівні документи
│
├── deployment/                  # Розгортання
│   ├── DOCKER.md
│   ├── KUBERNETES.md
│   └── BARE_METAL.md
│
├── configuration/                # Конфігурація
│   └── PRODUCTION.md
│
├── monitoring/                   # Моніторинг
│   ├── PROMETHEUS.md
│   ├── GRAFANA.md
│   └── ALERTS.md
│
├── security/                     # Безпека
│   └── BEST_PRACTICES.md
│
├── performance/                  # Продуктивність
│   ├── TUNING.md
│   └── BENCHMARKS.md
│
├── troubleshooting/              # Troubleshooting
│   └── COMMON_ISSUES.md
│
├── migration/                    # Міграція
│   └── MIGRATION.md
│
├── vm/                          # VM Module
│   └── ISOLATION_IMPLEMENTATION.md
│
└── [інші документи]             # ADR, протоколи, тощо
```

## 📝 Правила роботи з документацією

### Створення нових документів

1. **Визначте категорію**: Виберіть відповідний каталог або створіть новий
2. **Використовуйте правильні назви**: 
   - UPPERCASE для основних документів
   - lowercase для допоміжних файлів
   - Використовуйте підкреслення замість пробілів: `CURRENT_STATUS.md`
3. **Оновіть індекс**: Додайте посилання в `docs/README.md`

### Посилання між документами

- Використовуйте відносні шляхи від поточного файлу
- Приклад: `[Current Status](../status/CURRENT_STATUS.md)`
- Для файлів в тому ж каталозі: `[File](./FILE.md)`
- Для файлів в підкаталозі: `[File](./subdir/FILE.md)`

### Оновлення посилань

При переміщенні файлів:
1. Оновіть всі посилання на файл
2. Перевірте посилання в Rust коді (коментарі)
3. Оновіть індекс `docs/README.md`

## 🔍 Пошук документації

### За категорією

- **Статус проекту**: `docs/status/`
- **Плани розробки**: `docs/development/`
- **Розгортання**: `docs/deployment/`
- **Безпека**: `docs/security/`
- **Продуктивність**: `docs/performance/`

### За типом

- **ADR (Architecture Decision Records)**: `docs/ADR_*.md`
- **Протоколи**: `docs/*_PROTOCOL.md`
- **Roadmaps**: `docs/DEVELOPMENT_ROADMAP.md`
- **Quick Start**: `docs/QUICK_START.md`

## 🚀 Інтеграція з Cursor

Файл `.cursorrules` містить правила для Cursor IDE:
- Автоматичне створення файлів в правильних каталогах
- Оновлення посилань при переміщенні
- Дотримання структури документації

## 📊 Статистика

- **Загальна кількість документів**: 87+ файлів
- **Активні документи**: ~20 файлів
- **Архівні документи**: ~60 файлів
- **Категорій**: 12+

---

**Останнє оновлення**: 2025-12-30  
**Версія структури**: 1.0

