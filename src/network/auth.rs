//! Authentication and authorization module
//!
//! Provides JWT-based authentication, role-based access control (RBAC),
//! and middleware for protecting API endpoints.
//!
//! # Features
//!
//! - **JWT Token Generation**: Create and validate JWT tokens with claims
//! - **Role-Based Access Control**: Admin, Operator, and Viewer roles with permissions
//! - **Authentication Middleware**: Protect routes with token validation
//! - **Permission Middleware**: Check specific permissions for route access
//! - **User Authentication**: Authenticate users and generate tokens
//!
//! # Example
//!
//! ```no_run
//! use poolai::network::auth::{authenticate_user, AuthRequest, UserManager, UserRole};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Authenticate user
//! let auth_req = AuthRequest {
//!     username: "admin".to_string(),
//!     password: "admin123".to_string(),
//! };
//!
//! let manager = std::sync::Arc::new(UserManager::new());
//! let response = authenticate_user(auth_req, manager).await.expect("authenticate");
//! println!("Token: {}", response.token);
//! println!("Role: {:?}", response.role);
//! # Ok(())
//! # }
//! ```
use axum::{
    extract::Request,
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::core::error::{AppError, ErrorContext};
use crate::network::json_errors::HttpAppError;

fn auth_http_err(
    code: &'static str,
    message: impl Into<String>,
    ctx: ErrorContext,
    status: StatusCode,
) -> HttpAppError {
    HttpAppError::new(AppError::RestError {
        code,
        message: message.into(),
    })
    .with_context(ctx)
    .with_status(status)
}
// JWT support (optional - enabled with feature "jwt")
#[cfg(not(feature = "jwt"))]
use base64::Engine;
#[cfg(feature = "jwt")]
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub use crate::core::user_manager::{User, UserInfo, UserManager, UserRole};

/// JWT Claims structure
///
/// Contains user information embedded in JWT tokens including user ID,
/// expiration time, role, and permissions.
///
/// # Example
///
/// ```rust
/// use poolai::network::auth::{Claims, UserRole};
/// use std::time::{SystemTime, UNIX_EPOCH};
///
/// let now = SystemTime::now()
///     .duration_since(UNIX_EPOCH)
///     .unwrap()
///     .as_secs() as usize;
///
/// let claims = Claims {
///     sub: "user123".to_string(),
///     exp: now + 3600,
///     iat: now,
///     role: UserRole::Admin,
///     permissions: vec!["read:all".to_string(), "write:all".to_string()],
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,              // User ID
    pub exp: usize,               // Expiration time
    pub iat: usize,               // Issued at
    pub role: UserRole,           // User role
    pub permissions: Vec<String>, // User permissions
}

/// Authentication request structure
///
/// Contains username and password for user authentication.
///
/// # Example
///
/// ```rust
/// use poolai::network::auth::AuthRequest;
///
/// let auth_req = AuthRequest {
///     username: "admin".to_string(),
///     password: "admin123".to_string(),
/// };
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

/// Authentication response structure
///
/// Contains the JWT token, token type, expiration time, and user role
/// after successful authentication.
///
/// # Example
///
/// ```rust
/// use poolai::network::auth::{AuthResponse, UserRole};
///
/// let response = AuthResponse {
///     token: "dev_token_...".to_string(),
///     token_type: "Bearer".to_string(),
///     expires_in: 3600,
///     role: UserRole::Admin,
///     bootstrap_default_admin: false,
/// };
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub token_type: String,
    pub expires_in: usize,
    pub role: UserRole,
    /// When `true`, the UI may show a first-run reminder to change the default admin password.
    #[serde(default)]
    pub bootstrap_default_admin: bool,
}

// Структура для перевірки прав доступу
#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionCheck {
    pub resource: String,
    pub action: String,
}

// Глобальні налаштування JWT
pub struct JwtConfig {
    pub secret: String,
    pub expiration: usize,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: "your-super-secret-key-change-in-production".to_string(),
            expiration: 3600, // 1 година
        }
    }
}

/// Generate JWT token for a user
///
/// Creates a JWT token containing user claims (ID, role, permissions, expiration).
/// Currently uses a development token format (base64-encoded JSON) until JWT
/// library is fully configured with gcc/ring support.
///
/// # Arguments
///
/// * `_username` - Username to include in token claims
/// * `_role` - User role to include in token claims
///
/// # Returns
///
/// Returns a token string on success, or an error if token generation fails.
///
/// # Example
///
/// ```rust
/// use poolai::network::auth::{generate_token, UserRole};
///
/// let token = generate_token("admin", UserRole::Admin)?;
/// println!("Generated token: {}", token);
/// # Ok::<(), String>(())
/// ```
pub fn generate_token(_username: &str, _role: UserRole) -> Result<String, String> {
    let config = JwtConfig::default();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        sub: _username.to_string(),
        exp: now + config.expiration,
        iat: now,
        role: _role.clone(),
        permissions: _role.get_permissions(),
    };

    #[cfg(feature = "jwt")]
    {
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(config.secret.as_bytes()),
        )
        .map_err(|e| format!("JWT encode error: {}", e))
    }

    #[cfg(not(feature = "jwt"))]
    {
        // Dev-only opaque token (NOT a standard JWT)
        let token_json = serde_json::to_string(&claims).unwrap_or_default();
        Ok(format!(
            "dev_token_{}",
            base64::engine::general_purpose::STANDARD.encode(token_json.as_bytes())
        ))
    }
}

/// Validate JWT token
///
/// Validates a JWT token and extracts claims. Checks token format, expiration,
/// and signature (when JWT feature is enabled).
///
/// # Arguments
///
/// * `token` - JWT token string to validate
///
/// # Returns
///
/// Returns `Claims` if token is valid, or an error if validation fails.
///
/// # Example
///
/// ```rust
/// use poolai::network::auth::validate_token;
///
/// let token = "dev_token_...";
/// match validate_token(token) {
///     Ok(claims) => println!("User: {}, Role: {:?}", claims.sub, claims.role),
///     Err(e) => println!("Token validation failed: {}", e),
/// }
/// ```
pub fn validate_token(token: &str) -> Result<Claims, String> {
    #[cfg(feature = "jwt")]
    {
        // Real JWT token validation
        let config = JwtConfig::default();
        let key = DecodingKey::from_secret(config.secret.as_ref());
        let validation = Validation::default();

        decode::<Claims>(token, &key, &validation)
            .map(|data| data.claims)
            .map_err(|e| format!("Token validation failed: {}", e))
    }

    #[cfg(not(feature = "jwt"))]
    {
        // Fallback: Simple validation (NOT SECURE - for development only)
        if !token.starts_with("dev_token_") {
            return Err("Invalid token format".to_string());
        }

        let token_data = &token[10..]; // Skip "dev_token_"
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(token_data)
            .map_err(|e| format!("Decode error: {}", e))?;
        let claims: Claims =
            serde_json::from_slice(&decoded).map_err(|e| format!("Parse error: {}", e))?;

        // Check expiration
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        if claims.exp < now {
            return Err("Token expired".to_string());
        }

        Ok(claims)
    }
}

/// Decode token claims without rejecting expired sessions (for `POST /api/v1/refresh` only).
pub fn decode_token_claims_allow_expired(token: &str) -> Result<Claims, String> {
    #[cfg(feature = "jwt")]
    {
        let config = JwtConfig::default();
        let key = DecodingKey::from_secret(config.secret.as_ref());
        let mut validation = Validation::default();
        validation.validate_exp = false;
        decode::<Claims>(token, &key, &validation)
            .map(|data| data.claims)
            .map_err(|e| format!("Token decode failed: {}", e))
    }

    #[cfg(not(feature = "jwt"))]
    {
        if !token.starts_with("dev_token_") {
            return Err("Invalid token format".to_string());
        }

        let token_data = &token[10..];
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(token_data)
            .map_err(|e| format!("Decode error: {}", e))?;
        serde_json::from_slice(&decoded).map_err(|e| format!("Parse error: {}", e))
    }
}

pub fn bearer_token_from_authorization_header(req: &Request) -> Option<String> {
    req.headers()
        .get(AUTHORIZATION)
        .and_then(|auth| auth.to_str().ok())
        .and_then(|auth_str| {
            auth_str
                .strip_prefix("Bearer ")
                .or_else(|| auth_str.strip_prefix("bearer "))
                .map(|s| s.to_string())
        })
}

/// Issue a new access token using a still-decodable (possibly expired) bearer token.
pub async fn refresh_access_token(
    token: &str,
    user_manager: Arc<UserManager>,
) -> Result<AuthResponse, HttpAppError> {
    let claims = decode_token_claims_allow_expired(token).map_err(|_| {
        auth_http_err(
            "AUTH_INVALID_TOKEN",
            "Invalid or unreadable token",
            ErrorContext::new("refresh_access_token"),
            StatusCode::UNAUTHORIZED,
        )
    })?;

    if let Err(e) = user_manager.initialize().await {
        return Err(auth_http_err(
            "AUTH_USER_MANAGER_INIT_FAILED",
            format!("User manager initialization failed: {}", e),
            ErrorContext::new("refresh_access_token"),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    let user = user_manager
        .get_user_by_username(&claims.sub)
        .await
        .map_err(|e| {
            auth_http_err(
                "AUTH_INTERNAL",
                format!("User retrieval error: {}", e),
                ErrorContext::new("refresh_access_token"),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    let Some(user) = user else {
        return Err(auth_http_err(
            "AUTH_INVALID_TOKEN",
            "User no longer exists",
            ErrorContext::new("refresh_access_token"),
            StatusCode::UNAUTHORIZED,
        ));
    };

    let role = user.role;
    let username = user.username;
    let new_token = generate_token(&username, role.clone()).map_err(|_| {
        auth_http_err(
            "AUTH_TOKEN_GENERATION_FAILED",
            "Failed to generate token",
            ErrorContext::new("refresh_access_token").with_resource("username", &username),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    let config = JwtConfig::default();

    let bootstrap_default_admin = user_manager
        .is_default_bootstrap_admin_account(&username)
        .await
        .map_err(|e| {
            auth_http_err(
                "AUTH_INTERNAL",
                format!("User lookup error: {}", e),
                ErrorContext::new("refresh_access_token"),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    Ok(AuthResponse {
        token: new_token,
        token_type: "Bearer".to_string(),
        expires_in: config.expiration,
        role,
        bootstrap_default_admin,
    })
}

/// Authentication middleware
///
/// Middleware that validates JWT tokens from the `Authorization` header
/// and adds claims to request extensions for use in route handlers.
///
/// # Returns
///
/// Returns the response from the next middleware/handler, or an error
/// if authentication fails.
///
/// # Example
///
/// ```no_run
/// use poolai::network::auth::auth_middleware;
/// use axum::{middleware, Router, routing::post};
///
/// // Protect a route with authentication
/// async fn handler() -> &'static str { "ok" }
/// let app: Router<()> = Router::new()
///     .route("/api/workers", post(handler))
///     .layer(middleware::from_fn(auth_middleware));
/// ```
pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, HttpAppError> {
    // Отримуємо токен з заголовка Authorization
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|auth| auth.to_str().ok())
        .and_then(|auth_str| auth_str.strip_prefix("Bearer ").map(|s| s.to_string()));

    let token = auth_header.ok_or_else(|| {
        auth_http_err(
            "AUTH_MISSING_HEADER",
            "Missing or invalid authorization header",
            ErrorContext::new("auth_middleware"),
            StatusCode::UNAUTHORIZED,
        )
    })?;

    // Валідуємо токен
    let claims = validate_token(&token).map_err(|_| {
        auth_http_err(
            "AUTH_INVALID_TOKEN",
            "Invalid or expired token",
            ErrorContext::new("auth_middleware"),
            StatusCode::UNAUTHORIZED,
        )
    })?;

    // Додаємо claims до request extensions
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

// Middleware для перевірки прав доступу
pub async fn permission_middleware(
    req: Request,
    next: Next,
    required_permission: &str,
) -> Result<Response, HttpAppError> {
    let claims = req.extensions().get::<Claims>().ok_or_else(|| {
        auth_http_err(
            "AUTH_NOT_AUTHENTICATED",
            "User not authenticated",
            ErrorContext::new("permission_middleware"),
            StatusCode::UNAUTHORIZED,
        )
    })?;

    // Перевіряємо права доступу
    if !claims
        .permissions
        .contains(&required_permission.to_string())
    {
        return Err(auth_http_err(
            "AUTH_FORBIDDEN",
            "Insufficient permissions",
            ErrorContext::new("permission_middleware")
                .with_resource("permission", required_permission)
                .with_details(format!("user_permissions={:?}", claims.permissions)),
            StatusCode::FORBIDDEN,
        ));
    }

    Ok(next.run(req).await)
}

/// Authenticate user and generate token
///
/// Validates user credentials and generates a JWT token with appropriate
/// role and permissions. Currently uses hardcoded credentials for development.
///
/// # Arguments
///
/// * `auth_req` - Authentication request with username and password
///
/// # Returns
///
/// Returns `AuthResponse` with token and user information on success,
/// or an error if authentication fails.
///
/// # Example
///
/// ```no_run
/// use poolai::network::auth::{authenticate_user, AuthRequest, UserManager};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let auth_req = AuthRequest {
///     username: "admin".to_string(),
///     password: "admin123".to_string(),
/// };
///
/// let manager = std::sync::Arc::new(UserManager::new());
/// let response = authenticate_user(auth_req, manager).await?;
/// println!("Token: {}", response.token);
/// # Ok(())
/// # }
/// ```
pub async fn authenticate_user(
    auth_req: AuthRequest,
    user_manager: Arc<UserManager>,
) -> Result<AuthResponse, HttpAppError> {
    // Future improvement: Реальна перевірка користувача з бази даних
    // 1. Підключення до бази даних (SQLite, PostgreSQL, MySQL, тощо)
    //    - Використовувати async database driver (sqlx, diesel, sea-orm)
    //    - Зберігати connection pool в global state або config
    // 2. Запит до бази для перевірки користувача
    //    - SELECT * FROM users WHERE username = ? AND password_hash = ?
    //    - Використовувати prepared statements для безпеки
    //    - Хешувати пароль перед порівнянням (bcrypt, argon2, scrypt)
    // 3. Перевірка пароля
    //    - Використовувати secure password hashing (argon2 рекомендовано)
    //    - Перевіряти password hash з stored hash
    //    - Handle timing attacks (constant-time comparison)
    // 4. Отримання ролі користувача
    //    - Завантажувати role з users table або roles join table
    //    - Map database role to UserRole enum
    // 5. Error handling
    //    - Handle database connection errors gracefully
    //    - Return generic error for invalid credentials (don't leak user existence)
    //    - Log authentication failures for security monitoring
    // Example:
    //    let user = db.query_user_by_username(&auth_req.username).await?;
    //    if !verify_password(&auth_req.password, &user.password_hash)? {
    //        return Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid credentials"}))));
    //    }
    //    let role = UserRole::from_str(&user.role)?;
    let manager = user_manager;

    // Ensure manager is initialized
    if let Err(e) = manager.initialize().await {
        return Err(auth_http_err(
            "AUTH_USER_MANAGER_INIT_FAILED",
            format!("User manager initialization failed: {}", e),
            ErrorContext::new("authenticate_user"),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // Verify password
    let is_valid = manager
        .verify_password(&auth_req.username, &auth_req.password)
        .await
        .map_err(|e| {
            auth_http_err(
                "AUTH_INTERNAL",
                format!("Authentication error: {}", e),
                ErrorContext::new("authenticate_user"),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    if !is_valid {
        return Err(auth_http_err(
            "AUTH_INVALID_CREDENTIALS",
            "Invalid credentials",
            ErrorContext::new("authenticate_user"),
            StatusCode::UNAUTHORIZED,
        ));
    }

    // Get user to retrieve role
    let user = manager
        .get_user_by_username(&auth_req.username)
        .await
        .map_err(|e| {
            auth_http_err(
                "AUTH_INTERNAL",
                format!("User retrieval error: {}", e),
                ErrorContext::new("authenticate_user"),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    let (role, username) = if let Some(u) = user {
        (u.role, u.username)
    } else {
        return Err(auth_http_err(
            "AUTH_INVALID_CREDENTIALS",
            "Invalid credentials",
            ErrorContext::new("authenticate_user"),
            StatusCode::UNAUTHORIZED,
        ));
    };

    let token = generate_token(&username, role.clone()).map_err(|_| {
        auth_http_err(
            "AUTH_TOKEN_GENERATION_FAILED",
            "Failed to generate token",
            ErrorContext::new("authenticate_user").with_resource("username", &username),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    let config = JwtConfig::default();

    let bootstrap_default_admin = manager
        .is_default_bootstrap_admin_account(&username)
        .await
        .map_err(|e| {
            auth_http_err(
                "AUTH_INTERNAL",
                format!("User lookup error: {}", e),
                ErrorContext::new("authenticate_user"),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    Ok(AuthResponse {
        token,
        token_type: "Bearer".to_string(),
        expires_in: config.expiration,
        role,
        bootstrap_default_admin,
    })
}

/// Get current user from request
///
/// Extracts the authenticated user's claims from request extensions.
/// Returns `None` if user is not authenticated.
///
/// # Arguments
///
/// * `req` - HTTP request with authentication middleware applied
///
/// # Returns
///
/// Returns `Some(Claims)` if user is authenticated, `None` otherwise.
///
/// # Example
///
/// ```no_run
/// use poolai::network::auth::get_current_user;
/// use axum::extract::Request;
///
/// # async fn handler(req: Request) {
/// if let Some(claims) = get_current_user(&req) {
///     println!("Authenticated user: {}", claims.sub);
/// }
/// # }
/// ```
pub fn get_current_user(req: &Request) -> Option<&Claims> {
    req.extensions().get::<Claims>()
}

/// Check if user has required role
///
/// Verifies if the authenticated user has the specified role.
///
/// # Arguments
///
/// * `req` - HTTP request with authentication middleware applied
/// * `required_role` - Role to check for
///
/// # Returns
///
/// Returns `true` if user has the required role, `false` otherwise.
///
/// # Example
///
/// ```no_run
/// use poolai::network::auth::{has_role, UserRole};
/// use axum::extract::Request;
///
/// # async fn handler(req: Request) {
/// if has_role(&req, UserRole::Admin) {
///     println!("User is an admin");
/// }
/// # }
/// ```
pub fn has_role(req: &Request, required_role: UserRole) -> bool {
    req.extensions()
        .get::<Claims>()
        .map(|claims| claims.role == required_role)
        .unwrap_or(false)
}

/// Check if user has required permission
///
/// Verifies if the authenticated user has the specified permission.
///
/// # Arguments
///
/// * `req` - HTTP request with authentication middleware applied
/// * `required_permission` - Permission to check for (e.g., "read:metrics")
///
/// # Returns
///
/// Returns `true` if user has the required permission, `false` otherwise.
///
/// # Example
///
/// ```no_run
/// use poolai::network::auth::has_permission;
/// use axum::extract::Request;
///
/// # async fn handler(req: Request) {
/// if has_permission(&req, "read:metrics") {
///     println!("User can read metrics");
/// }
/// # }
/// ```
pub fn has_permission(req: &Request, required_permission: &str) -> bool {
    req.extensions()
        .get::<Claims>()
        .map(|claims| {
            claims
                .permissions
                .contains(&required_permission.to_string())
        })
        .unwrap_or(false)
}
