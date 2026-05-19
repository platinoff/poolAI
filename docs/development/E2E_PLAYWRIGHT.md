# Playwright E2E smoke (S23 / S27 / S29)

**Status:** Smoke + admin (повний P1 surface) + axe (`smoke.spec.ts`, `admin.spec.ts`, `a11y.spec.ts`).

| Spec | Сценарії |
|------|----------|
| `smoke.spec.ts` | login → `/ui` → `/ui/admin/users` (`#users-list`) |
| `admin.spec.ts` | tenants; monitoring; security; audit; raid; topology; workers; vm; **libs** (`#libraries-list`) |
| `a11y.spec.ts` | axe: `/ui/login`, `/ui/admin/users` (critical/serious = 0) |

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
```

**Env:** `POOLAI_HTTP_PORT`, `POOLAI_BASE_URL`, `POOLAI_E2E_USER`, `POOLAI_E2E_PASSWORD` (dev defaults `admin` / `admin123` — `user_manager.rs`).

## CI

[`.github/workflows/e2e.yml`](../../.github/workflows/e2e.yml) — **`workflow_dispatch`** only (не блокує merge на `main`; pa11y залишається каноном a11y).

Рекомендовані required checks на `main`: `Pa11y script contract`, `Pa11y WCAG 2.2` (при зміні UI) — див. [`ADMIN_A11Y_RUNBOOK.md`](./ADMIN_A11Y_RUNBOOK.md) §3.2.

## Розширення backlog

- axe Playwright (FM-019 backlog).
- Підключити `workflow_call` з `ci.yml` після стабілізації часу прогону.
- ~~raid, topology, vm, workers~~ **✅ S31/S33**
- ~~axe Playwright~~ **✅ S33** (`@axe-core/playwright`)
- ~~libs admin~~ **✅ S34**

**Last updated:** 2026-05-19 (S34 — admin E2E surface complete).
