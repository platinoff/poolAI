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
| Workers / Libraries / VM / RAID | `/ui/workers`, `/ui/libraries`, `/ui/vm`, `/ui/raid` | Create/Install modals: Tab у діалозі; Esc; closed `aria-modal="false"` |

**Критерій pass:** жоден Tab не «втикає» фокус під overlay; активна вкладка оголошується візуально + через `aria-selected`.

---

## 3. Опційно: pa11y (локально)

Потрібен запущений UI на `http://127.0.0.1:8080` (порт за вашим конфігом).

```bash
npx pa11y http://127.0.0.1:8080/ui/admin/users
npx pa11y http://127.0.0.1:8080/ui/admin/security
```

Поріг для baseline-спринту: **0 critical** на цих двох URL; warnings — у backlog FM-019 (повний WCAG).

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
- `pa11y` / axe у CI (окремий FM або розширення FM-019).
- Оновлення застарілих `[ ]` у [`UI_IMPROVEMENTS_PLAN.md`](../UI_IMPROVEMENTS_PLAN.md) (історичний чеклист).

**Last updated:** 2026-05-18 — FM-019 dashboard modals partial (AUTO_RUN 2026-06-09).
