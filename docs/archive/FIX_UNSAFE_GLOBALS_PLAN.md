# Plan: Fix Unsafe Global State

## 🎯 Objective
Replace `unsafe` global state with safe alternatives (`OnceLock` or `LazyLock`) to comply with Rust Best Practices (Concept Section 9.7.3).

## 📋 Files to Fix

### 1. `src/core/config.rs`
**Current**: `static mut CONFIG: Option<PoolAIConfig> = None;`
**Target**: `static CONFIG: OnceLock<PoolAIConfig> = OnceLock::new();`

**Changes**:
- Replace `static mut` with `OnceLock`
- Update `initialize_config()` to use `set()` instead of unsafe assignment
- Update `get_config()` to use `get()` instead of unsafe access
- Update `update_config()` to use `set()` instead of unsafe assignment

### 2. `src/pool/mod.rs`
**Current**: `static mut GLOBAL_POOL: Option<Pool> = None;`
**Target**: `static GLOBAL_POOL: OnceLock<Arc<RwLock<Pool>>> = OnceLock::new();`

**Changes**:
- Replace `static mut` with `OnceLock<Arc<RwLock<Pool>>>`
- Update `initialize()` to wrap Pool in Arc<RwLock<>> and use `set()`
- Update `initialize_with_config()` similarly
- Update `shutdown()` to clear the pool safely
- Update `health_check()` to use `get()` instead of unsafe access
- Update `get_global_pool()` to return `Option<&Arc<RwLock<Pool>>>`

### 3. `src/monitoring/mod.rs`
**Current**: `static mut GLOBAL_MONITORING: Option<Monitoring> = None;`
**Target**: `static GLOBAL_MONITORING: OnceLock<Arc<Monitoring>> = OnceLock::new();`

**Changes**:
- Replace `static mut` with `OnceLock<Arc<Monitoring>>`
- Update `initialize()` to wrap Monitoring in Arc and use `set()`
- Update `shutdown()` to clear safely
- Update `health_check()` to use `get()` instead of unsafe access

## 🔧 Implementation Steps

1. **Add `std::sync::OnceLock` import** to all three files
2. **Replace static mut declarations** with `OnceLock`
3. **Update initialization functions** to use `set()` method
4. **Update access functions** to use `get()` method
5. **Update shutdown functions** to clear safely
6. **Test compilation** to ensure no errors
7. **Create git branch** for these changes
8. **Commit changes** with descriptive message

## 📝 Branch Name
`fix/unsafe-globals-to-oncelock`

## ✅ Success Criteria
- ✅ No `unsafe` blocks in config.rs, pool/mod.rs, monitoring/mod.rs
- ✅ Code compiles without errors
- ✅ All tests pass (if any)
- ✅ Code follows Rust Best Practices from concept
- ✅ 100% compliance with concept Section 9.7.3

## 🚀 Git Workflow

```bash
# Create and switch to new branch
git checkout -b fix/unsafe-globals-to-oncelock

# Make changes (will be done in code)
# Stage changes
git add src/core/config.rs src/pool/mod.rs src/monitoring/mod.rs

# Commit
git commit -m "fix: replace unsafe global state with OnceLock

- Replace static mut CONFIG with OnceLock in core/config.rs
- Replace static mut GLOBAL_POOL with OnceLock<Arc<RwLock<Pool>>> in pool/mod.rs
- Replace static mut GLOBAL_MONITORING with OnceLock<Arc<Monitoring>> in monitoring/mod.rs
- Improves memory safety and complies with Rust Best Practices (Concept Section 9.7.3)
- Removes all unsafe blocks from these modules"

# Push branch
git push -u origin fix/unsafe-globals-to-oncelock
```

