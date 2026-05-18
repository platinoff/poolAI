//! Task outcome logic for `poolai-worker` (FM-016+++).

use serde_json::Value;

/// Inputs gathered by the worker before completing a task.
#[derive(Debug, Clone, Default)]
pub struct TaskRuntime {
    pub raid_wire_ok: Option<bool>,
    pub pool_worker_count: Option<usize>,
}

/// Returns `(completion_status, success)`.
pub fn complete_task(task_type: &str, payload: &Value, rt: &TaskRuntime) -> (String, bool) {
    match task_type {
        "ping" => ("ok".to_string(), true),
        "raid_health_check" => {
            let ok = rt.raid_wire_ok.unwrap_or(false);
            (
                if ok {
                    "raid_ok".to_string()
                } else {
                    "raid_failed".to_string()
                },
                ok,
            )
        }
        "pool_workers_probe" => {
            let count = rt.pool_worker_count.unwrap_or(0);
            (format!("pool_workers:{count}"), true)
        }
        "telegram_command" => handle_telegram_command(payload, rt),
        "telegram_message" => {
            let preview = payload
                .get("text")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .chars()
                .take(80)
                .collect::<String>();
            (format!("received:{preview}"), true)
        }
        other => (format!("unsupported:{other}"), false),
    }
}

fn handle_telegram_command(payload: &Value, rt: &TaskRuntime) -> (String, bool) {
    let text = payload
        .get("text")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim();
    let cmd = text.split_whitespace().next().unwrap_or(text);
    match cmd {
        "/status" => {
            let pool = rt.pool_worker_count.unwrap_or(0);
            let raid = rt
                .raid_wire_ok
                .map(|ok| if ok { "ok" } else { "fail" })
                .unwrap_or("unknown");
            (format!("status:pool={pool},raid={raid}"), true)
        }
        "/raid" => {
            let ok = rt.raid_wire_ok.unwrap_or(false);
            (
                if ok {
                    "raid_ok".to_string()
                } else {
                    "raid_failed".to_string()
                },
                ok,
            )
        }
        _ => (format!("ack:{cmd}"), true),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_and_pool_probe() {
        let rt = TaskRuntime {
            pool_worker_count: Some(3),
            ..Default::default()
        };
        assert_eq!(
            complete_task("ping", &Value::Null, &rt),
            ("ok".into(), true)
        );
        assert_eq!(
            complete_task("pool_workers_probe", &Value::Null, &rt),
            ("pool_workers:3".into(), true)
        );
    }

    #[test]
    fn telegram_status_command() {
        let payload = serde_json::json!({ "text": "/status" });
        let rt = TaskRuntime {
            raid_wire_ok: Some(true),
            pool_worker_count: Some(2),
        };
        let (status, ok) = complete_task("telegram_command", &payload, &rt);
        assert!(ok);
        assert!(status.contains("pool=2"));
        assert!(status.contains("raid=ok"));
    }
}
