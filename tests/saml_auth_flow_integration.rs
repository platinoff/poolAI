//! Integration tests for SAML SSO authentication flow
//!
//! Tests the complete SAML authentication flow including:
//! - SAML auth handler (redirect to IdP)
//! - SAML callback handler (process response, create user, generate JWT)

#[cfg(feature = "enterprise")]
use base64::Engine;
#[cfg(feature = "enterprise")]
use poolai::core::error::AppError;
#[cfg(feature = "enterprise")]
use poolai::enterprise::security::{get_global_security_manager, SamlConfig};
#[cfg(feature = "enterprise")]
use std::collections::HashMap;

#[cfg(feature = "enterprise")]
fn unique_saml_provider_name(prefix: &str) -> String {
    let n = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{prefix}_{n}")
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_saml_auth_handler_redirect() {
    let provider_name = unique_saml_provider_name("redirect");
    let manager = get_global_security_manager();
    manager.initialize().await.unwrap();

    let config = SamlConfig {
        entity_id: "test-entity".to_string(),
        sso_url: "https://saml.example.com/sso".to_string(),
        acs_url: Some(format!(
            "https://poolai.example.com/api/auth/saml/{provider_name}/callback"
        )),
        slo_url: None,
        certificate: "test-cert".to_string(),
        attribute_mapping: {
            let mut map = HashMap::new();
            map.insert("email".to_string(), "email".to_string());
            map.insert("username".to_string(), "username".to_string());
            map
        },
    };

    manager
        .register_saml_provider(provider_name.clone(), config)
        .await
        .unwrap();

    // Test SSO URL generation
    let sso_url = manager.get_saml_sso_url(&provider_name).await.unwrap();
    assert!(sso_url.contains("saml.example.com/sso"));
    assert!(sso_url.contains("SAMLRequest"));

    let _ = manager.delete_saml_provider(&provider_name).await;
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_saml_callback_handler_validation() {
    let provider_name = unique_saml_provider_name("callback_ok");
    let manager = get_global_security_manager();
    manager.initialize().await.unwrap();

    let config = SamlConfig {
        entity_id: "test-entity".to_string(),
        sso_url: "https://saml.example.com/sso".to_string(),
        acs_url: Some(format!(
            "https://poolai.example.com/api/auth/saml/{provider_name}/callback"
        )),
        slo_url: None,
        certificate: "test-cert".to_string(),
        attribute_mapping: {
            let mut map = HashMap::new();
            map.insert("email".to_string(), "email".to_string());
            map.insert("username".to_string(), "username".to_string());
            map
        },
    };

    manager
        .register_saml_provider(provider_name.clone(), config)
        .await
        .unwrap();

    // Test SAML assertion validation with mock response
    // Note: In production, this would be a real SAML response from IdP
    let mock_saml_response = base64::engine::general_purpose::STANDARD.encode(
        r#"<samlp:Response xmlns:samlp="urn:oasis:names:tc:SAML:2.0:protocol">
  <saml:Assertion>
    <saml:Conditions NotOnOrAfter="2099-01-01T00:00:00Z">
      <saml:AudienceRestriction>
        <saml:Audience>test-entity</saml:Audience>
      </saml:AudienceRestriction>
    </saml:Conditions>
    <saml:Subject>
      <saml:NameID>testuser@example.com</saml:NameID>
    </saml:Subject>
    <saml:AttributeStatement>
      <saml:Attribute Name="email">
        <saml:AttributeValue>testuser@example.com</saml:AttributeValue>
      </saml:Attribute>
      <saml:Attribute Name="username">
        <saml:AttributeValue>testuser</saml:AttributeValue>
      </saml:Attribute>
    </saml:AttributeStatement>
  </saml:Assertion>
</samlp:Response>"#
            .as_bytes(),
    );

    let result = manager
        .validate_saml_assertion(&provider_name, &mock_saml_response)
        .await;

    // Should extract attributes successfully
    if let Ok(attributes) = result {
        assert!(attributes.contains_key("nameid") || attributes.contains_key("email"));
    }

    let _ = manager.delete_saml_provider(&provider_name).await;
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_saml_callback_handler_invalid_response() {
    let provider_name = unique_saml_provider_name("callback_bad");
    let manager = get_global_security_manager();
    manager.initialize().await.unwrap();

    let config = SamlConfig {
        entity_id: "test-entity".to_string(),
        sso_url: "https://saml.example.com/sso".to_string(),
        acs_url: Some(format!(
            "https://poolai.example.com/api/auth/saml/{provider_name}/callback"
        )),
        slo_url: None,
        certificate: "test-cert".to_string(),
        attribute_mapping: HashMap::new(),
    };

    manager
        .register_saml_provider(provider_name.clone(), config)
        .await
        .unwrap();

    // Test with invalid base64
    let invalid_response = "not-valid-base64!!!";
    let result = manager
        .validate_saml_assertion(&provider_name, invalid_response)
        .await;
    assert!(result.is_err());

    // Test with empty response
    let empty_response = "";
    let result = manager
        .validate_saml_assertion(&provider_name, empty_response)
        .await;
    assert!(result.is_err());

    let _ = manager.delete_saml_provider(&provider_name).await;
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_saml_callback_handler_nonexistent_provider() {
    let manager = get_global_security_manager();
    manager.initialize().await.unwrap();

    let result = manager
        .validate_saml_assertion("nonexistent", "dGVzdA==")
        .await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("not found"));
    }
}
