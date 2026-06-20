# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-20 (completion v2 · master backlog **271** · active **PH-S740…S749** · vision **rev 274** · rust_ratio **94.57%**)

| **← наступний** | **`абракадабра`** (drain band 9 PH-S740…S749) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **271** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S740…S749, band 9 Galaxy **§6.6** signed capability admission) |
| **Після band 9** | promote PH-S750…S759 (band 10 Galaxy depth) |
| **Сесій drain** | **28** (`271÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S740…S749) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 9
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 9: **PH-S750…S759**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 271 → product-complete (PH-S740…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–8 | PH-S660…S739 | ✅ drained |
| 9 | PH-S740…S749 | **активна §5.12** — Galaxy **§6.6** signed capability admission |
| 10–14 | PH-S750…S789 | Galaxy depth (caps, prefetch, payout, fees, governance) |
| 15–19 | PH-S800…S849 | Admin wasm slim + stand smoke v2 + OpenAPI |
| 20–26 | PH-S850…S919 | Job/Memory/Solana + production gates |
| 27–29 | PH-S920…S949 | Ratio **95–96%** |
| 30–33 | PH-S950…S989 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 9 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S740** | signed capability strict gate | unsigned edge → 403 + metric |
| **PH-S741** | signed capability dev fixture pass | integration test register-remote OK |
| **PH-S742** | Admin capability doc panel extend | updates-compat capability section |
| **PH-S743** | Stand smoke signed-cap reject shape | export shape unit test |
| **PH-S744** | `capability_admission_depth_stub` | Galaxy §6.6 unit test |
| **PH-S745** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S745 |
| **PH-S746** | SECURITY_HARDENING ↔ §6.6 cross-link | docs canon |
| **PH-S747** | `poolai-vision-sync --check` | drift gate green |
| **PH-S748** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S749** | `galaxy_horizon_s740_integration` | §6.6 capability band close + docs |

**Після band 9 promote:** PH-S750…S759 (band 10 — Galaxy depth).

---

## Не повторювати (закрито)

PH-S730…S739 ✅ · band 8 network_profile persist · `galaxy_horizon_s730_integration` · rust_ratio **94.57%** advisory hold.

Band 7 PH-S720…S729 ✅ routing/re-migrate · band 6 PH-S710…S719 ✅ stand smoke parity · bands 1–5 ✅.

BLOCKED: PH-S02/S16/S35 LAN · Deferred: PH-S01/S15/S36 Cloud SDK.
