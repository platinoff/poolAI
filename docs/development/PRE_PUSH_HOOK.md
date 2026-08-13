# Pre-Push Hook — Vision Canon + Formatting

## Overview

The pre-push git hook runs **before every `git push`**:

1. **`cargo run --bin poolai-vision-sync`** — sync manifest/feed/extensions **and** canon docs (`README.md`, `docs/INDEX_2026-03-17.md`, `docs/development/README.md`, `NEXT_SESSION_PROMPT.md`, `GSV/docs/vision/vision.svg`)
2. **Fail if sync modified tracked files** — commit canon updates before push
3. **`poolai-vision-sync --check`** — FM ↔ manifest ↔ extensions ↔ `.mdc` + canon doc drift
4. **`cargo fmt --all --check`** — Rust formatting gate

## Location

| File | Role |
|------|------|
| `bin/pre-push-hook.sh` | Canonical script (tracked in git) |
| `bin/install-pre-push-hook.sh` | Installs hook into `.git/hooks/pre-push` |
| `.git/hooks/pre-push` | Thin wrapper → `bin/pre-push-hook.sh` |

Install or refresh after clone:

```bash
bash bin/install-pre-push-hook.sh
```

## Usage

### Normal push (canon + fmt OK)

```bash
# MSYS2 UCRT64 bash recommended
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
git push
```

### Push blocked — canon sync updated files

```bash
git push
# vision sync updated README.md, GSV/docs/vision/vision.svg, ...
git add README.md docs/INDEX_2026-03-17.md docs/development/README.md docs/development/NEXT_SESSION_PROMPT.md GSV/docs/vision/
git commit -m 'docs: vision canon sync'
git push
```

### Bypass (not recommended)

```bash
git push --no-verify
```

## Troubleshooting

### `cargo not found`

Hook **fails** (no silent skip). Use MSYS2 UCRT64 or ensure `~/.cargo/bin` is on `PATH`.

### Vision drift after band close

Run manually before commit:

```bash
cargo run --bin poolai-vision-sync
cargo run --bin poolai-vision-sync -- --check
```

## Related

- `docs/development/HANDOFF_NEW_SESSION.md` — vision close band
- `.cursor/rules/docs-vision.mdc` — vision map rules
- `.cursor/rules/git-workflow.mdc` — push workflow
