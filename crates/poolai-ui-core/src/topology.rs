//! Topology graph label helpers (PH-S566) — shared with `topology_graph.rs` / wasm.

use crate::format::{escape_html, format_topology_timestamp};
use serde_json::Value;

const MAX_HUB_LABEL_LEN: usize = 14;

fn t(i18n: &Value, key: &str, fallback: &str) -> String {
    i18n.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or(fallback)
        .to_string()
}

/// Topology stats strip with wasm-formatted last-updated timestamp (PH-S811).
pub fn render_topology_stats_strip_html(summary_json: &str, i18n_json: &str) -> String {
    let summary: Value = serde_json::from_str(summary_json).unwrap_or(Value::Null);
    let i18n: Value = serde_json::from_str(i18n_json).unwrap_or(Value::Null);
    let node_count = summary
        .get("node_count")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let latency = summary
        .get("latency_measurements")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let last_updated = summary.get("last_updated").and_then(|v| v.as_str());
    let ts = format_topology_timestamp(last_updated);

    format!(
        r#"<div class="admin-stat-card">
<div class="stat-value" id="topology-node-count">{node_count}</div>
<div class="stat-label">{nodes_lbl}</div>
</div>
<div class="admin-stat-card">
<div class="stat-value" id="topology-latency-measurements">{latency}</div>
<div class="stat-label">{latency_lbl}</div>
</div>
<div class="admin-stat-card">
<div class="stat-value" id="topology-last-updated">{last_upd}</div>
<div class="stat-label">{last_lbl}</div>
</div>"#,
        node_count = escape_html(&node_count.to_string()),
        latency = escape_html(&latency.to_string()),
        last_upd = escape_html(&ts),
        nodes_lbl = escape_html(&t(&i18n, "admin.topo.stat.nodes", "Nodes")),
        latency_lbl = escape_html(&t(
            &i18n,
            "admin.topo.stat.latencyMs",
            "Latency Measurements"
        )),
        last_lbl = escape_html(&t(&i18n, "admin.topo.stat.lastUpd", "Last Updated")),
    )
}

/// Short display id for topology tables and graph labels (PH-S198 / PH-S566).
pub fn short_topology_node_id(node_id: &str) -> String {
    let id = node_id.trim();
    if id.is_empty() {
        return "—".to_string();
    }
    let base = id
        .rsplit(':')
        .next()
        .and_then(|s| s.rsplit('/').next())
        .unwrap_or(id)
        .trim();
    let base = base.strip_prefix("node-").unwrap_or(base);
    if base.len() <= MAX_HUB_LABEL_LEN {
        base.to_string()
    } else {
        format!("{}…", &base[..MAX_HUB_LABEL_LEN.saturating_sub(1)])
    }
}

/// Hub-aware SVG label: highest-degree nodes (degree ≥ 2) get a `hub·` prefix.
pub fn topology_hub_label(node_id: &str, degree: usize, max_degree: usize) -> String {
    let short = short_topology_node_id(node_id);
    if max_degree >= 2 && degree == max_degree {
        format!("hub·{short}")
    } else {
        short
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_topology_node_id_truncates_long_ids_ph_s566() {
        assert_eq!(
            short_topology_node_id("cluster/node-very-long-name-here"),
            "very-long-nam…"
        );
    }

    #[test]
    fn topology_hub_label_marks_max_degree_hub_ph_s566() {
        assert_eq!(topology_hub_label("node-a", 2, 2), "hub·a");
        assert_eq!(topology_hub_label("node-b", 1, 2), "b");
    }

    #[test]
    fn render_topology_stats_strip_ph_s811() {
        let html = render_topology_stats_strip_html(
            r#"{"node_count":3,"latency_measurements":6,"last_updated":"2026-06-20T12:34:56Z"}"#,
            r#"{"admin.topo.stat.nodes":"Nodes"}"#,
        );
        assert!(html.contains("topology-node-count"));
        assert!(html.contains("topology-last-updated"));
        assert!(html.contains("2026-06-20 12:34:56 UTC"));
        assert!(html.contains(">3<"));
    }
}
