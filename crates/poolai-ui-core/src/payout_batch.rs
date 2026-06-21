//! Admin payout batch panel HTML renderer (PH-S564).

use crate::format::escape_html;
use serde_json::Value;

fn t(i18n: &Value, key: &str, fallback: &str) -> String {
    i18n.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or(fallback)
        .to_string()
}

fn format_lamports(n: Option<&Value>) -> String {
    match n.and_then(|v| v.as_u64()) {
        Some(v) => {
            let s = v.to_string();
            let mut out = String::new();
            for (i, ch) in s.chars().rev().enumerate() {
                if i > 0 && i % 3 == 0 {
                    out.push(',');
                }
                out.push(ch);
            }
            out.chars().rev().collect()
        }
        None => "—".to_string(),
    }
}

/// Render payout batch admin panel inner HTML from API JSON snapshots.
pub fn render_payout_batch_panel_html(
    latest_json: &str,
    history_json: &str,
    i18n_json: &str,
) -> String {
    let latest: Value = serde_json::from_str(latest_json).unwrap_or(Value::Null);
    let history: Value = serde_json::from_str(history_json).unwrap_or(Value::Null);
    let i18n: Value = serde_json::from_str(i18n_json).unwrap_or(Value::Null);

    let entry = latest.get("entry");
    let settlement_mode = latest
        .get("settlement_mode")
        .and_then(|v| v.as_str())
        .unwrap_or("offline_batch");
    let on_chain_pending = latest
        .get("on_chain_pending")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
        && entry.is_some();

    let rows: String = history
        .get("entries")
        .and_then(|v| v.as_array())
        .map(|entries| {
            entries
                .iter()
                .map(|row| {
                    format!(
                        "<tr><td><code>{}</code></td><td>{}</td><td>{}</td><td><code>{}</code></td></tr>",
                        escape_html(row.get("job_id").and_then(|v| v.as_str()).unwrap_or("—")),
                        escape_html(row.get("cleared_at").and_then(|v| v.as_str()).unwrap_or("—")),
                        escape_html(&format_lamports(row.get("gross_lamports"))),
                        escape_html(
                            row.get("payout_pubkey")
                                .and_then(|v| v.as_str())
                                .unwrap_or("—")
                        ),
                    )
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .unwrap_or_default();

    let rows_html = if rows.is_empty() {
        "<tr><td colspan=\"4\">—</td></tr>".to_string()
    } else {
        rows
    };

    format!(
        r#"<div class="admin-card"><h3>{latest_title}</h3><dl class="admin-dl">
<dt>{mode_label}</dt><dd><code>{settlement_mode}</code></dd>
<dt>{on_chain_label}</dt><dd>{on_chain_val}</dd>
<dt>{job_label}</dt><dd><code>{job_id}</code></dd>
<dt>{pubkey_label}</dt><dd><code>{pubkey}</code></dd>
</dl></div>
<div class="admin-card"><h3>{history_title}</h3>
<table class="admin-table"><thead><tr>
<th>{col_job}</th><th>{col_cleared}</th><th>{col_gross}</th><th>{col_pubkey}</th>
</tr></thead><tbody>{rows_html}</tbody></table></div>"#,
        latest_title = escape_html(&t(
            &i18n,
            "admin.payoutBatch.latest",
            "Latest cleared entry"
        )),
        mode_label = escape_html(&t(&i18n, "admin.payoutBatch.mode", "Settlement mode")),
        on_chain_label = escape_html(&t(&i18n, "admin.payoutBatch.onChain", "On-chain pending")),
        job_label = escape_html(&t(&i18n, "admin.payoutBatch.jobId", "Job ID")),
        pubkey_label = escape_html(&t(&i18n, "admin.payoutBatch.pubkey", "Payout pubkey")),
        history_title = escape_html(&t(&i18n, "admin.payoutBatch.history", "Recent history")),
        col_job = escape_html(&t(&i18n, "admin.payoutBatch.colJob", "Job")),
        col_cleared = escape_html(&t(&i18n, "admin.payoutBatch.colCleared", "Cleared")),
        col_gross = escape_html(&t(&i18n, "admin.payoutBatch.colGross", "Gross lamports")),
        col_pubkey = escape_html(&t(&i18n, "admin.payoutBatch.colPubkey", "Pubkey")),
        settlement_mode = escape_html(settlement_mode),
        on_chain_val = escape_html(if on_chain_pending { "yes" } else { "no" }),
        job_id = escape_html(
            entry
                .and_then(|e| e.get("job_id"))
                .and_then(|v| v.as_str())
                .unwrap_or("—")
        ),
        pubkey = escape_html(
            entry
                .and_then(|e| e.get("payout_pubkey"))
                .and_then(|v| v.as_str())
                .unwrap_or("—")
        ),
        rows_html = rows_html,
    )
}

/// Render payout batch history-only admin strip from history API JSON (PH-S771).
pub fn render_payout_batch_history_strip_html(history_json: &str, i18n_json: &str) -> String {
    let history: Value = serde_json::from_str(history_json).unwrap_or(Value::Null);
    let i18n: Value = serde_json::from_str(i18n_json).unwrap_or(Value::Null);
    let count = history
        .get("entries")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    let latest_job = history
        .get("entries")
        .and_then(|v| v.as_array())
        .and_then(|a| a.last())
        .and_then(|row| row.get("job_id"))
        .and_then(|v| v.as_str())
        .unwrap_or("—");
    format!(
        r#"<div class="admin-card admin-payout-batch-history-strip"><h3>{title}</h3>
<p><span>{count_label}</span>: <strong>{count}</strong></p>
<p><span>{latest_label}</span>: <code>{latest_job}</code></p></div>"#,
        title = escape_html(&t(
            &i18n,
            "admin.payoutBatch.historyStrip",
            "Recent payout batch history"
        )),
        count_label = escape_html(&t(&i18n, "admin.payoutBatch.historyCount", "Entries")),
        latest_label = escape_html(&t(&i18n, "admin.payoutBatch.historyLatest", "Latest job")),
        count = count,
        latest_job = escape_html(latest_job),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_payout_batch_history_strip_html_ph_s771() {
        let html = render_payout_batch_history_strip_html(
            r#"{"entries":[{"job_id":"j1","cleared_at":"t"},{"job_id":"j2","cleared_at":"t2"}]}"#,
            r#"{}"#,
        );
        assert!(html.contains("admin-payout-batch-history-strip"));
        assert!(html.contains("j2"));
        assert!(html.contains("<strong>2</strong>"));
    }

    #[test]
    fn render_payout_batch_panel_html_ph_s564() {
        let html = render_payout_batch_panel_html(
            r#"{"settlement_mode":"offline_batch","entry":{"job_id":"j1","payout_pubkey":"abc"}}"#,
            r#"{"entries":[{"job_id":"j1","cleared_at":"t","gross_lamports":1000,"payout_pubkey":"abc"}]}"#,
            r#"{}"#,
        );
        assert!(html.contains("j1"));
        assert!(html.contains("offline_batch"));
        assert!(html.contains("admin-table"));
    }
}
