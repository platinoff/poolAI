//! In-memory task queue for virtual-node workers (FM-016 phase 3).
//!
//! Telegram / device workers poll the coordinator, execute lightweight tasks,
//! and report completion. Production would persist to a job store.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualNodeTask {
    pub id: String,
    pub task_type: String,
    #[serde(default)]
    pub payload: Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCompletionRecord {
    pub task_id: String,
    pub status: String,
    pub detail: Option<String>,
    pub completed_at: DateTime<Utc>,
}

fn queues() -> &'static Mutex<HashMap<String, VecDeque<VirtualNodeTask>>> {
    static Q: OnceLock<Mutex<HashMap<String, VecDeque<VirtualNodeTask>>>> = OnceLock::new();
    Q.get_or_init(|| Mutex::new(HashMap::new()))
}

fn completions() -> &'static Mutex<HashMap<String, Vec<TaskCompletionRecord>>> {
    static C: OnceLock<Mutex<HashMap<String, Vec<TaskCompletionRecord>>>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(HashMap::new()))
}

pub struct VirtualNodeTaskService;

impl VirtualNodeTaskService {
    pub fn enqueue(peer_id: &str, task_type: &str, payload: Value) -> VirtualNodeTask {
        let task = VirtualNodeTask {
            id: Uuid::new_v4().to_string(),
            task_type: task_type.to_string(),
            payload,
            created_at: Utc::now(),
        };
        let mut guard = queues().lock().expect("virtual node task queue lock");
        guard
            .entry(peer_id.to_string())
            .or_default()
            .push_back(task.clone());
        task
    }

    /// Default tasks after virtual-node registration (ping + RAID wire check).
    pub fn enqueue_bootstrap_tasks(peer_id: &str) {
        if Self::pending_count(peer_id) > 0 || Self::completed_count(peer_id) > 0 {
            return;
        }
        Self::enqueue(peer_id, "ping", Value::Object(Default::default()));
        Self::enqueue(
            peer_id,
            "raid_health_check",
            Value::Object(Default::default()),
        );
    }

    pub fn poll(peer_id: &str) -> Option<VirtualNodeTask> {
        let mut guard = queues().lock().expect("virtual node task queue lock");
        guard.get_mut(peer_id)?.pop_front()
    }

    pub fn complete(peer_id: &str, task_id: &str, status: &str, detail: Option<String>) -> bool {
        let record = TaskCompletionRecord {
            task_id: task_id.to_string(),
            status: status.to_string(),
            detail,
            completed_at: Utc::now(),
        };
        let mut guard = completions().lock().expect("virtual node completions lock");
        guard.entry(peer_id.to_string()).or_default().push(record);
        true
    }

    pub fn pending_count(peer_id: &str) -> usize {
        let guard = queues().lock().expect("virtual node task queue lock");
        guard.get(peer_id).map(|q| q.len()).unwrap_or(0)
    }

    pub fn completed_count(peer_id: &str) -> usize {
        let guard = completions().lock().expect("virtual node completions lock");
        guard.get(peer_id).map(|v| v.len()).unwrap_or(0)
    }

    /// Test helper — reset queue state for a peer.
    pub fn clear_peer(peer_id: &str) {
        let mut q = queues().lock().expect("lock");
        q.remove(peer_id);
        let mut c = completions().lock().expect("lock");
        c.remove(peer_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poll_fifo_and_complete() {
        let peer = "test-peer-tasks";
        VirtualNodeTaskService::clear_peer(peer);
        VirtualNodeTaskService::enqueue(peer, "ping", Value::Null);
        let t = VirtualNodeTaskService::poll(peer).expect("task");
        assert_eq!(t.task_type, "ping");
        VirtualNodeTaskService::complete(peer, &t.id, "ok", None);
        assert_eq!(VirtualNodeTaskService::completed_count(peer), 1);
        VirtualNodeTaskService::clear_peer(peer);
    }
}
