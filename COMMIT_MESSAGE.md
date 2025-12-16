# Commit Message

fix: replace unsafe global state with OnceLock and fix compilation issues

## Summary

This commit addresses critical Rust safety issues and compilation problems:

### Critical Fixes
- Replace all `static mut` with `OnceLock` for thread-safe initialization
- Fix unsafe code in `core/config.rs`, `pool/mod.rs`, `monitoring/mod.rs`
- Remove dependency on `ring` crate (requires gcc) by temporarily disabling JWT
- Fix WebSocket routing in axum 0.7
- Fix base64 Engine trait import
- Fix AppState import path

### MSYS2 UCRT64 Configuration
- Configure Rust PATH for MSYS2 UCRT64 environment
- Set GNU toolchain as default (`stable-x86_64-pc-windows-gnu`)
- Add terminal configuration for automatic MSYS2 shell usage
- Create setup scripts for Rust/Cargo in MSYS2

### Dependencies
- Add `futures-util` for WebSocket support
- Add `base64` for temporary token encoding (dev only)
- Temporarily disable `jsonwebtoken` (requires ring/gcc)
- Temporarily disable `axum-server` (requires ring/gcc)
- Add `axum` WebSocket feature

### Code Quality
- Translate comments to English
- Fix unused imports and variables
- Add proper error handling
- Improve code formatting

## Files Changed

### Core Safety Fixes
- `src/core/config.rs` - OnceLock implementation
- `src/pool/mod.rs` - OnceLock<Arc<RwLock<>>> implementation
- `src/monitoring/mod.rs` - OnceLock<Arc<>> implementation

### Network & API
- `src/network/mod.rs` - Use axum::serve instead of axum-server
- `src/network/api.rs` - Fix WebSocket routing
- `src/network/auth.rs` - Temporary JWT stub with base64
- `src/network/ws.rs` - Fix futures_util imports

### Configuration
- `Cargo.toml` - Update dependencies, disable ring-dependent crates
- `.cargo/config.toml` - MSYS2 UCRT64 linker configuration
- `.vscode/settings.json` - Terminal and PATH configuration

### Documentation & Scripts
- `setup_rust_path.sh` - Automatic Rust PATH setup
- `install_gcc.sh` - GCC installation script
- `verify_build.sh` - Build verification script
- Multiple documentation files for setup and fixes

## Breaking Changes

- JWT authentication temporarily disabled (requires gcc installation)
- HTTPS support temporarily disabled (requires gcc installation)

## Next Steps

1. Install GCC via `bash install_gcc.sh` to enable JWT/HTTPS
2. Re-enable `jsonwebtoken` in Cargo.toml
3. Re-enable `axum-server` for HTTPS support

## Testing

- ✅ Compilation successful (with warnings)
- ✅ All unsafe blocks removed
- ✅ Thread-safe initialization implemented
- ✅ MSYS2 UCRT64 environment configured
