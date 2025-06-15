use actix_web::{web, App, HttpServer, HttpResponse, Responder, get, post, delete};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashMap;
use log::{info, warn, error};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use super::vm::{VmManager, VmConfig, VmStatus, VmStats};
use super::worker_interface::{WorkerInterfaceManager, HardwareInfo, WorkerMetrics};
use tokio::sync::Mutex;
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;
use cursor_codes::runtime::worker::WorkerManager;
use cursor_codes::runtime::scheduler::SchedulerSystem;
use cursor_codes::runtime::queue::QueueSystem;
use cursor_codes::runtime::cache::CacheSystem;
use cursor_codes::runtime::storage::StorageSystem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfig {
    pub admin_token: String,
    pub allowed_ips: Vec<String>,
    pub rate_limit: u32,
    pub session_timeout_minutes: u32,
}

pub struct AdminPanel {
    vm_manager: Arc<RwLock<VmManager>>,
    worker_interface: Arc<RwLock<WorkerInterfaceManager>>,
    config: AdminConfig,
    sessions: Arc<RwLock<HashMap<String, DateTime<Utc>>>>,
}

impl AdminPanel {
    pub fn new(
        vm_manager: Arc<RwLock<VmManager>>,
        worker_interface: Arc<RwLock<WorkerInterfaceManager>>,
        config: AdminConfig,
    ) -> Self {
        Self {
            vm_manager,
            worker_interface,
            config,
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start_server(&self, address: &str) -> std::io::Result<()> {
        let vm_manager = self.vm_manager.clone();
        let worker_interface = self.worker_interface.clone();
        let config = self.config.clone();
        let sessions = self.sessions.clone();

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(vm_manager.clone()))
                .app_data(web::Data::new(worker_interface.clone()))
                .app_data(web::Data::new(config.clone()))
                .app_data(web::Data::new(sessions.clone()))
                .service(get_vms)
                .service(add_vm)
                .service(remove_vm)
                .service(start_vm)
                .service(stop_vm)
                .service(get_vm_stats)
                .service(get_workers)
                .service(get_worker_metrics)
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

#[get("/vms")]
async fn get_vms(
    vm_manager: web::Data<Arc<RwLock<VmManager>>>,
) -> impl Responder {
    let vms = vm_manager.read().await.list_vms().await;
    HttpResponse::Ok().json(vms)
}

#[post("/vms")]
async fn add_vm(
    vm_config: web::Json<VmConfig>,
    vm_manager: web::Data<Arc<RwLock<VmManager>>>,
) -> impl Responder {
    match vm_manager.write().await.create_vm(vm_config.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "VM created"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

#[delete("/vms/{vm_id}")]
async fn remove_vm(
    vm_id: web::Path<String>,
    vm_manager: web::Data<Arc<RwLock<VmManager>>>,
) -> impl Responder {
    // First stop the VM if it's running
    if let Some(vm) = vm_manager.read().await.get_vm(&vm_id).await {
        if vm.status == VmStatus::Running {
            if let Err(e) = vm_manager.write().await.stop_vm(&vm_id).await {
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": e.to_string()
                }));
            }
        }
    }

    HttpResponse::Ok().json(serde_json::json!({
        "status": "VM removed"
    }))
}

#[post("/vms/{vm_id}/start")]
async fn start_vm(
    vm_id: web::Path<String>,
    vm_manager: web::Data<Arc<RwLock<VmManager>>>,
) -> impl Responder {
    match vm_manager.write().await.start_vm(&vm_id).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "VM started"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

#[post("/vms/{vm_id}/stop")]
async fn stop_vm(
    vm_id: web::Path<String>,
    vm_manager: web::Data<Arc<RwLock<VmManager>>>,
) -> impl Responder {
    match vm_manager.write().await.stop_vm(&vm_id).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "VM stopped"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

#[get("/vms/{vm_id}/stats")]
async fn get_vm_stats(
    vm_id: web::Path<String>,
    vm_manager: web::Data<Arc<RwLock<VmManager>>>,
) -> impl Responder {
    match vm_manager.read().await.get_vm_stats(&vm_id).await {
        Some(stats) => HttpResponse::Ok().json(stats),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": "VM not found"
        })),
    }
}

#[get("/workers")]
async fn get_workers(
    worker_interface: web::Data<Arc<RwLock<WorkerInterfaceManager>>>,
) -> impl Responder {
    let workers = worker_interface.read().await.get_workers().await;
    HttpResponse::Ok().json(workers)
}

#[get("/workers/{worker_id}/metrics")]
async fn get_worker_metrics(
    worker_id: web::Path<String>,
    worker_interface: web::Data<Arc<RwLock<WorkerInterfaceManager>>>,
) -> impl Responder {
    match worker_interface.read().await.get_worker_metrics(&worker_id).await {
        Some(metrics) => HttpResponse::Ok().json(metrics),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Worker not found"
        })),
    }
}

#[get("/")]
async fn serve_index() -> impl Responder {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>RAID Admin Panel</title>
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
            <h1>RAID Admin Panel</h1>
            
            <div class="tabs">
                <div class="tab active" onclick="showTab('vms')">Virtual Machines</div>
                <div class="tab" onclick="showTab('workers')">Workers</div>
            </div>

            <div id="vms" class="tab-content active">
                <div class="card">
                    <h2>Virtual Machines</h2>
                    <table id="vms-table">
                        <thead>
                            <tr>
                                <th>ID</th>
                                <th>Name</th>
                                <th>Status</th>
                                <th>CPU</th>
                                <th>Memory</th>
                                <th>Disk</th>
                                <th>Actions</th>
                            </tr>
                        </thead>
                        <tbody id="vms-body"></tbody>
                    </table>
                    <button class="btn" onclick="showAddVmForm()">Add VM</button>
                </div>
            </div>

            <div id="workers" class="tab-content">
                <div class="card">
                    <h2>Workers</h2>
                    <table id="workers-table">
                        <thead>
                            <tr>
                                <th>ID</th>
                                <th>Type</th>
                                <th>Status</th>
                                <th>CPU Usage</th>
                                <th>Memory Usage</th>
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

                // Load VMs
                const vmsResponse = await fetch('/vms', {
                    headers: { 'Authorization': sessionId }
                });
                const vms = await vmsResponse.json();
                const vmsBody = document.getElementById('vms-body');
                vmsBody.innerHTML = vms.map(vm => `
                    <tr>
                        <td>${vm.id}</td>
                        <td>${vm.name}</td>
                        <td>${vm.status}</td>
                        <td>${vm.cpu_cores}</td>
                        <td>${vm.memory_mb}MB</td>
                        <td>${vm.disk_gb}GB</td>
                        <td>
                            ${vm.status === 'Running' ? 
                                `<button class="btn btn-danger" onclick="stopVm('${vm.id}')">Stop</button>` :
                                `<button class="btn btn-success" onclick="startVm('${vm.id}')">Start</button>`
                            }
                            <button class="btn btn-danger" onclick="removeVm('${vm.id}')">Remove</button>
                        </td>
                    </tr>
                `).join('');

                // Load Workers
                const workersResponse = await fetch('/workers', {
                    headers: { 'Authorization': sessionId }
                });
                const workers = await workersResponse.json();
                const workersBody = document.getElementById('workers-body');
                workersBody.innerHTML = workers.map(worker => `
                    <tr>
                        <td>${worker.id}</td>
                        <td>${worker.device_type}</td>
                        <td>${worker.status}</td>
                        <td>${worker.metrics?.cpu_usage || 0}%</td>
                        <td>${worker.metrics?.memory_usage || 0}MB</td>
                        <td>
                            <button class="btn" onclick="showWorkerMetrics('${worker.id}')">Metrics</button>
                        </td>
                    </tr>
                `).join('');
            }

            async function addVm(vm) {
                const response = await fetch('/vms', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                        'Authorization': sessionId
                    },
                    body: JSON.stringify(vm)
                });
                if (response.ok) {
                    loadData();
                }
            }

            async function removeVm(vmId) {
                const response = await fetch(`/vms/${vmId}`, {
                    method: 'DELETE',
                    headers: { 'Authorization': sessionId }
                });
                if (response.ok) {
                    loadData();
                }
            }

            async function startVm(vmId) {
                const response = await fetch(`/vms/${vmId}/start`, {
                    method: 'POST',
                    headers: { 'Authorization': sessionId }
                });
                if (response.ok) {
                    loadData();
                }
            }

            async function stopVm(vmId) {
                const response = await fetch(`/vms/${vmId}/stop`, {
                    method: 'POST',
                    headers: { 'Authorization': sessionId }
                });
                if (response.ok) {
                    loadData();
                }
            }

            async function showWorkerMetrics(workerId) {
                const response = await fetch(`/workers/${workerId}/metrics`, {
                    headers: { 'Authorization': sessionId }
                });
                if (response.ok) {
                    const metrics = await response.json();
                    alert(JSON.stringify(metrics, null, 2));
                }
            }

            function showAddVmForm() {
                const vm = {
                    id: prompt('VM ID:'),
                    name: prompt('VM Name:'),
                    cpu_cores: parseInt(prompt('CPU Cores:')),
                    memory_mb: parseInt(prompt('Memory (MB):')),
                    disk_gb: parseInt(prompt('Disk (GB):')),
                    image: prompt('Image:'),
                    status: 'Stopped',
                    ports: [],
                    max_restart_attempts: 3,
                    restart_delay_ms: 5000,
                    health_check_interval_ms: 30000,
                    auto_restart: true,
                    devices: []
                };
                addVm(vm);
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
        let vm_manager = Arc::new(RwLock::new(VmManager::new()));
        let worker_interface = Arc::new(RwLock::new(WorkerInterfaceManager::new("test_token".to_string())));
        let panel = AdminPanel::new(vm_manager, worker_interface, config);
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(panel.vm_manager.clone()))
                .app_data(web::Data::new(panel.worker_interface.clone()))
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