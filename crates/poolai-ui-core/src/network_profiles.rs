//! Admin network profiles panel HTML renderer (PH-S732).

use crate::format::escape_html;
use serde_json::Value;

fn profile_field(row: &Value, key: &str) -> String {
    row.get("network_profile")
        .and_then(|np| np.get(key))
        .or_else(|| row.get(key))
        .map(|v| match v {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            _ => "—".to_string(),
        })
        .unwrap_or_else(|| "—".to_string())
}

/// Render network profiles admin table from API row snapshots.
pub fn render_network_profiles_panel_html(
    rows_json: &str,
    col_peer: &str,
    col_region: &str,
    col_latency: &str,
    col_bandwidth: &str,
    table_aria: &str,
    empty_message: &str,
) -> String {
    let rows: Value = serde_json::from_str(rows_json).unwrap_or(Value::Null);
    let entries = rows
        .as_array()
        .cloned()
        .or_else(|| rows.get("rows").and_then(|v| v.as_array()).cloned())
        .unwrap_or_default();
    if entries.is_empty() {
        return format!(r#"<p class="muted">{}</p>"#, escape_html(empty_message));
    }
    let body: String = entries
        .iter()
        .map(|row| {
            let peer_id = row.get("peer_id").and_then(|v| v.as_str()).unwrap_or("—");
            format!(
                "<tr><td><code>{}</code></td><td>{}</td><td>{}</td><td>{}</td></tr>",
                escape_html(peer_id),
                escape_html(profile_field(row, "region")),
                escape_html(profile_field(row, "latency_ms_p50")),
                escape_html(profile_field(row, "bandwidth_mbps")),
            )
        })
        .collect();
    format!(
        r#"<table class="admin-table" aria-label="{aria}"><thead><tr><th>{peer}</th><th>{region}</th><th>{latency}</th><th>{bandwidth}</th></tr></thead><tbody>{body}</tbody></table>"#,
        aria = escape_html(table_aria),
        peer = escape_html(col_peer),
        region = escape_html(col_region),
        latency = escape_html(col_latency),
        bandwidth = escape_html(col_bandwidth),
        body = body,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn render_network_profiles_panel_ph_s732() {
        let rows = json!([
            {
                "peer_id": "peer-a",
                "network_profile": {
                    "region": "eu-west",
                    "latency_ms_p50": 24,
                    "bandwidth_mbps": 500
                }
            }
        ]);
        let html = render_network_profiles_panel_html(
            &rows.to_string(),
            "Peer",
            "Region",
            "Latency p50",
            "Bandwidth Mbps",
            "Network profiles",
            "Empty",
        );
        assert!(html.contains("peer-a"));
        assert!(html.contains("eu-west"));
        assert!(html.contains("24"));
        assert!(html.contains("500"));
    }

    #[test]
    fn render_network_profiles_panel_empty_ph_s732() {
        let html = render_network_profiles_panel_html(
            "[]",
            "Peer",
            "Region",
            "Latency",
            "Bandwidth",
            "Network profiles",
            "No profiles",
        );
        assert!(html.contains("No profiles"));
    }
}
