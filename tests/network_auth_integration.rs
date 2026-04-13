//! Integration tests for Network Auth Module

use axum::http::StatusCode;
use std::sync::Arc;

use poolai::core::error::AppError;
use poolai::network::api::common::HttpAppError;
use poolai::network::auth::{
    authenticate_user, generate_token, validate_token, AuthRequest, UserManager, UserRole,
};

#[tokio::test]
async fn test_generate_token() {
    let token = generate_token("test_user", UserRole::Admin);
    assert!(token.is_ok());
    let token_str = token.unwrap();
    #[cfg(not(feature = "jwt"))]
    assert!(token_str.starts_with("dev_token_"));
    #[cfg(feature = "jwt")]
    assert!(
        token_str.contains('.') && token_str.matches('.').count() == 2,
        "expected compact JWT with two dots"
    );
}

#[tokio::test]
async fn test_validate_token() {
    let token = generate_token("test_user", UserRole::Admin).unwrap();
    let result = validate_token(&token);
    assert!(result.is_ok());

    let claims = result.unwrap();
    assert_eq!(claims.sub, "test_user");
    assert_eq!(claims.role, UserRole::Admin);
}

#[tokio::test]
async fn test_validate_token_invalid() {
    let result = validate_token("invalid_token");
    assert!(result.is_err());
}

#[tokio::test]
async fn test_authenticate_user_admin() {
    let auth_req = AuthRequest {
        username: "admin".to_string(),
        password: "admin123".to_string(),
    };

    let result = authenticate_user(auth_req, Arc::new(UserManager::new())).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.role, UserRole::Admin);
    assert_eq!(response.token_type, "Bearer");
    assert!(response.expires_in > 0);
    assert!(response.bootstrap_default_admin);
}

#[tokio::test]
async fn test_authenticate_user_operator() {
    let auth_req = AuthRequest {
        username: "operator".to_string(),
        password: "op123".to_string(),
    };

    let result = authenticate_user(auth_req, Arc::new(UserManager::new())).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.role, UserRole::Operator);
    assert!(!response.bootstrap_default_admin);
}

#[tokio::test]
async fn test_authenticate_user_viewer() {
    let auth_req = AuthRequest {
        username: "viewer".to_string(),
        password: "view123".to_string(),
    };

    let result = authenticate_user(auth_req, Arc::new(UserManager::new())).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.role, UserRole::Viewer);
    assert!(!response.bootstrap_default_admin);
}

#[tokio::test]
async fn test_authenticate_user_invalid() {
    let auth_req = AuthRequest {
        username: "invalid".to_string(),
        password: "invalid".to_string(),
    };

    let result = authenticate_user(auth_req, Arc::new(UserManager::new())).await;
    assert!(result.is_err());

    if let Err(e) = result {
        let e: HttpAppError = e;
        assert_eq!(e.status_override, Some(StatusCode::UNAUTHORIZED));
        match &e.err {
            AppError::RestError { code, message } => {
                assert_eq!(*code, "AUTH_INVALID_CREDENTIALS");
                assert_eq!(message, "Invalid credentials");
            }
            other => panic!("expected RestError, got {:?}", other),
        }
        assert!(e.context.is_some());
    } else {
        panic!("expected authentication error");
    }
}

#[tokio::test]
async fn test_user_role_permissions() {
    let admin_perms = UserRole::Admin.get_permissions();
    assert!(admin_perms.contains(&"read:all".to_string()));
    assert!(admin_perms.contains(&"write:all".to_string()));
    assert!(admin_perms.contains(&"admin:all".to_string()));

    let operator_perms = UserRole::Operator.get_permissions();
    assert!(operator_perms.contains(&"read:all".to_string()));
    assert!(operator_perms.contains(&"write:workers".to_string()));
    assert!(!operator_perms.contains(&"admin:all".to_string()));

    let viewer_perms = UserRole::Viewer.get_permissions();
    assert!(viewer_perms.contains(&"read:status".to_string()));
    assert!(viewer_perms.contains(&"read:metrics".to_string()));
    assert!(!viewer_perms.contains(&"write:all".to_string()));
}

#[tokio::test]
async fn test_user_role_equality() {
    assert_eq!(UserRole::Admin, UserRole::Admin);
    assert_ne!(UserRole::Admin, UserRole::Operator);
    assert_ne!(UserRole::Operator, UserRole::Viewer);
}
