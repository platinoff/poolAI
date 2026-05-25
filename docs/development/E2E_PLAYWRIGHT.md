# Playwright E2E smoke (S23 / S27 / S29 / PH-S11)

**Status:** Smoke + admin (повний P1 surface) + axe + **visual regression** (`smoke.spec.ts`, `admin.spec.ts`, `a11y.spec.ts`, `visual.spec.ts`).

| Spec | Сценарії |
|------|----------|
| `smoke.spec.ts` | login → `/ui` → `/ui/admin/users` (`#users-list`) |
| `admin.spec.ts` | tenants; monitoring; security (**PH-S27** rotation tab + OAuth2); audit; raid; topology; workers; vm (+ **PH-S03** create/delete); **libs**; **PH-S23** dashboard, users (+ modal), config tabs, instances list, topology refresh |
| `a11y.spec.ts` | axe: `/ui/login`, `/ui/admin/users` (critical/serious = 0); **PH-S14:** high-contrast `color-contrast` on login + admin |
| `visual.spec.ts` | **PH-S11:** login + 10 admin routes; **PH-S12:** theme × i18n matrix (+12); **PH-S13:** topology masked SVG (`topology.png`); див. [`VISUAL_REGRESSION_E2E.md`](./VISUAL_REGRESSION_E2E.md) |

Спільний логін: `e2e/tests/helpers.ts` (`loginAsAdmin`).

## Локально (MSYS2 / Linux)

Потрібні: Node.js 20+, `npm`, зібраний `poolai` (`enterprise` features).

```bash
export PATH="$HOME/.cargo/bin:/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI

# Варіант A: poolai вже на :8080
bash bin/e2e-playwright.sh

# Варіант B: збірка + старт + тести
bash bin/e2e-playwright.sh --start
# GitHub Actions (CI=true): debug, `CARGO_BUILD_JOBS=1`, features без `ml`; `npm run test:ci` (smoke+admin+a11y+visual, PH-S44 gate).
# Windows + AV: виключення для `target/` і `~/.cargo`; локально `export CARGO_BUILD_JOBS=1` перед `--start`.

# PH-S11: оновити visual baselines
bash bin/e2e-playwright.sh --start --update-snapshots
```

**Env:** `POOLAI_HTTP_PORT`, `POOLAI_BASE_URL`, `POOLAI_E2E_USER`, `POOLAI_E2E_PASSWORD` (dev defaults `admin` / `admin123` — `user_manager.rs`).

## CI

| Workflow | Тригер | Job у `ci.yml` |
|----------|--------|----------------|
| [`.github/workflows/e2e.yml`](../../.github/workflows/e2e.yml) | `workflow_call` + `workflow_dispatch` | **Playwright admin E2E** (paths-filter: `ui` або `e2e`) |
| [`.github/workflows/a11y.yml`](../../.github/workflows/a11y.yml) | `workflow_call` + `workflow_dispatch` | **Pa11y WCAG 2.2** (paths-filter: `ui`) |

**Paths-filter (`ci.yml` → `ui-changes`):** `e2e/**`, `bin/e2e-playwright.sh`, `src/ui/**`, `.github/workflows/e2e.yml`. Завжди на PR/push: **Pa11y script contract** (Rust-тест скрипта).

Рекомендовані required checks на `main`: `Pa11y script contract`, `Pa11y WCAG 2.2` (при зміні UI), **Playwright admin E2E** (при зміні UI/e2e) — див. [`ADMIN_A11Y_RUNBOOK.md`](./ADMIN_A11Y_RUNBOOK.md) §3.2.

## Розширення backlog

- ~~`workflow_call` з `ci.yml`~~ **✅ FM-039** (2026-05-23)
- ~~raid, topology, vm, workers~~ **✅ S31/S33**
- ~~axe Playwright~~ **✅ S33** (`@axe-core/playwright`)
- ~~libs admin~~ **✅ S34**
- ~~visual regression (Playwright snapshots)~~ **✅ PH-S11** — [`VISUAL_REGRESSION_E2E.md`](./VISUAL_REGRESSION_E2E.md)
- ~~theme/i18n visual matrix~~ **✅ PH-S12**
- ~~topology masked visual~~ **✅ PH-S13**
- **PH-S37 ✅** — Linux visual baselines workflow (`update-visual-baselines.yml`); refresh on-demand via Actions dispatch
- **PH-S44 ✅** — required CI gate: `test:ci` includes visual + axe on UI/e2e path changes (`ci.yml` paths-filter)
- **PH-S45** — E2E stability: vm create modal (`admin.spec.ts`); axe `/ui/admin/audit` (`a11y.spec.ts`)

**Last updated:** 2026-05-25 (PH-S44 visual in test:ci; PH-S37 workflow on-demand).
