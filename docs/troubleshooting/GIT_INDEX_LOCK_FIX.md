# Fix: Git index.lock та rm command not found в MSYS2

## Проблема

При виконанні git операцій в MSYS2 bash:
- `rm: command not found` - PATH не містить `/usr/bin`
- `fatal: Unable to create 'S:/rust/poolAI/.git/index.lock': File exists` - файл блокує git операції

## Рішення

### Крок 1: Закрити Source Control в Cursor

**ВАЖЛИВО**: Закрий **Source Control** панель в Cursor перед виконанням git команд.

### Крок 2: Налаштувати PATH правильно

В MSYS2 bash виконай:

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
```

Або використай повний шлях до `rm`:

```bash
/c/msys64/usr/bin/rm -f .git/index.lock
```

### Крок 3: Видалити index.lock

**Варіант 1**: Через повний шлях до rm:
```bash
/c/msys64/usr/bin/rm -f .git/index.lock
```

**Варіант 2**: Через PATH (після export):
```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
rm -f .git/index.lock
```

**Варіант 3**: Через Windows (якщо MSYS2 не працює):
```powershell
Remove-Item "S:\rust\poolAI\.git\index.lock" -Force -ErrorAction SilentlyContinue
```

### Крок 4: Виконати git операції

Після видалення index.lock:

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI
/c/msys64/usr/bin/rm -f .git/index.lock
cargo fmt --all
git add .
git status -sb
git commit -m "your commit message"
git push origin main
```

## Повний блок команд (copy-paste)

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI
/c/msys64/usr/bin/rm -f .git/index.lock
cargo fmt --all
git add .cursor/rules/rust-architect.mdc docs/status/RUST_ARCHITECT_UPDATE_2026-01-22.md
git add .cursor/rules/ai-assistant.mdc .cursor/rules/git-workflow.mdc .cursor/rules/msys2-windows.mdc .cursor/rules/scripts.mdc
git add Cargo.toml docs/CHANGELOG.md docs/cloud/CLOUD_SDK_STATUS.md docs/concept/poolAI_concept_root.txt
git add docs/development/NEXT_STEPS_2026-01-19.md docs/status/STABLE_STATE_SUMMARY.md scripts/README.md
git add src/cloud/providers/aws.rs src/lib.rs src/network/api/mod.rs src/network/enterprise_api.rs
git add tests/cloud_mock_integration.rs tests/integration/cloud/aws_tests.rs tests/integration/cloud/edge_cases_tests.rs tests/integration/mod.rs
git add src/ml/ src/network/api/ai_ml.rs
git status -sb
git commit -m "docs(architect): update rust-architect.mdc with current state (v0.2.2)"
git push origin main
```

## Профілактика

1. **Завжди закривай Source Control** в Cursor перед git операціями
2. **Використовуй зовнішній MSYS2 UCRT64** термінал (не термінал Cursor)
3. **Налаштуй PATH** на початку сесії:
   ```bash
   export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
   ```
4. **Перевіряй index.lock** перед git операціями:
   ```bash
   /c/msys64/usr/bin/rm -f .git/index.lock
   ```

## Детальніше

- [`git-push.md`](../../.cursor/commands/git-push.md) - Git workflow
- [`git-workflow.mdc`](../../.cursor/rules/git-workflow.mdc) - Git правила
- [`msys2-windows.mdc`](../../.cursor/rules/msys2-windows.mdc) - MSYS2 налаштування
