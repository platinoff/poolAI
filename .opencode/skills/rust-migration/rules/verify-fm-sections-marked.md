# FM Section Rules

## verify-fm-sections-marked

**> Check that all GPULimits migration FM sections have ✅ marks.**

**Why It Matters**: Ensures FUNCTION_MANAGEMENT.md documents completed migration bands with verification markers.

**Bad**:
```markdown
### 5.126 GPULimits migration 3 (PH-S1909.S1918, band 127, 2026-08-12) ❓
**????????? ? 5.12:** **0** (band 126 · §5.126 PH-S1909.S1918). Vision rev **478**.
```

**Good**:
```markdown
### 5.126 GPULimits migration 3 (PH-S1909.S1918, band 127, 2026-08-12) ✅
**????????? ? 5.12:** **0** (band 126 ✅ · §5.126 PH-S1909.S1918). Vision rev **480**.
```

**Pattern**: All sections §5.105 through §5.129 must have ✅ marks, not ❓ or ?.

**Example Check**:
```python
# Verify all FM sections have ✅
sections = ["5.105", "5.125", "5.126", "5.127", "5.128", "5.129"]
for section in sections:
    assert f"band {section.split('.')[1][0]} ✅" in fm_content, f"Missing ✅ in {section}"
```

**Related Rules**: migration-depth-stub, vision-sync-rules, test-pattern-rules

**See Also**: FUNCTION_MANAGEMENT.md, HANDOFF_NEW_SESSION.md, NEXT_SESSION_PROMPT.md