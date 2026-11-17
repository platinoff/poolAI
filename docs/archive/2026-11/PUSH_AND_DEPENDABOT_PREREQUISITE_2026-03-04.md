# Git push (доадаптація) та обов'язкова умова: 6 Dependabot PR

**Дата**: 2026-03-04  
**Мета**: Запушити поточні зміни з самарі; перед продовженням розробки обробити 6 відкритих Dependabot PR.

---

## 1. Самарі змін (адаптація до стану та подальшої розробки)

- **Стабільний стан (docs)**  
  Оновлено `STABLE_STATE_SUMMARY.md`: Git статус (ahead of origin), актуальні документи (таблиця), наступні кроки P0–P2 (git → v0.3.0 на main → ML.1/ML.2/ML.3).

- **Наступні кроки (docs)**  
  Оновлено `NEXT_STEPS_2026-01-19.md`: пріоритети P0–P3, ML.4–ML.6 / Context Memory / Runtime library на main, план дій (git → тести/CHANGELOG → pruning, pipeline/aggregation).

- **Roadmap**  
  Оновлено `DEVELOPMENT_ROADMAP.md`: Next Steps під v0.2.2 done, P0 (git), P2 (v0.3.0 на main), P3 (далі); Version History 14.0.

- **Перевірка Cursor і кроки**  
  Оновлено `CURSOR_AND_NEXT_STEPS_VERIFICATION_2026-03-04.md`: секція наступних кроків узгоджена зі стабільним станом.

- **Правила чату**  
  Оновлено `.cursor/rules/chat-context.md` та `ai-assistant.md`: посилання на документ перевірки та наступні кроки.

- **Інші зміни в робочій копії**  
  Є й інші змінені файли (docs, src/ml, tests, scripts) — їх можна включити в цей коміт разом із доадаптацією або закомітити окремо пізніше.

---

## 2. Команди для Git push (виконувати у зовнішньому MSYS2 bash)

**Важливо**: Термінал — тільки **MSYS2 UCRT64** (`C:\msys64\usr\bin\bash.exe`). Закрити Source Control у Cursor перед push. Push виконувати у зовнішньому bash (не в терміналі Cursor).

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI

# Перевірка стану
git status --short
git log origin/main..HEAD --oneline

# Форматування (pre-push hook це перевіряє)
cargo fmt --all

# Додати всі змінені файли та закомітити одним комітом із самарі
git add -A
git commit -m "docs: adapt stable state and next steps for 2026-03-04

- STABLE_STATE_SUMMARY: git status (ahead of origin), doc refs, P0-P2 next steps
- NEXT_STEPS_2026-01-19: P0-P3, ML.4-6/Runtime on main, plan of action
- DEVELOPMENT_ROADMAP: Next Steps, Version 14.0
- CURSOR_AND_NEXT_STEPS_VERIFICATION: align section 3 with current state
- .cursor/rules: chat-context, ai-assistant refs to verification doc
- Add PUSH_AND_DEPENDABOT_PREREQUISITE_2026-03-04.md (push + 6 PR prerequisite)"

# Push (потрібен PAT або SSH у зовнішньому середовищі)
git push origin main
```

Якщо хочете закомітити тільки файли доадаптації docs (без решти 70+ файлів):

```bash
git add docs/status/STABLE_STATE_SUMMARY.md \
        docs/development/NEXT_STEPS_2026-01-19.md \
        docs/DEVELOPMENT_ROADMAP.md \
        docs/CURSOR_AND_NEXT_STEPS_VERIFICATION_2026-03-04.md \
        docs/PUSH_AND_DEPENDABOT_PREREQUISITE_2026-03-04.md
# Якщо .cursor відслідковується:
git add .cursor/rules/chat-context.md .cursor/rules/ai-assistant.md
git commit -m "docs: adapt stable state and next steps (2026-03-04)"
git push origin main
```

Після push перевірити на GitHub, що main оновився і (за бажанням) що CI проходить.

---

## 3. Перед подальшою розробкою: обробити 6 Dependabot PR

На репозиторії відкриті **6 Dependabot pull requests** (оновлення залежностей). Їх потрібно змерджити або закрити перед продовженням розробки, щоб main не роз’їжджався з залежностями.

| PR    | Опис |
|-------|------|
| **#47** | `chore(deps): bump nix from 0.30.1 to 0.31.1` |
| **#48** | `chore(deps): bump rusqlite from 0.32.1 to 0.38.0` |
| **#49** | `chore(deps): bump azure_core from 0.30.1 to 0.31.0` |
| **#50** | `chore(deps): bump azure_identity from 0.30.0 to 0.31.0` |
| **#51** | `chore(deps): bump windows-sys from 0.52.0 to 0.61.2` |
| **#55** | `chore(deps): bump the minor-and-patch group across 1 directory with 14 updates` |

**Рекомендований порядок обробки** (зменшує ризик конфліктів):

1. **#49 і #50** — Azure (azure_core, azure_identity) краще мерджити разом або спочатку #49, потім #50.
2. **#47** — nix (використовується в VM/isolation).
3. **#48** — rusqlite (major/minor jump 0.32 → 0.38 — перевірити changelog, можливі breaking changes).
4. **#51** — windows-sys.
5. **#55** — group bump (14 updates); мерджити останнім або після того, як main уже має #47–#51.

**Дії на GitHub**:

- Зайти в кожен PR → перевірити CI (якщо зелений) → Merge (squash або merge commit за правилами репо).
- Після мерджу: локально виконати `git fetch origin` та `git merge origin/main` (або `git pull origin main`) у MSYS2 bash, потім `cargo test` і `cargo build`.

**Альтернатива (все локально)**:

- Взяти зміни з кожного PR локально (`git fetch origin pull/47/head:pr47` тощо), змерджити в одну гілку, вирішити конфлікти, прогнати `cargo test`, потім push у main або в окрему гілку і мерджити через GitHub.

Після обробки всіх 6 PR можна продовжувати розробку (ML.1 pruning, ML.2/ML.3, v0.3.0 тощо) з актуальними залежностями.

---

**Підсумок**:  
1) Виконати push командами вище у MSYS2 bash.  
2) Перед подальшою розробкою обробити 6 Dependabot PR (#47–#51, #55) у вказаному порядку та перевірити збірку/тести локально після мерджу.
