# Bin – git, cargo, запуск, тести

Скрипти для **запуску проєкту**, **git**, **cargo**, **тестів**. Bash — MSYS2 / Git Bash / WSL; PowerShell — `*.ps1`.

## Запуск усього проєкту (канон)

Детально: [`docs/development/RUN_LOCAL.md`](../docs/development/RUN_LOCAL.md).

```bash
bash bin/run-poolai.sh single          # 1 вузол, UI + Admin :8080
bash bin/run-poolai.sh virtual-node    # coordinator + worker
bash bin/run-poolai.sh lan             # 2+ вузли (FM-003)
bash bin/run-poolai.sh single --bg     # у фоні
bash bin/run-poolai.sh stop
bash bin/run-poolai.sh status
```

PowerShell: `.\bin\run-poolai.ps1 single`

## Git / cargo / тести

З кореня проєкту `poolAI`:

```bash
bash bin/git-status.sh
bash bin/cargo-check.sh
bash bin/cargo-test.sh
bash bin/cargo-test.sh raid
bash bin/cargo-fmt.sh
```

На Windows з MSYS2 скрипти самі додають `ucrt64` до `PATH` і переключають toolchain на GNU.

## Що робить кожен скрипт

| Скрипт | Дія |
|--------|-----|
| **`run-poolai.sh`** | **Єдиний лаунчер:** single / lan / virtual-node / docker / build / stop / status |
| `run-poolai.ps1` | Те саме для PowerShell |
| `git-status.sh` | `git status --short`, `git log -5`, поточна гілка |
| `cargo-check.sh` | `cargo check --no-default-features --lib` |
| `cargo-test.sh` | `cargo test --lib`; з аргументом `raid` — raid_cross + raid_smallworld |
| `cargo-fmt.sh` | `cargo fmt --all` |
| `run-lan-nodes.sh` | 2+ PoolAI вузли на одному хості (FM-003 dev stand; `POOLAI_HTTP_PORT`) |
| `run-lan-nodes.ps1` | Те саме для Windows PowerShell |
| `e2e-playwright.sh` | Playwright smoke: login → `/ui/admin/users` (S23; див. `docs/development/E2E_PLAYWRIGHT.md`) |
| `pa11y-ci.sh` | pa11y WCAG 2.2 (FM-019; `ADMIN_A11Y_RUNBOOK.md`) |

Детальніша інформація: `docs/status/STABLE_STATE_SUMMARY.md`, `docs/performance/LAN_BENCHMARK_RUNBOOK.md` §5.
