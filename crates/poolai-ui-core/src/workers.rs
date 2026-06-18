//! Admin workers table HTML (PH-S480 wasm glue).

use serde_json::Value;

/// Workers table HTML for admin panel (PH-S480).
pub fn render_workers_panel_html(
    workers_json: &str,
    col_id: &str,
    col_status: &str,
    col_metrics: &str,
    col_actions: &str,
    table_aria: &str,
    healthy_label: &str,
    unhealthy_label: &str,
    req_label: &str,
    delete_label: &str,
    empty_message: &str,
) -> String {
    use crate::format::escape_html;
    use crate::table::empty_state_html;

    let workers: Vec<Value> = serde_json::from_str(workers_json).unwrap_or_default();
    if workers.is_empty() {
        return empty_state_html(empty_message, None, "👷", None);
    }
    let rows: String = workers
        .iter()
        .map(|w| {
            let wid = w
                .get("id")
                .or_else(|| w.get("worker_id"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let healthy = w.get("is_healthy").and_then(|v| v.as_bool()) == Some(true);
            let status_badge = if healthy {
                format!(
                    r#"<span class="status-badge active">{}</span>"#,
                    escape_html(healthy_label)
                )
            } else {
                format!(
                    r#"<span class="status-badge error">{}</span>"#,
                    escape_html(unhealthy_label)
                )
            };
            let requests = w
                .get("total_requests_processed")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            format!(
                r#"<tr>
                  <td>{}</td>
                  <td>{}</td>
                  <td>{} {}</td>
                  <td><button type="button" class="btn btn-danger" data-worker-id="{}">{}</button></td>
                </tr>"#,
                escape_html(wid),
                status_badge,
                escape_html(req_label),
                requests,
                escape_html(wid),
                escape_html(delete_label)
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
              </tr>
            </thead>
            <tbody>{}</tbody>
          </table></div>"#,
        escape_html(table_aria),
        escape_html(col_id),
        escape_html(col_status),
        escape_html(col_metrics),
        escape_html(col_actions),
        rows
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_workers_panel_html_ph_s480() {
        let html = render_workers_panel_html(
            r#"[{"id":"w1","is_healthy":true,"total_requests_processed":3}]"#,
            "ID",
            "Status",
            "Metrics",
            "Actions",
            "Workers",
            "Healthy",
            "Unhealthy",
            "Requests:",
            "Delete",
            "No workers",
        );
        assert!(html.contains("w1"));
        assert!(html.contains("Healthy"));
        assert!(html.contains("Requests:"));
    }

    #[test]
    fn render_workers_panel_empty_ph_s480() {
        let html = render_workers_panel_html(
            "[]",
            "ID",
            "Status",
            "Metrics",
            "Actions",
            "Workers",
            "Healthy",
            "Unhealthy",
            "Requests:",
            "Delete",
            "No workers found",
        );
        assert!(html.contains("No workers found"));
    }
}
