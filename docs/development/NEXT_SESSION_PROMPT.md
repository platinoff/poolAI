# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-21 (completion v2 · master backlog **221** · active **PH-S790…S799** · vision **rev 279** · rust_ratio **94.67%**)

| **← наступний** | **`абракадабра`** (drain band 14 PH-S790…S799) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **221** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S790…S799, band 14 Galaxy governance) |
| **Після band 14** | promote PH-S800…S809 (band 15 admin wasm slim) |
| **Сесій drain** | **23** (`221÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S790…S799) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 14
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 14: **PH-S800…S809**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 221 → product-complete (PH-S790…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–13 | PH-S660…S789 | ✅ drained |
| 14 | PH-S790…S799 | **активна §5.12** — Galaxy governance |
| 15–19 | PH-S800…S849 | admin wasm slim + stand smoke v2 |
| 20–26 | PH-S850…S919 | Job/Memory/Solana + production gates |
| 27–29 | PH-S920…S949 | Ratio **95–96%** |
| 30–33 | PH-S950…S989 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 14 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S790** | Update policy env stub wire | `galaxy_update_policy` HTTP read + test |
| **PH-S791** | Security advisory metric/export shape | stand smoke or unit test |
| **PH-S792** | Admin updates-compat governance extend | wasm panel |
| **PH-S793** | Stand smoke governance metrics | runner |
| **PH-S794** | `governance_depth_stub` | unit test |
| **PH-S795** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S795 |
| **PH-S796** | Docs SECURITY_HARDENING hub sync | docs canon |
| **PH-S797** | `poolai-vision-sync --check` | drift gate green |
| **PH-S798** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S799** | `galaxy_horizon_s790_integration` | governance band close |

**Після band 14 promote:** PH-S800…S809 (band 15 — admin wasm slim monitoring/payout).

---

## Не повторювати

PH-S780…S789 ✅ (fee split production band). BLOCKED: PH-S02/S16/S35 LAN. Deferred: PH-S01/S15/S36 Cloud SDK.
