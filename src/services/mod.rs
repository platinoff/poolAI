//! Application service layer (business orchestration above domain modules).
//!
//! HTTP handlers in `network::api` should stay thin: parse input, call `services::*`, map to responses.
//! See `docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md` (Priority 2).

pub mod raid_service;
