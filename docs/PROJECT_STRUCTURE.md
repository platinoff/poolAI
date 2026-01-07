# 📁 Структура проекту PoolAI

**Останнє оновлення**: 2025-12-30  
**Версія**: 2.0

## 🎯 Принципи організації

1. **Чистий корінь проекту** - тільки README файли та конфігурація
2. **Логічна структура** - все організовано за категоріями
3. **GitHub clean structure** - відповідає best practices
4. **Легка навігація** - чіткі каталоги та індекси

## 📂 Повна структура проекту

```
poolAI/
│
├── 📄 README.md                    # Основний README
├── 📄 README.uk.md                 # Український README
├── 📄 .cursorrules                  # Правила для Cursor IDE
├── 📄 .gitignore                   # Git ignore rules
│
├── 📁 src/                         # Rust source code
│   ├── core/                      # Core module
│   ├── pool/                      # Pool module
│   ├── monitoring/                # Monitoring module
│   ├── network/                   # Network module
│   ├── platform/                  # Platform module
│   ├── runtime/                   # Runtime module
│   ├── rewards/                   # Rewards module
│   ├── tgbot/                     # Telegram bot
│   ├── security/                  # Security module
│   ├── libs/                      # Libs module
│   ├── vm/                        # VM module
│   ├── raid/                      # RAID module
│   ├── ui/                        # UI module
│   └── ...
│
├── 📁 tests/                       # Integration tests
│   ├── integration/
│   ├── vm_isolation_integration.rs
│   └── ...
│
├── 📁 docs/                        # 📚 ВСЯ ДОКУМЕНТАЦІЯ ТУТ
│   │
│   ├── 📄 README.md                # Індекс документації
│   ├── 📄 STRUCTURE.md             # Опис структури
│   ├── 📄 PROJECT_STRUCTURE.md     # Цей файл
│   ├── 📄 CURSOR_WORKFLOW.md       # Правила роботи з Cursor
│   │
│   ├── 📁 status/                   # Поточний стан проекту
│   │   ├── CURRENT_STATUS.md
│   │   └── STABLE_STATE_SUMMARY.md
│   │
│   ├── 📁 development/             # Плани розробки
│   │   ├── NEXT_STEPS_PLAN.md
│   │   ├── NEXT_DEVELOPMENT_PHASE.md
│   │   └── DEVELOPMENT_PLAN_UPDATED.md
│   │
│   ├── 📁 concept/                 # Концепція проекту
│   │   └── poolAI_concept.txt
│   │
│   ├── 📁 archive/                 # Архівні документи (60+ файлів)
│   │   ├── PHASE*.md
│   │   ├── WEEK*.md
│   │   ├── GIT_PUSH*.md
│   │   └── ...
│   │
│   ├── 📁 deployment/              # Розгортання
│   │   ├── DOCKER.md
│   │   ├── KUBERNETES.md
│   │   └── BARE_METAL.md
│   │
│   ├── 📁 configuration/           # Конфігурація
│   │   └── PRODUCTION.md
│   │
│   ├── 📁 monitoring/              # Моніторинг
│   │   ├── PROMETHEUS.md
│   │   ├── GRAFANA.md
│   │   └── ALERTS.md
│   │
│   ├── 📁 security/                 # Безпека
│   │   └── BEST_PRACTICES.md
│   │
│   ├── 📁 performance/             # Продуктивність
│   │   ├── TUNING.md
│   │   └── BENCHMARKS.md
│   │
│   ├── 📁 troubleshooting/         # Troubleshooting
│   │   └── COMMON_ISSUES.md
│   │
│   ├── 📁 migration/                # Міграція
│   │   └── MIGRATION.md
│   │
│   ├── 📁 vm/                      # VM Module
│   │   └── ISOLATION_IMPLEMENTATION.md
│   │
│   └── 📄 [інші документи]         # ADR, протоколи, тощо
│       ├── ADR_001_DISTRIBUTED_RAID.md
│       ├── DEVELOPMENT_ROADMAP.md
│       ├── DISTRIBUTED_RAID_PROTOCOL.md
│       ├── QUICK_START.md
│       ├── SECURITY.md
│       └── ...
│
├── 📁 scripts/                      # 🔧 ВСІ СКРИПТИ ТУТ
│   ├── 📄 README.md                # Документація скриптів
│   ├── fix_cargo_now.sh
│   ├── fix_gcc.sh
│   ├── install_gcc.sh
│   ├── setup_rust_path.sh
│   ├── QUICK_FIX_RUST_PATH.sh
│   ├── verify_build.sh
│   └── PUSH_COMMANDS.sh
│
├── 📁 certs/                        # SSL сертифікати
├── 📁 target/                       # Build artifacts (gitignored)
│
└── 📄 [конфігураційні файли]        # Cargo.toml, config.toml, тощо
    ├── Cargo.toml
    ├── Cargo.lock
    ├── config.toml
    └── ...
```

## 📊 Статистика

### Документація
- **Загальна кількість**: 87+ файлів
- **Активні документи**: ~20 файлів
- **Архівні документи**: ~60 файлів
- **Категорій**: 12+

### Скрипти
- **Загальна кількість**: 7 файлів
- **Категорії**: Setup, Fix, Install, Verify, Git

### Код
- **Модулів**: 13+
- **Тестів**: 177+ passing
- **Покриття**: Core functionality fully tested

## 🎯 Правила розміщення файлів

### ✅ ДОЗВОЛЕНО в корені:
- `README.md` - основний README
- `README.uk.md` - український README
- Конфігураційні файли: `Cargo.toml`, `config.toml`, `.gitignore`
- `.cursorrules` - правила для Cursor

### ❌ ЗАБОРОНЕНО в корені:
- `.md` файли (окрім README)
- `.sh` файли
- Документація
- Скрипти
- Тимчасові файли

### 📁 Де що розміщувати:

| Тип файлу | Розташування | Приклад |
|-----------|--------------|---------|
| Документація | `docs/[category]/` | `docs/status/CURRENT_STATUS.md` |
| Скрипти | `scripts/` | `scripts/fix_cargo_now.sh` |
| Концепція | `docs/concept/` | `docs/concept/poolAI_concept.txt` |
| Архів | `docs/archive/` | `docs/archive/PHASE5.md` |
| Код | `src/` | `src/core/mod.rs` |
| Тести | `tests/` | `tests/integration.rs` |

## 🔍 Швидкий пошук

### Знайти документацію:
- **Статус**: `docs/status/`
- **Плани**: `docs/development/`
- **Розгортання**: `docs/deployment/`
- **Безпека**: `docs/security/`
- **Індекс**: `docs/README.md`

### Знайти скрипти:
- **Всі скрипти**: `scripts/`
- **Документація**: `scripts/README.md`

## 📚 Посилання

- [`docs/README.md`](./README.md) - Індекс документації
- [`docs/STRUCTURE.md`](./STRUCTURE.md) - Детальний опис структури
- [`docs/CURSOR_WORKFLOW.md`](./CURSOR_WORKFLOW.md) - Правила роботи з Cursor
- [`scripts/README.md`](../scripts/README.md) - Документація скриптів

## ✅ Перевірка структури

Для перевірки чи все правильно:

```powershell
cd S:\rust\poolAI

# Перевірка: чи є .md файли в корені (окрім README)
Get-ChildItem -Filter *.md -File | Where-Object { $_.Name -ne 'README.md' -and $_.Name -ne 'README.uk.md' }
# Має бути порожньо!

# Перевірка: чи є .sh файли в корені
Get-ChildItem -Filter *.sh -File
# Має бути порожньо!

# Перевірка: чи всі скрипти в scripts/
Get-ChildItem scripts -Filter *.sh
# Має показати всі скрипти

# Перевірка: чи вся документація в docs/
Get-ChildItem docs -Recurse -Filter *.md | Measure-Object
# Має показати кількість файлів
```

---

**Висновок**: Структура проекту повністю організована, відповідає вимогам Rust Architect! 🎉

