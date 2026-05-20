//! Deterministic TQ01 + wire JSON size snapshot (FM-028 / P2b single-host stand).
//! Same 64×256 matrix as `distributed_raid_wire_integration::wire_put_artifact_tq01_*`.
//!
//! ```bash
//! cargo run --bin poolai-p2b-tq01-snapshot --features ml
//! ```

use base64::{engine::general_purpose::STANDARD as B64, Engine};
use poolai::ml::turboquant::pack_uniform_rows;
use poolai::raid::protocol::{ArtifactMetadata, ProtocolMessage, PutArtifactPayload, SyncMode};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
struct Tq01Snapshot {
    matrix_rows: usize,
    matrix_cols: usize,
    tq01_bytes_in: u64,
    tq01_bytes_out: u64,
    tq01_compress_ratio: f64,
    raw_f32_bytes: u64,
    wire_json_bytes_tq01: usize,
    wire_json_bytes_raw_f32: usize,
    wire_json_savings_pct: f64,
}

fn put_artifact_json(data_b64: Option<String>, logical_name: &str, data_len: u64) -> Vec<u8> {
    let metadata = ArtifactMetadata {
        name: logical_name.to_string(),
        version: "1.0.0".to_string(),
        size_bytes: data_len,
        checksum: "fm028-snapshot".to_string(),
        created_at: chrono::Utc::now(),
        content_type: Some("application/octet-stream".to_string()),
        tags: None,
    };
    let payload = PutArtifactPayload {
        artifact_id: "fm028-snapshot-artifact".to_string(),
        source_node: "node-A".to_string(),
        data: data_b64,
        metadata,
        replication_factor: 1,
        sync_mode: SyncMode::Async,
    };
    let msg = ProtocolMessage::put_artifact("node-B".to_string(), payload).unwrap();
    serde_json::to_vec(&msg).expect("wire JSON")
}

fn main() {
    let rows_n = 64usize;
    let cols_n = 256usize;
    let rows: Vec<Vec<f32>> = (0..rows_n)
        .map(|r| {
            (0..cols_n)
                .map(|c| ((r * cols_n + c) as f32) * 0.001)
                .collect()
        })
        .collect();

    let packed = pack_uniform_rows(&rows).expect("pack TQ01");
    let ratio = packed.bytes_in as f64 / packed.bytes_out.max(1) as f64;

    let mut raw_le = Vec::new();
    for r in &rows {
        for f in r {
            raw_le.extend_from_slice(&f.to_le_bytes());
        }
    }
    let raw_len = raw_le.len() as u64;

    let json_tq = put_artifact_json(
        Some(B64.encode(&packed.bytes)),
        "tq01-weights",
        packed.bytes_out,
    );
    let json_raw = put_artifact_json(Some(B64.encode(&raw_le)), "raw-weights", raw_len);

    let savings = if json_raw.is_empty() {
        0.0
    } else {
        (1.0 - (json_tq.len() as f64 / json_raw.len() as f64)) * 100.0
    };

    let snap = Tq01Snapshot {
        matrix_rows: rows_n,
        matrix_cols: cols_n,
        tq01_bytes_in: packed.bytes_in,
        tq01_bytes_out: packed.bytes_out,
        tq01_compress_ratio: ratio,
        raw_f32_bytes: raw_len,
        wire_json_bytes_tq01: json_tq.len(),
        wire_json_bytes_raw_f32: json_raw.len(),
        wire_json_savings_pct: savings,
    };

    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    eprintln!("poolai-p2b-tq01-snapshot (FM-028) ts={stamp}");
    println!("{}", serde_json::to_string_pretty(&snap).expect("json"));
}
