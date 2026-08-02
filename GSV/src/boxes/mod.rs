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

pub mod hooks;
pub mod ide;
pub mod preview;
pub mod sli;
pub mod terminal;
pub mod toolchain;
pub mod update;

pub use ide::{IdeSelection, IdeSession, IdeWire};
pub use preview::PreviewWire;
pub use sli::{SliCatalog, SliEntry, SliWire};
pub use terminal::{TerminalRequest, TerminalResponse};
pub use toolchain::{ToolchainEntry, ToolchainWire};
pub use update::{UpdateCheckParams, UpdateWire};
