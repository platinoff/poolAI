//! Signed release manifest verification (Galaxy Grid §9.2, PH-S66).
//!
//! Verifies an ed25519 signature over the raw manifest bytes and optional artifact SHA-256
//! entries from the manifest JSON.

mod error;
mod manifest;
mod trust;
mod verify;

pub use error::VerifyReleaseError;
pub use manifest::{ReleaseArtifact, ReleaseManifest};
pub use verify::{verify_release, VerifyReleaseOptions, VerifyReleaseReport};
