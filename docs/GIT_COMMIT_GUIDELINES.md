# 📝 Git Commit Guidelines для PoolAI

**Для Rust Architect та Git Best Practices**

## 🎯 Принципи

### 1. Conventional Commits
Використовуємо стандарт [Conventional Commits](https://www.conventionalcommits.org/) для консистентності:

```
<type>(<scope>): <subject>

<body>

<footer>
```

### 2. Типи комітів (Type)

| Тип | Опис | Приклад |
|-----|------|---------|
| `feat` | Нова функціональність | `feat(vm): add network isolation` |
| `fix` | Виправлення помилки | `fix(ui): correct modal focus trap` |
| `docs` | Зміни в документації | `docs: update project structure` |
| `style` | Форматування, відступи | `style: format code with rustfmt` |
| `refactor` | Рефакторинг без зміни функціональності | `refactor(raid): simplify replication logic` |
| `perf` | Покращення продуктивності | `perf(cache): optimize LRU eviction` |
| `test` | Додавання/зміна тестів | `test(vm): add isolation integration tests` |
| `build` | Зміни в системі збірки | `build: update Cargo.toml dependencies` |
| `ci` | Зміни в CI/CD | `ci: add GitHub Actions workflow` |
| `chore` | Інші зміни (не код) | `chore: update .gitignore` |
| `revert` | Відкат попереднього коміту | `revert: revert "feat(ui): add new component"` |

### 3. Scope (Область)

Вказуємо модуль або компонент:
- `vm` - VM Module
- `ui` - UI Module
- `raid` - RAID Module
- `network` - Network Module
- `docs` - Documentation
- `scripts` - Scripts
- Без scope для загальних змін

### 4. Subject (Тема)

- **До 50 символів**
- **Починається з малої літери** (окрім абревіатур)
- **Без крапки в кінці**
- **У формі імперативу**: "add", "fix", "update", не "added", "fixed", "updated"

### 5. Body (Тіло)

- **Опціонально**, але рекомендовано для складних змін
- **Пояснює "що" та "чому"**, не "як"
- **Відділяється порожнім рядком від subject**
- **Кожен рядок до 72 символів**

### 6. Footer (Підвал)

- **Breaking changes**: `BREAKING CHANGE: <опис>`
- **Issue references**: `Closes #123`, `Fixes #456`

## ✅ Приклади правильних комітів

### Простий коміт
```
docs: add project structure visualization
```

### Коміт з scope
```
feat(vm): add loopback interface setup for network isolation
```

### Коміт з body
```
fix(ui): correct modal focus trap implementation

The previous implementation didn't properly trap focus when
opening modals. This fix ensures keyboard navigation works
correctly for accessibility compliance.

Closes #123
```

### Breaking change
```
feat(api): refactor authentication system

BREAKING CHANGE: JWT token format changed from v1 to v2.
All clients must update to the new token format.
```

### Multiple changes
```
feat(vm): add bind mounts and read-only mounts

- Add bind mounts for allowed_paths
- Add read-only mounts with MS_RDONLY flag
- Automatically create target directories
- Add integration tests for mount operations
```

## ❌ Приклади неправильних комітів

### Неправильно
```
Update README
```
**Проблеми**: Немає типу, неконкретний опис

### Неправильно
```
docs: Updated the README file with new information
```
**Проблеми**: "Updated" замість "update", зайве слово "file"

### Неправильно
```
fix: bug fix
```
**Проблеми**: Занадто загальний опис, незрозуміло що виправлено

## 🎯 Правила для Rust Architect

### 1. Атомарність комітів
- **Один коміт = одна логічна зміна**
- Не змішуємо різні типи змін (feat + fix)
- Розбиваємо великі зміни на менші коміти

### 2. Описовість
- **Чіткий опис що змінилося**
- **Пояснення чому** (якщо не очевидно)
- **Посилання на issues/PR** (якщо є)

### 3. Тести
- **Коміти з новою функціональністю повинні містити тести**
- **Коміти з виправленнями повинні містити тести що перевіряють виправлення**

### 4. Документація
- **Нова функціональність = оновлення документації**
- **Breaking changes = оновлення документації + migration guide**

## 📋 Checklist перед комітом

- [ ] Код компілюється без помилок (`cargo check`)
- [ ] Тести проходять (`cargo test`)
- [ ] Код відформатований (`cargo fmt`)
- [ ] Лінтер не знаходить проблем (`cargo clippy`)
- [ ] Документація оновлена (якщо потрібно)
- [ ] Commit message відповідає guidelines
- [ ] Зміни атомарні та логічні

## 🔧 Налаштування Git

### Git hooks (опціонально)

Створити `.git/hooks/commit-msg`:
```bash
#!/bin/sh
# Перевірка формату commit message
commit_regex='^(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)(\(.+\))?: .{1,50}'

if ! grep -qE "$commit_regex" "$1"; then
    echo "❌ Invalid commit message format!"
    echo "Format: <type>(<scope>): <subject>"
    exit 1
fi
```

## 📊 Статистика комітів

Для перевірки формату комітів:
```bash
git log --pretty=format:"%s" -30 | grep -E "^(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)"
```

## 🎯 Приклад workflow

```bash
# 1. Створити feature branch
git checkout -b feat/vm-network-isolation

# 2. Зробити зміни
# ... редагування коду ...

# 3. Перевірити
cargo check
cargo test
cargo fmt
cargo clippy

# 4. Додати зміни
git add src/vm/isolation/linux.rs
git add tests/vm_isolation_integration.rs

# 5. Закомітити з правильним форматом
git commit -m "feat(vm): add network interface configuration

- Add veth pairs setup
- Add macvlan support
- Add integration tests
- Update documentation

Closes #123"

# 6. Push
git push origin feat/vm-network-isolation
```

---

**Пам'ятайте**: Хороші commit messages допомагають розуміти історію проекту та спрощують code review! 🎯

