//! Task queue for virtual-node workers (FM-016 phase 3, FM-016+ persistence).
//!
//! Telegram / device workers poll the coordinator, execute lightweight tasks,
//! and report completion. Set `POOLAI_VIRTUAL_NODE_DATA_DIR` for file-backed queues.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

use super::virtual_node_store;

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct PeerTaskSnapshot {
    queue: VecDeque<VirtualNodeTask>,
    completed: Vec<TaskCompletionRecord>,
}

fn load_peer_snapshot(peer_id: &str) -> PeerTaskSnapshot {
    let Some(path) = virtual_node_store::peer_tasks_path(peer_id) else {
        return PeerTaskSnapshot::default();
    };
    if !path.exists() {
        return PeerTaskSnapshot::default();
    }
    match std::fs::read(&path) {
        Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
        Err(e) => {
            tracing::warn!("load virtual node tasks {}: {e}", path.display());
            PeerTaskSnapshot::default()
        }
    }
}

fn persist_peer_snapshot(peer_id: &str, snapshot: &PeerTaskSnapshot) {
    let Some(path) = virtual_node_store::peer_tasks_path(peer_id) else {
        return;
    };
    if let Err(e) = virtual_node_store::write_json_atomic(&path, snapshot) {
        tracing::warn!("persist virtual node tasks {}: {e}", path.display());
    }
}

fn ensure_peer_loaded(peer_id: &str) {
    let mut q = queues().lock().expect("virtual node task queue lock");
    if q.contains_key(peer_id) {
        return;
    }
    let snapshot = load_peer_snapshot(peer_id);
    if !snapshot.queue.is_empty() {
        q.insert(peer_id.to_string(), snapshot.queue);
    }
    drop(q);
    let mut c = completions().lock().expect("virtual node completions lock");
    if !snapshot.completed.is_empty() {
        c.insert(peer_id.to_string(), snapshot.completed);
    }
}

fn persist_peer(peer_id: &str) {
    let q = queues().lock().expect("virtual node task queue lock");
    let c = completions().lock().expect("virtual node completions lock");
    let snapshot = PeerTaskSnapshot {
        queue: q.get(peer_id).cloned().unwrap_or_default(),
        completed: c.get(peer_id).cloned().unwrap_or_default(),
    };
    drop(q);
    drop(c);
    persist_peer_snapshot(peer_id, &snapshot);
}

pub struct VirtualNodeTaskService;

impl VirtualNodeTaskService {
    pub fn enqueue(peer_id: &str, task_type: &str, payload: Value) -> VirtualNodeTask {
        ensure_peer_loaded(peer_id);
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
        drop(guard);
        persist_peer(peer_id);
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
        Self::enqueue(
            peer_id,
            "pool_workers_probe",
            Value::Object(Default::default()),
        );
    }

    pub fn poll(peer_id: &str) -> Option<VirtualNodeTask> {
        ensure_peer_loaded(peer_id);
        let mut guard = queues().lock().expect("virtual node task queue lock");
        let task = guard.get_mut(peer_id)?.pop_front();
        drop(guard);
        if task.is_some() {
            persist_peer(peer_id);
        }
        task
    }

    pub fn complete(peer_id: &str, task_id: &str, status: &str, detail: Option<String>) -> bool {
        ensure_peer_loaded(peer_id);
        let record = TaskCompletionRecord {
            task_id: task_id.to_string(),
            status: status.to_string(),
            detail,
            completed_at: Utc::now(),
        };
        let mut guard = completions().lock().expect("virtual node completions lock");
        guard.entry(peer_id.to_string()).or_default().push(record);
        drop(guard);
        persist_peer(peer_id);
        true
    }

    pub fn pending_count(peer_id: &str) -> usize {
        ensure_peer_loaded(peer_id);
        let guard = queues().lock().expect("virtual node task queue lock");
        guard.get(peer_id).map(|q| q.len()).unwrap_or(0)
    }

    pub fn completed_count(peer_id: &str) -> usize {
        ensure_peer_loaded(peer_id);
        let guard = completions().lock().expect("virtual node completions lock");
        guard.get(peer_id).map(|v| v.len()).unwrap_or(0)
    }

    /// Test helper — reset queue state for a peer.
    pub fn clear_peer(peer_id: &str) {
        let mut q = queues().lock().expect("lock");
        q.remove(peer_id);
        let mut c = completions().lock().expect("lock");
        c.remove(peer_id);
        if let Some(path) = virtual_node_store::peer_tasks_path(peer_id) {
            let _ = std::fs::remove_file(path);
        }
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
