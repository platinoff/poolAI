# Cursor hooks (PoolAI)

**PH-S201** — post-push VDT reminder after `PH-S*` sprint commits.

| Hook | Event | Script |
|------|-------|--------|
| Post-push docs sync | `postToolUse` (Shell) | `post-push-ph-s-notify.sh` |

## Behavior

After a successful **`git push`** (Shell tool, exit 0), if `git log -1` subject contains `PH-S*`, the hook returns `additional_context` with the FM/HANDOFF/NEXT/vision checklist.

## Verify locally

```bash
bash .cursor/hooks/post-push-ph-s-notify.sh --self-test
```

Reload: save `.cursor/hooks.json` or restart Cursor. Debug: **Hooks** output channel in Cursor.

## Canon

- MSYS2 bash only (no `.ps1`) — `.cursor/CHANGELOG.md` 2026-04-06
- Git **pre-push** fmt gate: `docs/development/PRE_PUSH_HOOK.md` (separate from this Cursor hook)
