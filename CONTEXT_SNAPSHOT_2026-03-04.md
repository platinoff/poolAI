# Context snapshot (2026-03-04)

Quick resume for next session. See also: `.cursor/rules/chat-context.md`, `docs/status/STABLE_STATE_SUMMARY.md`, `docs/development/NEXT_STEPS_2026-01-19.md`.

## Done this session
- **CI**: Required test step now uses `--features ml,enterprise,cloud`; fixes exit 101 (ml/enterprise/cloud tests compile).
- **ML tests**: Type annotations added (Experiment, MLPipeline, ModelVersion, etc.); ml_experiments gated with `cfg(feature = "ml")`.
- **Docs**: STABLE_STATE, NEXT_STEPS, DEVELOPMENT_ROADMAP, CURSOR_AND_NEXT_STEPS_VERIFICATION, PUSH_AND_DEPENDABOT_PREREQUISITE updated; CONTEXT_SNAPSHOT in docs/.

## Current state
- v0.2.2 | Rust 1.92.0 | main pushed.
- Before next dev: handle 6 Dependabot PRs (#47–#51, #55). See `docs/PUSH_AND_DEPENDABOT_PREREQUISITE_2026-03-04.md`.
- Next: v0.3.0 prep, ML.1 pruning, ML.2/ML.3 pipeline/aggregation.

## Key paths
- Chat context: `.cursor/rules/chat-context.md`
- Status: `docs/status/STABLE_STATE_SUMMARY.md`
- Next steps: `docs/development/NEXT_STEPS_2026-01-19.md`
- Concept: `docs/concept/poolAI_concept_root.txt` (if present)

Note: `docs/` and `.cursor/` are in `.gitignore`; doc updates are local unless you change that.
