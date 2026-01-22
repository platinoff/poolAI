# PoolAI Documentation

Цей каталог містить всю документацію проекту PoolAI, організовану за категоріями.

> 📖 **Для розробників**: 
> - [`CURSOR_WORKFLOW.md`](./CURSOR_WORKFLOW.md) - Правила роботи з документацією в Cursor IDE
> - [`PROJECT_STRUCTURE.md`](./PROJECT_STRUCTURE.md) - Повна структура проекту з візуалізацією
> - [`STRUCTURE.md`](./STRUCTURE.md) - Детальний опис структури документації

## 📚 Структура документації

### 🎯 Основна документація
- [`QUICK_START.md`](./QUICK_START.md) - Швидкий старт
- [`DEVELOPMENT_ROADMAP.md`](./DEVELOPMENT_ROADMAP.md) - Дорожня карта розробки
- [`SECURITY.md`](./SECURITY.md) - Безпека системи

### 📊 Статус та плани розробки

**Актуальні документи (v0.2.2)**:
- [`status/PROJECT_STATUS_REPORT_2026-01-19.md`](./status/PROJECT_STATUS_REPORT_2026-01-19.md) - **ОСНОВНИЙ** статус проекту
- [`status/STABLE_STATE_UPDATE_2026-01-19.md`](./status/STABLE_STATE_UPDATE_2026-01-19.md) - Стабільний стан (v0.2.2)
- [`status/RUST_ARCHITECT_UPDATE_2026-01-22.md`](./status/RUST_ARCHITECT_UPDATE_2026-01-22.md) - Останнє оновлення Rust Architect
- [`development/NEXT_STEPS_2026-01-19.md`](./development/NEXT_STEPS_2026-01-19.md) - **АКТУАЛЬНІ** наступні кроки
- [`development/NEXT_STEPS_ARCHITECT_2026-01-22.md`](./development/NEXT_STEPS_ARCHITECT_2026-01-22.md) - Останній план Rust Architect
- [`development/FUTURE_DEVELOPMENT_ROADMAP.md`](./development/FUTURE_DEVELOPMENT_ROADMAP.md) - Майбутній roadmap

**Індекси**:
- [`status/README.md`](./status/README.md) - Індекс статусних документів
- [`development/README.md`](./development/README.md) - Індекс планів розробки

### 🏗️ Архітектура та Концепція

**PRIMARY Концепція** (завжди перевіряй спочатку):
- [`concept/poolAI_concept_root.txt`](./concept/poolAI_concept_root.txt) - **PRIMARY** концепція проекту
- [`concept/CONCEPT_UPDATE_2026-01-19.md`](./concept/CONCEPT_UPDATE_2026-01-19.md) - Оновлення концепції (v7)

**Архітектурні рішення**:
- [`ADR_001_DISTRIBUTED_RAID.md`](./ADR_001_DISTRIBUTED_RAID.md) - Architecture Decision Record для Distributed RAID
- [`DISTRIBUTED_RAID_PROTOCOL.md`](./DISTRIBUTED_RAID_PROTOCOL.md) - Протокол Distributed RAID
- [`RAFT_LIBRARY_EVALUATION.md`](./RAFT_LIBRARY_EVALUATION.md) - Оцінка бібліотеки Raft
- [`ARCHITECTURE_BEST_PRACTICES.md`](./ARCHITECTURE_BEST_PRACTICES.md) - Best practices архітектури

### 🚀 Розгортання
- [`deployment/DOCKER.md`](./deployment/DOCKER.md) - Docker deployment
- [`deployment/KUBERNETES.md`](./deployment/KUBERNETES.md) - Kubernetes deployment
- [`deployment/BARE_METAL.md`](./deployment/BARE_METAL.md) - Bare metal deployment

### ⚙️ Конфігурація
- [`configuration/PRODUCTION.md`](./configuration/PRODUCTION.md) - Production конфігурація

### 📈 Моніторинг
- [`monitoring/PROMETHEUS.md`](./monitoring/PROMETHEUS.md) - Prometheus setup
- [`monitoring/GRAFANA.md`](./monitoring/GRAFANA.md) - Grafana dashboards
- [`monitoring/ALERTS.md`](./monitoring/ALERTS.md) - Alerting configuration

### 🎨 UI/UX
- [`UI_IMPROVEMENTS_PLAN.md`](./UI_IMPROVEMENTS_PLAN.md) - План покращень UI

### 🔒 Безпека
- [`security/BEST_PRACTICES.md`](./security/BEST_PRACTICES.md) - Best practices для безпеки

### ⚡ Продуктивність
- [`performance/TUNING.md`](./performance/TUNING.md) - Performance tuning
- [`performance/BENCHMARKS.md`](./performance/BENCHMARKS.md) - Benchmark results

### 🔧 Troubleshooting

**Актуальні гайди**:
- [`troubleshooting/COMMON_ISSUES.md`](./troubleshooting/COMMON_ISSUES.md) - Типові проблеми та рішення
- [`troubleshooting/GIT_AUTH_FIX.md`](./troubleshooting/GIT_AUTH_FIX.md) - Виправлення git аутентифікації
- [`troubleshooting/GIT_INDEX_LOCK_FIX.md`](./troubleshooting/GIT_INDEX_LOCK_FIX.md) - Виправлення index.lock
- [`troubleshooting/GIT_PUSH_FAILED.md`](./troubleshooting/GIT_PUSH_FAILED.md) - Проблеми з git push
- [`troubleshooting/QUICK_PUSH.md`](./troubleshooting/QUICK_PUSH.md) - Швидкий push (copy-paste команди)
- [`troubleshooting/PUSH_FIX_NOW.md`](./troubleshooting/PUSH_FIX_NOW.md) - Швидке виправлення push
- [`troubleshooting/QUICK_FIX_MSYS2.md`](./troubleshooting/QUICK_FIX_MSYS2.md) - Швидке виправлення MSYS2
- [`troubleshooting/GCC_DLLTOOL_NOT_FOUND.md`](./troubleshooting/GCC_DLLTOOL_NOT_FOUND.md) - Проблеми з gcc/dlltool

### 🔄 Міграція
- [`migration/MIGRATION.md`](./migration/MIGRATION.md) - Migration guides

### 🖥️ VM Module
- [`vm/ISOLATION_IMPLEMENTATION.md`](./vm/ISOLATION_IMPLEMENTATION.md) - VM isolation implementation

### 📦 Архів
- [`archive/`](./archive/) - Архівні документи (статуси, milestone, старі плани)
- [`DOCUMENTATION_CLEANUP_PLAN_2026-01-22.md`](./DOCUMENTATION_CLEANUP_PLAN_2026-01-22.md) - План очищення документації

### 🏗️ Rust Architect
- [`RUST_ARCHITECT_ITERATIVE_PLAN_2026-01-22.md`](./RUST_ARCHITECT_ITERATIVE_PLAN_2026-01-22.md) - План ітераційної розробки з моніторингом контексту
- [`CHAT_SUMMARY_2026-01-22.md`](./CHAT_SUMMARY_2026-01-22.md) - Саммари чату
- [`ARCHITECTURE_UPDATE_2026-01-22.md`](./ARCHITECTURE_UPDATE_2026-01-22.md) - Актуалізація архітектури
- [`GIT_WORKFLOW_2026-01-22.md`](./GIT_WORKFLOW_2026-01-22.md) - Git workflow для Rust Architect
- [`GIT_PUSH_NOW_2026-01-22.md`](./GIT_PUSH_NOW_2026-01-22.md) - Команди для git push зараз
- [`NEXT_STEPS_AFTER_PUSH_2026-01-22.md`](./NEXT_STEPS_AFTER_PUSH_2026-01-22.md) - Наступні кроки після push
- [`RUST_ARCHITECT_FINAL_COMMANDS.md`](./RUST_ARCHITECT_FINAL_COMMANDS.md) - Фінальні команди для виконання
- [`EXECUTE_NOW.md`](./EXECUTE_NOW.md) - ⚡ Виконай зараз (git push + наступні кроки)
- [`FIX_AND_PUSH_NOW.md`](./FIX_AND_PUSH_NOW.md) - Виправлення проблем та push
- [`PUSH_COMMIT_NOW.md`](./PUSH_COMMIT_NOW.md) - Push існуючого коміту
- [`FIX_AUTH_AND_PUSH.md`](./FIX_AUTH_AND_PUSH.md) - ⚡ Виправлення аутентифікації та push (актуально)
- [`PUSH_WITH_AUTH_FIX.md`](./PUSH_WITH_AUTH_FIX.md) - Детальний гайд з push та аутентифікації
- [`PUSH_NOW_SSH_OR_PAT.md`](./PUSH_NOW_SSH_OR_PAT.md) - ⚡ Push зараз: SSH або PAT (найшвидше)
- [`PUSH_SSH_OR_PAT.md`](./PUSH_SSH_OR_PAT.md) - Детальний гайд з SSH та PAT
- [`PUSH_FINAL_SOLUTION.md`](./PUSH_FINAL_SOLUTION.md) - ⚡ Фінальне рішення: SSH або Credentials File (актуально)
- [`CHECK_SYSTEM_NOW.md`](./CHECK_SYSTEM_NOW.md) - 🔍 Перевірка системи (SSH, git config, credentials)
- [`SYSTEM_CHECK_REPORT.md`](./SYSTEM_CHECK_REPORT.md) - 📊 Звіт про перевірку системи (автоматична перевірка)

---

## 🎯 Швидкий Старт

1. **Новий розробник**: Почни з [`QUICK_START.md`](./QUICK_START.md)
2. **Поточний стан**: Перевір [`status/PROJECT_STATUS_REPORT_2026-01-19.md`](./status/PROJECT_STATUS_REPORT_2026-01-19.md)
3. **Концепція**: Прочитай [`concept/poolAI_concept_root.txt`](./concept/poolAI_concept_root.txt)
4. **Наступні кроки**: Дивись [`development/NEXT_STEPS_2026-01-19.md`](./development/NEXT_STEPS_2026-01-19.md)

---

## 📝 Правила

- ✅ Всі нові документи створюються в `docs/` згідно з категоріями
- ✅ README.md та README.uk.md залишаються в корені проекту
- ✅ Застарілі документи переміщуються в `archive/`
- ✅ Актуальні документи мають дату 2026-01-19 або пізніше
- ✅ Поточна версія проекту: **v0.2.2**

---

**Останнє оновлення**: 2026-01-22  
**Версія проекту**: v0.2.2 Production Ready

