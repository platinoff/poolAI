# Migration Depth Rules

## verify-migration-depth-stub

**> Check that gpu_limits_migration_depth<N>_stub returns correct enum values.**

**Why It Matters**: Ensures the migration depth stub function correctly handles all feature flag combinations (None, DepthModule, StoreStrip, ..., FullBand<N>).

**Bad**:
```rust
// Missing feature flag handling, returns Wrong enum variant
gpu_limits_migration_depth2_stub(None) // panics or wrong variant
```

**Good**:
```rust
// Properly handles None, partial features, and full features
gpu_limits_migration_depth2_stub(None) == GpuLimitsMigrationDepth2::None
gpu_limits_migration_depth2_stub(Some(&json!({"gpu_limits_migration_depth2": true}))) == GpuLimitsMigrationDepth2::DepthModule
gpu_limits_migration_depth2_stub(Some(&json!({
    "gpu_limits_migration_depth2": true,
    "store_strip": true,
    "query_ops_glue": true,
    "html_contracts": true,
    "verify_dev_stand_hook": true,
    "stand_smoke_export": true,
    "loc_audit_flag": true,
    "docs_canon": true,
    "ratio_hold": true,
    "band_close": true,
})) == GpuLimitsMigrationDepth2::FullBand126
```

**Related Rules**: migration-depth-pattern, fm-section-marked

**See Also**: gpu_limits_migration_depth2_stub_ph_s1899, gpu_limits_migration_depth3_stub_ph_s1909, gpu_limits_migration_depth4_stub_ph_s1919, gpu_limits_migration_depth5_stub_ph_s1929