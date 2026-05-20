# Автопрогін Post-Horizon (розробка) — PoolAI

**Дата старту:** 2026-05-20 · **Після:** Horizon S35–S40 ✅ · A+B+C **100%** · job store JSON ✅ (`cd1aaad`)

**Канон FM:** [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) **§5.1**, **§5.7** · **Промпт:** [`NEXT_SESSION_PROMPT.md`](./NEXT_SESSION_PROMPT.md)

**Не повторювати:** autoprogon S21–S34, Horizon S35–S40, FM-001…019 baseline.

---

## 1. Scope

| В обсязі | Поза обсягом |
|----------|----------------|
| FM-020…031 (один FM за сесію) | FM-003 §4 sign-off без 2 хостів |
| `cargo test-ci` після `src/` | mainnet Solana deploy |
| OpenAPI при нових маршрутах | KYC / регуляторика |
| | Повний re-implement FM-004/006/009/010 |

---

## 2. Черга (пріоритет FM)

| Пор. | FM | Фокус | Критерій | test-ci |
|------|-----|--------|----------|---------|
| 1 | **FM-020** | Job scheduler MVP | `Submitted`→`Scheduled`; unit + API; без VM bind | так |
| 2 | **FM-021** | Jobs API розширення | `PATCH /jobs/{id}` (status); OpenAPI `/jobs` | так |
| 3 | **FM-022** | Memory layer API | `GET/POST` shard refs або map до RAID; `src/memory/` | так |
| 4 | **FM-023** | Grid wire integration | Job/Result у discovery або distributed path; тести | так |
| 5 | **FM-024** | Solana sidecar RPC stub | devnet config; NDJSON → mock RPC; crate only | так (crate) |
| 6 | **FM-025** | OpenAPI DTO backlog | VM template bodies; `OPENAPI_GAP_AUDIT` | docs/так |
| 7 | **FM-026** | Jobs contract/E2E | `admin_ui` або Playwright `/jobs` | так |
| 8 | **FM-027** | LAN §4 runbook | 2-host checklist; **BLOCKED** до інфра | docs |
| 9 | **FM-028** | P2b single-host metrics | TQ01+RAID на `run-lan-nodes`; рядок у `BENCHMARKS.md` | ops |
| 10 | **FM-029** | Job store SQLite | optional feature; migrate з JSON | так |
| 11 | **FM-030** | Monitoring persistence | `MONITORING_PERSISTENCE_PLAN` MVP | так |
| 12 | **FM-031** | FM-019 WCAG expand | додаткові admin URLs у pa11y/axe | CI |

---

## 3. Команди

```bash
export PATH="$HOME/.cargo/bin:/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI

cargo test job --lib --features ml,enterprise,cloud,test-utils
cargo test-ci
# LAN dev (1 host):
# .\bin\run-lan-nodes.ps1 -SkipBuild
```

---

## 4. Чеклист сесії

- [x] FM-020 Job scheduler
- [x] FM-021 Jobs PATCH + OpenAPI
- [x] FM-022 Memory API
- [x] FM-023 Grid integration
- [x] FM-024 Solana RPC stub
- [x] FM-025 OpenAPI DTO
- [x] FM-026 Jobs E2E
- [x] FM-027 LAN runbook (prep)
- [x] FM-028 P2b metrics
- [ ] FM-029 Job SQLite
- [ ] FM-030 Monitoring persist
- [ ] FM-031 WCAG expand
- [ ] push MSYS2 + Summary

---

**Перша сесія:** **FM-020** (scheduler MVP).
