//! Worker Management page
//!
//! PH-S222: workers page uses slim `admin_layout_workers` + `admin_workers_patch`.

use crate::ui::admin::admin_layout_workers;
use axum::response::Html;

/// Worker management page
pub async fn admin_workers() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }
    function Ep() { return typeof poolaiT === 'function' ? poolaiT('err.errorPrefix', 'Error: ') : 'Error: '; }

    async function loadWorkers() {
      adminShowLoading('workers-list', T('admin.wrk.loading', 'Loading workers…'));
      try {
        const workers = await fetchJson('/api/v1/workers');
        renderWorkers(workers);
      } catch (e) {
        adminShowInlineError('workers-list', e);
        showNotification(T('admin.wrk.errLoad', 'Error loading workers: ') + e.message, 'error');
      }
    }
    
    function renderWorkers(workers) {
      const el = document.getElementById('workers-list');
      if (!el) return;
      el.innerHTML = poolaiRenderWorkersPanel(workers, {
        id: T('admin.wrk.col.id', 'ID'),
        status: T('admin.wrk.col.status', 'Status'),
        metrics: T('admin.wrk.col.metrics', 'Metrics'),
        actions: T('admin.wrk.col.actions', 'Actions'),
        tableAria: T('admin.nav.workers', 'Workers'),
        healthy: T('workers.healthy', 'Healthy'),
        unhealthy: T('workers.unhealthy', 'Unhealthy'),
        reqLabel: T('admin.wrk.reqLabel', 'Requests:'),
        delete: T('ui.delete', 'Delete'),
        empty: T('admin.wrk.empty', 'No workers found'),
      });
      el.querySelectorAll('[data-worker-id]').forEach(function(btn) {
        btn.addEventListener('click', function() {
          deleteWorker(btn.getAttribute('data-worker-id'));
        });
      });
    }
    
    async function deleteWorker(id) {
      if (!confirm(T('admin.wrk.confirmDel', 'Delete worker {id}?').replace(/\{id\}/g, id))) return;
      try {
        await fetchJson(`/api/v1/workers/${encodeURIComponent(id)}`, { method: 'DELETE' });
        showNotification(T('admin.wrk.deletedOk', 'Worker deleted'), 'success');
        loadWorkers();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      }
    }
    
    function showCreateWorkerModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissionsAdminOp', 'Insufficient permissions. Admin or Operator role required.'), 'error');
        return;
      }
      showModal('createWorkerModal');
    }
    
    async function handleCreateWorker(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = T('workers.creatingSubmit', 'Creating…');
      
      try {
        const payload = {
          worker_id: document.getElementById('workerId').value,
          max_concurrent_requests: document.getElementById('workerMaxConcurrent').value ? parseInt(document.getElementById('workerMaxConcurrent').value, 10) : undefined,
          request_timeout_ms: document.getElementById('workerTimeout').value ? parseInt(document.getElementById('workerTimeout').value, 10) : undefined,
          health_check_interval_ms: document.getElementById('workerHealthCheck').value ? parseInt(document.getElementById('workerHealthCheck').value, 10) : undefined,
          enable_caching: document.getElementById('workerEnableCaching').checked,
          cache_size: document.getElementById('workerCacheSize').value ? parseInt(document.getElementById('workerCacheSize').value, 10) : undefined,
          max_memory_mb: document.getElementById('workerMaxMemory').value ? parseInt(document.getElementById('workerMaxMemory').value, 10) : undefined,
          cpu_priority: document.getElementById('workerCpuPriority').value ? parseInt(document.getElementById('workerCpuPriority').value, 10) : undefined,
          gpu_device: document.getElementById('workerGpuDevice').value ? parseInt(document.getElementById('workerGpuDevice').value, 10) : undefined,
          auto_restart: document.getElementById('workerAutoRestart').checked,
          resource_monitoring: document.getElementById('workerResourceMonitoring').checked
        };
        
        Object.keys(payload).forEach(key => {
          if (payload[key] === undefined) {
            delete payload[key];
          }
        });
        
        await fetchJson('/api/v1/workers', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification(T('workers.createdOk', 'Worker created successfully'), 'success');
        hideModal('createWorkerModal');
        form.reset();
        
        setTimeout(() => {
          loadWorkers();
        }, 500);
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    loadWorkers();
    setInterval(loadWorkers, 5000);

    async function loadGalaxyVirtualNodes() {
      const el = document.getElementById('galaxy-virtual-nodes-list');
      if (!el) return;
      adminShowLoading('galaxy-virtual-nodes-list', T('admin.wrk.vnLoading', 'Loading virtual nodes…'));
      try {
        const data = await fetchJson('/api/v1/discovery/virtual-nodes');
        const nodes = (data && data.nodes) ? data.nodes : [];
        nodes.sort(function(a, b) {
          var la = (a.galaxy && a.galaxy.telemetry && a.galaxy.telemetry.latency_ms_p50) || 999999;
          var lb = (b.galaxy && b.galaxy.telemetry && b.galaxy.telemetry.latency_ms_p50) || 999999;
          return la - lb;
        });
        el.innerHTML = poolaiRenderGalaxyVirtualNodesPanel(nodes, {
          peer: T('admin.wrk.vnColPeer', 'Peer'),
          origin: T('admin.wrk.vnColOrigin', 'Origin'),
          region: T('admin.wrk.vnColRegion', 'Region'),
          latency: T('admin.wrk.vnColLatency', 'Latency ms p50'),
          stale: T('admin.wrk.vnColStale', 'Liveness'),
          tableAria: T('admin.wrk.vnSection', 'Galaxy virtual nodes'),
          empty: T('admin.wrk.vnEmpty', 'No virtual nodes registered'),
        });
      } catch (e) {
        adminShowInlineError('galaxy-virtual-nodes-list', e);
      }
    }
    loadGalaxyVirtualNodes();
    setInterval(loadGalaxyVirtualNodes, 8000);
    "#;

    admin_layout_workers(
        "admin.page.workers",
        "Worker Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2 data-i18n="admin.wrk.section">Workers</h2>
            <button type="button" class="btn btn-primary" onclick="showCreateWorkerModal()" data-i18n="admin.wrk.createBtn" data-i18n-aria="workers.createBtnAria">Create Worker</button>
          </div>
          <div id="workers-list"></div>
        </div>

        <div class="admin-section">
          <div class="admin-header">
            <h2 data-i18n="admin.wrk.vnSection">Galaxy virtual nodes</h2>
          </div>
          <p class="muted admin-hint" data-i18n="admin.wrk.vnHint">
            Origin badges and latency sort from unified Galaxy worker DTO (PH-S507/S508).
          </p>
          <div id="galaxy-virtual-nodes-list"></div>
        </div>
        
        <div id="createWorkerModal" class="modal" role="dialog" aria-labelledby="createWorkerModalTitle" aria-modal="false" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createWorkerModalTitle" data-i18n="admin.wrk.title">Create Worker</h3>
              <button type="button" class="modal-close" data-i18n-aria="ui.closeDialogAria" onclick="hideModal('createWorkerModal')">&times;</button>
            </div>
            <form id="createWorkerForm" onsubmit="handleCreateWorker(event)">
              <div class="form-group">
                <label for="workerId"><span data-i18n="workers.label.id">Worker ID</span> <span class="required">*</span></label>
                <input type="text" id="workerId" name="worker_id" required data-i18n-placeholder="workers.ph.id" placeholder="worker-001" pattern="[a-zA-Z0-9_-]+" />
                <small class="form-hint" data-i18n="admin.wrk.hintId">Alphanumeric, hyphens, and underscores only</small>
              </div>
              <div class="form-group">
                <label for="workerMaxConcurrent" data-i18n="workers.label.maxConcurrent">Max Concurrent Requests</label>
                <input type="number" id="workerMaxConcurrent" name="max_concurrent_requests" min="1" max="1000" value="10" />
              </div>
              <div class="form-group">
                <label for="workerTimeout" data-i18n="workers.label.timeout">Request Timeout (ms)</label>
                <input type="number" id="workerTimeout" name="request_timeout_ms" min="100" max="300000" value="5000" />
              </div>
              <div class="form-group">
                <label for="workerHealthCheck" data-i18n="workers.label.healthInterval">Health Check Interval (ms)</label>
                <input type="number" id="workerHealthCheck" name="health_check_interval_ms" min="100" max="60000" value="1000" />
              </div>
              <div class="form-group">
                <label for="workerEnableCaching">
                  <input type="checkbox" id="workerEnableCaching" name="enable_caching" checked />
                  <span data-i18n="workers.enableCache">Enable Caching</span>
                </label>
              </div>
              <div class="form-group">
                <label for="workerCacheSize" data-i18n="workers.label.cacheSize">Cache Size</label>
                <input type="number" id="workerCacheSize" name="cache_size" min="0" max="100000" value="1000" />
              </div>
              <div class="form-group">
                <label for="workerMaxMemory" data-i18n="workers.label.maxMemory">Max Memory (MB)</label>
                <input type="number" id="workerMaxMemory" name="max_memory_mb" min="128" max="131072" value="2048" />
              </div>
              <div class="form-group">
                <label for="workerCpuPriority" data-i18n="workers.label.cpuPriority">CPU Priority (0-10)</label>
                <input type="number" id="workerCpuPriority" name="cpu_priority" min="0" max="10" value="5" />
              </div>
              <div class="form-group">
                <label for="workerGpuDevice" data-i18n="workers.label.gpuDevice">GPU Device ID (optional)</label>
                <input type="number" id="workerGpuDevice" name="gpu_device" min="0" data-i18n-placeholder="workers.ph.gpu" placeholder="Leave empty for no GPU" />
              </div>
              <div class="form-group">
                <label for="workerAutoRestart">
                  <input type="checkbox" id="workerAutoRestart" name="auto_restart" checked />
                  <span data-i18n="admin.wrk.autoRestart">Auto Restart</span>
                </label>
              </div>
              <div class="form-group">
                <label for="workerResourceMonitoring">
                  <input type="checkbox" id="workerResourceMonitoring" name="resource_monitoring" checked />
                  <span data-i18n="admin.wrk.resourceMon">Resource Monitoring</span>
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createWorkerModal')" data-i18n="ui.cancel">Cancel</button>
                <button type="submit" class="btn btn-primary" data-i18n="ui.create">Create</button>
              </div>
            </form>
          </div>
        </div>
        "#,
        script,
    )
}

#[tokio::test]
async fn admin_workers_page_slim_workers_i18n_patch_ph_s222() {
    let html = admin_workers().await.0;
    assert!(html.contains("window.__poolaiAdminI18nRust="));
    assert!(html.contains(r#""admin.page.workers""#));
    assert!(html.contains(r#""admin.wrk.section""#));
    assert!(!html.contains(r#""admin.jobs.leaseState.active""#));
    assert!(!html.contains(r#""admin.lib.loading""#));
}
