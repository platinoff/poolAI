# Playwright E2E smoke (S23 / S27 / S29)

**Status:** Smoke + admin tenants/monitoring/security/audit/raid/topology (`e2e/tests/smoke.spec.ts`, `e2e/tests/admin.spec.ts`).

| Spec | Сценарії |
|------|----------|
| `smoke.spec.ts` | login → `/ui` → `/ui/admin/users` (`#users-list`) |
| `admin.spec.ts` | tenants (`#tenants-list`); monitoring (`#monitoring-content`); security (`#security-content`, OAuth2 tab); audit (`#audit-events`); raid (`#raid-admin`, `#raid-artifacts`); topology (`#topology-node-count`, `#topology-nodes-list`) |

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
- ~~raid, topology~~ **✅ S31** (`admin.spec.ts`)
- Додаткові admin routes (vm, workers, libs) — за потреби.

**Last updated:** 2026-05-19 (S31).
