//! Solana adapter / on-chain concept depth stub (PH-S874, FM-010).

/// Solana sidecar integration depth (concept §3–§9).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolanaDepth {
    None,
    SchemaV1,
    NdjsonPipe,
    MockRpc,
    DevnetHorizon,
}

/// Classify Solana adapter depth from feature flags (PH-S874).
pub fn solana_depth_stub(
    schema_v1: bool,
    ndjson_pipe: bool,
    mock_rpc: bool,
    devnet_config: bool,
) -> SolanaDepth {
    if devnet_config && mock_rpc && ndjson_pipe && schema_v1 {
        SolanaDepth::DevnetHorizon
    } else if mock_rpc && ndjson_pipe && schema_v1 {
        SolanaDepth::MockRpc
    } else if ndjson_pipe && schema_v1 {
        SolanaDepth::NdjsonPipe
    } else if schema_v1 {
        SolanaDepth::SchemaV1
    } else {
        SolanaDepth::None
    }
}

/// Wire label for API / stand smoke.
pub fn solana_depth_wire_label(depth: SolanaDepth) -> &'static str {
    match depth {
        SolanaDepth::None => "none",
        SolanaDepth::SchemaV1 => "schema_v1",
        SolanaDepth::NdjsonPipe => "ndjson_pipe",
        SolanaDepth::MockRpc => "mock_rpc",
        SolanaDepth::DevnetHorizon => "devnet_horizon",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solana_depth_stub_ph_s874() {
        assert_eq!(
            solana_depth_stub(false, false, false, false),
            SolanaDepth::None
        );
        assert_eq!(
            solana_depth_stub(true, false, false, false),
            SolanaDepth::SchemaV1
        );
        assert_eq!(
            solana_depth_stub(true, true, false, false),
            SolanaDepth::NdjsonPipe
        );
        assert_eq!(
            solana_depth_stub(true, true, true, false),
            SolanaDepth::MockRpc
        );
        assert_eq!(
            solana_depth_stub(true, true, true, true),
            SolanaDepth::DevnetHorizon
        );
        assert_eq!(solana_depth_wire_label(SolanaDepth::MockRpc), "mock_rpc");
    }
}
