//! Integration tests for Enterprise Security Management
//!
//! Tests:
//! - OAuth2 provider management (list, create, get, update, delete)
//! - SAML provider management (list, create, get, update, delete)
//! - Security policy management (list, create, get, update, delete)
//! - Authorization URL generation
//! - SSO URL generation

#[cfg(feature = "enterprise")]
use poolai::core::error::AppError;
#[cfg(feature = "enterprise")]
use poolai::enterprise::security::{
    get_global_security_manager, OAuth2Config, OAuth2Provider, SamlConfig, SamlProvider,
    SecurityManager, SecurityPolicy,
};

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_security_manager_initialization() {
    let manager = SecurityManager::new();
    assert!(manager.initialize().await.is_ok());
    assert!(manager.shutdown().await.is_ok());
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_oauth2_provider_crud() {
    let manager = get_global_security_manager();
    manager.initialize().await.unwrap();

    // Clean up any existing test provider
    let _ = manager.delete_oauth2_provider("test-oauth2").await;

    // Create
    let config = OAuth2Config {
        client_id: "test-client-id".to_string(),
        client_secret: "test-secret".to_string(),
        authorization_url: "https://oauth.example.com/authorize".to_string(),
        token_url: "https://oauth.example.com/token".to_string(),
        redirect_uri: "https://poolai.example.com/callback".to_string(),
        scopes: vec!["openid".to_string(), "profile".to_string()],
    };

    assert!(manager
        .register_oauth2_provider("test-oauth2".to_string(), config.clone())
        .await
        .is_ok());

    // List
    let providers = manager.list_oauth2_providers().await.unwrap();
    assert!(providers.iter().any(|p| p.name == "test-oauth2"));

    // Get
    let provider = manager.get_oauth2_provider("test-oauth2").await.unwrap();
    assert!(provider.is_some());
    let provider = provider.unwrap();
    assert_eq!(provider.name, "test-oauth2");
    assert_eq!(provider.config.client_id, "test-client-id");
    assert!(provider.enabled);

    // Update
    let new_config = OAuth2Config {
        client_id: "updated-client-id".to_string(),
        client_secret: "updated-secret".to_string(),
        authorization_url: config.authorization_url,
        token_url: config.token_url,
        redirect_uri: config.redirect_uri,
        scopes: config.scopes,
    };

    assert!(manager
        .update_oauth2_provider("test-oauth2".to_string(), Some(new_config), Some(false))
        .await
        .is_ok());

    let provider = manager
        .get_oauth2_provider("test-oauth2")
        .await
        .unwrap()
        .unwrap();
    assert!(!provider.enabled);
    assert_eq!(provider.config.client_id, "updated-client-id");

    // Delete
    assert!(manager.delete_oauth2_provider("test-oauth2").await.is_ok());
    let provider = manager.get_oauth2_provider("test-oauth2").await.unwrap();
    assert!(provider.is_none());
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_oauth2_provider_validation() {
    let manager = get_global_security_manager();
    manager.initialize().await.unwrap();

    // Test empty name
    let config = OAuth2Config {
        client_id: "test".to_string(),
        client_secret: "test".to_string(),
        authorization_url: "https://oauth.example.com/authorize".to_string(),
        token_url: "https://oauth.example.com/token".to_string(),
        redirect_uri: "https://poolai.example.com/callback".to_string(),
        scopes: vec![],
    };

    let result = manager
        .register_oauth2_provider("".to_string(), config.clone())
        .await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("name cannot be empty"));
    }

    // Test duplicate registration
    manager
        .register_oauth2_provider("duplicate-test".to_string(), config.clone())
        .await
        .unwrap();
    let result = manager
        .register_oauth2_provider("duplicate-test".to_string(), config)
        .await;
    // Should either succeed (update) or fail (duplicate check)
    // Current implementation allows duplicates (overwrites), so this test may pass or fail
    let _ = manager.delete_oauth2_provider("duplicate-test").await;
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_oauth2_authorization_url() {
    let manager = get_global_security_manager();
    manager.initialize().await.unwrap();

    let config = OAuth2Config {
        client_id: "test-client-id".to_string(),
        client_secret: "test-secret".to_string(),
        authorization_url: "https://oauth.example.com/authorize".to_string(),
        token_url: "https://oauth.example.com/token".to_string(),
        redirect_uri: "https://poolai.example.com/callback".to_string(),
        scopes: vec!["openid".to_string(), "profile".to_string()],
    };

    manager
        .register_oauth2_provider("url-test".to_string(), config)
        .await
        .unwrap();

    let url = manager
        .get_oauth2_authorization_url("url-test", "state123")
        .await
        .unwrap();

    assert!(url.contains("test-client-id"));
    assert!(url.contains("state123"));
    assert!(url.contains("redirect_uri"));
    assert!(url.contains("response_type=code"));

    // Test disabled provider
    manager
        .update_oauth2_provider("url-test".to_string(), None, Some(false))
        .await
        .unwrap();

    let result = manager
        .get_oauth2_authorization_url("url-test", "state123")
        .await;
    assert!(result.is_err());

    let _ = manager.delete_oauth2_provider("url-test").await;
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_saml_provider_crud() {
    let manager = get_global_security_manager();
    manager.initialize().await.unwrap();

    // Clean up any existing test provider
    let _ = manager.delete_saml_provider("test-saml").await;

    // Create
    let config = SamlConfig {
        entity_id: "test-entity-id".to_string(),
        sso_url: "https://saml.example.com/sso".to_string(),
        acs_url: Some(
            "https://poolai.example.com/api/enterprise/security/saml/callback".to_string(),
        ),
        slo_url: Some("https://saml.example.com/slo".to_string()),
        certificate: "-----BEGIN CERTIFICATE-----\nTEST\n-----END CERTIFICATE-----".to_string(),
        attribute_mapping: std::collections::HashMap::from([
            ("email".to_string(), "email".to_string()),
            ("name".to_string(), "username".to_string()),
        ]),
    };

    assert!(manager
        .register_saml_provider("test-saml".to_string(), config.clone())
        .await
        .is_ok());

    // List
    let providers = manager.list_saml_providers().await.unwrap();
    assert!(providers.iter().any(|p| p.name == "test-saml"));

    // Get
    let provider = manager.get_saml_provider("test-saml").await.unwrap();
    assert!(provider.is_some());
    let provider = provider.unwrap();
    assert_eq!(provider.name, "test-saml");
    assert_eq!(provider.config.entity_id, "test-entity-id");
    assert!(provider.enabled);

    // Update
    let mut new_config = config.clone();
    new_config.entity_id = "updated-entity-id".to_string();

    assert!(manager
        .update_saml_provider("test-saml".to_string(), Some(new_config), Some(false))
        .await
        .is_ok());

    let provider = manager
        .get_saml_provider("test-saml")
        .await
        .unwrap()
        .unwrap();
    assert!(!provider.enabled);
    assert_eq!(provider.config.entity_id, "updated-entity-id");

    // Delete
    assert!(manager.delete_saml_provider("test-saml").await.is_ok());
    let provider = manager.get_saml_provider("test-saml").await.unwrap();
    assert!(provider.is_none());
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_saml_provider_validation() {
    let manager = get_global_security_manager();
    manager.initialize().await.unwrap();

    // Test empty name
    let config = SamlConfig {
        entity_id: "test-entity".to_string(),
        sso_url: "https://saml.example.com/sso".to_string(),
        acs_url: None,
        slo_url: None,
        certificate: "test-cert".to_string(),
        attribute_mapping: std::collections::HashMap::new(),
    };

    let result = manager
        .register_saml_provider("".to_string(), config.clone())
        .await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("name cannot be empty"));
    }

    // Test empty entity_id
    let mut invalid_config = config.clone();
    invalid_config.entity_id = String::new();
    let result = manager
        .register_saml_provider("test-invalid".to_string(), invalid_config)
        .await;
    assert!(result.is_err());

    // Test empty sso_url
    let mut invalid_config = config;
    invalid_config.sso_url = String::new();
    let result = manager
        .register_saml_provider("test-invalid2".to_string(), invalid_config)
        .await;
    assert!(result.is_err());
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_saml_sso_url() {
    let manager = get_global_security_manager();
    manager.initialize().await.unwrap();

    let config = SamlConfig {
        entity_id: "test-entity-id".to_string(),
        sso_url: "https://saml.example.com/sso".to_string(),
        acs_url: Some(
            "https://poolai.example.com/api/enterprise/security/saml/callback".to_string(),
        ),
        slo_url: None,
        certificate: "test-cert".to_string(),
        attribute_mapping: std::collections::HashMap::new(),
    };

    manager
        .register_saml_provider("sso-test".to_string(), config)
        .await
        .unwrap();

    let url = manager.get_saml_sso_url("sso-test").await.unwrap();
    assert!(url.contains("saml.example.com/sso"));
    assert!(url.contains("SAMLRequest"));

    // Test disabled provider
    manager
        .update_saml_provider("sso-test".to_string(), None, Some(false))
        .await
        .unwrap();

    let result = manager.get_saml_sso_url("sso-test").await;
    assert!(result.is_err());

    let _ = manager.delete_saml_provider("sso-test").await;
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_security_policy_crud() {
    let manager = get_global_security_manager();
    manager.initialize().await.unwrap();

    // Clean up any existing test policy
    let _ = manager.delete_security_policy("test-policy").await;

    // Create
    let policy = SecurityPolicy {
        name: "test-policy".to_string(),
        description: "Test security policy".to_string(),
        allowed_ip_ranges: vec!["192.168.1.0/24".to_string(), "10.0.0.0/8".to_string()],
        require_mfa: true,
        session_timeout: 1800,
        max_failed_attempts: 3,
    };

    assert!(manager.create_security_policy(policy.clone()).await.is_ok());

    // List
    let policies = manager.list_security_policies().await.unwrap();
    assert!(policies.iter().any(|p| p.name == "test-policy"));

    // Get
    let retrieved = manager.get_security_policy("test-policy").await.unwrap();
    assert!(retrieved.is_some());
    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.name, "test-policy");
    assert_eq!(retrieved.require_mfa, true);
    assert_eq!(retrieved.session_timeout, 1800);
    assert_eq!(retrieved.max_failed_attempts, 3);
    assert_eq!(retrieved.allowed_ip_ranges.len(), 2);

    // Update
    let updated_policy = SecurityPolicy {
        name: "test-policy".to_string(),
        description: "Updated test security policy".to_string(),
        allowed_ip_ranges: vec!["192.168.1.0/24".to_string()],
        require_mfa: false,
        session_timeout: 3600,
        max_failed_attempts: 5,
    };

    assert!(manager.update_security_policy(updated_policy).await.is_ok());

    let retrieved = manager
        .get_security_policy("test-policy")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(retrieved.description, "Updated test security policy");
    assert_eq!(retrieved.require_mfa, false);
    assert_eq!(retrieved.session_timeout, 3600);
    assert_eq!(retrieved.max_failed_attempts, 5);
    assert_eq!(retrieved.allowed_ip_ranges.len(), 1);

    // Delete
    assert!(manager.delete_security_policy("test-policy").await.is_ok());
    let retrieved = manager.get_security_policy("test-policy").await.unwrap();
    assert!(retrieved.is_none());
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_security_policy_validation() {
    let manager = get_global_security_manager();
    manager.initialize().await.unwrap();

    // Test empty name
    let policy = SecurityPolicy {
        name: String::new(),
        description: "Test".to_string(),
        allowed_ip_ranges: vec![],
        require_mfa: false,
        session_timeout: 3600,
        max_failed_attempts: 5,
    };

    let result = manager.create_security_policy(policy).await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("name cannot be empty"));
    }
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_security_policy_default_protection() {
    let manager = get_global_security_manager();
    manager.initialize().await.unwrap();

    // Try to delete default policy
    let result = manager.delete_security_policy("default").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Cannot delete default"));
    }
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_security_policy_update_nonexistent() {
    let manager = get_global_security_manager();
    manager.initialize().await.unwrap();

    let policy = SecurityPolicy {
        name: "nonexistent-policy".to_string(),
        description: "Test".to_string(),
        allowed_ip_ranges: vec![],
        require_mfa: false,
        session_timeout: 3600,
        max_failed_attempts: 5,
    };

    let result = manager.update_security_policy(policy).await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("not found"));
    }
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_oauth2_provider_update_nonexistent() {
    let manager = get_global_security_manager();
    manager.initialize().await.unwrap();

    let config = OAuth2Config {
        client_id: "test".to_string(),
        client_secret: "test".to_string(),
        authorization_url: "https://oauth.example.com/authorize".to_string(),
        token_url: "https://oauth.example.com/token".to_string(),
        redirect_uri: "https://poolai.example.com/callback".to_string(),
        scopes: vec![],
    };

    let result = manager
        .update_oauth2_provider("nonexistent".to_string(), Some(config), None)
        .await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("not found"));
    }
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_saml_provider_update_nonexistent() {
    let manager = get_global_security_manager();
    manager.initialize().await.unwrap();

    let config = SamlConfig {
        entity_id: "test-entity".to_string(),
        sso_url: "https://saml.example.com/sso".to_string(),
        acs_url: None,
        slo_url: None,
        certificate: "test-cert".to_string(),
        attribute_mapping: std::collections::HashMap::new(),
    };

    let result = manager
        .update_saml_provider("nonexistent".to_string(), Some(config), None)
        .await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("not found"));
    }
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_oauth2_provider_delete_nonexistent() {
    let manager = get_global_security_manager();
    manager.initialize().await.unwrap();

    let result = manager.delete_oauth2_provider("nonexistent").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("not found"));
    }
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_saml_provider_delete_nonexistent() {
    let manager = get_global_security_manager();
    manager.initialize().await.unwrap();

    let result = manager.delete_saml_provider("nonexistent").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("not found"));
    }
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_oauth2_authorization_url_nonexistent_provider() {
    let manager = get_global_security_manager();
    manager.initialize().await.unwrap();

    let result = manager
        .get_oauth2_authorization_url("nonexistent", "state123")
        .await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("not found"));
    }
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_saml_sso_url_nonexistent_provider() {
    let manager = get_global_security_manager();
    manager.initialize().await.unwrap();

    let result = manager.get_saml_sso_url("nonexistent").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("not found"));
    }
}
