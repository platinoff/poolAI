//! Advanced security module
//!
//! Provides OAuth2, SAML, and advanced security policies.
//!
//! # Features
//!
//! - OAuth2 authentication (Authorization Code, Client Credentials flows)
//! - SAML SSO support
//! - Security policies and rules
//! - Advanced RBAC with tenant-aware permissions
//! - Token management and refresh
//!
//! # Example
//!
//! ```rust,no_run
//! use poolai::enterprise::security::{SecurityManager, OAuth2Provider, OAuth2Config};
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let manager = SecurityManager::new();
//! manager.initialize().await?;
//!
//! // Configure OAuth2 provider
//! let oauth2_config = OAuth2Config {
//!     client_id: "client-id".to_string(),
//!     client_secret: "client-secret".to_string(),
//!     authorization_url: "https://oauth.example.com/authorize".to_string(),
//!     token_url: "https://oauth.example.com/token".to_string(),
//!     redirect_uri: "https://poolai.example.com/callback".to_string(),
//!     scopes: vec!["openid".to_string(), "profile".to_string()],
//! };
//!
//! manager.register_oauth2_provider("google", oauth2_config).await?;
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

// OAuth2 token exchange response from provider
#[derive(Debug, Deserialize)]
struct OAuth2TokenResponseRaw {
    access_token: String,
    token_type: Option<String>,
    expires_in: Option<u64>,
    refresh_token: Option<String>,
    id_token: Option<String>,
    scope: Option<String>,
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    error_description: Option<String>,
}

/// OAuth2 provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Config {
    /// OAuth2 client ID
    pub client_id: String,
    /// OAuth2 client secret
    pub client_secret: String,
    /// Authorization endpoint URL
    pub authorization_url: String,
    /// Token endpoint URL
    pub token_url: String,
    /// Redirect URI after authorization
    pub redirect_uri: String,
    /// Requested OAuth2 scopes
    pub scopes: Vec<String>,
}

/// OAuth2 provider information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Provider {
    /// Provider name (e.g., "google", "github", "microsoft")
    pub name: String,
    /// Provider configuration
    pub config: OAuth2Config,
    /// Whether provider is enabled
    pub enabled: bool,
}

/// OAuth2 authorization request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2AuthRequest {
    /// Provider name
    pub provider: String,
    /// State parameter for CSRF protection
    pub state: String,
    /// Optional tenant ID for multi-tenancy
    pub tenant_id: Option<Uuid>,
}

/// OAuth2 token response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2TokenResponse {
    /// Access token
    pub access_token: String,
    /// Token type (usually "Bearer")
    pub token_type: String,
    /// Expires in seconds
    pub expires_in: Option<u64>,
    /// Refresh token (if provided)
    pub refresh_token: Option<String>,
    /// ID token (for OpenID Connect)
    pub id_token: Option<String>,
    /// Scope granted
    pub scope: Option<String>,
}

/// OAuth2 user information from provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2UserInfo {
    /// Provider-specific user ID
    pub id: String,
    /// Username/login from provider
    pub username: String,
    /// Email address (if available)
    pub email: Option<String>,
    /// Display name (if available)
    pub name: Option<String>,
    /// Avatar/profile picture URL (if available)
    pub avatar_url: Option<String>,
}

/// SAML provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamlConfig {
    /// SAML entity ID
    pub entity_id: String,
    /// SSO URL (Identity Provider)
    pub sso_url: String,
    /// ACS URL (Assertion Consumer Service URL, optional)
    /// Defaults to "/api/enterprise/security/saml/callback" if not provided
    pub acs_url: Option<String>,
    /// SLO URL (Single Logout, optional)
    pub slo_url: Option<String>,
    /// X.509 certificate for signature verification
    pub certificate: String,
    /// Attribute mapping (SAML attribute -> user field)
    pub attribute_mapping: HashMap<String, String>,
}

/// SAML provider information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamlProvider {
    /// Provider name
    pub name: String,
    /// Provider configuration
    pub config: SamlConfig,
    /// Whether provider is enabled
    pub enabled: bool,
}

/// Security policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    /// Policy name
    pub name: String,
    /// Policy description
    pub description: String,
    /// Allowed IP ranges (CIDR notation)
    pub allowed_ip_ranges: Vec<String>,
    /// Required MFA for this policy
    pub require_mfa: bool,
    /// Session timeout in seconds
    pub session_timeout: u64,
    /// Maximum failed login attempts
    pub max_failed_attempts: usize,
}

/// Security manager
///
/// Manages OAuth2, SAML, and security policies.
pub struct SecurityManager {
    oauth2_providers: Arc<RwLock<HashMap<String, OAuth2Provider>>>,
    saml_providers: Arc<RwLock<HashMap<String, SamlProvider>>>,
    security_policies: Arc<RwLock<HashMap<String, SecurityPolicy>>>,
    initialized: Arc<RwLock<bool>>,
}

impl SecurityManager {
    /// Creates a new security manager
    pub fn new() -> Self {
        Self {
            oauth2_providers: Arc::new(RwLock::new(HashMap::new())),
            saml_providers: Arc::new(RwLock::new(HashMap::new())),
            security_policies: Arc::new(RwLock::new(HashMap::new())),
            initialized: Arc::new(RwLock::new(false)),
        }
    }

    /// Initializes the security manager
    pub async fn initialize(&self) -> Result<(), AppError> {
        let mut initialized = self.initialized.write().await;
        if *initialized {
            return Ok(());
        }

        // Load default security policies
        self.load_default_policies().await?;

        *initialized = true;
        info!("Security manager initialized");
        Ok(())
    }

    /// Registers an OAuth2 provider
    ///
    /// # Errors
    ///
    /// Returns `AppError` if provider registration fails.
    pub async fn register_oauth2_provider(
        &self,
        name: String,
        config: OAuth2Config,
    ) -> Result<(), AppError> {
        if name.is_empty() {
            return Err(AppError::ValidationError(
                "OAuth2 provider name cannot be empty".to_string(),
            ));
        }

        if config.client_id.is_empty() || config.client_secret.is_empty() {
            return Err(AppError::ValidationError(
                "OAuth2 client_id and client_secret cannot be empty".to_string(),
            ));
        }

        let provider = OAuth2Provider {
            name: name.clone(),
            config,
            enabled: true,
        };

        let mut providers = self.oauth2_providers.write().await;
        providers.insert(name.clone(), provider);

        info!("Registered OAuth2 provider: {}", name);
        Ok(())
    }

    /// Gets OAuth2 authorization URL
    ///
    /// Generates the authorization URL for OAuth2 flow.
    ///
    /// # Errors
    ///
    /// Returns `AppError` if provider is not found or URL generation fails.
    pub async fn get_oauth2_authorization_url(
        &self,
        provider_name: &str,
        state: &str,
    ) -> Result<String, AppError> {
        let providers = self.oauth2_providers.read().await;
        let provider = providers.get(provider_name).ok_or_else(|| {
            AppError::ValidationError(format!(
                "OAuth2 provider not found: {}. \
                Context: Cannot generate authorization URL for unknown provider. \
                Suggestion: Register the provider first using register_oauth2_provider().",
                provider_name
            ))
        })?;

        if !provider.enabled {
            return Err(AppError::ValidationError(format!(
                "OAuth2 provider is disabled: {}",
                provider_name
            )));
        }

        // Build authorization URL with parameters
        let mut url = format!("{}?", provider.config.authorization_url);
        url.push_str(&format!(
            "client_id={}",
            urlencoding::encode(&provider.config.client_id)
        ));
        url.push_str(&format!(
            "&redirect_uri={}",
            urlencoding::encode(&provider.config.redirect_uri)
        ));
        url.push_str(&format!("&response_type=code"));
        url.push_str(&format!("&state={}", urlencoding::encode(state)));

        if !provider.config.scopes.is_empty() {
            let scopes = provider.config.scopes.join(" ");
            url.push_str(&format!("&scope={}", urlencoding::encode(&scopes)));
        }

        Ok(url)
    }

    /// Exchanges authorization code for access token
    ///
    /// # Errors
    ///
    /// Returns `AppError` if token exchange fails.
    pub async fn exchange_oauth2_code(
        &self,
        provider_name: &str,
        code: &str,
    ) -> Result<OAuth2TokenResponse, AppError> {
        let providers = self.oauth2_providers.read().await;
        let provider = providers.get(provider_name).ok_or_else(|| {
            AppError::ValidationError(format!("OAuth2 provider not found: {}", provider_name))
        })?;

        if !provider.enabled {
            return Err(AppError::ValidationError(format!(
                "OAuth2 provider is disabled: {}",
                provider_name
            )));
        }

        // Create HTTP client for OAuth2 token exchange
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| {
                AppError::NetworkError(format!(
                    "Failed to create HTTP client for OAuth2 token exchange. Context: Cannot initialize HTTP client. \
                    Suggestion: Check network configuration and ensure reqwest crate is properly configured. \
                    Provider: '{}', Error: {}",
                    provider_name, e
                ))
            })?;

        // Prepare form data for token exchange (OAuth2 requires application/x-www-form-urlencoded)
        // Build URL-encoded form body manually since reqwest without default features doesn't have form() method
        let form_body = format!(
            "grant_type=authorization_code&code={}&client_id={}&client_secret={}&redirect_uri={}",
            urlencoding::encode(code),
            urlencoding::encode(&provider.config.client_id),
            urlencoding::encode(&provider.config.client_secret),
            urlencoding::encode(&provider.config.redirect_uri)
        );

        // Make HTTP POST request to token endpoint
        let response = client
            .post(&provider.config.token_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Accept", "application/json")
            .body(form_body)
            .send()
            .await
            .map_err(|e| {
                AppError::NetworkError(format!(
                    "OAuth2 token exchange request failed. Context: HTTP request to token endpoint failed. \
                    Suggestion: Check network connectivity, verify token URL is correct, and ensure provider is accessible. \
                    Provider: '{}', Token URL: '{}', Error: {}",
                    provider_name, provider.config.token_url, e
                ))
            })?;

        // Check response status
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::NetworkError(format!(
                "OAuth2 token exchange failed with status {}. Context: Token endpoint returned error. \
                Suggestion: Verify authorization code is valid and not expired, check client credentials, and ensure redirect_uri matches. \
                Provider: '{}', Status: {}, Response: {}",
                status, provider_name, status, error_text
            )));
        }

        // Parse JSON response
        let token_response: OAuth2TokenResponseRaw = response
            .json()
            .await
            .map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to parse OAuth2 token response. Context: Token endpoint returned invalid JSON. \
                    Suggestion: Verify provider token endpoint format matches OAuth2 specification. \
                    Provider: '{}', Error: {}",
                    provider_name, e
                ))
            })?;

        // Check for OAuth2 error response
        if let Some(error) = token_response.error {
            return Err(AppError::ValidationError(format!(
                "OAuth2 token exchange error: {}. Description: {}. Context: Provider returned OAuth2 error during token exchange. \
                Suggestion: Check authorization code validity, client credentials, and redirect URI. \
                Provider: '{}', Error: '{}'",
                error,
                token_response.error_description.unwrap_or_default(),
                provider_name,
                error
            )));
        }

        info!(
            "OAuth2 token exchange successful for provider: {}",
            provider_name
        );

        // Convert to our response format
        Ok(OAuth2TokenResponse {
            access_token: token_response.access_token,
            token_type: token_response
                .token_type
                .unwrap_or_else(|| "Bearer".to_string()),
            expires_in: token_response.expires_in,
            refresh_token: token_response.refresh_token,
            id_token: token_response.id_token,
            scope: token_response.scope,
        })
    }

    /// Gets user information from OAuth2 provider
    ///
    /// After exchanging authorization code for access token, this method
    /// retrieves user information from the OAuth2 provider's user info endpoint.
    ///
    /// # Arguments
    ///
    /// * `provider_name` - Name of the OAuth2 provider (e.g., "github", "google")
    /// * `access_token` - Access token obtained from token exchange
    ///
    /// # Errors
    ///
    /// Returns `AppError` if user info retrieval fails.
    pub async fn get_oauth2_user_info(
        &self,
        provider_name: &str,
        access_token: &str,
    ) -> Result<OAuth2UserInfo, AppError> {
        let user_info_url = match provider_name {
            "github" => "https://api.github.com/user",
            "google" => "https://www.googleapis.com/oauth2/v2/userinfo",
            _ => {
                return Err(AppError::ValidationError(format!(
                    "User info endpoint not configured for provider: {}",
                    provider_name
                )));
            }
        };

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| {
                AppError::NetworkError(format!(
                    "Failed to create HTTP client for OAuth2 user info. Context: Cannot initialize HTTP client. \
                    Suggestion: Check network configuration. Provider: '{}', Error: {}",
                    provider_name, e
                ))
            })?;

        let response = client
            .get(user_info_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Accept", "application/json")
            .header("User-Agent", "PoolAI/0.1.0")
            .send()
            .await
            .map_err(|e| {
                AppError::NetworkError(format!(
                    "OAuth2 user info request failed. Context: HTTP request to user info endpoint failed. \
                    Suggestion: Check network connectivity and verify access token. \
                    Provider: '{}', User Info URL: '{}', Error: {}",
                    provider_name, user_info_url, e
                ))
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::NetworkError(format!(
                "OAuth2 user info request failed with status {}. Context: User info endpoint returned error. \
                Suggestion: Verify access token is valid and not expired. \
                Provider: '{}', Status: {}, Response: {}",
                status, provider_name, status, error_text
            )));
        }

        // Parse response based on provider
        match provider_name {
            "github" => {
                #[derive(Deserialize)]
                struct GitHubUser {
                    login: String,
                    id: u64,
                    name: Option<String>,
                    email: Option<String>,
                    avatar_url: Option<String>,
                }

                let github_user: GitHubUser = response.json().await.map_err(|e| {
                    AppError::ConfigError(format!(
                        "Failed to parse GitHub user info. Context: User info endpoint returned invalid JSON. \
                        Suggestion: Verify GitHub API response format. Provider: '{}', Error: {}",
                        provider_name, e
                    ))
                })?;

                Ok(OAuth2UserInfo {
                    id: github_user.id.to_string(),
                    username: github_user.login,
                    email: github_user.email,
                    name: github_user.name,
                    avatar_url: github_user.avatar_url,
                })
            }
            "google" => {
                #[derive(Deserialize)]
                struct GoogleUser {
                    id: String,
                    email: String,
                    name: Option<String>,
                    picture: Option<String>,
                }

                let google_user: GoogleUser = response.json().await.map_err(|e| {
                    AppError::ConfigError(format!(
                        "Failed to parse Google user info. Context: User info endpoint returned invalid JSON. \
                        Suggestion: Verify Google API response format. Provider: '{}', Error: {}",
                        provider_name, e
                    ))
                })?;

                Ok(OAuth2UserInfo {
                    id: google_user.id,
                    username: google_user.email.clone(),
                    email: Some(google_user.email),
                    name: google_user.name,
                    avatar_url: google_user.picture,
                })
            }
            _ => Err(AppError::ValidationError(format!(
                "Unsupported provider for user info: {}",
                provider_name
            ))),
        }
    }

    /// Registers a SAML provider
    ///
    /// # Errors
    ///
    /// Returns `AppError` if provider registration fails.
    pub async fn register_saml_provider(
        &self,
        name: String,
        config: SamlConfig,
    ) -> Result<(), AppError> {
        if name.is_empty() {
            return Err(AppError::ValidationError(
                "SAML provider name cannot be empty".to_string(),
            ));
        }

        if config.entity_id.is_empty() || config.sso_url.is_empty() {
            return Err(AppError::ValidationError(
                "SAML entity_id and sso_url cannot be empty".to_string(),
            ));
        }

        let provider = SamlProvider {
            name: name.clone(),
            config,
            enabled: true,
        };

        let mut providers = self.saml_providers.write().await;
        providers.insert(name.clone(), provider);

        info!("Registered SAML provider: {}", name);
        Ok(())
    }

    /// Gets SAML SSO URL
    ///
    /// Generates the SAML SSO request URL.
    ///
    /// # Errors
    ///
    /// Returns `AppError` if provider is not found or URL generation fails.
    pub async fn get_saml_sso_url(&self, provider_name: &str) -> Result<String, AppError> {
        let providers = self.saml_providers.read().await;
        let provider = providers.get(provider_name).ok_or_else(|| {
            AppError::ValidationError(format!(
                "SAML provider not found: {}. \
                Context: Cannot generate SSO URL for unknown provider. \
                Suggestion: Register the provider first using register_saml_provider().",
                provider_name
            ))
        })?;

        if !provider.enabled {
            return Err(AppError::ValidationError(format!(
                "SAML provider is disabled: {}",
                provider_name
            )));
        }

        // Generate SAML AuthnRequest
        // Create basic SAML 2.0 AuthnRequest XML
        let request_id = uuid::Uuid::new_v4().to_string();
        let issue_instant = chrono::Utc::now().to_rfc3339();

        // Build AuthnRequest XML
        // SAML 2.0 AuthnRequest format (simplified - without signing)
        let acs_url = provider.config.acs_url.clone().unwrap_or_else(|| {
            // Default ACS URL if not provided (should be configured in production)
            "/api/enterprise/security/saml/callback".to_string()
        });

        let authn_request = format!(
            r#"<samlp:AuthnRequest xmlns:samlp="urn:oasis:names:tc:SAML:2.0:protocol" xmlns:saml="urn:oasis:names:tc:SAML:2.0:assertion" ID="_{}" Version="2.0" IssueInstant="{}" Destination="{}" AssertionConsumerServiceURL="{}" ProtocolBinding="urn:oasis:names:tc:SAML:2.0:bindings:HTTP-POST">
  <saml:Issuer>{}</saml:Issuer>
</samlp:AuthnRequest>"#,
            request_id, issue_instant, provider.config.sso_url, acs_url, provider.config.entity_id
        );

        // Deflate and Base64 encode (SAML 2.0 HTTP-Redirect binding)
        // For simplicity, we'll use base64 encoding without deflate
        // Full implementation would use deflate compression before base64
        let encoded_request =
            base64::engine::general_purpose::STANDARD.encode(authn_request.as_bytes());

        // URL encode the SAMLRequest parameter
        let url_encoded_request = urlencoding::encode(&encoded_request);

        // Build final SSO URL
        let sso_url = format!(
            "{}?SAMLRequest={}",
            provider.config.sso_url, url_encoded_request
        );

        info!(
            "Generated SAML SSO URL for provider {} (request ID: {})",
            provider_name, request_id
        );

        Ok(sso_url)
    }

    /// Validates SAML assertion and extracts user attributes
    ///
    /// # Arguments
    ///
    /// * `provider_name` - Name of the SAML provider
    /// * `saml_response` - Base64-encoded SAML response (SAMLResponse parameter)
    ///
    /// # Errors
    ///
    /// Returns `AppError` if validation fails.
    ///
    /// # Note
    ///
    /// This is a simplified implementation. In production, you should:
    /// - Parse full SAML XML response
    /// - Verify XML signature using X.509 certificate
    /// - Validate NotBefore/NotOnOrAfter timestamps
    /// - Check Audience restriction
    /// - Verify InResponseTo matches AuthnRequest ID
    pub async fn validate_saml_assertion(
        &self,
        provider_name: &str,
        saml_response: &str,
    ) -> Result<HashMap<String, String>, AppError> {
        let providers = self.saml_providers.read().await;
        let provider = providers.get(provider_name).ok_or_else(|| {
            AppError::ValidationError(format!(
                "SAML provider not found: {}. Context: Cannot validate assertion for unknown provider. \
                Suggestion: Register the provider first using register_saml_provider().",
                provider_name
            ))
        })?;

        if !provider.enabled {
            return Err(AppError::ValidationError(format!(
                "SAML provider is disabled: {}",
                provider_name
            )));
        }

        // Decode base64 SAML response
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(saml_response)
            .map_err(|e| {
                AppError::ValidationError(format!(
                    "Failed to decode SAML response. Context: Invalid base64 encoding. \
                    Suggestion: Verify SAML response format. Error: {}",
                    e
                ))
            })?;

        // Parse SAML XML (simplified - in production use proper XML parser and signature verification)
        let xml_str = String::from_utf8(decoded).map_err(|e| {
            AppError::ValidationError(format!(
                "Failed to parse SAML response as UTF-8. Context: Invalid character encoding. \
                Suggestion: Verify SAML response format. Error: {}",
                e
            ))
        })?;

        // Extract user attributes from SAML response (simplified parsing)
        // In production, use proper XML parsing library (e.g., quick-xml) and verify signature
        let mut attributes = HashMap::new();

        // Simple attribute extraction (for testing/demo purposes)
        // Production implementation should:
        // 1. Parse XML properly
        // 2. Verify signature using provider.config.certificate
        // 3. Extract attributes based on provider.config.attribute_mapping
        // 4. Validate timestamps and audience

        // Extract NameID (user identifier)
        if let Some(nameid_start) = xml_str.find("<saml:NameID>") {
            if let Some(nameid_end) = xml_str[nameid_start..].find("</saml:NameID>") {
                let nameid = xml_str[nameid_start + 13..nameid_start + nameid_end].trim();
                attributes.insert("nameid".to_string(), nameid.to_string());
            }
        }

        // Extract attributes based on attribute mapping
        for (saml_attr, user_field) in &provider.config.attribute_mapping {
            // Simple attribute extraction (in production, use proper XML parsing)
            let attr_pattern = format!("<saml:Attribute Name=\"{}\">", saml_attr);
            if let Some(attr_start) = xml_str.find(&attr_pattern) {
                if let Some(attr_value_start) = xml_str[attr_start..].find("<saml:AttributeValue>")
                {
                    if let Some(attr_value_end) =
                        xml_str[attr_start + attr_value_start..].find("</saml:AttributeValue>")
                    {
                        let attr_value = xml_str[attr_start + attr_value_start + 22
                            ..attr_start + attr_value_start + attr_value_end]
                            .trim();
                        attributes.insert(user_field.clone(), attr_value.to_string());
                    }
                }
            }
        }

        // If no attributes extracted, use NameID as username
        if attributes.is_empty() {
            if let Some(nameid) = attributes.get("nameid") {
                attributes.insert("username".to_string(), nameid.clone());
            } else {
                return Err(AppError::ValidationError(
                    "Failed to extract user attributes from SAML response. Context: No valid attributes found. \
                    Suggestion: Verify SAML response format and attribute mapping configuration.".to_string()
                ));
            }
        }

        info!(
            "SAML assertion validated for provider {} (extracted {} attributes)",
            provider_name,
            attributes.len()
        );

        Ok(attributes)
    }

    /// Creates a security policy
    ///
    /// # Errors
    ///
    /// Returns `AppError` if policy creation fails.
    pub async fn create_security_policy(&self, policy: SecurityPolicy) -> Result<(), AppError> {
        if policy.name.is_empty() {
            return Err(AppError::ValidationError(
                "Security policy name cannot be empty".to_string(),
            ));
        }

        let mut policies = self.security_policies.write().await;
        policies.insert(policy.name.clone(), policy.clone());

        info!("Created security policy: {}", policy.name);
        Ok(())
    }

    /// Gets a security policy
    ///
    /// # Errors
    ///
    /// Returns `AppError` if policy is not found.
    pub async fn get_security_policy(
        &self,
        name: &str,
    ) -> Result<Option<SecurityPolicy>, AppError> {
        let policies = self.security_policies.read().await;
        Ok(policies.get(name).cloned())
    }

    /// Loads default security policies
    async fn load_default_policies(&self) -> Result<(), AppError> {
        let default_policy = SecurityPolicy {
            name: "default".to_string(),
            description: "Default security policy".to_string(),
            allowed_ip_ranges: Vec::new(), // Allow all by default
            require_mfa: false,
            session_timeout: 3600, // 1 hour
            max_failed_attempts: 5,
        };

        let mut policies = self.security_policies.write().await;
        policies.insert("default".to_string(), default_policy);

        Ok(())
    }

    /// Lists all OAuth2 providers
    pub async fn list_oauth2_providers(&self) -> Result<Vec<OAuth2Provider>, AppError> {
        let providers = self.oauth2_providers.read().await;
        Ok(providers.values().cloned().collect())
    }

    /// Gets an OAuth2 provider by name
    pub async fn get_oauth2_provider(
        &self,
        name: &str,
    ) -> Result<Option<OAuth2Provider>, AppError> {
        let providers = self.oauth2_providers.read().await;
        Ok(providers.get(name).cloned())
    }

    /// Updates an OAuth2 provider
    pub async fn update_oauth2_provider(
        &self,
        name: String,
        config: Option<OAuth2Config>,
        enabled: Option<bool>,
    ) -> Result<(), AppError> {
        let mut providers = self.oauth2_providers.write().await;
        let provider = providers.get_mut(&name).ok_or_else(|| {
            AppError::ValidationError(format!(
                "OAuth2 provider not found: {}. Context: Cannot update non-existent provider. Suggestion: Check provider name and ensure provider exists.",
                name
            ))
        })?;

        if let Some(new_config) = config {
            provider.config = new_config;
        }

        if let Some(new_enabled) = enabled {
            provider.enabled = new_enabled;
        }

        info!("Updated OAuth2 provider: {}", name);
        Ok(())
    }

    /// Deletes an OAuth2 provider
    pub async fn delete_oauth2_provider(&self, name: &str) -> Result<(), AppError> {
        let mut providers = self.oauth2_providers.write().await;
        if providers.remove(name).is_none() {
            return Err(AppError::ValidationError(format!(
                "OAuth2 provider not found: {}. Context: Cannot delete non-existent provider. Suggestion: Check provider name.",
                name
            )));
        }

        info!("Deleted OAuth2 provider: {}", name);
        Ok(())
    }

    /// Lists all SAML providers
    pub async fn list_saml_providers(&self) -> Result<Vec<SamlProvider>, AppError> {
        let providers = self.saml_providers.read().await;
        Ok(providers.values().cloned().collect())
    }

    /// Gets a SAML provider by name
    pub async fn get_saml_provider(&self, name: &str) -> Result<Option<SamlProvider>, AppError> {
        let providers = self.saml_providers.read().await;
        Ok(providers.get(name).cloned())
    }

    /// Updates a SAML provider
    pub async fn update_saml_provider(
        &self,
        name: String,
        config: Option<SamlConfig>,
        enabled: Option<bool>,
    ) -> Result<(), AppError> {
        let mut providers = self.saml_providers.write().await;
        let provider = providers.get_mut(&name).ok_or_else(|| {
            AppError::ValidationError(format!(
                "SAML provider not found: {}. Context: Cannot update non-existent provider. Suggestion: Check provider name and ensure provider exists.",
                name
            ))
        })?;

        if let Some(new_config) = config {
            provider.config = new_config;
        }

        if let Some(new_enabled) = enabled {
            provider.enabled = new_enabled;
        }

        info!("Updated SAML provider: {}", name);
        Ok(())
    }

    /// Deletes a SAML provider
    pub async fn delete_saml_provider(&self, name: &str) -> Result<(), AppError> {
        let mut providers = self.saml_providers.write().await;
        if providers.remove(name).is_none() {
            return Err(AppError::ValidationError(format!(
                "SAML provider not found: {}. Context: Cannot delete non-existent provider. Suggestion: Check provider name.",
                name
            )));
        }

        info!("Deleted SAML provider: {}", name);
        Ok(())
    }

    /// Lists all security policies
    pub async fn list_security_policies(&self) -> Result<Vec<SecurityPolicy>, AppError> {
        let policies = self.security_policies.read().await;
        Ok(policies.values().cloned().collect())
    }

    /// Updates a security policy
    pub async fn update_security_policy(&self, policy: SecurityPolicy) -> Result<(), AppError> {
        let mut policies = self.security_policies.write().await;
        if !policies.contains_key(&policy.name) {
            return Err(AppError::ValidationError(format!(
                "Security policy not found: {}. Context: Cannot update non-existent policy. Suggestion: Check policy name and ensure policy exists.",
                policy.name
            )));
        }

        policies.insert(policy.name.clone(), policy.clone());
        info!("Updated security policy: {}", policy.name);
        Ok(())
    }

    /// Deletes a security policy
    pub async fn delete_security_policy(&self, name: &str) -> Result<(), AppError> {
        if name == "default" {
            return Err(AppError::ValidationError(
                "Cannot delete default security policy. Context: Default policy is required for system operation. Suggestion: Create a new policy instead of deleting the default one.".to_string()
            ));
        }

        let mut policies = self.security_policies.write().await;
        if policies.remove(name).is_none() {
            return Err(AppError::ValidationError(format!(
                "Security policy not found: {}. Context: Cannot delete non-existent policy. Suggestion: Check policy name.",
                name
            )));
        }

        info!("Deleted security policy: {}", name);
        Ok(())
    }

    /// Shuts down the security manager
    pub async fn shutdown(&self) -> Result<(), AppError> {
        *self.initialized.write().await = false;
        info!("Security manager shut down");
        Ok(())
    }
}

impl Default for SecurityManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global security manager instance
static SECURITY_MANAGER: OnceLock<Arc<SecurityManager>> = OnceLock::new();

/// Get global security manager instance.
///
/// This function returns a singleton instance of `SecurityManager` that can be used
/// throughout the application. The instance is created on first access and
/// reused for subsequent calls.
pub fn get_global_security_manager() -> Arc<SecurityManager> {
    SECURITY_MANAGER
        .get_or_init(|| Arc::new(SecurityManager::new()))
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_manager_initialization() {
        let manager = SecurityManager::new();
        assert!(manager.initialize().await.is_ok());
        assert!(manager.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn test_register_oauth2_provider() {
        let manager = SecurityManager::new();
        manager.initialize().await.unwrap();

        let config = OAuth2Config {
            client_id: "test-client-id".to_string(),
            client_secret: "test-secret".to_string(),
            authorization_url: "https://oauth.example.com/authorize".to_string(),
            token_url: "https://oauth.example.com/token".to_string(),
            redirect_uri: "https://poolai.example.com/callback".to_string(),
            scopes: vec!["openid".to_string(), "profile".to_string()],
        };

        assert!(manager
            .register_oauth2_provider("test-provider".to_string(), config)
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_get_oauth2_authorization_url() {
        let manager = SecurityManager::new();
        manager.initialize().await.unwrap();

        let config = OAuth2Config {
            client_id: "test-client-id".to_string(),
            client_secret: "test-secret".to_string(),
            authorization_url: "https://oauth.example.com/authorize".to_string(),
            token_url: "https://oauth.example.com/token".to_string(),
            redirect_uri: "https://poolai.example.com/callback".to_string(),
            scopes: vec!["openid".to_string()],
        };

        manager
            .register_oauth2_provider("test-provider".to_string(), config)
            .await
            .unwrap();

        let url = manager
            .get_oauth2_authorization_url("test-provider", "state123")
            .await
            .unwrap();
        assert!(url.contains("test-client-id"));
        assert!(url.contains("state123"));
    }

    #[tokio::test]
    async fn test_create_security_policy() {
        let manager = SecurityManager::new();
        manager.initialize().await.unwrap();

        let policy = SecurityPolicy {
            name: "test-policy".to_string(),
            description: "Test policy".to_string(),
            allowed_ip_ranges: vec!["192.168.1.0/24".to_string()],
            require_mfa: true,
            session_timeout: 1800,
            max_failed_attempts: 3,
        };

        assert!(manager.create_security_policy(policy).await.is_ok());

        let retrieved = manager.get_security_policy("test-policy").await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().require_mfa, true);
    }
}
