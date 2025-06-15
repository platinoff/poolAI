use actix_web::{web, HttpResponse, Responder, error};
use std::sync::Arc;
use crate::core::state::AppState;
use crate::core::error::CursorError;
use crate::monitoring::logger::LoggerSystem;
use crate::monitoring::alert::AlertSystem;
use serde::{Deserialize, Serialize};
use log::{info, warn, error};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use chrono::{Utc, Duration};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::time::{Duration as StdDuration, Instant};
use ring::rand::SecureRandom;
use ring::digest;
use actix_web::error::ErrorUnauthorized;
use ring::pbkdf2::{derive, PBKDF2_HMAC_SHA256};

lazy_static::lazy_static! {
    static ref LOGIN_ATTEMPTS: RwLock<HashMap<String, (u32, Instant)>> = RwLock::new(HashMap::new());
}

const MAX_LOGIN_ATTEMPTS: u32 = 5;
const LOGIN_TIMEOUT: StdDuration = StdDuration::from_secs(300);
const JWT_SECRET: &[u8] = b"your-secret-key"; // In production, use environment variable
const JWT_EXPIRATION: i64 = 3600; // 1 hour

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: i64,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
    expires_in: u64,
}

pub fn hash_password(password: &str, salt: &[u8]) -> Vec<u8> {
    let mut hash = vec![0; 32];
    derive(
        PBKDF2_HMAC_SHA256,
        std::num::NonZeroU32::new(100_000).unwrap(),
        salt,
        password.as_bytes(),
        &mut hash,
    );
    hash
}

pub async fn login(
    app_state: web::Data<Arc<AppState>>,
    login_data: web::Json<LoginRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    // Check rate limiting
    let mut attempts = LOGIN_ATTEMPTS.write();
    if let Some((count, timestamp)) = attempts.get(&login_data.username) {
        if *count >= MAX_LOGIN_ATTEMPTS && timestamp.elapsed() < LOGIN_TIMEOUT {
            warn!("Too many login attempts for user: {}", login_data.username);
            return Err(ErrorUnauthorized("Too many login attempts. Please try again later."));
        }
    }

    // TODO: Replace with proper user database lookup
    let hashed_password = hash_password(&login_data.password, b"");
    if login_data.username == "admin" && hashed_password == hash_password("admin", b"") {
        info!("Successful login for user: {}", login_data.username);
        
        // Reset login attempts
        attempts.remove(&login_data.username);
        
        // Generate JWT token
        let expiration = Utc::now() + Duration::seconds(JWT_EXPIRATION);
        let claims = Claims {
            sub: login_data.username.clone(),
            exp: expiration.timestamp(),
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(JWT_SECRET),
        ).map_err(|e| {
            error!("Failed to generate token: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to generate token")
        })?;
        
        let response = LoginResponse {
            token,
            expires_in: JWT_EXPIRATION as u64,
        };
        
        Ok(HttpResponse::Ok().json(response))
    } else {
        // Increment failed attempts
        let count = attempts.entry(login_data.username.clone())
            .or_insert((0, Instant::now()));
        count.0 += 1;
        count.1 = Instant::now();
        
        warn!("Failed login attempt for user: {}", login_data.username);
        Err(ErrorUnauthorized("Invalid credentials"))
    }
} 