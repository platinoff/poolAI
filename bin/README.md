# Bin – git, cargo, тести (без PowerShell)

Скрипти для швидкого запуску **git**, **cargo**, **тестів**. Використовуй **bash** (MSYS2, Git Bash, WSL).

## Як запускати

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
| `git-status.sh` | `git status --short`, `git log -5`, поточна гілка |
| `cargo-check.sh` | `cargo check --no-default-features --lib` |
| `cargo-test.sh` | `cargo test --lib`; з аргументом `raid` — raid_cross + raid_smallworld |
| `cargo-fmt.sh` | `cargo fmt --all` |
| `run-lan-nodes.sh` | 2+ PoolAI вузли на одному хості (FM-003 dev stand; `POOLAI_HTTP_PORT`) |
| `run-lan-nodes.ps1` | Те саме для Windows PowerShell |
| `e2e-playwright.sh` | Playwright smoke: login → `/ui/admin/users` (S23; див. `docs/development/E2E_PLAYWRIGHT.md`) |
| `pa11y-ci.sh` | pa11y WCAG 2.2 (FM-019; `ADMIN_A11Y_RUNBOOK.md`) |

Детальніша інформація: `docs/status/STABLE_STATE_SUMMARY.md`, `docs/performance/LAN_BENCHMARK_RUNBOOK.md` §5.
