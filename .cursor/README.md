# Cursor Agent Configuration (PoolAI)

**Оновлено:** 2026-05-27 · hooks **PH-S201** post-push VDT notify

## Rules (`rules/`)

| Файл | `alwaysApply` | Призначення |
|------|---------------|-------------|
| [`poolai-agent-roles.mdc`](rules/poolai-agent-roles.mdc) | ✅ | Ролі людини / оркестратора / субагентів; FM §5.1 vs §5.12 |
| [`virtual-development-team.mdc`](rules/virtual-development-team.mdc) | ✅ | VDT: спринти, локальний CI, staging |
| [`runtime-stack-policy.mdc`](rules/runtime-stack-policy.mdc) | ✅ | Rust-only; без Python |
| [`poolai-session-iteration.mdc`](rules/poolai-session-iteration.mdc) | globs | S0, MSYS2, commit/push |
| [`git-commit-msys.mdc`](rules/git-commit-msys.mdc) | globs | Hook, `amend-head-msg.sh` |
| [`functionality-management.mdc`](rules/functionality-management.mdc) | on-demand | FM-менеджер |
| [`autonomous-orchestrator.mdc`](rules/autonomous-orchestrator.mdc) | on-demand | AUTO_RUN |
| [`docs-vision.mdc`](rules/docs-vision.mdc) | globs | `GSV/docs/vision/` sync |
| [`chat-context.mdc`](rules/chat-context.mdc) | globs | Стартовий контекст чату; ключові документи |
| [`ai-assistant.mdc`](rules/ai-assistant.mdc) | globs | AI-асистент: ключові документи, sync концепт→статус |
| [`documentation.mdc`](rules/documentation.mdc) | globs | Куди писати доки (кроки 11–12), структура `docs/` |
| [`git-workflow.mdc`](rules/git-workflow.mdc) | globs | Git workflow: CL, Summary, MSYS2 push |
| [`msys2-windows.mdc`](rules/msys2-windows.mdc) | globs | MSYS2/Windows dev env (bash only, GNU) |
| [`project-structure.mdc`](rules/project-structure.mdc) | globs | Організація репозиторію (src/, bin/, scripts/) |
| [`rust-architect.mdc`](rules/rust-architect.mdc) | globs | Rust Architect workflow, `target/` disk, pre-push |
| [`rust.mdc`](rules/rust.mdc) | globs | Rust стиль і патерни |
| [`scripts.mdc`](rules/scripts.mdc) | globs | Скрипти: bash only, `scripts/` структура |

Повний індекс правил — [`rules/.cursorrules`](rules/.cursorrules).

## Commands (`commands/`)

- [`git-push.md`](commands/git-push.md) — MSYS2 commit + push (канон)
- [`check.md`](commands/check.md) — `cargo test-ci` parity
- [`test.md`](commands/test.md), [`review.md`](commands/review.md), [`pr.md`](commands/pr.md)

## Skills

- [`skills/poolai-documentation/SKILL.md`](skills/poolai-documentation/SKILL.md) — docs map 1–12

## Hooks (`hooks.json`, PH-S201)

- [`hooks/post-push-ph-s-notify.sh`](hooks/post-push-ph-s-notify.sh) — after successful `git push` with `PH-S*` in commit subject → `additional_context` VDT docs-sync checklist
- Self-test: `bash .cursor/hooks/post-push-ph-s-notify.sh --self-test`
- Config: [`hooks.json`](hooks.json) · readme: [`hooks/README.md`](hooks/README.md)

## Ітераційна сесія

Старт: [`docs/development/NEXT_SESSION_PROMPT.md`](../docs/development/NEXT_SESSION_PROMPT.md) (copy-paste блок).
