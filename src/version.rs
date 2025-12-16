//! Version information module
//!
//! Provides application version and build time information.
//! Build time is generated at compile time by build.rs.

/// Application version string
pub const APP_VERSION: &str = "0.1.0";

/// Build time (generated at compile time)
/// This is included from the build script output
include!(concat!(env!("OUT_DIR"), "/build_time.rs")); 