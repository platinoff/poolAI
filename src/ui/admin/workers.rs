//! Worker Management page
//!
//! Provides worker pool configuration and monitoring.

use crate::ui::admin::admin_layout;
use axum::response::Html;

/// Worker management page
pub async fn admin_workers() -> Html<String> {
    let script = r#"
    async function loadWorkers() {
      adminShowLoading('workers-list', 'Loading workers…');
      try {
        const workers = await fetchJson('/api/v1/workers');
        renderWorkers(workers);
      } catch (e) {
        adminShowInlineError('workers-list', e);
        showNotification('Error loading workers: ' + e.message, 'error');
      }
    }
    
    function renderWorkers(workers) {
      const el = document.getElementById('workers-list');
      if (!el) return;
      if (!workers || workers.length === 0) {
        el.innerHTML = '<div class="muted">No workers found</div>';
        return;
      }
      el.innerHTML = `
        <table class="admin-table">
          <thead>
            <tr>
              <th>ID</th>
              <th>Status</th>
              <th>Metrics</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            ${workers.map(w => `
              <tr>
                <td>${w.id || w.worker_id || 'unknown'}</td>
                <td><span class="status-badge ${w.is_healthy ? 'active' : 'error'}">${w.is_healthy ? 'Healthy' : 'Unhealthy'}</span></td>
                <td>Requests: ${w.total_requests_processed || 0}</td>
                <td>
                  <button class="btn btn-danger" onclick="deleteWorker('${w.id || w.worker_id}')">Delete</button>
                </td>
              </tr>
            `).join('')}
          </tbody>
        </table>
      `;
    }
    
    async function deleteWorker(id) {
      if (!confirm('Delete worker ' + id + '?')) return;
      try {
        await fetchJson(`/api/v1/workers/${id}`, { method: 'DELETE' });
        showNotification('Worker deleted', 'success');
        loadWorkers();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      }
    }
    
    function showCreateWorkerModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      showModal('createWorkerModal');
    }
    
    async function handleCreateWorker(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = 'Creating...';
      
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
        
        // Remove undefined fields
        Object.keys(payload).forEach(key => {
          if (payload[key] === undefined) {
            delete payload[key];
          }
        });
        
        const result = await fetchJson('/api/v1/workers', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification('Worker created successfully', 'success');
        hideModal('createWorkerModal');
        form.reset();
        
        setTimeout(() => {
          loadWorkers();
        }, 500);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    loadWorkers();
    setInterval(loadWorkers, 5000);
    "#;

    admin_layout(
        "Worker Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2>Workers</h2>
            <button class="btn btn-primary" onclick="showCreateWorkerModal()" aria-label="Create new worker">Create Worker</button>
          </div>
          <div id="workers-list"></div>
        </div>
        
        <!-- Create Worker Modal -->
        <div id="createWorkerModal" class="modal" role="dialog" aria-labelledby="createWorkerModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createWorkerModalTitle">Create Worker</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('createWorkerModal')">&times;</button>
            </div>
            <form id="createWorkerForm" onsubmit="handleCreateWorker(event)">
              <div class="form-group">
                <label for="workerId">Worker ID <span class="required">*</span></label>
                <input type="text" id="workerId" name="worker_id" required placeholder="worker-001" pattern="[a-zA-Z0-9_-]+" />
                <small class="form-hint">Alphanumeric, hyphens, and underscores only</small>
              </div>
              <div class="form-group">
                <label for="workerMaxConcurrent">Max Concurrent Requests</label>
                <input type="number" id="workerMaxConcurrent" name="max_concurrent_requests" min="1" max="1000" value="10" />
              </div>
              <div class="form-group">
                <label for="workerTimeout">Request Timeout (ms)</label>
                <input type="number" id="workerTimeout" name="request_timeout_ms" min="100" max="300000" value="5000" />
              </div>
              <div class="form-group">
                <label for="workerHealthCheck">Health Check Interval (ms)</label>
                <input type="number" id="workerHealthCheck" name="health_check_interval_ms" min="100" max="60000" value="1000" />
              </div>
              <div class="form-group">
                <label for="workerEnableCaching">
                  <input type="checkbox" id="workerEnableCaching" name="enable_caching" checked />
                  Enable Caching
                </label>
              </div>
              <div class="form-group">
                <label for="workerCacheSize">Cache Size</label>
                <input type="number" id="workerCacheSize" name="cache_size" min="0" max="100000" value="1000" />
              </div>
              <div class="form-group">
                <label for="workerMaxMemory">Max Memory (MB)</label>
                <input type="number" id="workerMaxMemory" name="max_memory_mb" min="128" max="131072" value="2048" />
              </div>
              <div class="form-group">
                <label for="workerCpuPriority">CPU Priority (0-10)</label>
                <input type="number" id="workerCpuPriority" name="cpu_priority" min="0" max="10" value="5" />
              </div>
              <div class="form-group">
                <label for="workerGpuDevice">GPU Device ID (optional)</label>
                <input type="number" id="workerGpuDevice" name="gpu_device" min="0" />
              </div>
              <div class="form-group">
                <label for="workerAutoRestart">
                  <input type="checkbox" id="workerAutoRestart" name="auto_restart" checked />
                  Auto Restart
                </label>
              </div>
              <div class="form-group">
                <label for="workerResourceMonitoring">
                  <input type="checkbox" id="workerResourceMonitoring" name="resource_monitoring" checked />
                  Resource Monitoring
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createWorkerModal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Create</button>
              </div>
            </form>
          </div>
        </div>
        "#,
        script,
    )
}
