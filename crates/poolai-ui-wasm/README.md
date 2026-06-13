# poolai-ui-wasm

**PH-S147:** wasm32 POC — grid-pricing panel helpers from [`poolai-ui-core`](../poolai-ui-core).

## Build

From repo root (MSYS2 UCRT64 or any host with Rust + `wasm32-unknown-unknown`):

```bash
bash bin/build-ui-wasm.sh
```

Outputs:

- `src/ui/wasm/poolai_ui_wasm_bg.wasm` — compiled module
- `src/ui/wasm/poolai_ui_wasm.js` — wasm-bindgen ES module glue

Requires [`wasm-bindgen-cli`](https://rustwasm.github.io/wasm-bindgen/reference/cli.html) (`cargo install wasm-bindgen-cli`).

## Exported helpers (grid-pricing panel)

| JS name | Rust source | Admin JS parity |
|---------|-------------|-----------------|
| `formatUsdMicro` | `pricing::format_usd_micro` | `formatUsdMicro` in `grid_pricing.rs` |
| `formatUnixSecs` | `pricing::format_unix_secs` | `formatUnixSecs` |
| `leaseStateLabel` | `lease::lease_state` | `leaseState` badge input |

Browser wiring into `/ui/admin/grid-pricing` is optional post-POC; host tests remain in `poolai-ui-core`.
