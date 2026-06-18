//! Galaxy Grid `network_profile` wire parser (PH-S140, §8.1).
//!
//! Parses `metadata.network_profile` on `POST /api/v1/discovery/register-remote` and
//! normalizes to canonical JSON for peer metadata storage.

use crate::grid::galaxy_locality::LocalityNetworkProfile;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// WAN egress class (Galaxy §8.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GalaxyEgressPolicy {
    Direct,
    VpnProxy,
    WhiteIp,
    LanOnly,
}

/// Full `network_profile` wire object (Galaxy §8.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GalaxyNetworkProfile {
    pub region: String,
    pub latency_ms_p50: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms_p95: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_mbps: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_policy: Option<GalaxyEgressPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topology_ring: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub white_ip_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_measured_at: Option<String>,
}

/// Parse/validation failure for `network_profile`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkProfileParseError {
    pub message: String,
}

impl NetworkProfileParseError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for NetworkProfileParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Locality subset required for PH-S128 scheduling (`region` + `latency_ms_p50`).
impl GalaxyNetworkProfile {
    pub fn locality_subset(&self) -> LocalityNetworkProfile {
        LocalityNetworkProfile {
            region: self.region.clone(),
            latency_ms_p50: self.latency_ms_p50,
            profile_age_secs: None,
        }
    }

    /// Locality subset with computed profile age at `now` (PH-S519).
    pub fn locality_subset_at(&self, now: DateTime<Utc>) -> LocalityNetworkProfile {
        LocalityNetworkProfile {
            region: self.region.clone(),
            latency_ms_p50: self.latency_ms_p50,
            profile_age_secs: self.profile_age_secs_at(now),
        }
    }

    /// Canonical JSON string for peer `metadata["network_profile"]`.
    pub fn to_storage_json(&self) -> Result<String, NetworkProfileParseError> {
        serde_json::to_string(self).map_err(|e| {
            NetworkProfileParseError::new(format!("network_profile serialize failed: {e}"))
        })
    }

    /// Seconds since `last_measured_at`; `None` when missing or unparsable (§8.1).
    pub fn profile_age_secs_at(&self, now: DateTime<Utc>) -> Option<u64> {
        profile_age_secs_from_measured_at(self.last_measured_at.as_deref(), now)
    }
}

/// Compute profile age from RFC3339 timestamp (PH-S519).
pub fn profile_age_secs_from_measured_at(
    last_measured_at: Option<&str>,
    now: DateTime<Utc>,
) -> Option<u64> {
    let raw = last_measured_at?;
    let parsed = DateTime::parse_from_rfc3339(raw).ok()?.with_timezone(&Utc);
    Some(now.signed_duration_since(parsed).num_seconds().max(0) as u64)
}

/// Refresh `last_measured_at` on stored profile JSON (PH-S519).
pub fn refresh_network_profile_measured_at(
    profile_json: &str,
    now: DateTime<Utc>,
) -> Result<String, NetworkProfileParseError> {
    let value: Value = serde_json::from_str(profile_json).map_err(|e| {
        NetworkProfileParseError::new(format!("network_profile JSON parse failed: {e}"))
    })?;
    let mut profile = parse_network_profile_value(&value)?;
    profile.last_measured_at = Some(now.to_rfc3339());
    profile.to_storage_json()
}

/// Parse `network_profile` from a JSON object or JSON-encoded string.
pub fn parse_network_profile_value(
    value: &Value,
) -> Result<GalaxyNetworkProfile, NetworkProfileParseError> {
    let object = match value {
        Value::Object(_) => value.clone(),
        Value::String(raw) => {
            let trimmed = raw.trim();
            if trimmed.is_empty() {
                return Err(NetworkProfileParseError::new(
                    "network_profile must not be empty",
                ));
            }
            serde_json::from_str(trimmed).map_err(|e| {
                NetworkProfileParseError::new(format!("network_profile JSON invalid: {e}"))
            })?
        }
        Value::Null => {
            return Err(NetworkProfileParseError::new(
                "network_profile must be an object",
            ));
        }
        _ => {
            return Err(NetworkProfileParseError::new(
                "network_profile must be an object or JSON string",
            ));
        }
    };

    let profile: GalaxyNetworkProfile = serde_json::from_value(object).map_err(|e| {
        NetworkProfileParseError::new(format!("network_profile schema invalid: {e}"))
    })?;
    validate_network_profile(&profile)?;
    Ok(profile)
}

fn validate_network_profile(
    profile: &GalaxyNetworkProfile,
) -> Result<(), NetworkProfileParseError> {
    validate_region(&profile.region)?;
    Ok(())
}

fn validate_region(region: &str) -> Result<(), NetworkProfileParseError> {
    let len = region.len();
    if !(2..=32).contains(&len) {
        return Err(NetworkProfileParseError::new(
            "network_profile.region must be 2..=32 characters",
        ));
    }
    if !region
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Err(NetworkProfileParseError::new(
            "network_profile.region must match [a-z0-9-]",
        ));
    }
    Ok(())
}

fn json_value_to_metadata_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// Normalize register-remote metadata and parse optional `network_profile`.
pub fn normalize_register_metadata(
    raw: HashMap<String, Value>,
) -> Result<HashMap<String, String>, NetworkProfileParseError> {
    let mut out = HashMap::new();

    for (key, value) in raw {
        if key == "network_profile" {
            let profile = parse_network_profile_value(&value)?;
            out.insert(key, profile.to_storage_json()?);
            continue;
        }
        out.insert(key, json_value_to_metadata_string(&value));
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_object_with_locality_subset() {
        let profile = parse_network_profile_value(&json!({
            "region": "eu-west",
            "latency_ms_p50": 24,
            "bandwidth_mbps": 500,
            "egress_policy": "vpn_proxy"
        }))
        .unwrap();
        assert_eq!(profile.region, "eu-west");
        assert_eq!(profile.latency_ms_p50, 24);
        assert_eq!(profile.bandwidth_mbps, Some(500));
        assert_eq!(profile.egress_policy, Some(GalaxyEgressPolicy::VpnProxy));
        let subset = profile.locality_subset();
        assert_eq!(subset.region, "eu-west");
        assert_eq!(subset.latency_ms_p50, 24);
    }

    #[test]
    fn parse_json_string_value() {
        let raw = json!("{\"region\":\"us-east\",\"latency_ms_p50\":80}");
        let profile = parse_network_profile_value(&raw).unwrap();
        assert_eq!(profile.region, "us-east");
        assert_eq!(profile.latency_ms_p50, 80);
    }

    #[test]
    fn rejects_invalid_region_chars() {
        let err = parse_network_profile_value(&json!({
            "region": "EU_West",
            "latency_ms_p50": 10
        }))
        .unwrap_err();
        assert!(err.message.contains("region"));
    }

    #[test]
    fn rejects_region_too_short() {
        let err = parse_network_profile_value(&json!({
            "region": "e",
            "latency_ms_p50": 10
        }))
        .unwrap_err();
        assert!(err.message.contains("2..=32"));
    }

    #[test]
    fn normalize_metadata_preserves_other_keys() {
        let mut raw = HashMap::new();
        raw.insert("role".to_string(), json!("virtual_node"));
        raw.insert(
            "network_profile".to_string(),
            json!({"region": "ap-south", "latency_ms_p50": 120}),
        );
        let out = normalize_register_metadata(raw).unwrap();
        assert_eq!(out.get("role").map(String::as_str), Some("virtual_node"));
        let stored = out.get("network_profile").expect("stored profile");
        let parsed: GalaxyNetworkProfile = serde_json::from_str(stored).unwrap();
        assert_eq!(parsed.region, "ap-south");
        assert_eq!(parsed.latency_ms_p50, 120);
    }
}
