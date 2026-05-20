//! RPC layer for the sidecar — FM-024 mock stub (no `solana-sdk`).

pub mod mock;

pub use mock::{MockRpcClient, MockRpcError, MockSubmitResult, RpcSubmitStatus};
