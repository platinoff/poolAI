//! Verification checker tasks table HTML (PH-S512).

use serde_json::Value;

/// Checker tasks table for admin panel (PH-S512).
pub fn render_grid_verification_panel_html(
    tasks_json: &str,
    pending_total: u64,
    col_job: &str,
    col_type: &str,
    col_pending: &str,
    table_aria: &str,
    empty_message: &str,
) -> String {
    use crate::format::escape_html;
    use crate::table::empty_state_html;

    let tasks: Vec<Value> = serde_json::from_str(tasks_json).unwrap_or_default();
    let pending_hint = format!("{col_pending}: {pending_total}");
    if tasks.is_empty() {
        return format!(
            "{}{}",
            empty_state_html(empty_message, Some(&pending_hint), "🔍", None),
            format!(r#"<p class="muted">{}</p>"#, escape_html(&pending_hint))
        );
    }
    let rows: String = tasks
        .iter()
        .map(|t| {
            let job_id = t.get("job_id").and_then(|v| v.as_str()).unwrap_or("—");
            let task_type = t.get("task_type").and_then(|v| v.as_str()).unwrap_or("—");
            format!(
                r#"<tr><td><code>{}</code></td><td>{}</td></tr>"#,
                escape_html(job_id),
                escape_html(task_type),
            )
        })
        .collect();
    format!(
        r#"<p class="muted admin-hint">{}</p>
        <div class="admin-table-container"><table class="admin-table" aria-label="{}">
            <thead><tr><th>{}</th><th>{}</th></tr></thead>
            <tbody>{}</tbody>
          </table></div>"#,
        escape_html(&pending_hint),
        escape_html(table_aria),
        escape_html(col_job),
        escape_html(col_type),
        rows,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_grid_verification_panel_ph_s512() {
        let json = r#"[{"job_id":"job-1","task_type":"verification_checker"}]"#;
        let html = render_grid_verification_panel_html(
            json,
            1,
            "Job",
            "Type",
            "Pending",
            "Checker tasks",
            "No tasks",
        );
        assert!(html.contains("job-1"));
        assert!(html.contains("Pending: 1"));
    }
}
