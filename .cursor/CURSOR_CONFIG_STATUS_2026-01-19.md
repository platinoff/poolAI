# Cursor Configuration Status

**Оновлено:** 2026-06-15

**Статус:** ✅ VDT agent roles + §5.12 sync; Cursor **3.7.36** baseline; slim `alwaysApply` (roles + VDT + runtime-stack)

---

## Always-on rules (~120 рядків сумарно)

- `poolai-agent-roles.mdc` — ролі, субагенти, §5.1 / §5.12
- `virtual-development-team.mdc` — спринти, CI, staging
- `runtime-stack-policy.mdc` — Rust, no Python

## On-demand / globs

- `poolai-session-iteration.mdc` — S0, MSYS2, commit
- `git-commit-msys.mdc` — hook / Co-authored-by fix
- `functionality-management.mdc`, `autonomous-orchestrator.mdc`, `docs-vision.mdc`

## hooks.json

`hooks: {}` — тести перед push вручну в MSYS2 ([`commands/git-push.md`](commands/git-push.md)).

## Після змін у `.cursor/`

```bash
git add -f .cursor/rules/poolai-agent-roles.mdc .cursor/rules/virtual-development-team.mdc
# … інші змінені шляхи під .cursor/
```

---

**Наступна ітерація розробки:** `docs/development/NEXT_SESSION_PROMPT.md` (PH-S197).
