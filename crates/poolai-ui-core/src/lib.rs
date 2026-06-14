//! Shared admin UI validators and formatters (PH-S146).
//!
//! Parity targets: `src/ui/admin_common.js`, embedded JS in `src/ui/admin/*.rs`,
//! and `src/ui/admin_charts.js`. Browser WASM exports: `crates/poolai-ui-wasm` (PH-S147).

pub mod api_error;
pub mod format;
pub mod i18n;
pub mod lease;
pub mod ml;
pub mod modal;
pub mod pricing;
pub mod table;
pub mod theme;
pub mod validate;
