//! Job store persistence depth classification stub (PH-S854, Job store RAID band).

/// Job persistence backend depth (production path band 20).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobStoreDepth {
    None,
    JsonFile,
    SqliteDb,
    RaidSnapshot,
    RaidRestartPersist,
}

/// Classify job store depth from backend label and optional RAID snapshot count (PH-S854).
pub fn job_store_depth_stub(backend: Option<&str>, raid_snapshot_count: u32) -> JobStoreDepth {
    let key = backend.unwrap_or("").trim().to_ascii_lowercase();
    match key.as_str() {
        "raid" if raid_snapshot_count > 0 => JobStoreDepth::RaidRestartPersist,
        "raid" => JobStoreDepth::RaidSnapshot,
        "sqlite" => JobStoreDepth::SqliteDb,
        "json" => JobStoreDepth::JsonFile,
        "" => JobStoreDepth::JsonFile,
        _ => JobStoreDepth::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn job_store_depth_stub_ph_s854() {
        assert_eq!(job_store_depth_stub(None, 0), JobStoreDepth::JsonFile);
        assert_eq!(
            job_store_depth_stub(Some("json"), 0),
            JobStoreDepth::JsonFile
        );
        assert_eq!(
            job_store_depth_stub(Some("sqlite"), 0),
            JobStoreDepth::SqliteDb
        );
        assert_eq!(
            job_store_depth_stub(Some("raid"), 0),
            JobStoreDepth::RaidSnapshot
        );
        assert_eq!(
            job_store_depth_stub(Some("raid"), 2),
            JobStoreDepth::RaidRestartPersist
        );
        assert_eq!(
            job_store_depth_stub(Some("unknown"), 0),
            JobStoreDepth::None
        );
    }
}
