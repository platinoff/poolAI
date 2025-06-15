use actix_web::{web, App, HttpServer, HttpResponse, Responder, get, post, delete};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashMap;
use log::{info, warn, error};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use super::pool_cok::{PoolNode, PoolMigrationManager, MigrationTask, PoolError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfig {
    pub admin_token: String,
    pub allowed_ips: Vec<String>,
    pub rate_limit: u32,
}

pub struct AdminPanel {
    manager: Arc<PoolMigrationManager>,
    config: AdminConfig,
    sessions: Arc<RwLock<HashMap<String, DateTime<Utc>>>>,
}

impl AdminPanel {
    pub fn new(manager: Arc<PoolMigrationManager>, config: AdminConfig) -> Self {
        Self {
            manager,
            config,
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start_server(&self, address: &str) -> std::io::Result<()> {
        let manager = self.manager.clone();
        let config = self.config.clone();
        let sessions = self.sessions.clone();

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(manager.clone()))
                .app_data(web::Data::new(config.clone()))
                .app_data(web::Data::new(sessions.clone()))
                .service(get_nodes)
                .service(add_node)
                .service(remove_node)
                .service(get_migrations)
                .service(force_migration)
                .service(login)
                .service(logout)
        })
        .bind(address)?
        .run()
        .await
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    token: String,
}

#[post("/login")]
async fn login(
    req: web::Json<LoginRequest>,
    config: web::Data<AdminConfig>,
    sessions: web::Data<Arc<RwLock<HashMap<String, DateTime<Utc>>>>>,
) -> impl Responder {
    if req.token != config.admin_token {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid token"
        }));
    }

    let session_id = Uuid::new_v4().to_string();
    let mut sessions = sessions.write();
    sessions.insert(session_id.clone(), Utc::now());

    HttpResponse::Ok().json(serde_json::json!({
        "session_id": session_id
    }))
}

#[post("/logout")]
async fn logout(
    session_id: web::Header<String>,
    sessions: web::Data<Arc<RwLock<HashMap<String, DateTime<Utc>>>>>,
) -> impl Responder {
    let mut sessions = sessions.write();
    sessions.remove(&session_id.to_string());
    HttpResponse::Ok().json(serde_json::json!({
        "status": "logged out"
    }))
}

#[get("/nodes")]
async fn get_nodes(
    manager: web::Data<Arc<PoolMigrationManager>>,
) -> impl Responder {
    let nodes = manager.nodes.read();
    HttpResponse::Ok().json(nodes.values().collect::<Vec<_>>())
}

#[post("/nodes")]
async fn add_node(
    node: web::Json<PoolNode>,
    manager: web::Data<Arc<PoolMigrationManager>>,
) -> impl Responder {
    manager.add_node(node.into_inner());
    HttpResponse::Ok().json(serde_json::json!({
        "status": "node added"
    }))
}

#[delete("/nodes/{node_id}")]
async fn remove_node(
    node_id: web::Path<String>,
    manager: web::Data<Arc<PoolMigrationManager>>,
) -> impl Responder {
    manager.remove_node(&node_id);
    HttpResponse::Ok().json(serde_json::json!({
        "status": "node removed"
    }))
}

#[get("/migrations")]
async fn get_migrations(
    manager: web::Data<Arc<PoolMigrationManager>>,
) -> impl Responder {
    let migrations = manager.get_migration_history();
    HttpResponse::Ok().json(migrations)
}

#[post("/migrations/force")]
async fn force_migration(
    task: web::Json<MigrationTask>,
    manager: web::Data<Arc<PoolMigrationManager>>,
) -> impl Responder {
    match manager.execute_migration(
        &task.source_node,
        &task.target_node,
        task.task_data.clone(),
        task.priority,
    ).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "migration started"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn test_login() {
        let config = AdminConfig {
            admin_token: "test_token".to_string(),
            allowed_ips: vec![],
            rate_limit: 100,
        };
        let manager = Arc::new(PoolMigrationManager::new(vec![0; 32]));
        let panel = AdminPanel::new(manager, config);
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(panel.manager.clone()))
                .app_data(web::Data::new(panel.config.clone()))
                .app_data(web::Data::new(panel.sessions.clone()))
                .service(login)
        ).await;

        let req = test::TestRequest::post()
            .uri("/login")
            .set_json(&LoginRequest {
                token: "test_token".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
} 