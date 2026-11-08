# Weight-Space Programming (WSP) — pre-engineering brief

**Status:** ECO-9.1 open · **Date:** 2026-11-08  
**Owner intent:** optionally **replace or hybridize** classical RL / on-policy Decision adaptation (NanoJev-like one-pass policies) with **weight-space interventions** when fixing smaller model weights in the poolAI grid.

**Full agent prompt + acceptance:**  
`S:/rust/GSV/docs/superpowers/plans/2026-11-08-poolai-wsp-feasibility.md`  
**Free-agent runbook:** `S:/rust/GSV/docs/gsv/GSV_OPENCODE_FREE_AGENTS.md`

## Why this lives here

PoolAI owns the **grid / job / mining / Solana** surface. Inference engines that would apply patches are plugins:

- `S:/rust/rusty-engine` — GGUF-first MoE (Q4_K_M / Q8_0 …)
- `S:/rust/llama-rs` — llama.cpp wrapper (`llama-cpp-2 =0.1.154` pinned)

Any WSP code path must stay behind a **feasibility gate** (ECO-9.1) before touching kernels or job kinds.

## Current adaptation baseline (conceptual)

| Layer | Today | Candidate WSP role |
|-------|--------|--------------------|
| Job kinds | Inference / Training / FineTune / … (`JOB_LAYER_CONCEPT`) | New kind e.g. `WeightPatch` / `ActivationAudit` (only after 9.1) |
| Policy / RL | On-policy Decision / NanoJev-like one-pass (owner track) | Keep for routing/mining decisions; do **not** assume text-loop MI ports 1:1 |
| Rewards / Solana | Job verify → reward | Validate **delta patches** (hash + peer repro) before reward |
| Engines | rusty-engine + llama-rs | Surgical edit vs block-quant overhead is the hard gate |

## Rules for agents

1. ECO-9.1 = **docs-only** feasibility (%, blockers, hybrid). No GGUF edits.
2. Ukrainian tokenizer / MoE expert damage risk must be explicit in blockers.
3. GGUF block quant: state whether single-FP16 write needs dequant/requant and RAM cost.
4. If direct RL→WSP replacement is infeasible, recommend **hybrid**: RL for policy, WSP for sparse bug/loop patches on small local models.
5. Do not open rebook tickets on the GSV hub board for this work.

## Verdict (fill after ECO-9.1)

_Pending._
