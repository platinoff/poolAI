//! Trust score persistence depth classification (PH-S914, Galaxy §6.5 band 26).

use crate::grid::galaxy_trust_score_store::{current_trust_store_backend, TrustStoreBackend};

/// Trust score store persistence depth (Galaxy §6.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustPersistDepth {
    None,
    Ephemeral,
    JsonFile,
    JsonRestartPersist,
    SqliteDb,
    SqliteRestartPersist,
}

/// Classify trust persist depth from backend + persisted peer count (PH-S914).
pub fn trust_persist_depth_stub(
    backend: TrustStoreBackend,
    persisted_peer_count: u32,
) -> TrustPersistDepth {
    match backend {
        TrustStoreBackend::Ephemeral => {
            if persisted_peer_count > 0 {
                TrustPersistDepth::Ephemeral
            } else {
                TrustPersistDepth::None
            }
        }
        TrustStoreBackend::Json => {
            if persisted_peer_count > 0 {
                TrustPersistDepth::JsonRestartPersist
            } else {
                TrustPersistDepth::JsonFile
            }
        }
        TrustStoreBackend::Sqlite => {
            if persisted_peer_count > 0 {
                TrustPersistDepth::SqliteRestartPersist
            } else {
                TrustPersistDepth::SqliteDb
            }
        }
    }
}

/// Wire label for trust-metrics / stand smoke (PH-S913).
pub fn trust_persist_depth_wire_label(depth: TrustPersistDepth) -> &'static str {
    match depth {
        TrustPersistDepth::None => "none",
        TrustPersistDepth::Ephemeral => "ephemeral",
        TrustPersistDepth::JsonFile => "json",
        TrustPersistDepth::JsonRestartPersist => "json_restart",
        TrustPersistDepth::SqliteDb => "sqlite",
        TrustPersistDepth::SqliteRestartPersist => "sqlite_restart",
    }
}

/// Runtime trust persist depth from in-process store.
pub fn current_trust_persist_depth() -> TrustPersistDepth {
    trust_persist_depth_stub(
        current_trust_store_backend(),
        crate::grid::galaxy_trust_score_store::persisted_trust_peer_count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trust_persist_depth_stub_ph_s914() {
        assert_eq!(
            trust_persist_depth_stub(TrustStoreBackend::Ephemeral, 0),
            TrustPersistDepth::None
        );
        assert_eq!(
            trust_persist_depth_stub(TrustStoreBackend::Json, 0),
            TrustPersistDepth::JsonFile
        );
        assert_eq!(
            trust_persist_depth_stub(TrustStoreBackend::Json, 2),
            TrustPersistDepth::JsonRestartPersist
        );
        assert_eq!(
            trust_persist_depth_stub(TrustStoreBackend::Sqlite, 0),
            TrustPersistDepth::SqliteDb
        );
        assert_eq!(
            trust_persist_depth_stub(TrustStoreBackend::Sqlite, 1),
            TrustPersistDepth::SqliteRestartPersist
        );
        assert_eq!(
            trust_persist_depth_wire_label(TrustPersistDepth::SqliteRestartPersist),
            "sqlite_restart"
        );
    }
}
