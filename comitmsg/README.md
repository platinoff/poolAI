# Commit message drafts (`comitmsg/`)

Локальні чернетки повідомлень для `git commit-tree` / `bin/amend-head-msg.sh` (обхід commit-msg hook, коли subject лишається `Co-authored-by:`).

| Патерн файлу | Приклад |
|--------------|---------|
| `.commit-msg-ph-sNN.txt` | `comitmsg/.commit-msg-ph-s118.txt` |
| `.commit-msg-head*.txt` | handoff / docs-sync |
| `.commit-msg-docs-*.txt` | лише доки |

**Не комітити** вміст `comitmsg/*.txt` (див. `.gitignore`). У git лишається лише цей README.

**MSYS2:**

```bash
bash bin/amend-head-msg.sh comitmsg/.commit-msg-ph-s118.txt
# або (basename → comitmsg/):
bash bin/amend-head-msg.sh .commit-msg-ph-s118.txt
```

**Тимчасовий subject** (поза `comitmsg/`): `.git/COMMIT_MSG_TMP.txt`.
