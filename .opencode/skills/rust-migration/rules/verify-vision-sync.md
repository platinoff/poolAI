# Vision Sync Rules

## verify-vision-sync

**> Check that vision revision is consistent across all migrated bands.**

**Why It Matters**: Ensures all migrated bands (124-129) use the same vision revision number, preventing drift and canonical divergence.

**Bad**:
```text
Vision rev 478  # Inconsistent across bands
```

**Good**:
```text
Vision rev 480  # Consistent: bands 124-129
```

**Pattern**: All bands must reference the same vision revision. The revision increments when new bands are migrated.

**Example Check**:
```python
# Verify vision revision consistency
bands = [124, 125, 126, 127, 128, 129]
revisions = set()
for band in bands:
    revision = get_vision_revision(band)  # reads from FM or manifest
    revisions.add(revision)
assert len(revisions) == 1, f"Vision rev inconsistent: {revisions}"
```

**Related Rules**: fm-section-rules, migration-depth-rules, handoff-rules

**See Also**: FUNCTION_MANAGEMENT.md, docs/catalog/FUNCTION_MANAGEMENT.md, vision-sync documentation