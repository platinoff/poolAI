//! Integration tests for Security (JWT/HTTPS)
//!
//! Tests:
//! - JWT token generation (with and without feature)
//! - JWT token validation
//! - Authentication flow
//! - Fallback mode when JWT feature disabled

use std::sync::Arc;

use poolai::network::auth::{
    authenticate_user, generate_token, validate_token, AuthRequest, UserManager, UserRole,
};

#[tokio::test]
async fn test_token_generation_fallback() {
    // Test token generation in fallback mode (without JWT feature)
    let token = generate_token("test_user", UserRole::Viewer).unwrap();

    // In fallback mode, token should start with "dev_token_"
    #[cfg(not(feature = "jwt"))]
    {
        assert!(
            token.starts_with("dev_token_"),
            "Fallback token should start with 'dev_token_'"
        );
    }

    // Token should be valid
    let claims = validate_token(&token).unwrap();
    assert_eq!(claims.sub, "test_user");
    assert_eq!(claims.role, UserRole::Viewer);
}

#[tokio::test]
async fn test_token_validation_fallback() {
    // Test token validation in fallback mode
    let token = generate_token("test_user", UserRole::Operator).unwrap();

    let claims = validate_token(&token).unwrap();
    assert_eq!(claims.sub, "test_user");
    assert_eq!(claims.role, UserRole::Operator);
    assert!(!claims.permissions.is_empty());
}

#[tokio::test]
async fn test_token_expiration() {
    // Test token expiration check
    let token = generate_token("test_user", UserRole::Viewer).unwrap();

    // Token should be valid immediately after generation
    let result = validate_token(&token);
    assert!(
        result.is_ok(),
        "Token should be valid immediately after generation"
    );

    // Invalid token should fail
    let invalid_result = validate_token("invalid_token");
    assert!(
        invalid_result.is_err(),
        "Invalid token should fail validation"
    );
}

#[tokio::test]
async fn test_authentication_flow() {
    // Test full authentication flow
    let auth_req = AuthRequest {
        username: "admin".to_string(),
        password: "admin123".to_string(),
    };

    let response = authenticate_user(auth_req, Arc::new(UserManager::new())).await;
    assert!(
        response.is_ok(),
        "Authentication should succeed for valid credentials"
    );

    let auth_response = response.unwrap();
    assert_eq!(auth_response.token_type, "Bearer");
    assert_eq!(auth_response.role, UserRole::Admin);
    assert!(auth_response.expires_in > 0);
    assert!(auth_response.bootstrap_default_admin);
}

#[tokio::test]
async fn test_authentication_invalid_credentials() {
    // Test authentication with invalid credentials
    let auth_req = AuthRequest {
        username: "invalid".to_string(),
        password: "invalid".to_string(),
    };

    let response = authenticate_user(auth_req, Arc::new(UserManager::new())).await;
    assert!(
        response.is_err(),
        "Authentication should fail for invalid credentials"
    );
}

#[tokio::test]
async fn test_user_roles() {
    // Test different user roles
    let roles = vec![UserRole::Admin, UserRole::Operator, UserRole::Viewer];

    for role in roles {
        let token = generate_token("test_user", role.clone()).unwrap();
        let claims = validate_token(&token).unwrap();

        assert_eq!(claims.role, role);
        assert!(!claims.permissions.is_empty());

        // Check role-specific permissions
        let permissions = role.get_permissions();
        assert_eq!(claims.permissions, permissions);
    }
}

#[tokio::test]
async fn test_admin_permissions() {
    // Test Admin role has all permissions
    let admin_token = generate_token("admin", UserRole::Admin).unwrap();
    let claims = validate_token(&admin_token).unwrap();

    assert!(claims.permissions.contains(&"read:all".to_string()));
    assert!(claims.permissions.contains(&"write:all".to_string()));
    assert!(claims.permissions.contains(&"delete:all".to_string()));
    assert!(claims.permissions.contains(&"admin:all".to_string()));
}

#[tokio::test]
async fn test_operator_permissions() {
    // Test Operator role has limited permissions
    let operator_token = generate_token("operator", UserRole::Operator).unwrap();
    let claims = validate_token(&operator_token).unwrap();

    assert!(claims.permissions.contains(&"read:all".to_string()));
    assert!(claims.permissions.contains(&"write:workers".to_string()));
    assert!(claims.permissions.contains(&"write:models".to_string()));
    assert!(!claims.permissions.contains(&"admin:all".to_string()));
}

#[tokio::test]
async fn test_viewer_permissions() {
    // Test Viewer role has read-only permissions
    let viewer_token = generate_token("viewer", UserRole::Viewer).unwrap();
    let claims = validate_token(&viewer_token).unwrap();

    assert!(claims.permissions.contains(&"read:status".to_string()));
    assert!(claims.permissions.contains(&"read:metrics".to_string()));
    assert!(!claims.permissions.contains(&"write:all".to_string()));
    assert!(!claims.permissions.contains(&"admin:all".to_string()));
}
