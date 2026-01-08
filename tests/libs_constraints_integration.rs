//! Integration tests for Libs Constraints Module

use poolai::libs::constraints::{ConstraintOp, VersionConstraint};

#[test]
fn test_version_constraint_creation() {
    let constraint = VersionConstraint {
        op: ConstraintOp::Eq,
        version: "1.0.0".to_string(),
    };
    
    assert_eq!(constraint.op, ConstraintOp::Eq);
    assert_eq!(constraint.version, "1.0.0");
}

#[test]
fn test_constraint_op_variants() {
    let ops = vec![
        ConstraintOp::Eq,
        ConstraintOp::Gt,
        ConstraintOp::Gte,
        ConstraintOp::Lt,
        ConstraintOp::Lte,
        ConstraintOp::Ne,
    ];
    
    for op in ops {
        let constraint = VersionConstraint {
            op: op.clone(),
            version: "1.0.0".to_string(),
        };
        assert_eq!(constraint.op, op);
    }
}

#[test]
fn test_version_constraint_clone() {
    let constraint = VersionConstraint {
        op: ConstraintOp::Gte,
        version: "2.0.0".to_string(),
    };
    
    let cloned = constraint.clone();
    assert_eq!(constraint.op, cloned.op);
    assert_eq!(constraint.version, cloned.version);
}

#[test]
fn test_version_constraint_serialization() {
    let constraint = VersionConstraint {
        op: ConstraintOp::Eq,
        version: "1.0.0".to_string(),
    };
    
    let json = serde_json::to_string(&constraint).unwrap();
    assert!(json.contains("1.0.0"));
    assert!(json.contains("Eq") || json.contains("eq"));
}
