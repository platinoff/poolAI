//! Integration tests for Runtime Instance Library Model Loading
//!
//! Tests the functionality of loading models from libraries in runtime instances.

use poolai::runtime::instance::{InstanceManager, InstancePlacement, PlacementStrategy};
use std::collections::HashMap;

#[tokio::test]
async fn test_instance_creation_with_library_model() {
    let instance_manager = InstanceManager::new();

    // Create placement
    let placement = InstancePlacement {
        strategy: PlacementStrategy::Single,
        node_ids: vec!["node1".to_string()],
        memory_by_node: HashMap::new(),
        memory_delta: 0,
        error: None,
    };

    // Create instance - library loading happens automatically if library exists
    let instance_id = instance_manager
        .create_instance("library-model".to_string(), placement, HashMap::new())
        .await
        .unwrap();

    // Verify instance was created
    let got = instance_manager.get_instance(&instance_id).await;
    assert!(got.is_some());

    let instance = got.unwrap();
    assert_eq!(instance.model_id, "library-model");
    assert_eq!(instance.instance_id, instance_id);
}
