//! VM Management page
//!
//! Provides VM instance lifecycle management.

use crate::ui::admin::admin_layout;
use axum::response::Html;

/// VM management page
pub async fn admin_vm() -> Html<String> {
    let script = r#"
    async function loadVmInstances() {
      try {
        const instances = await fetchJson('/api/v1/vm/instances');
        renderVmInstances(instances);
      } catch (e) {
        showNotification('Error loading VM instances: ' + e.message, 'error');
      }
    }
    
    function renderVmInstances(instances) {
      const el = document.getElementById('vm-instances');
      if (!el) return;
      if (!instances || instances.length === 0) {
        el.innerHTML = '<div class="muted">No VM instances found</div>';
        return;
      }
      el.innerHTML = `
        <table class="admin-table">
          <thead>
            <tr>
              <th>Name</th>
              <th>Status</th>
              <th>Resources</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            ${instances.map(i => `
              <tr>
                <td>${i.name}</td>
                <td><span class="status-badge ${i.status.toLowerCase()}">${i.status}</span></td>
                <td>CPU: ${i.resources.cpu_cores}, Memory: ${i.resources.memory_mb}MB</td>
                <td>
                  <button class="btn" onclick="vmAction('${i.id}', 'start')">Start</button>
                  <button class="btn" onclick="vmAction('${i.id}', 'stop')">Stop</button>
                  <button class="btn btn-danger" onclick="vmAction('${i.id}', 'delete')">Delete</button>
                </td>
              </tr>
            `).join('')}
          </tbody>
        </table>
      `;
    }
    
    async function vmAction(id, action) {
      try {
        await fetchJson(`/api/v1/vm/instances/${id}/${action}`, { method: 'POST' });
        showNotification(`VM ${action} successful`, 'success');
        loadVmInstances();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      }
    }
    
    function showCreateVmModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      showModal('createVmModal');
    }
    
    async function handleCreateVm(event) {
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
          name: document.getElementById('vmName').value,
          resources: {
            cpu_cores: parseInt(document.getElementById('vmCpuCores').value, 10),
            memory_mb: parseInt(document.getElementById('vmMemoryMb').value, 10),
            gpu_required: document.getElementById('vmGpuRequired').checked
          },
          isolation: document.getElementById('vmIsolation').value
        };
        
        const result = await fetchJson('/api/v1/vm/instances', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification('VM instance created successfully', 'success');
        hideModal('createVmModal');
        form.reset();
        
        setTimeout(() => {
          loadVmInstances();
        }, 500);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    loadVmInstances();
    "#;

    admin_layout(
        "VM Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2>VM Instances</h2>
            <button class="btn btn-primary" onclick="showCreateVmModal()" aria-label="Create new VM instance">Create VM Instance</button>
          </div>
          <div id="vm-instances"></div>
        </div>
        
        <!-- Create VM Modal -->
        <div id="createVmModal" class="modal" role="dialog" aria-labelledby="createVmModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createVmModalTitle">Create VM Instance</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('createVmModal')">&times;</button>
            </div>
            <form id="createVmForm" onsubmit="handleCreateVm(event)">
              <div class="form-group">
                <label for="vmName">Instance Name <span class="required">*</span></label>
                <input type="text" id="vmName" name="name" required placeholder="my-vm-instance" />
              </div>
              <div class="form-group">
                <label for="vmCpuCores">CPU Cores <span class="required">*</span></label>
                <input type="number" id="vmCpuCores" name="cpu_cores" required min="1" max="64" value="2" />
              </div>
              <div class="form-group">
                <label for="vmMemoryMb">Memory (MB) <span class="required">*</span></label>
                <input type="number" id="vmMemoryMb" name="memory_mb" required min="256" max="131072" value="2048" />
              </div>
              <div class="form-group">
                <label for="vmGpuRequired">
                  <input type="checkbox" id="vmGpuRequired" name="gpu_required" />
                  GPU Required
                </label>
              </div>
              <div class="form-group">
                <label for="vmIsolation">Isolation Type <span class="required">*</span></label>
                <select id="vmIsolation" name="isolation" required>
                  <option value="ProcessSandbox">Process Sandbox</option>
                  <option value="HardwareVm">Hardware VM</option>
                </select>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createVmModal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Create</button>
              </div>
            </form>
          </div>
        </div>
        "#,
        script,
    )
}
