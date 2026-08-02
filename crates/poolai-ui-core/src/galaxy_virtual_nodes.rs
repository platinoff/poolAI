//! Galaxy virtual-node table with origin badges (PH-S508).

use serde_json::Value;

/// Virtual nodes table with origin badge column (PH-S508).
#[allow(clippy::too_many_arguments)]
pub fn render_galaxy_virtual_nodes_panel_html(
    nodes_json: &str,
    col_peer: &str,
    col_origin: &str,
    col_region: &str,
    col_latency: &str,
    col_stale: &str,
    table_aria: &str,
    empty_message: &str,
) -> String {
    use crate::format::escape_html;
    use crate::table::empty_state_html;

    let nodes: Vec<Value> = serde_json::from_str(nodes_json).unwrap_or_default();
    if nodes.is_empty() {
        return empty_state_html(empty_message, None, "🌐", None);
    }
    let rows: String = nodes
        .iter()
        .map(|n| {
            let peer_id = n
                .get("peer")
                .and_then(|p| p.get("peer_id"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let galaxy = n.get("galaxy").cloned().unwrap_or(Value::Null);
            let origin = galaxy
                .get("origin")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let badge_class = match origin {
                "local" => "active",
                "cloud" => "info",
                "telegram_edge" => "warning",
                _ => "muted",
            };
            let region = galaxy
                .get("network_profile")
                .and_then(|p| p.get("region"))
                .and_then(|v| v.as_str())
                .unwrap_or("—");
            let latency = galaxy
                .get("telemetry")
                .and_then(|t| t.get("latency_ms_p50"))
                .and_then(|v| v.as_u64())
                .map(|v| v.to_string())
                .unwrap_or_else(|| "—".into());
            let stale = n.get("stale").and_then(|v| v.as_bool()) == Some(true);
            let stale_label = if stale { "stale" } else { "live" };
            format!(
                r#"<tr>
                  <td><code>{}</code></td>
                  <td><span class="status-badge {}">{}</span></td>
                  <td>{}</td>
                  <td>{}</td>
                  <td>{}</td>
                </tr>"#,
                escape_html(peer_id),
                badge_class,
                escape_html(origin),
                escape_html(region),
                escape_html(&latency),
                escape_html(stale_label),
            )
        })
        .collect();
    format!(
        r#"<div class="admin-table-container"><table class="admin-table" aria-label="{}">
            <thead>
              <tr>
                <th>{}</th>
                <th>{}</th>
                <th>{}</th>
                <th>{}</th>
                <th>{}</th>
              </tr>
            </thead>
            <tbody>{}</tbody>
          </table></div>"#,
        escape_html(table_aria),
        escape_html(col_peer),
        escape_html(col_origin),
        escape_html(col_region),
        escape_html(col_latency),
        escape_html(col_stale),
        rows,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_galaxy_virtual_nodes_origin_badge_ph_s508() {
        let json = r#"[{"peer":{"peer_id":"p1"},"galaxy":{"origin":"telegram_edge","network_profile":{"region":"eu"},"telemetry":{"latency_ms_p50":12}},"stale":false}]"#;
        let html = render_galaxy_virtual_nodes_panel_html(
            json,
            "Peer",
            "Origin",
            "Region",
            "Latency",
            "Stale",
            "Virtual nodes",
            "Empty",
        );
        assert!(html.contains("telegram_edge"));
        assert!(html.contains("eu"));
        assert!(html.contains("12"));
    }
}
