use actix_web::{web, HttpResponse, Responder, error};
use std::sync::Arc;
use crate::core::state::AppState;
use crate::core::error::CursorError;
use crate::monitoring::logger::LoggerSystem;
use crate::monitoring::alert::AlertSystem;
use serde::{Deserialize, Serialize};
use log::{info, warn, error};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use jsonwebtoken::{decode, DecodingKey, Validation};
use actix_web::error::ErrorUnauthorized;
use crate::pool::login::Claims;

lazy_static::lazy_static! {
    static ref CACHED_RESULTS: RwLock<HashMap<String, (ModelTestResponse, Instant)>> = RwLock::new(HashMap::new());
}

const MAX_INPUT_SIZE: usize = 1024 * 1024; // 1MB
const CACHE_DURATION: Duration = Duration::from_secs(300);
const JWT_SECRET: &[u8] = b"your-secret-key"; // Should match login.rs

#[derive(Debug, Deserialize)]
pub struct ModelTestRequest {
    model_name: String,
    input_data: Vec<f32>,
    token: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ModelTestResponse {
    output: Vec<f32>,
    processing_time: f64,
    cache_hit: bool,
}

fn validate_token(token: &str) -> Result<String, actix_web::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    ).map_err(|e| {
        error!("Invalid token: {}", e);
        ErrorUnauthorized("Invalid token")
    })?;
    
    Ok(token_data.claims.sub)
}

fn validate_input(input: &[f32]) -> Result<(), actix_web::Error> {
    if input.is_empty() {
        return Err(error::ErrorBadRequest("Input data cannot be empty"));
    }
    
    if input.len() > MAX_INPUT_SIZE {
        return Err(error::ErrorBadRequest("Input data too large"));
    }
    
    if !input.iter().all(|&x| x.is_finite()) {
        return Err(error::ErrorBadRequest("Input data contains invalid values"));
    }
    
    Ok(())
}

pub async fn playground(
    app_state: web::Data<Arc<AppState>>,
    test_data: web::Json<ModelTestRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    // Validate token
    let username = validate_token(&test_data.token)?;
    
    // Validate input
    validate_input(&test_data.input_data)?;
    
    // Check cache
    let cache_key = format!("{}-{:?}", test_data.model_name, test_data.input_data);
    let mut cache = CACHED_RESULTS.write();
    if let Some((cached_result, timestamp)) = cache.get(&cache_key) {
        if timestamp.elapsed() < CACHE_DURATION {
            info!("Cache hit for model {} by user {}", test_data.model_name, username);
            let mut response = cached_result.clone();
            response.cache_hit = true;
            return Ok(HttpResponse::Ok().json(response));
        }
    }
    
    let start_time = Instant::now();
    
    let models = app_state.models.read();
    if let Some(model) = models.get(&test_data.model_name) {
        match model.process_data(&test_data.input_data) {
            Ok(output) => {
                let processing_time = start_time.elapsed().as_secs_f64();
                info!(
                    "Model {} processed data in {} seconds for user {}",
                    test_data.model_name, processing_time, username
                );
                
                let response = ModelTestResponse {
                    output,
                    processing_time,
                    cache_hit: false,
                };
                
                // Update cache
                cache.insert(cache_key, (response.clone(), Instant::now()));
                
                Ok(HttpResponse::Ok().json(response))
            }
            Err(e) => {
                error!(
                    "Model {} processing failed for user {}: {}",
                    test_data.model_name, username, e
                );
                Err(error::ErrorInternalServerError(format!(
                    "Model processing failed: {}",
                    e
                )))
            }
        }
    } else {
        warn!(
            "Model {} not found for user {}",
            test_data.model_name, username
        );
        Err(error::ErrorNotFound(format!(
            "Model {} not found",
            test_data.model_name
        )))
    }
}

pub async fn verify_token(token: &str) -> Result<Claims, String> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    ).map_err(|e| {
        error!("Invalid token: {}", e);
        ErrorUnauthorized("Invalid token")
    })?;
    
    Ok(token_data.claims)
} 