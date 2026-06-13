//! Shared admin UI validators and formatters (PH-S146).
//!
//! Parity targets: `src/ui/admin_common.js`, embedded JS in `src/ui/admin/*.rs`,
//! and `src/ui/admin_charts.js`. Browser wiring is PH-S147 (wasm POC).

pub mod api_error;
pub mod format;
pub mod lease;
pub mod ml;
pub mod pricing;
pub mod validate;
