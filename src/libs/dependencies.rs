//! Dependency Resolver - Resolves library dependencies
//!
//! Provides:
//! - Dependency resolution
//! - Conflict detection
//! - Dependency graph

use crate::core::error::AppError;
use crate::libs::constraints::{VersionConstraint, parse_constraints, satisfies_all};
use crate::libs::registry::LibraryRegistry;
use std::collections::{HashMap, HashSet};
use tracing::info;

/// Dependency specification
#[derive(Debug, Clone)]
pub struct DependencySpec {
    pub name: String,
    pub constraints: Vec<VersionConstraint>,
}

/// Dependency Resolver - Resolves library dependencies
pub struct DependencyResolver {
    dependency_graph: HashMap<String, Vec<DependencySpec>>, // name -> dependencies with constraints
}

/// Resolved dependency with selected version.
#[derive(Debug, Clone)]
pub struct ResolvedDependency {
    pub name: String,
    pub version: String,
}

impl DependencyResolver {
    /// Create new dependency resolver
    pub fn new() -> Self {
        Self {
            dependency_graph: HashMap::new(),
        }
    }

    /// Resolve dependencies for a library
    pub async fn resolve(
        &self,
        name: &str,
        version: &str,
    ) -> Result<Vec<String>, AppError> {
        info!("Resolving dependencies for {} v{}", name, version);
        
        // Check if library has dependencies
        if let Some(deps) = self.dependency_graph.get(name) {
            // Filter dependencies that satisfy constraints
            let mut resolved = Vec::new();
            
            for dep_spec in deps {
                // For now, we'll return all dependencies
                // In full implementation, we'd check version constraints
                resolved.push(dep_spec.name.clone());
            }
            
            info!("Resolved {} dependencies for {} v{}", resolved.len(), name, version);
            Ok(resolved)
        } else {
            Ok(Vec::new())
        }
    }

    /// Resolve dependencies and select versions using the registry (production-min).
    pub fn resolve_versions(
        &self,
        name: &str,
        version: &str,
        registry: &LibraryRegistry,
    ) -> Result<Vec<ResolvedDependency>, AppError> {
        info!("Resolving dependency versions for {} v{}", name, version);

        let Some(deps) = self.dependency_graph.get(name) else {
            return Ok(Vec::new());
        };

        let mut resolved = Vec::new();
        for dep_spec in deps {
            let versions = registry
                .get_versions(&dep_spec.name)
                .ok_or_else(|| AppError::ConfigError(format!("No versions available for dependency {}", dep_spec.name)))?;

            // choose latest version that satisfies constraints (versions are sorted in registry)
            let mut chosen: Option<String> = None;
            for v in versions.iter().rev() {
                let ok = satisfies_all(v, &dep_spec.constraints)?;
                if ok {
                    chosen = Some(v.clone());
                    break;
                }
            }

            let version = chosen.ok_or_else(|| {
                AppError::ConfigError(format!(
                    "No compatible version for dependency {} (constraints: {})",
                    dep_spec.name,
                    constraints_to_string(&dep_spec.constraints)
                ))
            })?;

            resolved.push(ResolvedDependency {
                name: dep_spec.name.clone(),
                version,
            });
        }

        Ok(resolved)
    }
    
    /// Add dependency with version constraints
    pub fn add_dependency_with_constraints(
        &mut self,
        library: &str,
        dependency: &str,
        constraints: &str,
    ) -> Result<(), AppError> {
        let parsed_constraints = parse_constraints(constraints)?;
        
        let deps = self.dependency_graph
            .entry(library.to_string())
            .or_insert_with(Vec::new);
        
        deps.push(DependencySpec {
            name: dependency.to_string(),
            constraints: parsed_constraints,
        });
        
        Ok(())
    }

    /// Add dependency relationship (simple version without constraints)
    pub fn add_dependency(&mut self, library: &str, dependency: &str) {
        self.dependency_graph
            .entry(library.to_string())
            .or_insert_with(Vec::new)
            .push(DependencySpec {
                name: dependency.to_string(),
                constraints: Vec::new(),
            });
    }

    /// Check for dependency conflicts
    pub async fn check_conflicts(
        &self,
        dependencies: &[String],
    ) -> Result<(), AppError> {
        // Check for circular dependencies
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        for dep in dependencies {
            if self.has_circular_dependency(dep, &mut visited, &mut rec_stack)? {
                return Err(AppError::ConfigError(
                    format!("Circular dependency detected involving: {}", dep)
                ));
            }
        }
        
        Ok(())
    }
    
    /// Check for circular dependencies using DFS
    fn has_circular_dependency(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> Result<bool, AppError> {
        if rec_stack.contains(node) {
            return Ok(true); // Circular dependency found
        }
        
        if visited.contains(node) {
            return Ok(false); // Already processed
        }
        
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        
        if let Some(deps) = self.dependency_graph.get(node) {
            for dep_spec in deps {
                if self.has_circular_dependency(&dep_spec.name, visited, rec_stack)? {
                    return Ok(true);
                }
            }
        }
        
        rec_stack.remove(node);
        Ok(false)
    }

    /// Build dependency graph
    pub fn build_graph(&self, root: &str) -> Result<Vec<String>, AppError> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        
        self.dfs(root, &mut visited, &mut result)?;
        
        Ok(result)
    }

    /// Depth-first search for dependency graph
    fn dfs(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        result: &mut Vec<String>,
    ) -> Result<(), AppError> {
        if visited.contains(node) {
            return Ok(());
        }
        
        visited.insert(node.to_string());
        
        if let Some(deps) = self.dependency_graph.get(node) {
            for dep_spec in deps {
                self.dfs(&dep_spec.name, visited, result)?;
            }
        }
        
        result.push(node.to_string());
        Ok(())
    }
}

fn constraints_to_string(constraints: &[VersionConstraint]) -> String {
    if constraints.is_empty() {
        return "(none)".to_string();
    }
    constraints
        .iter()
        .map(|c| format!("{:?}{}", c.operator, c.version))
        .collect::<Vec<_>>()
        .join(",")
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

