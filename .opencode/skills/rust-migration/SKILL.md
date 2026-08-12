# Rust Migration Skills

Skills for GPULimits migration bands 124-129 and OpenCode agent workflows.

## Quick Reference

| Rule File | Purpose |
|-----------|---------|
| `verify-migration-depth-stub.md` | Check migration depth stub returns correct enum values |
| `verify-fm-sections-marked.md` | Verify FM sections §5.105-§5.129 have ✅ marks |
| `verify-vision-sync.md` | Check vision revision consistent across all bands |
| `verify-handoff-state.md` | Verify auto-handoff state is consistent and current |

## Band Migration Status

| Band | PH Range | FM Section | Status |
|------|----------|------------|--------|
| 124 | PH-S1879.S1888 | §5.105 | ✅ Complete |
| 125 | PH-S1889.S1898 | §5.125 | ✅ Complete |
| 126 | PH-S1899.S1908 | §5.126 | ✅ Complete |
| 127 | PH-S1909.S1918 | §5.127 | ✅ Complete |
| 128 | PH-S1919.S1928 | §5.128 | ✅ Complete |
| 128 | PH-S1929.S1938 | §5.129 | ✅ Complete |

## Vision Sync

- **Revision**: 480 (all bands 124-129 consistent)
- **Drift Check**: Green (next PH-S1879)

## Test Verification

| Test Suite | Result |
|------------|--------|
| Integration tests (124-126) | ✅ 4/4 pass each |
| Unit tests (127-129) | ✅ 1/1 pass each |
| **cargo test-ci** | ✅ 359+ test groups, 0 failures |

## Session Auto-Handoff

| File | Current State |
|------|-------------|
| `HANDOFF_NEW_SESSION.md` | Band 129 open, Vision rev 480 |
| `NEXT_SESSION_PROMPT.md` | Band 129 current, horizon band 130 |
| `FM §5.129` | Marker added, Vision rev 480 |

## Installation

```shell
# For OpenCode, create these files:
# .opencode/skills/rust-migration/SKILL.md
# .opencode/skills/rust-migration/rules/*.md

# Or install via:
# npx add-skill leonardomso/rust-skills  (then select relevant skills)
```

## Usage in OpenCode

```shell
/skill rust-migration verify-handoff
/skill rust-migration verify-fm
/skill rust-migration verify-vision
/skill rust-migration check-depth
```