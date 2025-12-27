//! Version information module
//!
//! Provides application version and build time information.
//! Build time is generated at compile time by build.rs.

/// Application version string
pub const APP_VERSION: &str = "0.1.0";

// Build time (generated at compile time).
// Note: rustdoc does not attach doc comments to items produced by macros/includes.
include!(concat!(env!("OUT_DIR"), "/build_time.rs"));
