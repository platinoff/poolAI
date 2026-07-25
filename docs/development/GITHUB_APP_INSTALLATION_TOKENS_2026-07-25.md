# GitHub App installation tokens — PoolAI adaptation (2026-07-25)

**Джерело:** [GitHub Changelog — Per-request override header (2026-05-15)](https://github.blog/changelog/2026-05-15-github-app-installation-tokens-per-request-override-header/)  
**Service band:** FM §5.16 **PH-SVC65…SVC74**  
**Cursor / VDT:** правила + agent permissions + CI hygiene (не product PH-S* drain)

---

## 1. Що змінюється

GitHub ролає **новий формат** installation access tokens (server-to-server), включно з **Actions `GITHUB_TOKEN`**:

| Формат | Ознаки |
|--------|--------|
| **Stateful (classic)** | короткий opaque `ghs_…`, **без** крапок |
| **Stateless (JWT)** | `ghs_`-prefixed JWT, **~520** символів, **дві** крапки після префікса |

Тимчасовий override на `POST /app/installations/:installation_id/access_tokens`:

| Header | Значення | Ефект |
|--------|----------|--------|
| `X-GitHub-Stateless-S2S-Token` | `enabled` | форсувати JWT |
| | `disabled` | форсувати classic opaque |
| | absent / інше | rollout за замовчуванням |

Рекомендований regex (обидва формати): `ghs_[A-Za-z0-9\.\-_]{36,}`  
Header **тимчасовий** — після deprecation усі eligible apps отримують stateless. **GitHub Enterprise Server** не зачіпається.

---

## 2. Impact на PoolAI

| Шар | Чи зачіпає | Висновок |
|-----|------------|----------|
| Product Rust (`src/`, App installation mint) | Ні | Немає викликів `POST …/access_tokens` |
| Local `git push` / `абракадабра` MSYS2 | Ні | User SSH/PAT — не App installation token |
| GitHub Actions `GITHUB_TOKEN` / `github.token` | **Так (rollout)** | `ci.yml`, `docs.yml`, `release.yml`, `update-visual-baselines.yml` |
| Cursor agents / rules | Так (гігієна) | Не валідувати довжину; не логувати `ghs_*` |
| Майбутній OAuth / GitHub App (UI plan) | Горизонт | Зберігати opaque; колонки ≥520; regex вище |

**Third-party Actions у репо** (передають token як opaque string):

- `rustsec/audit-check` ← `secrets.GITHUB_TOKEN`
- `peaceiris/actions-gh-pages` ← `secrets.GITHUB_TOKEN`
- `softprops/action-gh-release` ← `GITHUB_TOKEN`
- `gh pr create` у visual-baselines ← `GH_TOKEN: ${{ github.token }}`

Очікування: ці actions уже трактують token як opaque. Якщо після rollout з’являться auth fail у PH-SVC34 — перевірити версії actions / issues upstream, **не** додавати локальний length-check.

---

## 3. Адаптації (зроблені в PH-SVC65…74)

| # | Артефакт | Зміна |
|---|----------|--------|
| 1 | цей research | канон + impact |
| 2 | [`SECRETS_MANAGEMENT.md`](../security/SECRETS_MANAGEMENT.md) §5 | opaque `ghs_*`, regex, ≥520 |
| 3 | `cursor-environment-baseline.mdc` | ops note Actions token format |
| 4 | `poolai-agent-roles.mdc` + `poolai-session-iteration.mdc` | hygiene + `абракадабра` CI |
| 5 | `ci-scripts-maintenance.mdc` | no hardcoded token length |
| 6 | `.cursor/permissions.json` | block print/log `GITHUB_TOKEN` / `ghs_` |
| 7 | HANDOFF / NEXT / README / INDEX / ENV / CHANGELOG / `file_list.csv` | service zriz |
| 8 | `poolai-vision-sync --check` | drift gate (rev **394**) |

**Не робити в product drain:**

- Не додавати `X-GitHub-Stateless-S2S-Token` у PoolAI runtime (немає App mint).
- Не парсити JWT payload installation tokens у CI/scripts.
- Не комітити / не друкувати `GITHUB_TOKEN` у логах агента.

---

## 4. Чекліст для агентів (`абракадабра` / service)

1. Treat `GITHUB_TOKEN` / `ghs_*` як **opaque** (не length, не «має бути без крапок»).
2. Якщо колись з’явиться mint installation token — тестити `enabled` і `disabled`, потім **прибрати** override header.
3. Storage (якщо з’явиться) — ≥ **520** символів.
4. Після push — PH-SVC34: дивитись Auth fail у jobs, що передають token у third-party actions.
5. Auto-review лишається каноном; Cloud Agents ≠ Run Modes (без змін).

---

## 5. Service band PH-SVC65…SVC74

| Sprint | Focus | Статус |
|--------|--------|--------|
| **PH-SVC65** | Research (цей файл) | ✅ |
| **PH-SVC66** | SECRETS_MANAGEMENT §5 | ✅ |
| **PH-SVC67** | Rules: baseline + agent-roles + session-iteration | ✅ |
| **PH-SVC68** | `ci-scripts-maintenance.mdc` | ✅ |
| **PH-SVC69** | `.cursor/permissions.json` | ✅ |
| **PH-SVC70** | HANDOFF + NEXT_SESSION | ✅ |
| **PH-SVC71** | README / INDEX / ENV pointer | ✅ |
| **PH-SVC72** | `file_list.csv` + `.cursor/CHANGELOG` | ✅ |
| **PH-SVC73** | `poolai-vision-sync --check` | ✅ |
| **PH-SVC74** | git push + самарі | ✅ |

**Наступна product-сесія:** **`абракадабра`** → band 82 (PH-S1459…S1468). Відкриті: PH-SVC34 · PH-SVC35 (OWNER).
