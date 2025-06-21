use actix_web::{web, HttpResponse, Responder, error};
use std::sync::Arc;
use crate::core::state::AppState;
use log::info;
use crate::core::error::NotFoundError;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use actix_web::middleware::Logger;
use actix_files as fs;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use parking_lot::RwLock;
use std::error::Error;

pub mod pool;
pub mod pool_cok;
pub mod miner;
pub mod reward_system;
pub mod bridges;
pub mod home;
pub mod login;
pub mod playground;

pub use pool::*;
pub use pool_cok::*;
pub use miner::*;
pub use reward_system::*;
pub use bridges::*;
pub use home::*;
pub use login::*;
pub use playground::*;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/pool")
            .route("/health", web::get().to(health_check))
            .route("/home", web::get().to(home::home))
            .route("/login", web::post().to(login::login))
            .route("/playground", web::get().to(playground::playground))
    );
}

/// Инициализация pool модуля
pub async fn initialize() -> Result<(), Box<dyn Error>> {
    log::info!("Initializing pool module");
    Ok(())
}

/// Остановка pool модуля
pub async fn shutdown() -> Result<(), Box<dyn Error>> {
    log::info!("Shutting down pool module");
    Ok(())
}

/// Проверка здоровья pool модуля
pub async fn health_check() -> Result<(), Box<dyn Error>> {
    log::debug!("Pool module health check passed");
    Ok(())
}

pub fn error_handlers() -> actix_web::middleware::ErrorHandlers<actix_web::body::BoxBody> {
    actix_web::middleware::ErrorHandlers::new()
        .handler(
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            |err: error::InternalError<_>| async move {
                error::InternalError::from_response(
                    err,
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Internal server error",
                        "message": err.to_string()
                    }))
                )
            },
        )
        .handler(
            actix_web::http::StatusCode::NOT_FOUND,
            |err: NotFoundError| async move {
                error::InternalError::from_response(
                    err,
                    HttpResponse::NotFound().json(serde_json::json!({
                        "error": "Not found",
                        "message": err.to_string()
                    }))
                )
            },
        )
}

pub async fn handle_error(err: NotFoundError) -> HttpResponse {
    // ... existing code ...
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    pub name: String,
    pub description: String,
    pub max_workers: u32,
    pub max_memory_gb: u32,
    pub max_cpu_cores: u32,
    pub auto_scale: bool,
    pub min_workers: u32,
    pub max_workers_per_vm: u32,
    pub vm_template: String,
    pub network_mode: String,
    pub security_groups: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStats {
    pub total_workers: u32,
    pub active_workers: u32,
    pub total_memory_gb: u32,
    pub total_cpu_cores: u32,
    pub average_load: f32,
    pub last_scale_time: Option<DateTime<Utc>>,
    pub total_tasks: u64,
    pub completed_tasks: u64,
    pub failed_tasks: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolMetrics {
    pub config: PoolConfig,
    pub stats: PoolStats,
}

pub struct PoolManager {
    pools: Arc<Mutex<HashMap<String, PoolMetrics>>>,
}

impl PoolManager {
    pub fn new() -> Self {
        Self {
            pools: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn create_pool(&self, config: PoolConfig) -> Result<(), String> {
        let mut pools = self.pools.lock().await;
        
        if pools.contains_key(&config.name) {
            return Err(format!("Pool '{}' already exists", config.name));
        }

        // Validate pool configuration
        self.validate_pool_config(&config)?;

        let metrics = PoolMetrics {
            config,
            stats: PoolStats {
                total_workers: 0,
                active_workers: 0,
                total_memory_gb: 0,
                total_cpu_cores: 0,
                average_load: 0.0,
                last_scale_time: None,
                total_tasks: 0,
                completed_tasks: 0,
                failed_tasks: 0,
            },
        };

        pools.insert(metrics.config.name.clone(), metrics);
        info!("Created new pool: {}", metrics.config.name);
        Ok(())
    }

    fn validate_pool_config(&self, config: &PoolConfig) -> Result<(), String> {
        if config.max_workers == 0 {
            return Err("max_workers must be greater than 0".to_string());
        }
        if config.max_memory_gb == 0 {
            return Err("max_memory_gb must be greater than 0".to_string());
        }
        if config.max_cpu_cores == 0 {
            return Err("max_cpu_cores must be greater than 0".to_string());
        }
        if config.auto_scale && config.min_workers >= config.max_workers {
            return Err("min_workers must be less than max_workers when auto_scale is enabled".to_string());
        }
        Ok(())
    }

    pub async fn get_pool(&self, name: &str) -> Option<PoolMetrics> {
        self.pools.lock().await.get(name).cloned()
    }

    pub async fn list_pools(&self) -> Vec<PoolMetrics> {
        self.pools.lock().await.values().cloned().collect()
    }

    pub async fn update_pool(&self, name: &str, new_config: PoolConfig) -> Result<(), String> {
        let mut pools = self.pools.lock().await;
        
        if let Some(pool) = pools.get_mut(name) {
            self.validate_pool_config(&new_config)?;
            pool.config = new_config;
            info!("Updated pool: {}", name);
            Ok(())
        } else {
            Err(format!("Pool '{}' not found", name))
        }
    }

    pub async fn delete_pool(&self, name: &str) -> Result<(), String> {
        let mut pools = self.pools.lock().await;
        
        if pools.remove(name).is_some() {
            info!("Deleted pool: {}", name);
            Ok(())
        } else {
            Err(format!("Pool '{}' not found", name))
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/pool")
            .wrap(Logger::default())
            .service(fs::Files::new("/static", "./static").index_file("index.html"))
            .service(
                web::scope("/api")
                    .route("/pools", web::get().to(get_pools))
                    .route("/pools", web::post().to(create_pool))
                    .route("/pools/{name}", web::get().to(get_pool))
                    .route("/pools/{name}", web::put().to(update_pool))
                    .route("/pools/{name}", web::delete().to(delete_pool))
                    .route("/pools/{name}/scale", web::post().to(scale_pool))
                    .route("/pools/{name}/stats", web::get().to(get_pool_stats))
            )
    );
}

async fn get_pools(
    pool_manager: web::Data<PoolManager>,
) -> impl Responder {
    match pool_manager.list_pools().await {
        Ok(pools) => HttpResponse::Ok().json(pools),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

async fn create_pool(
    pool_manager: web::Data<PoolManager>,
    config: web::Json<PoolConfig>,
) -> impl Responder {
    match pool_manager.create_pool(config.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

async fn get_pool(
    pool_manager: web::Data<PoolManager>,
    name: web::Path<String>,
) -> impl Responder {
    match pool_manager.get_pool(&name).await {
        Some(pool) => HttpResponse::Ok().json(pool),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn update_pool(
    pool_manager: web::Data<PoolManager>,
    name: web::Path<String>,
    config: web::Json<PoolConfig>,
) -> impl Responder {
    match pool_manager.update_pool(&name, config.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

async fn delete_pool(
    pool_manager: web::Data<PoolManager>,
    name: web::Path<String>,
) -> impl Responder {
    match pool_manager.delete_pool(&name).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

async fn scale_pool(
    pool_manager: web::Data<PoolManager>,
    name: web::Path<String>,
    scale: web::Json<u32>,
) -> impl Responder {
    // Implement pool scaling logic
    HttpResponse::Ok().finish()
}

async fn get_pool_stats(
    pool_manager: web::Data<PoolManager>,
    name: web::Path<String>,
) -> impl Responder {
    match pool_manager.get_pool(&name).await {
        Some(pool) => HttpResponse::Ok().json(pool.stats),
        None => HttpResponse::NotFound().finish(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfig {
    pub admin_token: String,
    pub allowed_ips: Vec<String>,
    pub rate_limit: u32,
    pub session_timeout_minutes: u32,
}

pub struct PoolAdminPanel {
    bridge_manager: Arc<BridgeManager>,
    pool_manager: Arc<PoolManager>,
    config: AdminConfig,
    sessions: Arc<RwLock<HashMap<String, DateTime<Utc>>>>,
}

impl PoolAdminPanel {
    pub fn new(
        bridge_manager: Arc<BridgeManager>,
        pool_manager: Arc<PoolManager>,
        config: AdminConfig,
    ) -> Self {
        Self {
            bridge_manager,
            pool_manager,
            config,
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start_server(&self, address: &str) -> std::io::Result<()> {
        let bridge_manager = self.bridge_manager.clone();
        let pool_manager = self.pool_manager.clone();
        let config = self.config.clone();
        let sessions = self.sessions.clone();

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(bridge_manager.clone()))
                .app_data(web::Data::new(pool_manager.clone()))
                .app_data(web::Data::new(config.clone()))
                .app_data(web::Data::new(sessions.clone()))
                .service(get_bridges)
                .service(add_bridge)
                .service(remove_bridge)
                .service(get_bridge_transactions)
                .service(get_pools)
                .service(add_pool)
                .service(remove_pool)
                .service(get_pool_stats)
                .service(get_worker_stats)
                .service(login)
                .service(logout)
                .service(serve_index)
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

#[get("/bridges")]
async fn get_bridges(
    bridge_manager: web::Data<Arc<BridgeManager>>,
) -> impl Responder {
    let bridges = bridge_manager.get_all_bridges().await;
    HttpResponse::Ok().json(bridges)
}

#[post("/bridges")]
async fn add_bridge(
    config: web::Json<BridgeConfig>,
    bridge_manager: web::Data<Arc<BridgeManager>>,
) -> impl Responder {
    match bridge_manager.add_bridge(config.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "bridge added"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

#[delete("/bridges/{bridge_id}")]
async fn remove_bridge(
    bridge_id: web::Path<String>,
    bridge_manager: web::Data<Arc<BridgeManager>>,
) -> impl Responder {
    match bridge_manager.remove_bridge(&bridge_id).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "bridge removed"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

#[get("/bridges/{bridge_id}/transactions")]
async fn get_bridge_transactions(
    bridge_id: web::Path<String>,
    bridge_manager: web::Data<Arc<BridgeManager>>,
) -> impl Responder {
    match bridge_manager.get_transactions_by_status(bridge_id.into_inner()).await {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

#[get("/pools")]
async fn get_pools(
    pool_manager: web::Data<Arc<PoolManager>>,
) -> impl Responder {
    let pools = pool_manager.get_all_pools().await;
    HttpResponse::Ok().json(pools)
}

#[post("/pools")]
async fn add_pool(
    config: web::Json<PoolConfig>,
    pool_manager: web::Data<Arc<PoolManager>>,
) -> impl Responder {
    match pool_manager.add_pool(config.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "pool added"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

#[delete("/pools/{pool_id}")]
async fn remove_pool(
    pool_id: web::Path<String>,
    pool_manager: web::Data<Arc<PoolManager>>,
) -> impl Responder {
    match pool_manager.remove_pool(&pool_id).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "pool removed"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

#[get("/pools/{pool_id}/stats")]
async fn get_pool_stats(
    pool_id: web::Path<String>,
    pool_manager: web::Data<Arc<PoolManager>>,
) -> impl Responder {
    match pool_manager.get_pool_stats(&pool_id).await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

#[get("/pools/{pool_id}/workers/{worker_id}/stats")]
async fn get_worker_stats(
    path: web::Path<(String, String)>,
    pool_manager: web::Data<Arc<PoolManager>>,
) -> impl Responder {
    let (pool_id, worker_id) = path.into_inner();
    match pool_manager.get_worker_stats(&pool_id, &worker_id).await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

#[get("/")]
async fn serve_index() -> impl Responder {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Pool Admin Panel</title>
        <style>
            body { font-family: Arial, sans-serif; margin: 20px; }
            .container { max-width: 1200px; margin: 0 auto; }
            .card { border: 1px solid #ddd; padding: 20px; margin: 10px 0; border-radius: 5px; }
            .btn { padding: 10px 20px; background: #007bff; color: white; border: none; border-radius: 5px; cursor: pointer; }
            .btn-danger { background: #dc3545; }
            .btn-success { background: #28a745; }
            table { width: 100%; border-collapse: collapse; }
            th, td { padding: 10px; border: 1px solid #ddd; text-align: left; }
            .form-group { margin: 10px 0; }
            input, select { padding: 8px; width: 100%; }
            .tabs { display: flex; margin-bottom: 20px; }
            .tab { padding: 10px 20px; cursor: pointer; border: 1px solid #ddd; }
            .tab.active { background: #007bff; color: white; }
            .tab-content { display: none; }
            .tab-content.active { display: block; }
        </style>
    </head>
    <body>
        <div class="container">
            <h1>Pool Admin Panel</h1>
            
            <div class="tabs">
                <div class="tab active" onclick="showTab('pools')">Pools</div>
                <div class="tab" onclick="showTab('bridges')">Bridges</div>
                <div class="tab" onclick="showTab('workers')">Workers</div>
            </div>

            <div id="pools" class="tab-content active">
                <div class="card">
                    <h2>Pools</h2>
                    <table id="pools-table">
                        <thead>
                            <tr>
                                <th>Name</th>
                                <th>Workers</th>
                                <th>Hashrate</th>
                                <th>Shares</th>
                                <th>Status</th>
                                <th>Actions</th>
                            </tr>
                        </thead>
                        <tbody id="pools-body"></tbody>
                    </table>
                    <button class="btn" onclick="showAddPoolForm()">Add Pool</button>
                </div>
            </div>

            <div id="bridges" class="tab-content">
                <div class="card">
                    <h2>Bridges</h2>
                    <table id="bridges-table">
                        <thead>
                            <tr>
                                <th>Name</th>
                                <th>Source</th>
                                <th>Target</th>
                                <th>Status</th>
                                <th>Actions</th>
                            </tr>
                        </thead>
                        <tbody id="bridges-body"></tbody>
                    </table>
                    <button class="btn" onclick="showAddBridgeForm()">Add Bridge</button>
                </div>
            </div>

            <div id="workers" class="tab-content">
                <div class="card">
                    <h2>Workers</h2>
                    <table id="workers-table">
                        <thead>
                            <tr>
                                <th>ID</th>
                                <th>Pool</th>
                                <th>Hashrate</th>
                                <th>Shares</th>
                                <th>Status</th>
                                <th>Actions</th>
                            </tr>
                        </thead>
                        <tbody id="workers-body"></tbody>
                    </table>
                </div>
            </div>
        </div>

        <script>
            let sessionId = null;

            async function login() {
                const token = prompt('Enter admin token:');
                const response = await fetch('/login', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ token })
                });
                if (response.ok) {
                    const data = await response.json();
                    sessionId = data.session_id;
                    loadData();
                } else {
                    alert('Login failed');
                }
            }

            async function loadData() {
                if (!sessionId) {
                    login();
                    return;
                }

                // Load Pools
                const poolsResponse = await fetch('/pools', {
                    headers: { 'Authorization': sessionId }
                });
                const pools = await poolsResponse.json();
                const poolsBody = document.getElementById('pools-body');
                poolsBody.innerHTML = pools.map(pool => `
                    <tr>
                        <td>${pool.config.name}</td>
                        <td>${pool.stats.active_workers}/${pool.stats.total_workers}</td>
                        <td>${pool.stats.total_hashrate.toFixed(2)} H/s</td>
                        <td>${pool.stats.total_shares}</td>
                        <td>${pool.config.maintenance_mode ? 'Maintenance' : 'Active'}</td>
                        <td>
                            <button class="btn" onclick="showPoolStats('${pool.config.name}')">Stats</button>
                            <button class="btn btn-danger" onclick="removePool('${pool.config.name}')">Remove</button>
                        </td>
                    </tr>
                `).join('');

                // Load Bridges
                const bridgesResponse = await fetch('/bridges', {
                    headers: { 'Authorization': sessionId }
                });
                const bridges = await bridgesResponse.json();
                const bridgesBody = document.getElementById('bridges-body');
                bridgesBody.innerHTML = bridges.map(bridge => `
                    <tr>
                        <td>${bridge.config.name}</td>
                        <td>${bridge.config.source_network}</td>
                        <td>${bridge.config.target_network}</td>
                        <td>${bridge.config.active ? 'Active' : 'Inactive'}</td>
                        <td>
                            <button class="btn" onclick="showBridgeTransactions('${bridge.config.name}')">Transactions</button>
                            <button class="btn btn-danger" onclick="removeBridge('${bridge.config.name}')">Remove</button>
                        </td>
                    </tr>
                `).join('');

                // Load Workers
                const workers = [];
                for (const pool of pools) {
                    for (const worker of pool.stats.worker_stats) {
                        workers.push({
                            ...worker,
                            pool_name: pool.config.name
                        });
                    }
                }
                const workersBody = document.getElementById('workers-body');
                workersBody.innerHTML = workers.map(worker => `
                    <tr>
                        <td>${worker.worker_id}</td>
                        <td>${worker.pool_name}</td>
                        <td>${worker.hashrate.toFixed(2)} H/s</td>
                        <td>${worker.shares}</td>
                        <td>${worker.last_share_time ? 'Active' : 'Inactive'}</td>
                        <td>
                            <button class="btn" onclick="showWorkerStats('${worker.pool_name}', '${worker.worker_id}')">Stats</button>
                        </td>
                    </tr>
                `).join('');
            }

            async function addPool(pool) {
                const response = await fetch('/pools', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                        'Authorization': sessionId
                    },
                    body: JSON.stringify(pool)
                });
                if (response.ok) {
                    loadData();
                }
            }

            async function removePool(poolId) {
                const response = await fetch(`/pools/${poolId}`, {
                    method: 'DELETE',
                    headers: { 'Authorization': sessionId }
                });
                if (response.ok) {
                    loadData();
                }
            }

            async function addBridge(bridge) {
                const response = await fetch('/bridges', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                        'Authorization': sessionId
                    },
                    body: JSON.stringify(bridge)
                });
                if (response.ok) {
                    loadData();
                }
            }

            async function removeBridge(bridgeId) {
                const response = await fetch(`/bridges/${bridgeId}`, {
                    method: 'DELETE',
                    headers: { 'Authorization': sessionId }
                });
                if (response.ok) {
                    loadData();
                }
            }

            async function showPoolStats(poolId) {
                const response = await fetch(`/pools/${poolId}/stats`, {
                    headers: { 'Authorization': sessionId }
                });
                if (response.ok) {
                    const stats = await response.json();
                    alert(JSON.stringify(stats, null, 2));
                }
            }

            async function showWorkerStats(poolId, workerId) {
                const response = await fetch(`/pools/${poolId}/workers/${workerId}/stats`, {
                    headers: { 'Authorization': sessionId }
                });
                if (response.ok) {
                    const stats = await response.json();
                    alert(JSON.stringify(stats, null, 2));
                }
            }

            async function showBridgeTransactions(bridgeId) {
                const response = await fetch(`/bridges/${bridgeId}/transactions`, {
                    headers: { 'Authorization': sessionId }
                });
                if (response.ok) {
                    const transactions = await response.json();
                    alert(JSON.stringify(transactions, null, 2));
                }
            }

            function showAddPoolForm() {
                const pool = {
                    name: prompt('Pool Name:'),
                    url: prompt('Pool URL:'),
                    api_key: prompt('API Key:'),
                    min_workers: parseInt(prompt('Min Workers:')),
                    max_workers: parseInt(prompt('Max Workers:')),
                    min_memory_gb: parseInt(prompt('Min Memory (GB):')),
                    max_memory_gb: parseInt(prompt('Max Memory (GB):')),
                    allowed_gpu_models: prompt('Allowed GPU Models (comma-separated):').split(','),
                    maintenance_mode: false,
                    algorithm: prompt('Algorithm:'),
                    difficulty: parseInt(prompt('Difficulty:')),
                    payout_threshold: parseFloat(prompt('Payout Threshold:')),
                    fee_percentage: parseFloat(prompt('Fee Percentage:')),
                };
                addPool(pool);
            }

            function showAddBridgeForm() {
                const bridge = {
                    name: prompt('Bridge Name:'),
                    source_network: prompt('Source Network:'),
                    target_network: prompt('Target Network:'),
                    fee_percentage: parseFloat(prompt('Fee Percentage:')),
                    min_amount: parseFloat(prompt('Min Amount:')),
                    max_amount: parseFloat(prompt('Max Amount:')),
                    source_network_url: prompt('Source Network URL:'),
                    target_network_url: prompt('Target Network URL:'),
                    api_key: prompt('API Key:'),
                    timeout: parseInt(prompt('Timeout (ms):')),
                    retry_attempts: parseInt(prompt('Retry Attempts:')),
                    active: true,
                };
                addBridge(bridge);
            }

            function showTab(tabId) {
                document.querySelectorAll('.tab').forEach(tab => tab.classList.remove('active'));
                document.querySelectorAll('.tab-content').forEach(content => content.classList.remove('active'));
                document.querySelector(`.tab[onclick="showTab('${tabId}')"]`).classList.add('active');
                document.getElementById(tabId).classList.add('active');
            }

            // Initial load
            login();
        </script>
    </body>
    </html>
    "#;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
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
            session_timeout_minutes: 30,
        };
        let bridge_manager = Arc::new(BridgeManager::new());
        let pool_manager = Arc::new(PoolManager::new());
        let panel = PoolAdminPanel::new(bridge_manager, pool_manager, config);
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(panel.bridge_manager.clone()))
                .app_data(web::Data::new(panel.pool_manager.clone()))
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