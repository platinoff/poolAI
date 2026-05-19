# Admin & dashboard a11y verification (FM-019)

**Status:** Baseline implemented in code (2026-06-06). **Канон зрізу:** [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) **§5.4**.

Повний WCAG 2.2 AA з автоматичним `pa11y` у CI — **не в обсязі** поточного baseline; нижче — регресійні перевірки після змін у `src/ui/` або admin HTML.

---

## 1. Автоматичні (обов’язково перед push)

MSYS2 UCRT64, з кореня репо:

```bash
export PATH="$HOME/.cargo/bin:/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI
cargo fmt --all
cargo test-ci
cargo test -p poolai --features enterprise ui::admin --lib
```

Очікування: усі тести `ui::admin::*` (modals, forms, tablist) — **ok**.

---

## 2. Ручна клавіатурна перевірка (admin)

Сервер: `cargo run --features enterprise` (або ваш звичний профіль). Увійти як Admin.

| Сторінка | URL | Перевірка |
|----------|-----|-----------|
| Users | `/ui/admin/users` | Tab лишається в модалці; Esc закриває; Create/Edit modals |
| Security | `/ui/admin/security` | Tab між OAuth2/SAML/Policies (`aria-selected`); modals OAuth2/SAML/Policy |
| Config | `/ui/admin/config` | Tab General…Health; фокус на панелі |
| Dashboard | `/ui` | Skip links; Ctrl+K search; Esc закриває modals |
| Workers / Libraries / VM / RAID | `/ui/workers`, `/ui/libs`, `/ui/vm`, `/ui/raid` | Create/Install modals: Tab у діалозі; Esc; closed `aria-modal="false"` |

**Критерій pass:** жоден Tab не «втикає» фокус під overlay; активна вкладка оголошується візуально + через `aria-selected`.

---

## 3. pa11y (локально + CI)

**Скрипт:** `bin/pa11y-ci.sh` — strict: `/ui/login`; з `PA11Y_ADMIN_STRICT=1` (CI default) — 9 auth URLs після login actions (`admin` / `admin123`, див. `DEFAULT_DEV_ADMIN_*` у `user_manager.rs`). Див. §3.1.

### 3.1 Матриця URL (strict / planned)

| URL | Режим | Стан у `pa11y-ci.sh` | Примітка |
|-----|--------|----------------------|----------|
| `/ui/login` | unauthenticated | **strict** | `STRICT_URLS` |
| `/ui/admin/users` | auth actions | **strict** (`PA11Y_ADMIN_STRICT=1`) | `ADMIN_URLS` |
| `/ui/admin/security` | auth actions | **strict** | tablist OAuth2/SAML |
| `/ui/workers` | auth actions | **strict** | dashboard modal slice |
| `/ui` | auth actions | **strict** | dashboard shell |
| `/ui/admin/config` | auth actions | **strict** | tablist General…Health |
| `/ui/libs` | auth actions | **strict** | libraries dashboard (маршрут `/ui/libs`, не `/ui/libraries`) |
| `/ui/vm` | auth actions | **strict** | VM instances |
| `/ui/raid` | auth actions | **strict** | RAID artifacts table |

**Зріз 2026-05-18 (S6):** `PA11Y_ADMIN_STRICT=1 bash bin/pa11y-ci.sh --start` — **0 errors** на login + 9 auth URLs.

```bash
# MSYS2: poolai вже на :8080
bash bin/pa11y-ci.sh

# strict admin (login fixture через pa11y actions)
PA11Y_ADMIN_STRICT=1 bash bin/pa11y-ci.sh

# або зібрати + підняти poolai, потім scan
bash bin/pa11y-ci.sh --start
```

**CI:** [`.github/workflows/a11y.yml`](../../.github/workflows/a11y.yml) — `workflow_dispatch`; `PA11Y_ADMIN_STRICT=1`.

Ручний одиночний URL:

```bash
npx pa11y http://127.0.0.1:8080/ui/admin/users --runner axe
```

Поріг strict URL: **0 errors** (`PA11Y_THRESHOLD`, default `0`). Admin без сесії може давати додаткові findings — optional крок у скрипті.

---

## 4. Що вже в коді (не дублювати в спринтах)

| Область | Файли |
|---------|--------|
| Modals (trap, Esc, `aria-modal`) | `src/ui/admin_common.js` |
| Forms (`aria-required`, labels) | `admin_common.js`, `admin/users.rs`, … |
| Tablist ARIA | `admin/security.rs`, `admin/config.rs`, `adminSyncTabA11y` |
| Dynamic tables | `adminEnhanceTablesA11y`, `adminObserveDynamicA11y` |
| Dashboard nav / search | `src/ui/mod.rs` (`dashMarkCurrentNav`, Ctrl+K) |
| Skip links / live regions | `admin/mod.rs`, FM-018 |

---

## 5. Backlog (поза baseline)

- ~~Повний прохід dashboard modals (workers, libs, VM, RAID)~~ — **Partial ✅ 2026-05-18** (`src/ui/mod.rs`, `ui::dashboard_a11y_tests`).
- ~~`pa11y` / axe у CI~~ — **Partial ✅** `a11y.yml` + `bin/pa11y-ci.sh` (login + `PA11Y_ADMIN_STRICT` auth actions).
- ~~Оновлення застарілих `[ ]` у [`UI_IMPROVEMENTS_PLAN.md`](../UI_IMPROVEMENTS_PLAN.md)~~ — **Archived ✅ 2026-05-18** (S4 docs; канон §5.4 + цей runbook).
- ~~Розширити `ADMIN_URLS`: `/ui`, `/ui/admin/config`~~ — **Partial ✅ 2026-05-18** (S5; 6 auth URLs strict).

**Last updated:** 2026-05-18 — S6 pa11y: `/ui/libs`, `/ui/vm`, `/ui/raid`; dark theme `--danger` sync; 0 errors.
