// network/auth.rs
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
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// JWT Claims структура
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,              // User ID
    pub exp: usize,               // Expiration time
    pub iat: usize,               // Issued at
    pub role: UserRole,           // User role
    pub permissions: Vec<String>, // User permissions
}

// Ролі користувачів
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

// Структура для аутентифікації
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

// Структура для відповіді аутентифікації
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

// Функція для генерації JWT токена
// Temporarily disabled - requires ring/gcc
// Install gcc via: bash install_gcc.sh
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

// Функція для валідації JWT токена
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

// Middleware для аутентифікації
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

// Функція для аутентифікації користувача
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
    // Зараз використовуємо хардкод для тестування

    let (role, username) = match (auth_req.username.as_str(), auth_req.password.as_str()) {
        ("admin", "admin123") => (UserRole::Admin, "admin"),
        ("operator", "op123") => (UserRole::Operator, "operator"),
        ("viewer", "view123") => (UserRole::Viewer, "viewer"),
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Invalid credentials"
                })),
            ));
        }
    };

    let token = generate_token(username, role.clone()).map_err(|_| {
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

// Функція для отримання поточного користувача з request
pub fn get_current_user(req: &Request) -> Option<&Claims> {
    req.extensions().get::<Claims>()
}

// Функція для перевірки ролі користувача
pub fn has_role(req: &Request, required_role: UserRole) -> bool {
    req.extensions()
        .get::<Claims>()
        .map(|claims| claims.role == required_role)
        .unwrap_or(false)
}

// Функція для перевірки прав доступу
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
