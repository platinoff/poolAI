//! Galaxy Grid worker ↔ coordinator protocol compatibility (§9.3, PH-S65).
//!
//! Negotiation on `POST /api/v1/discovery/register-remote` when `protocol_version` is set.
//! Legacy clients without `protocol_version` are accepted for backward compatibility.

use serde::{Deserialize, Serialize};

/// Wire protocol label advertised by this coordinator build.
pub const DEFAULT_COORDINATOR_PROTOCOL: &str = "1.2";

/// Docs-only hint for workers that must upgrade (Galaxy §9.3).
pub const MIN_COORDINATOR_VERSION_DOCS_URL: &str =
    "https://github.com/platinoff/poolAI/blob/main/docs/concept/POOLAI_GALAXY_GRID.md#93-protocol-versioning-та-compat-matrix";

/// Env: comma-separated allow-list of worker `build_id` values (PH-S520).
pub const ENV_ALLOWED_BUILD_IDS: &str = "POOLAI_ALLOWED_BUILD_IDS";

/// Env: minimum worker protocol version during sunset window (PH-S576, Galaxy §9.6).
pub const ENV_PROTOCOL_SUNSET_MIN: &str = "POOLAI_PROTOCOL_SUNSET_MIN";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolSunsetReject {
    pub worker_version: String,
    pub min_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildIdReject {
    pub build_id: Option<String>,
}

/// When allow-list env is set, reject unknown `build_id` (Galaxy §9.3 step 2).
pub fn check_build_id_allowed(build_id: Option<&str>) -> Result<(), BuildIdReject> {
    let raw = match std::env::var(ENV_ALLOWED_BUILD_IDS) {
        Ok(v) if !v.trim().is_empty() => v,
        _ => return Ok(()),
    };
    let allowed: Vec<String> = raw
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if allowed.is_empty() {
        return Ok(());
    }
    let Some(id) = build_id.map(str::trim).filter(|s| !s.is_empty()) else {
        return Err(BuildIdReject {
            build_id: build_id.map(str::to_string),
        });
    };
    if allowed.iter().any(|a| a == id) {
        Ok(())
    } else {
        Err(BuildIdReject {
            build_id: Some(id.to_string()),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatStatus {
    Accepted,
    UpgradeRequired,
    Unsupported,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProtocolNegotiation {
    pub status: CompatStatus,
    pub coordinator_protocol_version: String,
    pub min_coordinator_version: String,
    pub worker_protocol_version: Option<String>,
}

/// Parsed `major.minor` protocol tuple.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProtocolVersion {
    pub major: u8,
    pub minor: u8,
}

impl ProtocolVersion {
    pub fn parse(input: &str) -> Option<Self> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return None;
        }
        let core = trimmed.split_whitespace().next().unwrap_or(trimmed);
        let version_part = core.split('-').next().unwrap_or(core);
        let mut parts = version_part.split('.');
        let major: u8 = parts.next()?.parse().ok()?;
        if major != 1 {
            return None;
        }
        let minor: u8 = parts
            .next()
            .unwrap_or("0")
            .trim_end_matches('x')
            .parse()
            .ok()?;
        Some(Self { major, minor })
    }

    pub fn label(self) -> String {
        format!("{}.{}", self.major, self.minor)
    }
}

fn coordinator_protocol_version() -> ProtocolVersion {
    let raw = std::env::var("POOLAI_COORDINATOR_PROTOCOL_VERSION")
        .unwrap_or_else(|_| DEFAULT_COORDINATOR_PROTOCOL.to_string());
    ProtocolVersion::parse(&raw).unwrap_or(ProtocolVersion { major: 1, minor: 2 })
}

/// Compat matrix cell from Galaxy §9.3 (coordinator row, worker column).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatrixCell {
    Full,
    Limited,
    Reject,
}

fn matrix_cell(coordinator: ProtocolVersion, worker: ProtocolVersion) -> MatrixCell {
    if coordinator.major != 1 || worker.major != 1 {
        return MatrixCell::Reject;
    }
    match (coordinator.minor, worker.minor) {
        (0, 0) => MatrixCell::Full,
        (0, 1) => MatrixCell::Limited,
        (0, 2) => MatrixCell::Reject,
        (1, 0) => MatrixCell::Limited,
        (1, 1) => MatrixCell::Full,
        (1, 2) => MatrixCell::Limited,
        (2, 0) => MatrixCell::Reject,
        (2, 1) => MatrixCell::Full,
        (2, 2) => MatrixCell::Full,
        _ => MatrixCell::Reject,
    }
}

/// Negotiate worker `protocol_version` against an explicit coordinator version.
pub fn negotiate_with_coordinator(
    coordinator: ProtocolVersion,
    protocol_version: Option<&str>,
) -> ProtocolNegotiation {
    let coordinator_label = coordinator.label();
    let min_hint = coordinator_label.clone();

    let Some(raw) = protocol_version.map(str::trim).filter(|s| !s.is_empty()) else {
        return ProtocolNegotiation {
            status: CompatStatus::Accepted,
            coordinator_protocol_version: coordinator_label,
            min_coordinator_version: min_hint,
            worker_protocol_version: None,
        };
    };

    let worker = match ProtocolVersion::parse(raw) {
        Some(v) => v,
        None => {
            return ProtocolNegotiation {
                status: CompatStatus::Unsupported,
                coordinator_protocol_version: coordinator_label,
                min_coordinator_version: min_hint,
                worker_protocol_version: Some(raw.to_string()),
            };
        }
    };

    let status = match matrix_cell(coordinator, worker) {
        MatrixCell::Full => CompatStatus::Accepted,
        MatrixCell::Limited => CompatStatus::UpgradeRequired,
        MatrixCell::Reject => CompatStatus::Unsupported,
    };

    ProtocolNegotiation {
        status,
        coordinator_protocol_version: coordinator_label,
        min_coordinator_version: min_hint,
        worker_protocol_version: Some(worker.label()),
    }
}

/// Negotiate worker `protocol_version` against the coordinator matrix.
///
/// `protocol_version: None` — legacy client (FM-016); always accepted.
pub fn negotiate(protocol_version: Option<&str>) -> ProtocolNegotiation {
    negotiate_with_coordinator(coordinator_protocol_version(), protocol_version)
}

/// Reject workers below sunset minimum with HTTP 426 (PH-S576).
pub fn check_protocol_sunset(worker_version: Option<&str>) -> Result<(), ProtocolSunsetReject> {
    let min_raw = match std::env::var(ENV_PROTOCOL_SUNSET_MIN) {
        Ok(v) if !v.trim().is_empty() => v,
        _ => return Ok(()),
    };
    let min = match ProtocolVersion::parse(&min_raw) {
        Some(v) => v,
        None => return Ok(()),
    };
    let Some(raw) = worker_version.map(str::trim).filter(|s| !s.is_empty()) else {
        return Err(ProtocolSunsetReject {
            worker_version: "missing".into(),
            min_version: min.label(),
        });
    };
    let worker = match ProtocolVersion::parse(raw) {
        Some(v) => v,
        None => {
            return Err(ProtocolSunsetReject {
                worker_version: raw.to_string(),
                min_version: min.label(),
            });
        }
    };
    if worker < min {
        Err(ProtocolSunsetReject {
            worker_version: worker.label(),
            min_version: min.label(),
        })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_protocol_versions() {
        assert_eq!(
            ProtocolVersion::parse("1.2.x"),
            Some(ProtocolVersion { major: 1, minor: 2 })
        );
        assert_eq!(
            ProtocolVersion::parse("1.0"),
            Some(ProtocolVersion { major: 1, minor: 0 })
        );
        assert!(ProtocolVersion::parse("").is_none());
        assert!(ProtocolVersion::parse("2.0").is_none());
    }

    #[test]
    fn matrix_coordinator_1_2() {
        let coord = ProtocolVersion { major: 1, minor: 2 };
        assert_eq!(
            matrix_cell(coord, ProtocolVersion { major: 1, minor: 2 }),
            MatrixCell::Full
        );
        assert_eq!(
            matrix_cell(coord, ProtocolVersion { major: 1, minor: 1 }),
            MatrixCell::Full
        );
        assert_eq!(
            matrix_cell(coord, ProtocolVersion { major: 1, minor: 0 }),
            MatrixCell::Reject
        );
    }

    #[test]
    fn negotiate_legacy_missing_version() {
        let n = negotiate(None);
        assert_eq!(n.status, CompatStatus::Accepted);
        assert!(n.worker_protocol_version.is_none());
    }

    #[test]
    fn negotiate_on_1_2_coordinator() {
        let coord = ProtocolVersion { major: 1, minor: 2 };
        assert_eq!(
            negotiate_with_coordinator(coord, Some("1.2")).status,
            CompatStatus::Accepted
        );
        assert_eq!(
            negotiate_with_coordinator(coord, Some("1.1")).status,
            CompatStatus::Accepted
        );
        assert_eq!(
            negotiate_with_coordinator(coord, Some("1.0")).status,
            CompatStatus::Unsupported
        );
        assert_eq!(
            negotiate_with_coordinator(coord, Some("not-a-version")).status,
            CompatStatus::Unsupported
        );
    }

    #[test]
    fn negotiate_upgrade_required_when_limited() {
        let coord = ProtocolVersion { major: 1, minor: 1 };
        assert_eq!(
            negotiate_with_coordinator(coord, Some("1.2")).status,
            CompatStatus::UpgradeRequired
        );
    }

    #[test]
    fn build_id_allowlist_ph_s520() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        std::env::remove_var(ENV_ALLOWED_BUILD_IDS);
        assert!(check_build_id_allowed(Some("any")).is_ok());
        std::env::set_var(ENV_ALLOWED_BUILD_IDS, "ci-build,prod");
        assert!(check_build_id_allowed(Some("ci-build")).is_ok());
        assert!(check_build_id_allowed(Some("other")).is_err());
        std::env::remove_var(ENV_ALLOWED_BUILD_IDS);
    }

    #[test]
    fn protocol_sunset_min_ph_s576() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        std::env::remove_var(ENV_PROTOCOL_SUNSET_MIN);
        assert!(check_protocol_sunset(Some("1.2")).is_ok());
        std::env::set_var(ENV_PROTOCOL_SUNSET_MIN, "1.1");
        assert!(check_protocol_sunset(Some("1.2")).is_ok());
        assert!(check_protocol_sunset(Some("1.0")).is_err());
        std::env::remove_var(ENV_PROTOCOL_SUNSET_MIN);
    }
}
