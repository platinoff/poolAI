# План наступної сесії (PoolAI) — 2026-07-01

**Попередній прогін:** OpenAPI S14–S20 ✅ (`a2749689`); enterprise REST задокументовано.

**Мета сесії:** закрити прогалини з **канонічних** доків (FM §5.1/§5.3, Architect, HANDOFF) — не дублювати архівні `[ ]` з `docs/archive/`.

---

## 1. Зведення: що ще НЕ зроблено (за документацією)

### 1.1 BLOCKED (не стартувати без інфраструктури)

| ID | Пункт | Джерело | Умова розблокування |
|----|--------|---------|---------------------|
| **FM-003 §4** | Реальний LAN sign-off: реплікація артефактів + TQ01 на двох вузлах | Architect L126, `LAN_BENCHMARK_RUNBOOK.md` §4, FM §5.3 | **2 фізичні хости** в одній LAN |
| — | Dev stand | ✅ `bin/verify-dev-stand.*`, `run-virtual-node-dev.*` | — |

### 1.2 Partial (є код/MVP, не закрито повністю)

| ID | Пункт | Що лишилось | Джерело |
|----|--------|-------------|---------|
| **FM-019** | WCAG 2.2 / pa11y | ~~merge gate~~ **S22 ✅** (`pa11y-contract` + `pa11y-wcag22` paths-filter); axe Playwright — backlog S23 | FM §5.3 |
| **OpenAPI** | Дрібні прогалини | ~~`/ai-ml/optimization*`, `automl`, `federated`~~ — **S21 ✅** (2026-05-19); повний `rg` enterprise — backlog за потреби | — |
| **UI_QUALITY P1** | API ↔ admin UI | Точкова звірка полів JS ↔ JSON для сторінок поза contract tests | `UI_QUALITY_AND_E2E_PLAN_2026-04-06.md` P1 |
| **FM-016 / ML** | Pipeline ops | Hardening за `PIPELINE_MANAGEMENT.md` (метрики кроків, стенд) — продуктовий пріоритет | DIGEST §ML, FM §5.1 |

### 1.3 Backlog (заплановано, коду немає або опційно)

| ID | Пункт | Джерело |
|----|--------|---------|
| **UI E2E** | Playwright smoke | **S23 ✅** `e2e/tests/smoke.spec.ts`, `bin/e2e-playwright.sh`, `e2e.yml` dispatch |
| **OpenAPI gap audit** | `rg '\.route\(' src/network` vs `docs/openapi.yaml` (v1 + enterprise) | AUTO_RUN S20 |
| **Dashboard DELETE** | ~~501~~ | **S24 ✅** `MonitoringManager.delete_dashboard()` + `DELETE /ui/dashboards/{id}` → 204 |
| **BurstRAID metrics** | Опційно v0.2+ | `RUST_ARCHITECT_STATUS` (stale, FM §5.3) |

### 1.4 Deferred (явно поза автопрогоном — лише за запитом користувача)

| ID | Пункт | Джерело |
|----|--------|---------|
| **FM-004** | SIMD / прискорений TurboQuant у Rust | Architect P2b, FM §5.1 |
| **FM-006** | Azure/GCP гілки під `cloud-sdk` (`TODO` у `azure.rs`/`gcp.rs`) | Architect L186, `CLOUD_SDK_STATUS.md`, FM §5.3 |

### 1.5 Concept-only (немає цільової реалізації в `src/`)

| ID | Пункт | Джерело |
|----|--------|---------|
| **FM-009** | Grid protocol wire envelope | `GRID_PROTOCOL_CONCEPT` |
| **FM-010** | Solana / on-chain адаптер | `SOLANA_ADAPTER_CONCEPT` |
| **P6** | Grid / Job / Memory layers | Architect §7, `docs/concept/` |

### 1.6 Код: відомі «не реалізовано» (не завжди в FM-таблиці)

| Область | Де | Примітка |
|---------|-----|----------|
| VM isolation (Windows) | `src/vm/isolation/windows.rs` | AppContainer/Firewall — validated stub |
| VM resources (Windows) | `src/vm/resources.rs` | CPU/memory limits post-spawn |
| Hardware VM isolation | `src/vm/mod.rs` | «not yet implemented» |
| Cloud SDK deep paths | `src/cloud/providers/azure.rs`, `gcp.rs` | FM-006 Deferred |
| Raft membership from log | `src/raid/raft.rs` | snapshot/log extraction |

### 1.7 Закрито / не повторювати (канон 2026-05–06)

- **FM-001–018**, FM-005 JSON errors, FM-007/008 distributed RAID, FM-011–016, FM-012 OAuth/Telegram ✅  
- **P4** `poolai_health_load` → `BENCHMARKS.md` ✅  
- **OpenAPI S14–S20** (v1 + `/api/enterprise/*` основний surface) ✅  
- **FM-019 baseline** (modals, tabs, skip links, 18 pa11y auth URLs) ✅  
- Архівні плани січень–квітень — див. `AUTO_RUN_SESSION_2026-06-23.md` (не канон черги)

---

## 2. Рекомендована черга спринтів (наступна сесія)

| Спринт | Фокус | Критерій готовності | Поза обсягом |
|--------|--------|---------------------|---------------|
| **S21** | OpenAPI gap audit | ✅ `ai-ml/optimization*`, `automl`, `federated` у yaml (2026-05-19) | — |
| **S22** | FM-019 CI | ✅ `pa11y-wcag22` + `pa11y-contract` у `ci.yml` (2026-05-19); runbook §3.2 | LAN |
| **S23** | Playwright (опційно) | ✅ smoke login → `/ui/admin/users` (2026-05-19); `E2E_PLAYWRIGHT.md` | FM-004/006/009/010 |
| **S24** | UI dashboard DELETE | ✅ `delete_dashboard` + HTTP 204 (2026-05-19) | cloud-sdk |
| **S25** | UI_QUALITY P1 | ✅ admin contracts: tenants, OAuth2, dashboards (+3 tests) | — |
| **—** | FM-003 §4 | Лише при 2 хостах | — |

**Після кожного спринту з кодом/доками:** `cargo fmt` → `cargo test-ci` (MSYS2) → `git -c commit.template= commit -F …` → push (зовнішній MSYS2).

---

## 3. Команди звірки (P0 менеджера функціоналу)

```bash
# Відкриті чекбокси Architect
rg "\- \[ \]" docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md

# Маршрути vs OpenAPI
rg '\.route\(' src/network/api/ai_ml.rs
rg '^  /ai-ml' docs/openapi.yaml

# TODO у коді (вибірково)
rg "TODO|not yet implemented|NOT_IMPLEMENTED" src/network src/ui src/vm --glob '*.rs'
```

---

## 4. Критерії сесії (чеклист)

- [x] **S21** OpenAPI gap audit (optimization/automl/federated)
- [x] **S22** FM-019 CI merge gate (`pa11y-wcag22` + contract)
- [x] **S23** Playwright smoke E2E
- [x] **S24** `DELETE /ui/dashboards/{id}` (MonitoringManager)
- [x] **S25** UI_QUALITY P1 — tenants, OAuth2, monitoring dashboards contracts
- [ ] Наступний: UI_QUALITY P1 backlog (metrics/SAML/alert-rules) або FM-003 §4 (BLOCKED)
- [ ] Оновлено `HANDOFF_NEW_SESSION.md`, FM §5.1/§5.3, `CHANGELOG.md` (якщо публічний API/доки)
- [ ] `cargo test-ci` + push MSYS2
- [ ] Не стаджити `data/audit/*.log.gz`

**Не в обсязі без явного запиту:** FM-004, FM-006, FM-009, FM-010, FM-003 §4 (BLOCKED).
