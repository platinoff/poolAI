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
//! use poolai::network::auth::{authenticate_user, AuthRequest, UserRole};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Authenticate user
//! let auth_req = AuthRequest {
//!     username: "admin".to_string(),
//!     password: "admin123".to_string(),
//! };
//!
//! let response = authenticate_user(auth_req).await.map_err(|(code, json)| {
//!     format!("Authentication failed: {} - {:?}", code, json)
//! })?;
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
    Json,
};
// JWT support (optional - enabled with feature "jwt")
use base64::Engine;
#[cfg(feature = "jwt")]
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

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

/// User roles for role-based access control
///
/// Defines three roles with different permission levels:
/// - **Admin**: Full access to all resources
/// - **Operator**: Read access and write access to workers/models
/// - **Viewer**: Read-only access to status, metrics, and models
///
/// # Example
///
/// ```rust
/// use poolai::network::auth::UserRole;
///
/// let role = UserRole::Admin;
/// let permissions = role.get_permissions();
/// println!("Admin permissions: {:?}", permissions);
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UserRole {
    Admin,
    Operator,
    Viewer,
}

impl UserRole {
    pub fn get_permissions(&self) -> Vec<String> {
        match self {
            UserRole::Admin => vec![
                "read:all".to_string(),
                "write:all".to_string(),
                "delete:all".to_string(),
                "admin:all".to_string(),
            ],
            UserRole::Operator => vec![
                "read:all".to_string(),
                "write:workers".to_string(),
                "write:models".to_string(),
                "read:metrics".to_string(),
            ],
            UserRole::Viewer => vec![
                "read:status".to_string(),
                "read:metrics".to_string(),
                "read:models".to_string(),
            ],
        }
    }
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
/// };
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub token_type: String,
    pub expires_in: usize,
    pub role: UserRole,
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
    // Future improvement: Re-enable JWT token generation after installing gcc
    // 1. Install gcc compiler (required by ring crate for crypto operations)
    //    - Windows: Install MinGW-w64 or use MSVC build tools
    //    - Linux: Install gcc via package manager (apt-get install gcc, yum install gcc)
    //    - macOS: Install Xcode Command Line Tools (xcode-select --install)
    // 2. Verify gcc installation: gcc --version
    // 3. Re-enable JWT token generation code
    //    - Uncomment JWT signing logic
    //    - Use ring::hmac for HMAC-SHA256 signing
    //    - Use ring::signature for RSA signing (if needed)
    // 4. Test JWT token generation and validation
    //    - Generate token with proper claims (sub, role, exp)
    //    - Validate token signature and expiration
    // Note: For now, returning placeholder token for development
    // For now, return a simple placeholder token
    let config = JwtConfig::default();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    // Simple base64-like encoding (NOT SECURE - for development only)
    let claims = Claims {
        sub: _username.to_string(),
        exp: now + config.expiration,
        iat: now,
        role: _role.clone(),
        permissions: _role.get_permissions(),
    };

    // Use serde_json to create a simple token (NOT a real JWT)
    let token_json = serde_json::to_string(&claims).unwrap_or_default();
    Ok(format!(
        "dev_token_{}",
        base64::engine::general_purpose::STANDARD.encode(token_json.as_bytes())
    ))
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
        use jsonwebtoken::{decode, DecodingKey, Validation};
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
pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    // Отримуємо токен з заголовка Authorization
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|auth| auth.to_str().ok())
        .and_then(|auth_str| {
            if auth_str.starts_with("Bearer ") {
                Some(auth_str[7..].to_string())
            } else {
                None
            }
        });

    let token = auth_header.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "Missing or invalid authorization header"
            })),
        )
    })?;

    // Валідуємо токен
    let claims = validate_token(&token).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "Invalid or expired token"
            })),
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
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let claims = req.extensions().get::<Claims>().ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "User not authenticated"
            })),
        )
    })?;

    // Перевіряємо права доступу
    if !claims
        .permissions
        .contains(&required_permission.to_string())
    {
        return Err((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({
                "error": "Insufficient permissions",
                "required": required_permission,
                "user_permissions": claims.permissions
            })),
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
/// use poolai::network::auth::{authenticate_user, AuthRequest};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let auth_req = AuthRequest {
///     username: "admin".to_string(),
///     password: "admin123".to_string(),
/// };
///
/// let response = authenticate_user(auth_req).await.map_err(|(code, json)| {
///     format!("Authentication failed: {} - {:?}", code, json)
/// })?;
/// println!("Token: {}", response.token);
/// # Ok(())
/// # }
/// ```
pub async fn authenticate_user(
    auth_req: AuthRequest,
) -> Result<AuthResponse, (StatusCode, Json<serde_json::Value>)> {
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
    // Use UserManager for authentication
    let manager = get_global_user_manager();

    // Ensure manager is initialized
    if let Err(e) = manager.initialize().await {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("User manager initialization failed: {}", e)
            })),
        ));
    }

    // Verify password
    let is_valid = manager
        .verify_password(&auth_req.username, &auth_req.password)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Authentication error: {}", e)
                })),
            )
        })?;

    if !is_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "Invalid credentials"
            })),
        ));
    }

    // Get user to retrieve role
    let user = manager
        .get_user_by_username(&auth_req.username)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("User retrieval error: {}", e)
                })),
            )
        })?;

    let (role, username) = if let Some(u) = user {
        (u.role, u.username)
    } else {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "Invalid credentials"
            })),
        ));
    };

    let token = generate_token(&username, role.clone()).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Failed to generate token"
            })),
        )
    })?;

    let config = JwtConfig::default();

    Ok(AuthResponse {
        token,
        token_type: "Bearer".to_string(),
        expires_in: config.expiration,
        role,
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

/// User information structure
///
/// Represents a user account with username, password hash, role, and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier
    pub id: Uuid,
    /// Username
    pub username: String,
    /// Password hash (for security, never return in API responses)
    #[serde(skip_serializing)]
    pub password_hash: String,
    /// User role
    pub role: UserRole,
    /// Whether user is active
    pub active: bool,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last updated timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// User information for API responses (without password hash)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// Unique user identifier
    pub id: Uuid,
    /// Username
    pub username: String,
    /// User role
    pub role: UserRole,
    /// Whether user is active
    pub active: bool,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last updated timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            role: user.role,
            active: user.active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

/// User manager for user account management
///
/// Manages user accounts in memory with CRUD operations.
/// For production, this should be replaced with a database-backed implementation.
pub struct UserManager {
    users: Arc<RwLock<HashMap<Uuid, User>>>,
    username_index: Arc<RwLock<HashMap<String, Uuid>>>,
    initialized: Arc<RwLock<bool>>,
}

impl UserManager {
    /// Creates a new user manager
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            username_index: Arc::new(RwLock::new(HashMap::new())),
            initialized: Arc::new(RwLock::new(false)),
        }
    }

    /// Initializes the user manager with default users
    pub async fn initialize(&self) -> Result<(), String> {
        let mut initialized = self.initialized.write().await;
        if *initialized {
            return Ok(());
        }

        // Create default users
        let default_users = vec![
            ("admin", "admin123", UserRole::Admin),
            ("operator", "op123", UserRole::Operator),
            ("viewer", "view123", UserRole::Viewer),
        ];

        let mut users = self.users.write().await;
        let mut username_index = self.username_index.write().await;
        let now = chrono::Utc::now();

        for (username, password, role) in default_users {
            let id = Uuid::new_v4();
            let user = User {
                id,
                username: username.to_string(),
                password_hash: password.to_string(), // In production, use proper password hashing
                role,
                active: true,
                created_at: now,
                updated_at: now,
            };
            users.insert(id, user.clone());
            username_index.insert(username.to_string(), id);
        }

        *initialized = true;
        info!(
            "User manager initialized with {} default users",
            users.len()
        );
        Ok(())
    }

    /// Creates a new user
    pub async fn create_user(
        &self,
        username: String,
        password: String,
        role: UserRole,
    ) -> Result<UserInfo, String> {
        if username.is_empty() {
            return Err("Username cannot be empty".to_string());
        }

        let mut users = self.users.write().await;
        let mut username_index = self.username_index.write().await;

        // Check if username already exists
        if username_index.contains_key(&username) {
            return Err(format!("Username '{}' already exists", username));
        }

        let now = chrono::Utc::now();
        let id = Uuid::new_v4();
        let user = User {
            id,
            username: username.clone(),
            password_hash: password, // In production, use proper password hashing
            role,
            active: true,
            created_at: now,
            updated_at: now,
        };

        users.insert(id, user.clone());
        username_index.insert(username, id);

        info!("Created user: {} ({})", user.username, id);
        Ok(user.into())
    }

    /// Gets a user by ID
    pub async fn get_user(&self, id: Uuid) -> Result<Option<UserInfo>, String> {
        let users = self.users.read().await;
        Ok(users.get(&id).map(|u| u.clone().into()))
    }

    /// Gets a user by username
    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        let username_index = self.username_index.read().await;
        if let Some(&id) = username_index.get(username) {
            let users = self.users.read().await;
            Ok(users.get(&id).cloned())
        } else {
            Ok(None)
        }
    }

    /// Lists all users
    pub async fn list_users(&self) -> Result<Vec<UserInfo>, String> {
        let users = self.users.read().await;
        Ok(users.values().map(|u| u.clone().into()).collect())
    }

    /// Updates a user
    pub async fn update_user(
        &self,
        id: Uuid,
        username: Option<String>,
        password: Option<String>,
        role: Option<UserRole>,
        active: Option<bool>,
    ) -> Result<UserInfo, String> {
        let mut users = self.users.write().await;
        let mut username_index = self.username_index.write().await;

        let user = users.get_mut(&id).ok_or_else(|| {
            format!("User not found: {}. Context: Cannot update non-existent user. Suggestion: Check user ID and ensure user exists.", id)
        })?;

        if let Some(new_username) = username {
            if new_username != user.username {
                // Check if new username already exists
                if username_index.contains_key(&new_username) {
                    return Err(format!("Username '{}' already exists", new_username));
                }
                // Update username index
                username_index.remove(&user.username);
                username_index.insert(new_username.clone(), id);
                user.username = new_username;
            }
        }

        if let Some(new_password) = password {
            user.password_hash = new_password; // In production, use proper password hashing
        }

        if let Some(new_role) = role {
            user.role = new_role;
        }

        if let Some(new_active) = active {
            user.active = new_active;
        }

        user.updated_at = chrono::Utc::now();
        let updated_user = user.clone();

        info!("Updated user: {} ({})", updated_user.username, id);
        Ok(updated_user.into())
    }

    /// Deletes a user
    pub async fn delete_user(&self, id: Uuid) -> Result<(), String> {
        let username = {
            let users = self.users.read().await;
            let user = users.get(&id).ok_or_else(|| {
                format!("User not found: {}. Context: Cannot delete non-existent user. Suggestion: Check user ID.", id)
            })?;
            user.username.clone()
        };

        let mut users = self.users.write().await;
        let mut username_index = self.username_index.write().await;

        username_index.remove(&username);
        users.remove(&id);

        info!("Deleted user: {} ({})", username, id);
        Ok(())
    }

    /// Verifies user password
    pub async fn verify_password(&self, username: &str, password: &str) -> Result<bool, String> {
        if let Some(user) = self.get_user_by_username(username).await? {
            if !user.active {
                return Ok(false);
            }
            // In production, use proper password verification (bcrypt, argon2, etc.)
            Ok(user.password_hash == password)
        } else {
            Ok(false)
        }
    }
}

impl Default for UserManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global user manager instance
static USER_MANAGER: OnceLock<Arc<UserManager>> = OnceLock::new();

/// Get global user manager instance.
///
/// This function returns a singleton instance of `UserManager` that can be used
/// throughout the application. The instance is created on first access and
/// reused for subsequent calls.
pub fn get_global_user_manager() -> Arc<UserManager> {
    USER_MANAGER
        .get_or_init(|| Arc::new(UserManager::new()))
        .clone()
}
