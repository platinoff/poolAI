# Git Push (MSYS2 Bash, без .sh)

Add, commit, push — **тільки команди в MSYS2 bash**. Без скриптів .sh, без PowerShell, без cmd.

## ⚠️ Критично: тільки зовнішній MSYS2

**Не запускай git із терміналу Cursor.** З’являються:
- `CreateFileMapping` / Win32 error 5
- `index.lock`, `Permission denied` у `.git/objects`
- Обрізаний або некоректний вивід інтегрованого терміналу IDE

**Рішення:** відкривай **MSYS2 UCRT64** з меню Пуск (окреме вікно) і виконуй команди там.

## 1. Відкрий MSYS2 bash

- **MSYS2 UCRT64** з меню Пуск — **обов’язково зовнішнє вікно** (не термінал Cursor).

Закрий **Source Control** у Cursor перед git.

## 2. Перевірка (якщо команди не відпрацьовують)

Переконайся, що ти в **bash** і в **каталозі репо**:

```bash
which bash
pwd
```

Перейди в репо. Якщо диск **S:** — спробуй обидва варіанти:

```bash
cd /s/rust/poolAI
pwd
```

Якщо `cd /s/rust/poolAI` не працює:

```bash
cd "S:/rust/poolAI"
pwd
```

Має показати шлях до poolAI. Далі — блок з п.3.

## 3. Copy-paste блок (повторюваний цикл + Summary у коміті)

Виконуй **по одній команді** або копіюй блок цілом.

### Правило Summary (агент / Composer)

| У staged є зміни в… | Summary у тілі коміта |
|---------------------|------------------------|
| `src/`, `tests/`, `crates/`, `Cargo.toml`, `Cargo.lock`, `[[bin]]` / `[[test]]` | **Обов’язково** (п.3a): що змінено, які `cargo` команди, FM/scope якщо є |
| лише `docs/`, `.cursor/`, `README`, OpenAPI без коду | Рекомендовано короткий Summary (1–2 буллети) |
| нічого в `src/`/`tests/`/`crates/` | Subject достатньо, якщо лише docs/chore |

**Перед комітом:** `git diff --cached --stat` (або `git status -sb`) — якщо в diff є код → сформуй Summary з реального diff, не шаблон «заглушка».

**Агент/Composer:** subject + кілька рядків Summary в тілі коміта (див. п.3a); після push — самарі в чат (п.3b).

```bash
# rustup ПЕРШИЙ — інакше візьметься MSYS2 rustc 1.87 і cloud-sdk не збереться (потрібен rustc >= 1.88)
export PATH="$HOME/.cargo/bin:/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI
/c/msys64/usr/bin/rm -f .git/index.lock
git status -sb
cargo fmt --all
# За потреби перед push (зміни в коді/тестах), як у CI:
# cargo test --lib --tests --features ml,enterprise,cloud
# cargo clippy --all-targets --all-features
# cargo test --test cloud_mock_integration --features cloud,cloud-sdk -- --test-threads=1
git add README.md Cargo.toml src/ tests/ scripts/ file_list.csv
git add .gitignore
git add -f docs/CHANGELOG.md docs/cloud/CLOUD_SDK_STATUS.md docs/concept/poolAI_concept_root.txt docs/README.md docs/INDEX_2026-03-17.md docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md docs/ARCHITECTURE_BEST_PRACTICES.md docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md docs/development/HANDOFF_NEW_SESSION.md docs/development/README.md docs/development/NEXT_STEPS_2026-01-19.md docs/status/STABLE_STATE_SUMMARY.md docs/troubleshooting/GIT_PUSH_FAILED.md
git add -f .cursor/hooks.json .cursor/hooks/ .cursor/CHANGELOG.md .cursor/README.md .cursor/rules/ .cursor/commands/ .cursor/skills/
git status -sb
git commit -m "type(scope): subject" -m "Summary:" -m "- Зміни: (модулі/файли)" -m "- Перевірки: cargo fmt; (clippy/test — що саме)" -m "- Ризики/нотатки: не стаджити data/audit/*.log.gz; при os error 112 — cargo clean"
git push origin main
```

**Примітка**: Якщо `rm: command not found`, використай повний шлях `/c/msys64/usr/bin/rm` замість `rm`.

### 3a. Commit з розгорнутим Summary (обов’язково при зміні коду)

Якщо коміт містить зміни в **`src/`**, **`tests/`**, **`crates/`** або **`Cargo.toml`** — **не роби** `git commit` лише з одним рядком subject.

Кілька `-m` у Git — окремі абзаци в історії. Мінімум:

1. `type(scope): subject`
2. `Summary:` + буллети:
   - **Зміни:** модулі/файли або FM-id (напр. FM-029)
   - **Перевірки:** `cargo fmt`, `cargo test …`, `cargo test-ci` — що реально прогнано
   - **Нотатки:** feature flags, env, «не стаджити data/audit» — за потреби

Приклад:

```bash
git commit -m "feat(ml): extend pipeline step scheduling" \
  -m "Summary:" \
  -m "- Зміни: src/ml/pipeline.rs, tests/ml_pipeline_integration.rs" \
  -m "- Перевірки: cargo fmt --all; cargo test --test ml_pipeline_integration" \
  -m "- Нотатки: PATH з ~/.cargo/bin; target на S: при потребі cargo clean"
```

Не роби `git add -A` / `git add .` без потреби — легко підхопити `data/audit/*.log.gz`, `comitmsg/*.txt` або зайві артефакти.

### 3d. Hook лишає лише `Co-authored-by:`

Чернетки subject — у [`comitmsg/`](../../comitmsg/README.md) (не комітити `.txt` звідти). Після `git commit`:

```bash
export GIT_EDITOR=true
bash bin/amend-head-msg.sh comitmsg/.commit-msg-ph-sNN.txt
```

(`commit-tree` обходить commit-msg hook; `amend-head-msg` приймає basename і шукає файл у `comitmsg/`.)

### 3b. Короткий самарі після успішного push (для чату / PR)

**Обов’язково для агента**, якщо push включав зміни коду — надішли короткий самарі в чат (не лише «запушено»).

Скопіюй шаблон і заповни:

- **Branch**: `main`
- **Commit**: `<short-hash> type(scope): subject`
- **Зміни**: 1–3 речення
- **Тести**: що було прогнано
- **Known issues**: якщо є warnings / не проганяли повний suite

## 3c. Якщо не вистачає місця на диску (os error 112)

Спочатку `cargo clean` у репо, потім повтори збірку/тести. Audit-логи в `data/audit/` для звільнення місця зазвичай не чіпати (мізерний обсяг; див. security best practices у `docs/`).

Якщо `cd /s/rust/poolAI` не працює — замість неї використай `cd "S:/rust/poolAI"` (див. п.2).

## 4. Тільки push (add/commit уже зроблено)

```bash
export PATH="$HOME/.cargo/bin:/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI
git push origin main
```

## 5. Якщо падає

- **index.lock / Permission denied**: 
  - Закрий **Source Control** в Cursor
  - Видали `index.lock`: `/c/msys64/usr/bin/rm -f .git/index.lock`
  - Запускай у **зовнішньому** MSYS2 UCRT64 терміналі
- **rm: command not found**: 
  - Використай повний шлях: `/c/msys64/usr/bin/rm -f .git/index.lock`
  - Або налаштуй PATH: `export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"`
- **nothing to commit**: вже закомічено → тільки `git push origin main` (п.4).
- **Blocking waiting for file lock on build directory**: інший `cargo` тримає `target/` — закрий паралельні збірки або зачекай; не пуш поки не завершиться.
- **Push failed (auth/network)**: 
  - **HTTPS**: Створи Personal Access Token на GitHub, налаштуй `git config --global credential.helper store`, push знову (username + PAT як password)
  - **SSH**: `git remote set-url origin git@github.com:USER/poolAI.git`, додай SSH ключ до GitHub
  - Детальніше: `docs/troubleshooting/GIT_AUTH_FIX.md`
- **Команди не виконуються**: перевір п.2 (bash, шлях), виконуй команди **по одній**.

Детальніше: 
- `docs/troubleshooting/GIT_PUSH_FAILED.md` - загальні проблеми
- `docs/troubleshooting/GIT_INDEX_LOCK_FIX.md` - виправлення index.lock та rm
- `git-workflow.md` - git workflow правила
