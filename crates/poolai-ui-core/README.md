# poolai-ui-core

Shared validators and formatters extracted from PoolAI admin UI JavaScript (`src/ui/admin_common.js`, `admin/jobs.rs`, `grid_pricing.rs`, `admin_charts.js`).

**PH-S146:** Rust crate + unit tests. **PH-S147 ✅:** `crates/poolai-ui-wasm` — build with `bash bin/build-ui-wasm.sh`.

## Modules

| Module | JS source | Purpose |
|--------|-----------|---------|
| `lease` | `admin/jobs.rs` | Lease display state, epoch formatting |
| `pricing` | `admin/grid_pricing.rs` | USD micro, unix secs, unit keys |
| `api_error` | `admin_common.js` | API error body parsing, fetch hints |
| `format` | `admin_common.js` | HTML escaping |
| `validate` | `mod.rs` | Form field validation (pure) |
| `ml` | `admin_charts.js` | ML metric parsing and summaries |

## Tests

```bash
cargo test -p poolai-ui-core
```
