//! PH-S06: single-host multi-node Raft harness (HTTP RPC on ephemeral ports).
//!
//! Run: `cargo test-raft-ci` or
//! `cargo test -j 1 --test raft_multi_node_harness --features raft,test-utils -- --test-threads=1`

#![cfg(feature = "raft")]

use axum::Router;
use poolai::network::api::raft_rpc::create_raft_rpc_routes;
use poolai::raid::raft::{RaftConfig, RaidRaftNode};
use poolai::raid::{RaidConfig, RaidManager, RaidMode};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tempfile::TempDir;
use tokio::net::TcpListener;
use tokio::sync::{oneshot, RwLock};

struct HarnessNode {
    node_id: u64,
    base_url: String,
    raft: Arc<RaidRaftNode>,
    _shutdown: oneshot::Sender<()>,
}

async fn spawn_harness_node(temp: &TempDir, node_id: u64, members: &[u64]) -> HarnessNode {
    let node_path = temp.path().join(format!("node-{node_id}"));
    let raid_config = RaidConfig {
        mode: RaidMode::Local,
        base_path: node_path.clone(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };
    let raid_manager = Arc::new(RwLock::new(RaidManager::new(raid_config)));
    raid_manager
        .write()
        .await
        .initialize()
        .await
        .expect("raid init");

    let raft_config = RaftConfig {
        node_id,
        cluster_members: members.to_vec(),
        election_timeout: 500,
        heartbeat_interval: 50,
    };
    let raft = Arc::new(
        RaidRaftNode::new(raft_config, raid_manager, node_path.join("raft")).expect("raft node"),
    );

    let app: Router = create_raft_rpc_routes(raft.clone());
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("local addr");
    let base_url = format!("http://{addr}");
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                let _ = shutdown_rx.await;
            })
            .await
            .ok();
    });

    HarnessNode {
        node_id,
        base_url,
        raft,
        _shutdown: shutdown_tx,
    }
}

async fn wire_transport(nodes: &[&HarnessNode]) {
    for n in nodes {
        for peer in nodes {
            n.raft
                .transport()
                .add_node(peer.node_id, peer.base_url.clone())
                .await;
        }
    }
}

#[tokio::test]
async fn harness_two_node_http_rpc_listening() {
    let temp = TempDir::new().unwrap();
    let members = vec![1_u64, 2];
    let n1 = spawn_harness_node(&temp, 1, &members).await;
    let n2 = spawn_harness_node(&temp, 2, &members).await;

    let client = reqwest::Client::new();
    for url in [&n1.base_url, &n2.base_url] {
        let vote_url = format!("{url}/raft/vote");
        let res = client
            .post(&vote_url)
            .header("content-type", "application/json")
            .body("{}")
            .send()
            .await
            .expect("vote post");
        assert!(
            res.status().is_client_error() || res.status().is_server_error(),
            "raft vote endpoint should respond (not connection refused): {}",
            res.status()
        );
    }
}

#[tokio::test]
async fn harness_two_node_single_host_bootstrap() {
    let temp = TempDir::new().unwrap();
    let members = vec![1_u64, 2];
    let n1 = spawn_harness_node(&temp, 1, &members).await;
    let n2 = spawn_harness_node(&temp, 2, &members).await;
    wire_transport(&[&n1, &n2]).await;

    let url1 = n1.base_url.clone();
    let url2 = n2.base_url.clone();

    n1.raft.initialize().await.expect("init n1");
    n2.raft.initialize().await.expect("init n2");
    n1.raft.initialize_cluster().await.expect("cluster init");

    tokio::time::sleep(Duration::from_millis(800)).await;

    let leader1 = n1.raft.get_current_leader().await;
    let leader2 = n2.raft.get_current_leader().await;
    let metrics1 = n1.raft.get_metrics().await.expect("metrics n1");
    let metrics2 = n2.raft.get_metrics().await.expect("metrics n2");

    assert!(metrics1.contains("term:"));
    assert!(metrics2.contains("term:"));
    assert!(
        leader1.is_some() || leader2.is_some(),
        "expected a leader on at least one node (n1={leader1:?}, n2={leader2:?})"
    );

    let mut addrs: HashMap<u64, String> = HashMap::new();
    for n in [&n1, &n2] {
        addrs.insert(
            n.node_id,
            n.raft
                .transport()
                .get_node_address(n.node_id)
                .await
                .expect("addr"),
        );
    }
    assert_eq!(addrs.get(&1).unwrap(), &url1);
    assert_eq!(addrs.get(&2).unwrap(), &url2);
}
