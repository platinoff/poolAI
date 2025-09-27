// network/auth.rs
use axum::{
    extract::Request,
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// JWT Claims структура
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,        // User ID
    pub exp: usize,         // Expiration time
    pub iat: usize,         // Issued at
    pub role: UserRole,     // User role
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
pub fn generate_token(username: &str, role: UserRole) -> Result<String, jsonwebtoken::errors::Error> {
    let config = JwtConfig::default();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    
    let claims = Claims {
        sub: username.to_string(),
        exp: now + config.expiration,
        iat: now,
        role: role.clone(),
        permissions: role.get_permissions(),
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_ref()),
    )
}

// Функція для валідації JWT токена
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let config = JwtConfig::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_ref()),
        &Validation::default(),
    )?;
    
    Ok(token_data.claims)
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
    let claims = req
        .extensions()
        .get::<Claims>()
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "User not authenticated"
                })),
            )
        })?;

    // Перевіряємо права доступу
    if !claims.permissions.contains(&required_permission.to_string()) {
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
pub async fn authenticate_user(auth_req: AuthRequest) -> Result<AuthResponse, (StatusCode, Json<serde_json::Value>)> {
    // TODO: Реальна перевірка користувача з бази даних
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
        .map(|claims| claims.permissions.contains(&required_permission.to_string()))
        .unwrap_or(false)
} 