# PoolAI documentation

**Last updated:** 2026-04-06

## Canonical reading order

1. **[INDEX_2026-03-17.md](./INDEX_2026-03-17.md)** — map of the whole `docs/` tree (concept, status, ML, cloud, troubleshooting).
2. **[development/NEXT_STEPS_ARCHITECT_2026-03-17.md](./development/NEXT_STEPS_ARCHITECT_2026-03-17.md)** — main Rust Architect plan (priorities P1–P6, TurboQuant, CI verification).
3. **[development/HANDOFF_NEW_SESSION.md](./development/HANDOFF_NEW_SESSION.md)** — start here in a **new chat session** (branch `main`, doc order, git-push pointer, P2 snapshot, next steps).
4. **Concept** — [concept/poolAI_concept_root.txt](./concept/poolAI_concept_root.txt), Grid/Memory/Job under `concept/` and [development/JOB_LAYER_CONCEPT_2026-03-17.md](./development/JOB_LAYER_CONCEPT_2026-03-17.md).
5. **Repo inventory** — root [file_list.csv](../file_list.csv) (curated paths; update when adding `src/services/*`, `src/network/api/*`, `.cursor/*`); full file list: `git ls-files`.
6. **Architecture** — [ARCHITECTURE_REVIEW.md](./ARCHITECTURE_REVIEW.md), [ARCHITECTURE_BEST_PRACTICES.md](./ARCHITECTURE_BEST_PRACTICES.md).
7. **Git push (Windows)** — [`.cursor/commands/git-push.md`](../.cursor/commands/git-push.md).

## Short pointers

- **Status / plans** — `status/`, `development/` (see [development/README.md](./development/README.md) for plan index).
- **Unified API errors (P3)** — основний REST, **`enterprise_api.rs`**, **`raid.rs`**, **`auth.rs`**: структуровані помилки (`api_json_error`, `ErrorContext`, …). Деталі — `development/HANDOFF_NEW_SESSION.md`.
- **Benchmarks (P4)** — `docs/performance/BENCHMARKS.md`: Criterion таргети `runtime_benchmarks` (у т.ч. локальний RAID put) та `turboquant_benchmarks` (`--features ml`).
- **One-off historical notes** — `archive/` (includes former root `PUSH_*.md` files).
