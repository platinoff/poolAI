//! Application service layer (business orchestration above domain modules).
//!
//! HTTP handlers in `network::api` should stay thin: parse input, call `services::*`, map to responses.
//! See `docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md` (Priority 2).

pub mod admin_service;
#[cfg(feature = "cloud")]
pub mod cloud_service;
#[cfg(feature = "enterprise")]
pub mod enterprise_service;
pub mod library_service;
pub mod raid_service;
pub mod vm_service;
