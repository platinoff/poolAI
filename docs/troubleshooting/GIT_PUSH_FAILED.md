# Гітпуш не проведено (git push failed)

**Термінал**: **тільки зовнішній MSYS2 UCRT64** (меню Пуск). Не використовуй термінал Cursor, PowerShell, cmd, do-push.cmd.

## Симптоми

- `git push` не виконується або падає.
- **CreateFileMapping** / Win32 error 5 — git із Cursor (термінал або cmd).
- **Команди обрізаються** («…added», кінець не видно), помилки `Get-ChildItem`.
- `index.lock`, "Permission denied" у `.git/objects`.

---

## Перевірка перед блоком

Якщо команди не йдуть — переконайся, що ти в **bash** і в **репо**:

```bash
which bash
pwd
cd /s/rust/poolAI
pwd
```

Якщо `cd /s/rust/poolAI` дає помилку, спробуй: `cd "S:/rust/poolAI"` і знову `pwd`. Далі — блок нижче.

---

## Один блок (без `\`, по одній команді якщо треба)

Виконуй **тільки у зовнішньому MSYS2 UCRT64** (не в Cursor). Закрий Source Control. Копіюй по команді або блок цілком.

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI
rm -f .git/index.lock
cargo fmt --all
git add Cargo.toml src/ tests/ scripts/
git add -f docs/CHANGELOG.md docs/cloud/CLOUD_SDK_STATUS.md docs/concept/poolAI_concept_root.txt docs/development/NEXT_STEPS_2026-01-19.md docs/status/STABLE_STATE_SUMMARY.md docs/troubleshooting/GIT_PUSH_FAILED.md
git add -f .cursor/rules/ .cursor/commands/
git status -sb
git commit -m "feat(ml): ML.1 profiling, tuning, quantization; git без .sh; rules, docs"
git push origin main
```

Якщо шлях інший — замість `cd /s/rust/poolAI` використай `cd "S:/rust/poolAI"` (або свій шлях).

---

## Тільки push

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI
git push origin main
```

---

## Якщо падає

- **CreateFileMapping / Permission denied / обрізаний вивід**: не запускай git з Cursor. Тільки **зовнішній** MSYS2.
- **Команди не відпрацьовують**: див. "Перевірка перед блоком"; виконуй **по одній** команді.
- **index.lock / Permission denied**: закрий Source Control, запускай у **зовнішньому** MSYS2.
- **nothing to commit**: вже закомічено → лише `git push origin main`.
- **Push "failed to connect"**: мережа, VPN, `git config http.proxy`; або SSH remote.
- **Auth failed**: HTTPS → PAT; SSH → `git remote set-url origin git@github.com:USER/poolAI.git`.

Перевір незапушене: `git status -sb` і `git log origin/main..HEAD --oneline`. Див. `.cursor/commands/git-push.md`.
