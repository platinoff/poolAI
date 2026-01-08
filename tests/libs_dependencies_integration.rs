//! Integration tests for Libs Dependencies Module

use poolai::libs::dependencies::{DependencyResolver, ResolvedDependency};
use poolai::libs::registry::LibraryRegistry;

#[tokio::test]
async fn test_dependency_resolver_creation() {
    let resolver = DependencyResolver::new();
    // Just verify it can be created
    let _ = resolver;
}

#[tokio::test]
async fn test_dependency_resolver_resolve_versions() {
    let resolver = DependencyResolver::new();
    let registry = LibraryRegistry::new();
    
    // Test resolving dependencies (may return empty if no dependencies)
    let result = resolver.resolve_versions("test-lib", "1.0.0", &registry);
    
    // Should not panic, may return empty vector or error
    let _ = result;
}

#[tokio::test]
async fn test_resolved_dependency_structure() {
    let dep = ResolvedDependency {
        name: "test-dep".to_string(),
        version: "1.0.0".to_string(),
    };
    
    assert_eq!(dep.name, "test-dep");
    assert_eq!(dep.version, "1.0.0");
}

#[tokio::test]
async fn test_resolved_dependency_clone() {
    let dep = ResolvedDependency {
        name: "test-dep".to_string(),
        version: "1.0.0".to_string(),
    };
    
    let cloned = dep.clone();
    assert_eq!(dep.name, cloned.name);
    assert_eq!(dep.version, cloned.version);
}
