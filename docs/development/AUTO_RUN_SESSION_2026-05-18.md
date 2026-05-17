# Автономний прогін розробки (PoolAI) — 2026-05-18

**Попередній прогін:** [`AUTO_RUN_SESSION_2026-05-17.md`](./AUTO_RUN_SESSION_2026-05-17.md) (P0 патерни, FM-003 doc, test-ci).

**Призначення:** **FM-013** — розширення контрактних тестів admin UI ↔ API; FM-003 лишається ops (LAN без стенду).

**Правила:** [`.cursor/rules/autonomous-orchestrator.mdc`](../../.cursor/rules/autonomous-orchestrator.mdc), [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) **§5.1**.

---

## 1. Ціль сесії

| Фаза | Ціль |
|------|------|
| **S0** | HANDOFF + §5.1 + `git status` |
| **S1** | **FM-013:** `tests/admin_ui_api_contracts.rs` — libraries, topology, vm, workers; узгодити admin libs UI з `metadata.installed_at` |
| **S2** | Architect `- [ ]` → LAN (FM-003), cloud-sdk (FM-006, поза обсягом) |
| **S3** | `cargo fmt` + `cargo test-ci` (`K8S_OPENAPI_ENABLED_VERSION=1.28`) |
| **S4** | HANDOFF, FUNCTION_MANAGEMENT §5.1, CHANGELOG, STABLE_STATE |
| **Push** | MSYS2 + Summary за [`.cursor/commands/git-push.md`](../../.cursor/commands/git-push.md) |

**Поза обсягом:** FM-004, FM-006, FM-009, FM-010.

---

## 2. Стартовий промпт

```text
Продовж автономний прогін PoolAI за docs/development/AUTO_RUN_SESSION_2026-05-18.md.
Оркестратор: S0→S4; після коду — cargo fmt + cargo test-ci; push MSYS2 з Summary.
FM-004/006/009/010 не чіпати. LAN без стенду — FM-003 Planned (ops).
```

---

## 3. FM-003 (ops, не блокує)

- Немає 2+ хостів → оновити лише нотатку в runbook/BENCHMARKS за потреби.
- Не чекати LAN для закриття S1.
