// network/mod.rs
pub mod api;
pub mod ws;
pub mod auth;

use axum::Router;
use std::net::SocketAddr;
use tracing::info;

pub async fn start_server(addr: SocketAddr) {
    info!("Starting network server on {}", addr);
    
    let app = Router::new()
        .nest("/api/v1", api::create_api_routes());
    
    info!("Network server started successfully");
    
    // TODO: Реализовать реальный запуск сервера
    println!("[network] Server would start on {}", addr);
} 