//! Real devnet JSON-RPC submit (FM-033) — HTTP only, no `solana-client` crate.

use crate::config::AdapterConfig;
use crate::events::DomainEventEnvelope;
use crate::instruction::{build_submit_instruction, AnchorMode};
use crate::rpc::{MockSubmitResult, RpcSubmitStatus};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use serde_json::{json, Value};
use solana_hash::Hash;
use solana_keypair::Keypair;
use solana_signer::Signer;
use solana_transaction::Transaction;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

pub const ENV_KEYPAIR_PATH: &str = "POOLAI_SOLANA_KEYPAIR_PATH";
pub const ENV_PROGRAM_ID: &str = "POOLAI_SOLANA_PROGRAM_ID";

#[derive(Debug, PartialEq, Eq)]
pub enum DevnetRpcError {
    RealRpcDisabled,
    NoKeypair,
    InvalidKeypair(String),
    Transport(String),
    Rpc(String),
    Build(String),
}

impl fmt::Display for DevnetRpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RealRpcDisabled => write!(f, "real RPC is disabled (mock_rpc=true)"),
            Self::NoKeypair => write!(
                f,
                "no signing keypair; set {ENV_KEYPAIR_PATH} to a Solana CLI json keypair file"
            ),
            Self::InvalidKeypair(e) => write!(f, "invalid keypair: {e}"),
            Self::Transport(e) => write!(f, "rpc transport: {e}"),
            Self::Rpc(e) => write!(f, "rpc error: {e}"),
            Self::Build(e) => write!(f, "transaction build: {e}"),
        }
    }
}

impl std::error::Error for DevnetRpcError {}

/// JSON-RPC transport (injectable in tests).
pub trait RpcTransport: Send {
    fn post(&self, rpc_url: &str, method: &str, params: Value) -> Result<Value, DevnetRpcError>;
}

#[derive(Debug)]
pub struct HttpRpcTransport {
    client: reqwest::blocking::Client,
}

impl HttpRpcTransport {
    pub fn new() -> Result<Self, DevnetRpcError> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| DevnetRpcError::Transport(e.to_string()))?;
        Ok(Self { client })
    }
}

impl Default for HttpRpcTransport {
    fn default() -> Self {
        Self::new().expect("http client")
    }
}

impl RpcTransport for HttpRpcTransport {
    fn post(&self, rpc_url: &str, method: &str, params: Value) -> Result<Value, DevnetRpcError> {
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params,
        });
        let resp = self
            .client
            .post(rpc_url)
            .json(&body)
            .send()
            .map_err(|e| DevnetRpcError::Transport(e.to_string()))?;
        let status = resp.status();
        let text = resp
            .text()
            .map_err(|e| DevnetRpcError::Transport(e.to_string()))?;
        if !status.is_success() {
            return Err(DevnetRpcError::Transport(format!("http {status}: {text}")));
        }
        let v: Value =
            serde_json::from_str(&text).map_err(|e| DevnetRpcError::Transport(e.to_string()))?;
        if let Some(err) = v.get("error") {
            return Err(DevnetRpcError::Rpc(err.to_string()));
        }
        Ok(v)
    }
}

#[derive(Debug)]
pub struct DevnetRpcClient<T: RpcTransport = HttpRpcTransport> {
    transport: T,
    by_event_id: HashMap<String, MockSubmitResult>,
}

impl DevnetRpcClient<HttpRpcTransport> {
    pub fn with_http_transport() -> Result<Self, DevnetRpcError> {
        Ok(Self::new(HttpRpcTransport::new()?))
    }
}

impl Default for DevnetRpcClient<HttpRpcTransport> {
    fn default() -> Self {
        Self::with_http_transport().expect("http rpc transport")
    }
}

impl<T: RpcTransport> DevnetRpcClient<T> {
    pub fn new(transport: T) -> Self {
        Self {
            transport,
            by_event_id: HashMap::new(),
        }
    }

    pub fn submit_event(
        &mut self,
        config: &AdapterConfig,
        envelope: &DomainEventEnvelope,
    ) -> Result<MockSubmitResult, DevnetRpcError> {
        if config.mock_rpc {
            return Err(DevnetRpcError::RealRpcDisabled);
        }

        if let Some(existing) = self.by_event_id.get(&envelope.event_id) {
            return Ok(MockSubmitResult {
                status: RpcSubmitStatus::Duplicate,
                ..existing.clone()
            });
        }

        let keypair = load_keypair_from_env()?;
        let program_id = config.resolved_program_id();
        let anchor_mode = AnchorMode::for_program_id(&program_id)
            .as_wire_str()
            .to_string();
        let payer = keypair.pubkey();
        let ix = build_submit_instruction(&program_id, &payer, envelope)
            .map_err(|e| DevnetRpcError::Build(e.to_string()))?;

        let blockhash = fetch_latest_blockhash(&self.transport, &config.rpc_url)?;
        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer));
        tx.sign(&[&keypair], blockhash);

        let signature = send_transaction(&self.transport, &config.rpc_url, &tx)?;
        let slot = fetch_signature_slot(&self.transport, &config.rpc_url, &signature)?;

        let result = MockSubmitResult {
            status: RpcSubmitStatus::Submitted,
            cluster: config.cluster,
            rpc_url: config.rpc_url.clone(),
            signature,
            slot,
            anchor_mode,
        };
        self.by_event_id
            .insert(envelope.event_id.clone(), result.clone());
        Ok(result)
    }

    pub fn len(&self) -> usize {
        self.by_event_id.len()
    }
}

/// Legacy helper — prefer [`AdapterConfig::resolved_program_id`].
pub fn resolve_program_id(config: &AdapterConfig) -> String {
    config.resolved_program_id()
}

pub fn load_keypair_from_env() -> Result<Keypair, DevnetRpcError> {
    let path = std::env::var(ENV_KEYPAIR_PATH).map_err(|_| DevnetRpcError::NoKeypair)?;
    load_keypair_from_file(Path::new(&path))
}

pub fn load_keypair_from_file(path: &Path) -> Result<Keypair, DevnetRpcError> {
    let raw =
        std::fs::read_to_string(path).map_err(|e| DevnetRpcError::InvalidKeypair(e.to_string()))?;
    let bytes: Vec<u8> =
        serde_json::from_str(&raw).map_err(|e| DevnetRpcError::InvalidKeypair(e.to_string()))?;
    Keypair::try_from(bytes.as_slice()).map_err(|e| DevnetRpcError::InvalidKeypair(e.to_string()))
}

fn fetch_latest_blockhash(
    transport: &impl RpcTransport,
    rpc_url: &str,
) -> Result<Hash, DevnetRpcError> {
    let resp = transport.post(
        rpc_url,
        "getLatestBlockhash",
        json!([{ "commitment": "confirmed" }]),
    )?;
    let value = resp
        .pointer("/result/value/blockhash")
        .and_then(|v| v.as_str())
        .ok_or_else(|| DevnetRpcError::Rpc("missing blockhash in response".into()))?;
    Hash::from_str(value).map_err(|e| DevnetRpcError::Rpc(e.to_string()))
}

fn send_transaction(
    transport: &impl RpcTransport,
    rpc_url: &str,
    tx: &Transaction,
) -> Result<String, DevnetRpcError> {
    let wire = bincode::serialize(tx).map_err(|e| DevnetRpcError::Build(e.to_string()))?;
    let encoded = B64.encode(wire);
    let resp = transport.post(
        rpc_url,
        "sendTransaction",
        json!([
            encoded,
            {
                "encoding": "base64",
                "skipPreflight": false,
                "preflightCommitment": "confirmed",
                "maxRetries": 3
            }
        ]),
    )?;
    resp.pointer("/result")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| DevnetRpcError::Rpc("missing signature in sendTransaction response".into()))
}

fn fetch_signature_slot(
    transport: &impl RpcTransport,
    rpc_url: &str,
    signature: &str,
) -> Result<u64, DevnetRpcError> {
    let resp = transport.post(
        rpc_url,
        "getSignatureStatuses",
        json!([[signature], { "searchTransactionHistory": true }]),
    )?;
    let slot = resp
        .pointer("/result/value/0")
        .and_then(|v| v.get("slot"))
        .and_then(|v| v.as_u64());
    Ok(slot.unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AdapterConfig, CommitmentLevel, SolanaCluster};
    use crate::events::{DomainEvent, JobCompletedEvent};
    use std::sync::{Arc, Mutex};

    #[derive(Clone, Default)]
    struct FakeTransport {
        calls: Arc<Mutex<Vec<String>>>,
        blockhash: String,
        signature: String,
    }

    impl RpcTransport for FakeTransport {
        fn post(
            &self,
            _rpc_url: &str,
            method: &str,
            _params: Value,
        ) -> Result<Value, DevnetRpcError> {
            self.calls.lock().unwrap().push(method.to_string());
            match method {
                "getLatestBlockhash" => Ok(json!({
                    "result": { "value": { "blockhash": self.blockhash } }
                })),
                "sendTransaction" => Ok(json!({ "result": self.signature })),
                "getSignatureStatuses" => Ok(json!({
                    "result": { "value": [{ "slot": 42, "confirmationStatus": "confirmed" }] }
                })),
                other => Err(DevnetRpcError::Rpc(format!("unexpected method: {other}"))),
            }
        }
    }

    fn test_config() -> AdapterConfig {
        AdapterConfig {
            cluster: SolanaCluster::Devnet,
            rpc_url: "https://api.devnet.solana.com".into(),
            commitment: CommitmentLevel::Confirmed,
            mock_rpc: false,
            program_id: crate::instruction::PLACEHOLDER_PROGRAM_ID.into(),
        }
    }

    #[test]
    fn devnet_submit_requires_keypair() {
        let key_path = std::env::var(ENV_KEYPAIR_PATH).ok();
        std::env::remove_var(ENV_KEYPAIR_PATH);
        let mut client = DevnetRpcClient::new(FakeTransport::default());
        let env = DomainEventEnvelope::new(
            "no-key",
            DomainEvent::JobCompleted(JobCompletedEvent {
                job_id: "j".into(),
                executor_peer_id: "p".into(),
                payout_lamports: None,
                verification_digest: None,
            }),
        );
        let err = client.submit_event(&test_config(), &env).unwrap_err();
        assert_eq!(err, DevnetRpcError::NoKeypair);
        if let Some(p) = key_path {
            std::env::set_var(ENV_KEYPAIR_PATH, p);
        }
    }

    #[test]
    fn devnet_submit_with_fake_transport() {
        let dir = std::env::temp_dir().join(format!("poolai-solana-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let kp_path = dir.join("id.json");
        let kp = Keypair::new();
        std::fs::write(
            &kp_path,
            serde_json::to_string(&kp.to_bytes().to_vec()).unwrap(),
        )
        .unwrap();

        std::env::set_var(ENV_KEYPAIR_PATH, kp_path.to_string_lossy().to_string());

        let blockhash = Hash::new_unique();
        let transport = FakeTransport {
            blockhash: blockhash.to_string(),
            signature: "5".repeat(88),
            ..Default::default()
        };
        let mut client = DevnetRpcClient::new(transport.clone());
        let env = DomainEventEnvelope::new(
            "rpc-live-1",
            DomainEvent::JobCompleted(JobCompletedEvent {
                job_id: "j".into(),
                executor_peer_id: "p".into(),
                payout_lamports: None,
                verification_digest: None,
            }),
        );
        let result = client.submit_event(&test_config(), &env).unwrap();
        assert_eq!(result.status, RpcSubmitStatus::Submitted);
        assert_eq!(result.signature.len(), 88);
        assert_eq!(result.slot, 42);
        assert_eq!(result.anchor_mode, "memo");

        let calls = transport.calls.lock().unwrap();
        assert!(calls.iter().any(|m| m == "getLatestBlockhash"));
        assert!(calls.iter().any(|m| m == "sendTransaction"));

        let second = client.submit_event(&test_config(), &env).unwrap();
        assert_eq!(second.status, RpcSubmitStatus::Duplicate);

        let _ = std::fs::remove_dir_all(dir);
        std::env::remove_var(ENV_KEYPAIR_PATH);
    }
}
