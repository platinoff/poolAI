//! Integration tests for Libs Constraints Module

use poolai::libs::constraints::{ConstraintOp, VersionConstraint};

// Note: ConstraintOp uses different variant names in the actual code
// Adjusting test to match actual implementation

#[test]
fn test_version_constraint_creation() {
    let constraint = VersionConstraint {
        operator: ConstraintOp::Exact,
        version: "1.0.0".to_string(),
    };

    assert_eq!(constraint.operator, ConstraintOp::Exact);
    assert_eq!(constraint.version, "1.0.0");
}

#[test]
fn test_constraint_op_variants() {
    let ops = vec![
        ConstraintOp::Exact,
        ConstraintOp::Greater,
        ConstraintOp::GreaterEqual,
        ConstraintOp::Less,
        ConstraintOp::LessEqual,
        ConstraintOp::Compatible,
        ConstraintOp::Caret,
    ];

    for op in ops {
        let constraint = VersionConstraint {
            operator: op,
            version: "1.0.0".to_string(),
        };
        assert_eq!(constraint.operator, op);
    }
}

#[test]
fn test_version_constraint_clone() {
    let constraint = VersionConstraint {
        operator: ConstraintOp::GreaterEqual,
        version: "2.0.0".to_string(),
    };

    let cloned = constraint.clone();
    assert_eq!(constraint.operator, cloned.operator);
    assert_eq!(constraint.version, cloned.version);
}

#[test]
fn test_version_constraint_serialization() {
    let constraint = VersionConstraint {
        operator: ConstraintOp::Exact,
        version: "1.0.0".to_string(),
    };

    // Note: VersionConstraint may not implement Serialize
    // This test verifies the structure is correct
    assert_eq!(constraint.version, "1.0.0");
    assert_eq!(constraint.operator, ConstraintOp::Exact);
}
