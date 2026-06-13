# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-13 · PH-S135 ✅ · vision manifest **rev 64**

Один спринт за раз — канон у [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12.

---

## Спринти §5.12 (зріз)

| Стан | Sprint | Тема |
|------|--------|------|
| **← наступний** | **PH-S136** | Prefetch policy env wire stub |
| відкрито | PH-S137 | Trust gate settlement metrics stub |
| відкрито | PH-S138 | Locality rank integration test |
| відкрито | PH-S139 | Telegram wallet bind E2E |
| відкрито | PH-S140 | network_profile register-remote stub |
| відкрито | PH-S141 | Admin jobs migrating badge UI |
| відкрито | PH-S142 | Verification sample rate env stub |
| ✅ | PH-S135 | Telegram wallet GET lookup API |
| ✅ | PH-S134 | Protocol middleware E2E smoke |

**Відкритих:** **7** (PH-S136…S142)

---

## Copy-paste для агента

```
Привіт! PoolAI — спринт PH-S136 (один PH-S*, VDT).

S0: MSYS2 UCRT64 · git fetch · HANDOFF · FM §5.12 (7 відкритих)

PH-S136 — Prefetch policy env wire stub (Galaxy §5.6, dispatch.rs)
  PrefetchPolicyMode + POOLAI_GALAXY_* from_env; unit tests; no enqueue wire
  cargo fmt --all && cargo test-ci
  FM/HANDOFF/NEXT_SESSION/vision revision++
```

---

## Короткий зріз

| **Наступний** | PH-S136 — Prefetch policy env wire stub |
| **Відкритих** | **7** (PH-S136…S142) |
| **Vision** | http://127.0.0.1:8765/docs/vision/index.html · `.\bin\open-docs-vision.ps1` |
