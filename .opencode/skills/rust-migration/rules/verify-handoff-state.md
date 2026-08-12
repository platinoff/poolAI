# Handoff State Rules

## verify-handoff-state

**> Check that auto-handoff state is consistent and current.**

**Why It Matters**: Ensures HANDOFF_NEW_SESSION.md and NEXT_SESSION_PROMPT.md reflect the correct current band and vision revision, enabling fresh agents to continue work.

**Bad**:
```markdown
**????????? ? 5.12:** **0** (band 128 ❓)  # Unclear if band 128 complete
**Horizon**: band 128 → **PH-S1919.S1928**  # Unclear if this is current
```

**Good**:
```markdown
**????????? ? 5.12:** **0** (band 129 ✅)  # Clear band status
**Horizon**: band 129 → **PH-S1929.S1938**  # Current horizon known
```

**Pattern**: HANDOFF_NEW_SESSION.md must have:
- Current band marked ✅
- Vision revision consistent (480 for bands 124-129)
- Horizon pointing to next band (130)

NEXT_SESSION_PROMPT.md must have:
- Current band correctly stated
- 5.12 active count correct (0 for all bands complete)
- Horizon set to next band

**Example Check**:
```python
# Verify handoff state
handoff = read_handoff_file()
assert handoff.current_band == 129, f"Expected band 129, got {handoff.current_band}"
assert handoff.vision_revision == 480, f"Expected rev 480, got {handoff.vision_revision}"
assert handoff.next_band == 130, f"Expected band 130, got {handoff.next_band}"
```

**Related Rules**: fm-section-rules, vision-sync-rules, test-pattern-rules

**See Also**: HANDOFF_NEW_SESSION.md, NEXT_SESSION_PROMPT.md, FUNCTION_MANAGEMENT.md