//! Tenant Management page
//!
//! Provides tenant CRUD operations with resource quota management.

use crate::ui::admin::admin_layout;
use axum::response::Html;

/// Tenant management page
pub async fn admin_tenants() -> Html<String> {
    let script = r#"
    async function loadTenants() {
      adminShowLoading('tenants-list', 'Loading tenants…');
      try {
        const tenants = await fetchJson('/api/enterprise/tenants');
        renderTenants(tenants);
      } catch (e) {
        adminShowInlineError('tenants-list', e);
        showNotification('Error loading tenants: ' + e.message, 'error');
      }
    }
    
    function renderTenants(tenants) {
      const el = document.getElementById('tenants-list');
      if (!el) return;
      if (!tenants || tenants.length === 0) {
        el.innerHTML = '<div class="muted">No tenants found</div>';
        return;
      }
      el.innerHTML = `
        <table class="admin-table">
          <thead>
            <tr>
              <th>Name</th>
              <th>ID</th>
              <th>Status</th>
              <th>Resources</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            ${tenants.map(t => `
              <tr>
                <td>${t.name}</td>
                <td><code>${t.id}</code></td>
                <td><span class="status-badge ${t.config.active ? 'active' : 'inactive'}">${t.config.active ? 'Active' : 'Inactive'}</span></td>
                <td>Workers: ${t.usage.workers}/${t.config.max_workers || '∞'}, Memory: ${t.usage.memory_mb}MB/${t.config.max_memory_mb || '∞'}MB</td>
                <td>
                  <button class="btn" onclick="editTenant('${t.id}')">Edit</button>
                  <button class="btn btn-danger" onclick="deleteTenant('${t.id}')">Delete</button>
                </td>
              </tr>
            `).join('')}
          </tbody>
        </table>
      `;
    }
    
    function showCreateTenantModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      showModal('createTenantModal');
    }
    
    async function handleCreateTenant(event) {
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
          name: document.getElementById('tenantName').value,
          config: {
            max_workers: document.getElementById('tenantMaxWorkers').value ? parseInt(document.getElementById('tenantMaxWorkers').value, 10) : null,
            max_memory_mb: document.getElementById('tenantMaxMemory').value ? parseInt(document.getElementById('tenantMaxMemory').value, 10) : null,
            max_cpu_cores: document.getElementById('tenantMaxCpuCores').value ? parseInt(document.getElementById('tenantMaxCpuCores').value, 10) : null,
            max_storage_mb: document.getElementById('tenantMaxStorage').value ? parseInt(document.getElementById('tenantMaxStorage').value, 10) : null,
            max_vm_instances: document.getElementById('tenantMaxVmInstances').value ? parseInt(document.getElementById('tenantMaxVmInstances').value, 10) : null,
            active: document.getElementById('tenantActive').checked
          }
        };
        
        // Remove null fields
        Object.keys(payload.config).forEach(key => {
          if (payload.config[key] === null) {
            delete payload.config[key];
          }
        });
        
        const result = await fetchJson('/api/enterprise/tenants', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification('Tenant created successfully', 'success');
        hideModal('createTenantModal');
        form.reset();
        
        setTimeout(() => {
          loadTenants();
        }, 500);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function editTenant(id) {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      
      try {
        const tenant = await fetchJson(`/api/enterprise/tenants/${id}`);
        
        // Populate edit form
        document.getElementById('editTenantId').value = tenant.id;
        document.getElementById('editTenantName').value = tenant.name;
        document.getElementById('editTenantMaxWorkers').value = tenant.config.max_workers || '';
        document.getElementById('editTenantMaxMemory').value = tenant.config.max_memory_mb || '';
        document.getElementById('editTenantMaxCpuCores').value = tenant.config.max_cpu_cores || '';
        document.getElementById('editTenantMaxStorage').value = tenant.config.max_storage_mb || '';
        document.getElementById('editTenantMaxVmInstances').value = tenant.config.max_vm_instances || '';
        document.getElementById('editTenantActive').checked = tenant.config.active;
        
        showModal('editTenantModal');
      } catch (e) {
        showNotification('Error loading tenant: ' + e.message, 'error');
      }
    }
    
    async function handleEditTenant(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      const tenantId = document.getElementById('editTenantId').value;
      
      btn.disabled = true;
      btn.textContent = 'Updating...';
      
      try {
        const payload = {
          name: document.getElementById('editTenantName').value,
          config: {
            max_workers: document.getElementById('editTenantMaxWorkers').value ? parseInt(document.getElementById('editTenantMaxWorkers').value, 10) : null,
            max_memory_mb: document.getElementById('editTenantMaxMemory').value ? parseInt(document.getElementById('editTenantMaxMemory').value, 10) : null,
            max_cpu_cores: document.getElementById('editTenantMaxCpuCores').value ? parseInt(document.getElementById('editTenantMaxCpuCores').value, 10) : null,
            max_storage_mb: document.getElementById('editTenantMaxStorage').value ? parseInt(document.getElementById('editTenantMaxStorage').value, 10) : null,
            max_vm_instances: document.getElementById('editTenantMaxVmInstances').value ? parseInt(document.getElementById('editTenantMaxVmInstances').value, 10) : null,
            active: document.getElementById('editTenantActive').checked
          }
        };
        
        // Remove null fields
        Object.keys(payload.config).forEach(key => {
          if (payload.config[key] === null) {
            delete payload.config[key];
          }
        });
        
        const result = await fetchJson(`/api/enterprise/tenants/${tenantId}`, {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification('Tenant updated successfully', 'success');
        hideModal('editTenantModal');
        form.reset();
        
        setTimeout(() => {
          loadTenants();
        }, 500);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function deleteTenant(id) {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      
      if (!confirm('Are you sure you want to delete this tenant? This action cannot be undone.')) {
        return;
      }
      
      try {
        await fetchJson(`/api/enterprise/tenants/${id}`, { method: 'DELETE' });
        showNotification('Tenant deleted successfully', 'success');
        loadTenants();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      }
    }
    
    loadTenants();
    "#;

    admin_layout(
        "Tenant Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2>Tenants</h2>
            <button class="btn btn-primary" onclick="showCreateTenantModal()" aria-label="Create new tenant">Create Tenant</button>
          </div>
          <div id="tenants-list"></div>
        </div>
        
        <!-- Create Tenant Modal -->
        <div id="createTenantModal" class="modal" role="dialog" aria-labelledby="createTenantModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createTenantModalTitle">Create Tenant</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('createTenantModal')">&times;</button>
            </div>
            <form id="createTenantForm" onsubmit="handleCreateTenant(event)">
              <div class="form-group">
                <label for="tenantName">Tenant Name <span class="required">*</span></label>
                <input type="text" id="tenantName" name="name" required placeholder="tenant-abc" />
              </div>
              <div class="form-group">
                <label for="tenantMaxWorkers">Max Workers</label>
                <input type="number" id="tenantMaxWorkers" name="max_workers" min="0" placeholder="10" />
                <small class="form-hint">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="tenantMaxMemory">Max Memory (MB)</label>
                <input type="number" id="tenantMaxMemory" name="max_memory_mb" min="0" placeholder="1024" />
                <small class="form-hint">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="tenantMaxCpuCores">Max CPU Cores</label>
                <input type="number" id="tenantMaxCpuCores" name="max_cpu_cores" min="0" placeholder="4" />
                <small class="form-hint">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="tenantMaxStorage">Max Storage (MB)</label>
                <input type="number" id="tenantMaxStorage" name="max_storage_mb" min="0" placeholder="10000" />
                <small class="form-hint">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="tenantMaxVmInstances">Max VM Instances</label>
                <input type="number" id="tenantMaxVmInstances" name="max_vm_instances" min="0" placeholder="5" />
                <small class="form-hint">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="tenantActive">
                  <input type="checkbox" id="tenantActive" name="active" checked />
                  Active
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createTenantModal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Create</button>
              </div>
            </form>
          </div>
        </div>
        
        <!-- Edit Tenant Modal -->
        <div id="editTenantModal" class="modal" role="dialog" aria-labelledby="editTenantModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="editTenantModalTitle">Edit Tenant</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('editTenantModal')">&times;</button>
            </div>
            <form id="editTenantForm" onsubmit="handleEditTenant(event)">
              <input type="hidden" id="editTenantId" />
              <div class="form-group">
                <label for="editTenantName">Tenant Name <span class="required">*</span></label>
                <input type="text" id="editTenantName" name="name" required />
              </div>
              <div class="form-group">
                <label for="editTenantMaxWorkers">Max Workers</label>
                <input type="number" id="editTenantMaxWorkers" name="max_workers" min="0" />
                <small class="form-hint">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="editTenantMaxMemory">Max Memory (MB)</label>
                <input type="number" id="editTenantMaxMemory" name="max_memory_mb" min="0" />
                <small class="form-hint">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="editTenantMaxCpuCores">Max CPU Cores</label>
                <input type="number" id="editTenantMaxCpuCores" name="max_cpu_cores" min="0" />
                <small class="form-hint">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="editTenantMaxStorage">Max Storage (MB)</label>
                <input type="number" id="editTenantMaxStorage" name="max_storage_mb" min="0" />
                <small class="form-hint">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="editTenantMaxVmInstances">Max VM Instances</label>
                <input type="number" id="editTenantMaxVmInstances" name="max_vm_instances" min="0" />
                <small class="form-hint">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="editTenantActive">
                  <input type="checkbox" id="editTenantActive" name="active" />
                  Active
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('editTenantModal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Update</button>
              </div>
            </form>
          </div>
        </div>
        "#,
        script,
    )
}
