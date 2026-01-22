# 🔄 Git Workflow для Rust Architect
## Дата: 2026-01-22

**Формат**: Conventional Commits  
**Terminal**: MSYS2 bash only

---

## 📝 Формат Комітів

### Conventional Commits

```
type(scope): subject

[optional body]

[optional footer]
```

### Типи:
- `feat` - нова функціональність
- `fix` - виправлення помилки
- `docs` - зміни в документації
- `chore` - рутинні завдання
- `refactor` - рефакторинг коду
- `test` - додавання/зміна тестів
- `style` - форматування коду

### Scope Приклади:
- `architect` - зміни в Rust Architect rules
- `docs` - документація
- `vm`, `ui`, `raid`, `network`, `cloud`, `ml` - модулі
- `scripts` - скрипти
- `concept` - концепція

### Приклади:
```
docs(architect): update rust-architect.md with current state (v0.2.2)

feat(ml): implement ML.2 AutoML pipeline

fix(git): resolve authentication issues

chore(docs): organize documentation structure
```

---

## 🚀 Git Push Workflow

### Повний Блок Команд (MSYS2 bash)

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI
/c/msys64/usr/bin/rm -f .git/index.lock
cargo fmt --all
cargo clippy --all-targets --all-features
cargo test --all-features
git add .
git status -sb
git commit -m "type(scope): subject

- Detailed change 1
- Detailed change 2"
git push origin main
```

### Тільки Push (якщо вже закомічено)

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI
git push origin main
```

---

## ⚠️ Troubleshooting

### Authentication Failed
Дивись: `docs/troubleshooting/GIT_AUTH_FIX.md`

### index.lock
Дивись: `docs/troubleshooting/GIT_INDEX_LOCK_FIX.md`

### rm: command not found
Використай повний шлях: `/c/msys64/usr/bin/rm`

---

**Детальніше**: 
- `.cursor/commands/git-push.md` - повний workflow
- `docs/troubleshooting/` - troubleshooting гайди
