//! Version information module
//!
//! Provides application version and build time information.
//! Build time is generated at compile time by build.rs.

use std::sync::OnceLock;
use std::time::SystemTime;

/// Application version string
pub const APP_VERSION: &str = "0.2.2";

// Build time (generated at compile time).
// Note: rustdoc does not attach doc comments to items produced by macros/includes.
include!(concat!(env!("OUT_DIR"), "/build_time.rs"));

/// Global application start time for uptime calculation
static START_TIME: OnceLock<SystemTime> = OnceLock::new();

/// Initialize application start time
/// Should be called once at application startup
pub fn initialize_start_time() {
    START_TIME.get_or_init(SystemTime::now);
}

/// Get application uptime in seconds
pub fn get_uptime_seconds() -> u64 {
    let start = START_TIME.get().copied().unwrap_or_else(SystemTime::now);
    SystemTime::now()
        .duration_since(start)
        .unwrap_or_default()
        .as_secs()
}
