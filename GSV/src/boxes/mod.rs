//! GSV boxes — panels/capabilities of the Galaxy StarWalker Vision server.
//!
//! | Box | Rust module | Endpoint | Data source |
//! |-----|-------------|----------|-------------|
//! | Tracker | `tracker` | `/api/tracker` | FM §5.12, history, loc-audit |
//! | SLI console | `sli` | `/api/sli` | `bin/`, `scripts/`, `src/bin/` |
//! | Toolchain | `toolchain` | `/api/toolchain` | toolchain, env |
//! | IDE | `ide` | `/api/ide/…` | opencode/cursor sessions |
//! | Update | `update` | `/api/update` · `/events` | binary/version |
//! | Box preview | `preview` | `/api/preview` | files |
//! | SLI terminal | `terminal` | `/api/terminal` | SLI catalog |
//! | Tests/bench hooks | `hooks` | `/api/hooks/…` | `target/` artifacts |
//! | OmniRouter | `omni` | `/api/omni/…` | provider/model catalog + config + proxy |
//! | Ratio | `ratio` | `/api/ratio` | `GSV/data/rust_ratio.json` (Rust 95–100%) |
//! | Vision | `vision` | `/api/vision*` | `docs/vision/` manifest + feed mirror |

pub mod hooks;
pub mod ide;
pub mod omni;
pub mod preview;
pub mod ratio;
pub mod sli;
pub mod terminal;
pub mod toolchain;
pub mod update;
pub mod vision;

pub use ide::{IdeSelection, IdeSession, IdeWire};
pub use omni::{OmniConfig, OmniRouter, OmniWire, ProviderConfig, ProviderWire, RoutingConfig};
pub use preview::PreviewWire;
pub use ratio::{AuditConfig, CategoryLoc, ProductCategory, RustRatioReport};
pub use sli::{SliCatalog, SliEntry, SliWire};
pub use terminal::{TerminalRequest, TerminalResponse};
pub use toolchain::{ToolchainEntry, ToolchainWire};
pub use update::{UpdateCheckParams, UpdateWire};
