# CI Canon Gate — Local Dual-Gate Workflow

Canonical doc: [`CI_CANON.md`](./CI_CANON.md) (band 50, PH-S1147).

## Overview

The **CI canon gate** mirrors GitHub Actions jobs locally before push:

1. **`cargo test-ci`** — Rust integration/unit test suite (`.cargo/config.toml` alias)
2. **`cargo run --bin poolai-openapi-gap-audit`** — OpenAPI route coverage (0 missing)
3. **`cargo run --bin poolai-loc-audit -- --advisory --min-ratio 0.95`** — rust ratio hold advisory

Dual gate (PH-S1004): API scope requires **both** `test-ci` and `openapi-gap-audit` green.

## Quick start

```bash
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI

# Full local CI canon (manual)
cargo test-ci
cargo run --bin poolai-openapi-gap-audit
cargo run --bin poolai-loc-audit -- --advisory --min-ratio 0.95
```

## Dev stand hooks

| Env / flag | Action |
|------------|--------|
| `VERIFY_CI_CANON=1` | `verify-dev-stand.sh` → openapi-gap-audit + loc-audit `--ci-canon` |
| `quick --ci-canon` | After health wait: loc-audit `--ci-canon` + openapi-gap-audit |

```bash
VERIFY_CI_CANON=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --ci-canon
```

## Loc-audit fields

```bash
cargo run --bin poolai-loc-audit -- --ci-canon
```

| Field | Meaning |
|-------|---------|
| `ci_canon_mode` | `true` when `--ci-canon` (PH-S1140) |
| `ci_canon_criteria_total` | Registry size (7) |
| `ci_canon_criteria_met_count` | Markers found in canonical paths |

## GitHub CI parity

| Local | CI job (`.github/workflows/ci.yml`) |
|-------|-------------------------------------|
| `cargo test-ci` | Test Suite (ubuntu + windows) |
| `poolai-openapi-gap-audit` | `openapi-gap-audit` |
| `poolai-loc-audit --advisory --min-ratio 0.95` | `rust-ratio-audit` |

## Related

- `docs/development/PRE_PUSH_HOOK.md` — pre-push vision canon (band 49)
- `.cursor/rules/poolai-testing-policy.mdc` — dual gate PH-S1004
- `docs/development/RUN_LOCAL.md` — stand + verify hooks
