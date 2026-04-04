//! Short-lived OAuth2 CSRF state map (enterprise); stored on `AppState`.

use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct OAuth2PendingEntry {
    pub created_at: DateTime<Utc>,
}

pub async fn store_oauth2_pending(
    store: &Arc<RwLock<HashMap<String, OAuth2PendingEntry>>>,
    state: String,
) {
    let mut states = store.write().await;
    states.insert(
        state.clone(),
        OAuth2PendingEntry {
            created_at: Utc::now(),
        },
    );
    let cutoff = Utc::now() - Duration::minutes(10);
    states.retain(|_, s| s.created_at > cutoff);
}

pub async fn verify_oauth2_pending(
    store: &Arc<RwLock<HashMap<String, OAuth2PendingEntry>>>,
    state: &str,
) -> bool {
    let mut states = store.write().await;
    let cutoff = Utc::now() - Duration::minutes(10);
    states.retain(|_, s| s.created_at > cutoff);

    if let Some(entry) = states.get(state) {
        if entry.created_at > cutoff {
            states.remove(state);
            return true;
        }
    }
    false
}
