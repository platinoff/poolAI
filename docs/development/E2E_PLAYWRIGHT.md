# Playwright E2E smoke (S23 / S27 / S29 / PH-S11)

**Rust-first policy:** API/wire acceptance — **`tests/*_integration.rs` + `cargo test-ci`** (канон). Playwright — **лише browser/UI** (DOM, axe, visual, admin flows). Див. [`.cursor/rules/poolai-testing-policy.mdc`](../../.cursor/rules/poolai-testing-policy.mdc). **PH-S144 ✅:** legacy API-smoke specs archived у [`e2e/archive/api-smoke/`](../e2e/archive/api-smoke/README.md); канон — Rust integration tests.

**Status:** Smoke + admin (повний P1 surface) + axe + **visual regression** (`smoke.spec.ts`, `admin.spec.ts`, `a11y.spec.ts`, `visual.spec.ts`).

| Spec | Сценарії |
|------|----------|
| `smoke.spec.ts` | login → `/ui` → `/ui/admin/users` (`#users-list`) |
| `admin.spec.ts` | tenants; monitoring; security (**PH-S27** rotation tab + OAuth2); audit; raid; topology; workers; **jobs (PH-S53)**; vm (+ **PH-S03** create/delete); **libs**; **PH-S23** dashboard, users (+ modal), config tabs, instances list, topology refresh |
| `a11y.spec.ts` | axe: `/ui/login`, `/ui/admin/users` (critical/serious = 0); **PH-S14:** high-contrast `color-contrast` on login + admin |
| `visual.spec.ts` | **PH-S11:** login + 10 admin routes; **PH-S12:** theme × i18n matrix (+12); **PH-S13:** topology masked SVG (`topology.png`); див. [`VISUAL_REGRESSION_E2E.md`](./VISUAL_REGRESSION_E2E.md) |
| `jobs_raid.spec.ts` | **PH-S52:** legacy Playwright stand-restart (archived pattern); **PH-S156 ✅:** канон — `poolai-http-stand-smoke --raid-restart` (+ `bin/e2e-playwright.sh --start`); wire — `tests/job_store_raid_persistence` |

**Rust integration (PH-S144, замість archived Playwright API specs):** `tests/jobs_api_contracts.rs`, `tests/grid_pricing_integration.rs`, `tests/grid_envelope_lease_integration.rs`, `tests/protocol_middleware_integration.rs`, `tests/virtual_node_telegram_binding_integration.rs`.

Спільний логін: `e2e/tests/helpers.ts` (`loginAsAdmin`).

## Локально (MSYS2 / Linux)

Потрібні: Node.js 20+, `npm`, зібраний `poolai` (`enterprise` features).

```bash
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI

# Варіант A: poolai вже на :8080
bash bin/e2e-playwright.sh

# Варіант B: збірка + старт + тести
bash bin/e2e-playwright.sh --start
# GitHub Actions (CI=true): debug, `CARGO_BUILD_JOBS=1`, features без `ml`; `npm run test:ci` (smoke+admin+a11y+visual) + Rust `poolai-http-stand-smoke --raid-restart` (PH-S156).
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
- **PH-S45 ✅** — E2E stability: vm create via UI button + POST/DELETE wait (`admin.spec.ts`); axe `/ui/admin/audit` settle (`helpers.ts`); viewport 1920 for visual snapshots

**PH-S52 ✅:** `jobs_raid` у `npm run test:ci` (PH-S156: замінено на Rust stand smoke).

**PH-S156 ✅:** `jobs_raid` прибрано з `test:ci`; `bin/e2e-playwright.sh --start` запускає `poolai-http-stand-smoke --raid-restart` після browser gate.

**PH-S86 ✅ (archived):** `grid_pricing.spec.ts` → `e2e/archive/api-smoke/`; канон — `tests/grid_pricing_integration.rs` (PH-S144).

**PH-S107 ✅ (archived):** `jobs_lease.spec.ts` → `e2e/archive/api-smoke/`; канон — `tests/jobs_api_contracts.rs` (PH-S144).

**PH-S148 ✅:** `npm run test:ci` — лише `smoke admin a11y visual` (browser/UI); API-only TS patterns не повертаються з archive.

**Last updated:** 2026-06-14 (PH-S156 Rust stand smoke `--raid-restart`).
