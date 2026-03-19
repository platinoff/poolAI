# Git Workflow — Conventional Commits (CL) & Push

## ⚠️ CRITICAL: MSYS2 bash only

All git commands run in **MSYS2 bash** (`C:\msys64\usr\bin\bash.exe`).  
**Do NOT use** PowerShell, cmd, or **Cursor integrated terminal** for git (CreateFileMapping, index.lock, truncated output). Use **external** MSYS2 UCRT64 only.

## Git order (status → commit → push)

1. **`git status --short`** — check modified/unstaged first.
2. **Read/modify files** before `git add` if needed (AI: avoid staging files you still need to edit).
3. **`git add <paths>`** or `git add -A` (use with care).
4. **`git commit -m "type(scope): subject"`** — use **CL (Conventional Commits)** format.
5. **`git push`** — pre-push hook runs `cargo fmt --all --check`; fix format then re-commit if it fails.

## Commit format (CL = Conventional Commits)

```
<type>(<scope>): <subject>

[optional body - рекомендується: 2-4 bullet points]

[optional footer]
```

**Types**: `feat` | `fix` | `docs` | `style` | `refactor` | `perf` | `test` | `build` | `ci` | `chore` | `revert`  
**Scope examples**: `vm`, `ui`, `raid`, `network`, `cloud`, `ml`, `docs`, `scripts`, `concept`  
**Subject**: imperative, lowercase, ~50 chars.

**Commit summary/body (2-4 bullets)**:
- що саме зроблено (1-2 рядки сумарно)
- які перевірки виконані (наприклад: `cargo fmt`, `cargo clippy`, `cargo test`, `cargo build` з `--all-features`)

**Examples**:
- `feat(cloud): AWS EC2/ECS base_url_override`
- `feat(ml): Stage 4.4 AI/ML stubs (ML.1–ML.3)`
- `docs: update NEXT_STEPS and Cursor rules`

## Pre-push hook

- Runs `cargo fmt --all --check` before push.
- If it fails → run `cargo fmt --all`, commit, then push again.
- Bypass (not recommended): `git push --no-verify`.

## Quick block (MSYS2 bash, без .sh)

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI
rm -f .git/index.lock
git status --short
cargo fmt --all
cargo clippy --all-targets --all-features
cargo test --all-features
cargo build --all-features
git add Cargo.toml src/ tests/ scripts/
git add -f docs/CHANGELOG.md docs/cloud/CLOUD_SDK_STATUS.md docs/concept/poolAI_concept_root.txt docs/development/NEXT_STEPS_2026-01-19.md docs/status/STABLE_STATE_SUMMARY.md docs/troubleshooting/GIT_PUSH_FAILED.md
git add -f .cursor/rules/ .cursor/commands/
git commit -m "type(scope): subject" -m "Summary: cargo fmt/clippy/test/build (--all-features); updated code/docs/rules as needed"
git push origin main
```

Без скриптів. Якщо команди не відпрацьовують — перевір bash, `cd`-шлях, виконуй по одній; див. `.cursor/commands/git-push.md` (п.2 перевірка).
