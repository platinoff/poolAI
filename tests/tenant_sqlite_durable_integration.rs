//! PH-S1231: Tenant SQLite durable API contracts (band 59).
//! Marker: tenant_sqlite_durable_integration
//!
//! Restart-safe create/get + cross-tenant isolation under sqlite mode.

#[cfg(feature = "enterprise")]
use poolai::enterprise::multi_tenancy::{TenantConfig, TenantManager, TENANT_STORE_SQLITE_FILE};
#[cfg(feature = "enterprise")]
use uuid::Uuid;

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn sqlite_restart_create_get_survives_recreate_ph_s1231() {
    let dir = std::env::temp_dir().join(format!("poolai-tenant-durable-{}", Uuid::new_v4()));
    let db = dir.join(TENANT_STORE_SQLITE_FILE);
    let mgr = TenantManager::new_with_sqlite_path(db.clone());
    mgr.initialize().await.expect("init");
    let t = mgr
        .create_tenant("alpha".into(), TenantConfig::default())
        .await
        .expect("create");
    let id = t.id;
    drop(mgr);

    let reloaded = TenantManager::new_with_sqlite_path(db);
    reloaded.initialize().await.expect("reload init");
    let found = reloaded
        .get_tenant(id)
        .await
        .expect("get")
        .expect("present");
    assert_eq!(found.name, "alpha");
    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn sqlite_cross_tenant_isolation_ph_s1231() {
    let dir = std::env::temp_dir().join(format!("poolai-tenant-iso-{}", Uuid::new_v4()));
    let db = dir.join(TENANT_STORE_SQLITE_FILE);
    let mgr = TenantManager::new_with_sqlite_path(db.clone());
    mgr.initialize().await.expect("init");
    let a = mgr
        .create_tenant("tenant-a".into(), TenantConfig::default())
        .await
        .expect("a");
    let b = mgr
        .create_tenant("tenant-b".into(), TenantConfig::default())
        .await
        .expect("b");
    assert_ne!(a.id, b.id);
    mgr.increment_usage(a.id, 1, 64, 1, None, None)
        .await
        .expect("usage a");
    drop(mgr);

    let reloaded = TenantManager::new_with_sqlite_path(db);
    reloaded.initialize().await.expect("reload");
    let usage_a = reloaded.get_usage(a.id).await.expect("usage a");
    let usage_b = reloaded.get_usage(b.id).await.expect("usage b");
    assert_eq!(usage_a.workers, 1);
    assert_eq!(usage_a.memory_mb, 64);
    assert_eq!(usage_b.workers, 0);
    assert_eq!(usage_b.memory_mb, 0);
    let _ = std::fs::remove_dir_all(&dir);
}
