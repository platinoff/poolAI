//! Build distributed RAID PutArtifact probe messages for virtual-node workers (FM-016+++).

use crate::raid::protocol::{ArtifactMetadata, ProtocolMessage, PutArtifactPayload, SyncMode};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use chrono::Utc;
use serde_json::Value;
use uuid::Uuid;

const DEFAULT_PROBE_BYTES: &[u8] = b"poolai-vn-artifact-probe";

/// Small PutArtifact wire message for coordinator RAID replicate endpoint.
pub fn build_probe_message(
    worker_id: &str,
    task_payload: &Value,
) -> Result<ProtocolMessage, String> {
    let logical_name = task_payload
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("vn-probe");
    let bytes = task_payload
        .get("data_b64")
        .and_then(|v| v.as_str())
        .map(|s| B64.decode(s).map_err(|e| e.to_string()))
        .transpose()?
        .unwrap_or_else(|| DEFAULT_PROBE_BYTES.to_vec());

    let metadata = ArtifactMetadata {
        name: logical_name.to_string(),
        version: "probe-1".to_string(),
        size_bytes: bytes.len() as u64,
        checksum: format!("probe-{}", bytes.len()),
        created_at: Utc::now(),
        content_type: Some("application/octet-stream".to_string()),
        tags: Some(vec!["virtual_node".to_string(), "probe".to_string()]),
    };
    let payload = PutArtifactPayload {
        artifact_id: format!("vn-probe-{}", Uuid::new_v4()),
        source_node: worker_id.to_string(),
        data: Some(B64.encode(&bytes)),
        metadata,
        replication_factor: 1,
        sync_mode: SyncMode::Async,
    };
    ProtocolMessage::put_artifact(worker_id.to_string(), payload).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_put_artifact_envelope() {
        let msg = build_probe_message("vn-a", &Value::Null).expect("message");
        assert_eq!(msg.message_type, "put_artifact");
        assert_eq!(msg.node_id, "vn-a");
    }
}
