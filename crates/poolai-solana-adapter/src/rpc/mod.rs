//! RPC layer for the sidecar — FM-024 mock stub, FM-033 devnet HTTP submit.

pub mod devnet;
pub mod mock;

pub use devnet::{
    DevnetRpcClient, DevnetRpcError, HttpRpcTransport, RpcTransport, ENV_KEYPAIR_PATH,
    ENV_PROGRAM_ID,
};
pub use mock::{MockRpcClient, MockRpcError, MockSubmitResult, RpcSubmitStatus};
