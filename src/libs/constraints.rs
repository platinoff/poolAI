//! Version Constraints Module
//!
//! Provides:
//! - Version constraint parsing (>=, <=, ~, ^, ==)
//! - Constraint satisfaction checking
//! - Version range operations

use crate::core::error::AppError;
use std::cmp::Ordering;

/// Version constraint operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintOp {
    /// Greater than or equal (>=)
    GreaterEqual,
    /// Less than or equal (<=)
    LessEqual,
    /// Exact match (==)
    Exact,
    /// Compatible version (~)
    Compatible,
    /// Caret version (^)
    Caret,
    /// Greater than (>)
    Greater,
    /// Less than (<)
    Less,
}

/// Version constraint
#[derive(Debug, Clone)]
pub struct VersionConstraint {
    pub operator: ConstraintOp,
    pub version: String,
}

impl VersionConstraint {
    /// Parse version constraint from string
    /// Examples: ">=1.2.3", "~2.0.0", "^1.0.0", "==1.5.0"
    pub fn parse(constraint: &str) -> Result<Self, AppError> {
        let constraint = constraint.trim();
        
        if constraint.starts_with(">=") {
            Ok(Self {
                operator: ConstraintOp::GreaterEqual,
                version: constraint[2..].to_string(),
            })
        } else if constraint.starts_with("<=") {
            Ok(Self {
                operator: ConstraintOp::LessEqual,
                version: constraint[2..].to_string(),
            })
        } else if constraint.starts_with("==") {
            Ok(Self {
                operator: ConstraintOp::Exact,
                version: constraint[2..].to_string(),
            })
        } else if constraint.starts_with("~") {
            Ok(Self {
                operator: ConstraintOp::Compatible,
                version: constraint[1..].to_string(),
            })
        } else if constraint.starts_with("^") {
            Ok(Self {
                operator: ConstraintOp::Caret,
                version: constraint[1..].to_string(),
            })
        } else if constraint.starts_with(">") {
            Ok(Self {
                operator: ConstraintOp::Greater,
                version: constraint[1..].to_string(),
            })
        } else if constraint.starts_with("<") {
            Ok(Self {
                operator: ConstraintOp::Less,
                version: constraint[1..].to_string(),
            })
        } else {
            // Default to exact match
            Ok(Self {
                operator: ConstraintOp::Exact,
                version: constraint.to_string(),
            })
        }
    }
    
    /// Check if version satisfies constraint
    pub fn satisfies(&self, version: &str) -> Result<bool, AppError> {
        let ordering = compare_versions(version, &self.version)?;
        
        match self.operator {
            ConstraintOp::Exact => Ok(ordering == Ordering::Equal),
            ConstraintOp::GreaterEqual => Ok(ordering != Ordering::Less),
            ConstraintOp::LessEqual => Ok(ordering != Ordering::Greater),
            ConstraintOp::Greater => Ok(ordering == Ordering::Greater),
            ConstraintOp::Less => Ok(ordering == Ordering::Less),
            ConstraintOp::Compatible => {
                // ~1.2.3 means >=1.2.3 and <1.3.0
                let compatible_ordering = compare_versions(version, &self.version)?;
                if compatible_ordering == Ordering::Less {
                    return Ok(false);
                }
                
                // Parse version to get next minor version
                let parts: Vec<u32> = self.version
                    .split('.')
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect();
                
                if parts.len() >= 2 {
                    let next_minor = format!("{}.{}.0", parts[0], parts[1] + 1);
                    let next_ordering = compare_versions(version, &next_minor)?;
                    Ok(next_ordering == Ordering::Less)
                } else {
                    Ok(compatible_ordering != Ordering::Less)
                }
            }
            ConstraintOp::Caret => {
                // ^1.2.3 means >=1.2.3 and <2.0.0
                let caret_ordering = compare_versions(version, &self.version)?;
                if caret_ordering == Ordering::Less {
                    return Ok(false);
                }
                
                // Parse version to get next major version
                let parts: Vec<u32> = self.version
                    .split('.')
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect();
                
                if !parts.is_empty() {
                    let next_major = format!("{}.0.0", parts[0] + 1);
                    let next_ordering = compare_versions(version, &next_major)?;
                    Ok(next_ordering == Ordering::Less)
                } else {
                    Ok(caret_ordering != Ordering::Less)
                }
            }
        }
    }
}

/// Compare semantic versions
fn compare_versions(a: &str, b: &str) -> Result<Ordering, AppError> {
    let a_parts: Vec<u32> = a
        .split('.')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();
    
    let b_parts: Vec<u32> = b
        .split('.')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();
    
    if a_parts.is_empty() || b_parts.is_empty() {
        return Err(AppError::ConfigError(format!(
            "Invalid version format: {} or {}",
            a, b
        )));
    }
    
    // Compare major, minor, patch
    for i in 0..3 {
        let a_val = a_parts.get(i).copied().unwrap_or(0);
        let b_val = b_parts.get(i).copied().unwrap_or(0);
        
        match a_val.cmp(&b_val) {
            Ordering::Equal => continue,
            other => return Ok(other),
        }
    }
    
    // If all components are equal, compare as strings (for pre-release, build metadata)
    Ok(a.cmp(b))
}

/// Parse multiple constraints
pub fn parse_constraints(constraints: &str) -> Result<Vec<VersionConstraint>, AppError> {
    constraints
        .split(',')
        .map(|c| VersionConstraint::parse(c.trim()))
        .collect()
}

/// Check if version satisfies all constraints
pub fn satisfies_all(version: &str, constraints: &[VersionConstraint]) -> Result<bool, AppError> {
    for constraint in constraints {
        if !constraint.satisfies(version)? {
            return Ok(false);
        }
    }
    Ok(true)
}

