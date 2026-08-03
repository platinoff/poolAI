//! Ratio96 store-wire slice (PH-S1650, band 101 — F Ratio96 depth scaffold).
//!
//! Production-verify stub: reads `docs/development/rust_ratio.json` (the durable ratio store the
//! `poolai-loc-audit` bin writes) and classifies the 96% stretch gate state.

use serde_json::Value;

/// Relative path of the durable ratio store inside the repo.
pub const RATIO96_STORE_PATH: &str = "docs/development/rust_ratio.json";

/// Snapshot of the stretch-gate fields parsed from the ratio store.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ratio96StoreState {
    pub stretch_spirit: f64,
    pub below_stretch_spirit: bool,
    pub stretch_spirit_gate_met: bool,
    pub min_ratio: f64,
    pub meets_min_ratio: bool,
}

impl Ratio96StoreState {
    /// Stretch gate green means the 96% spirit target is met (PH-S1650).
    pub fn stretch_gate_met(&self) -> bool {
        self.stretch_spirit_gate_met
    }

    /// Hold gate green means the regression floor is met (PH-S1650).
    pub fn hold_gate_met(&self) -> bool {
        self.meets_min_ratio
    }
}

/// Parse stretch-gate fields from a `rust_ratio.json` document (PH-S1650).
pub fn ratio96_store_state(doc: &Value) -> Option<Ratio96StoreState> {
    Some(Ratio96StoreState {
        stretch_spirit: doc.get("stretch_spirit")?.as_f64()?,
        below_stretch_spirit: doc
            .get("below_stretch_spirit")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        stretch_spirit_gate_met: doc
            .get("stretch_spirit_gate_met")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        min_ratio: doc.get("min_ratio").and_then(Value::as_f64).unwrap_or(0.95),
        meets_min_ratio: doc
            .get("meets_min_ratio")
            .and_then(Value::as_bool)
            .unwrap_or(false),
    })
}

/// Production-verify stub: read the durable ratio store and classify stretch/hold gates (PH-S1650).
pub fn ratio96_store_wire() -> Result<Ratio96StoreState, String> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(RATIO96_STORE_PATH);
    let raw = std::fs::read_to_string(&path)
        .map_err(|e| format!("ratio96 store read {}: {e}", path.display()))?;
    let doc: Value = serde_json::from_str(&raw).map_err(|e| format!("ratio96 store parse: {e}"))?;
    ratio96_store_state(&doc).ok_or_else(|| "ratio96 store missing stretch fields".to_string())
}

/// Admin/ops store wire for `GET /api/v1/ops/ratio96` (PH-S1680): a lossless JSON snapshot of
/// the durable ratio store. `available: false` when the store file is missing or unparseable.
pub fn ratio96_store_wire_json() -> Value {
    match ratio96_store_wire() {
        Ok(s) => serde_json::json!({
            "mode": "repo_file",
            "available": true,
            "stretch_spirit": s.stretch_spirit,
            "below_stretch_spirit": s.below_stretch_spirit,
            "stretch_spirit_gate_met": s.stretch_spirit_gate_met,
            "min_ratio": s.min_ratio,
            "meets_min_ratio": s.meets_min_ratio,
            "stretch_gate_met": s.stretch_gate_met(),
            "hold_gate_met": s.hold_gate_met(),
        }),
        Err(_) => serde_json::json!({
            "mode": "missing",
            "available": false,
            "stretch_gate_met": false,
            "hold_gate_met": false,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn ratio96_store_state_ph_s1650() {
        let doc = json!({
            "stretch_spirit": 0.96,
            "below_stretch_spirit": true,
            "stretch_spirit_gate_met": false,
            "min_ratio": 0.95,
            "meets_min_ratio": false,
        });
        let state = ratio96_store_state(&doc).expect("store state");
        assert_eq!(state.stretch_spirit, 0.96);
        assert!(state.below_stretch_spirit);
        assert!(!state.stretch_gate_met());
        assert!(!state.hold_gate_met());
    }

    #[test]
    fn ratio96_store_state_stretch_gate_ph_s1650() {
        let doc = json!({
            "stretch_spirit": 0.96,
            "below_stretch_spirit": false,
            "stretch_spirit_gate_met": true,
            "min_ratio": 0.95,
            "meets_min_ratio": true,
        });
        let state = ratio96_store_state(&doc).expect("store state");
        assert!(state.stretch_gate_met());
        assert!(state.hold_gate_met());
    }

    #[test]
    fn ratio96_store_state_missing_ph_s1650() {
        assert!(
            ratio96_store_state(&json!({"stretch_spirit": "not_a_number"})).is_none(),
            "non-numeric stretch_spirit should return None"
        );
        assert!(
            ratio96_store_state(&json!({})).is_none(),
            "empty document should return None"
        );
    }

    #[test]
    fn ratio96_store_wire_json_shape_ph_s1680() {
        let wire = ratio96_store_wire_json();
        assert!(wire.get("mode").is_some(), "wire exposes mode");
        assert!(wire.get("available").is_some(), "wire exposes available");
        assert!(
            wire.get("stretch_gate_met").is_some(),
            "wire exposes stretch_gate_met"
        );
        assert!(
            wire.get("hold_gate_met").is_some(),
            "wire exposes hold_gate_met"
        );
    }
}
