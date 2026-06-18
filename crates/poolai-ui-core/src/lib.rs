//! Shared admin UI validators and formatters (PH-S146).
//!
//! Parity targets: `src/ui/admin_common.js`, embedded JS in `src/ui/admin/*.rs`,
//! and `src/ui/admin_charts.js`. Browser WASM exports: `crates/poolai-ui-wasm` (PH-S147).

pub mod admin_dom;
pub mod api_error;
pub mod design_tokens;
pub mod format;
pub mod galaxy_virtual_nodes;
pub mod grid_verification;
pub mod i18n;
pub mod instances;
pub mod lease;
pub mod ml;
pub mod modal;
pub mod pricing;
pub mod table;
pub mod theme;
pub mod updates_compat;
pub mod validate;
pub mod vm;
pub mod workers;
