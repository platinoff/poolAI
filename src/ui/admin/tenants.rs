//! Tenant Management page
//!
//! Provides tenant CRUD operations with resource quota management.
//! PH-S1180/S1181: store-wire status strip + usage/quota ops glue (band 54).

use crate::ui::admin::admin_layout_tenants;
use axum::response::Html;

/// Tenant management page
pub async fn admin_tenants() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }
    function Ep() { return typeof poolaiT === 'function' ? poolaiT('err.errorPrefix', 'Error: ') : 'Error: '; }

    function renderTenantStoreBadge(wire) {
      const el = document.getElementById('tenant-store-badge');
      if (!el) return;
      const mode = String((wire && wire.mode) || 'memory').toLowerCase();
      const configured = !!(wire && wire.configured);
      const path = (wire && wire.durable_path) ? String(wire.durable_path) : '';
      const storeLabel = T('admin.tenants.storeLabel', 'Store:');
      const storeHint = T('admin.tenants.storeHint', 'Tenant persistence backend (POOLAI_TENANT_STORE / POOLAI_TENANT_DATA_DIR)');
      const modeLabel = T('admin.tenants.store.' + mode, mode);
      const cfg = configured
        ? T('admin.tenants.store.configured', 'configured')
        : T('admin.tenants.store.unconfigured', 'unconfigured');
      const pathBit = path ? (' · ' + escapeHtml(path)) : '';
      el.innerHTML =
        '<span class="status-badge ' + (configured ? 'active' : 'inactive') + '" title="' + escapeHtml(storeHint) + '">' +
        escapeHtml(storeLabel) + ' ' + escapeHtml(modeLabel) + ' (' + escapeHtml(cfg) + ')' + pathBit +
        '</span>';
    }

    async function loadTenantStoreWire() {
      const el = document.getElementById('tenant-store-badge');
      if (el) {
        el.textContent = T('admin.tenants.storeLoading', 'Loading store…');
      }
      try {
        const wire = await fetchJson('/api/enterprise/tenants/store');
        renderTenantStoreBadge(wire);
      } catch (e) {
        if (el) {
          el.innerHTML = '<span class="status-badge inactive">' +
            escapeHtml(T('admin.tenants.storeErr', 'Store wire unavailable')) + '</span>';
        }
      }
    }

    async function refreshTenantUsage(id) {
      try {
        const usage = await fetchJson('/api/enterprise/tenants/' + encodeURIComponent(id) + '/usage');
        const msg = T('admin.tenants.usageOk', 'Usage:') +
          ' workers=' + (usage.workers || 0) +
          ', memory_mb=' + (usage.memory_mb || 0) +
          ', cpu_cores=' + (usage.cpu_cores || 0);
        showNotification(msg, 'success');
      } catch (e) {
        showNotification(T('admin.tenants.usageErr', 'Usage refresh failed: ') + e.message, 'error');
      }
    }

    async function probeTenantQuota(id) {
      try {
        const result = await fetchJson('/api/enterprise/tenants/' + encodeURIComponent(id) + '/quota', {
          method: 'POST',
          body: JSON.stringify({ workers: 1, memory_mb: 1 })
        });
        const allowed = !!(result && result.allowed);
        const reason = (result && result.reason) ? String(result.reason) : '';
        const msg = allowed
          ? T('admin.tenants.quotaAllow', 'Quota probe: allow')
          : (T('admin.tenants.quotaDeny', 'Quota probe: deny') + (reason ? (' — ' + reason) : ''));
        showNotification(msg, allowed ? 'success' : 'warning');
      } catch (e) {
        showNotification(T('admin.tenants.quotaErr', 'Quota probe failed: ') + e.message, 'error');
      }
    }

    async function loadTenants() {
      adminShowLoading('tenants-list', T('admin.tenants.loading', 'Loading tenants…'));
      try {
        const tenants = await fetchJson('/api/enterprise/tenants');
        renderTenants(tenants);
      } catch (e) {
        adminShowInlineError('tenants-list', e);
        showNotification(T('admin.tenants.errLoad', 'Error loading tenants: ') + e.message, 'error');
      }
    }
    
    function renderTenants(tenants) {
      const el = document.getElementById('tenants-list');
      if (!el) return;
      if (!tenants || tenants.length === 0) {
        el.innerHTML = adminEmptyStateHtml(T('admin.tenants.empty', 'No tenants found'));
        return;
      }
      el.innerHTML = `
        <div class="admin-table-container"><table class="admin-table" aria-label="${escapeHtml(T('admin.tenants.section', 'Tenants'))}">
          <thead>
            <tr>
              <th scope="col">${escapeHtml(T('admin.tenants.col.name', 'Name'))}</th>
              <th scope="col">${escapeHtml(T('admin.tenants.col.id', 'ID'))}</th>
              <th scope="col">${escapeHtml(T('admin.tenants.col.status', 'Status'))}</th>
              <th scope="col">${escapeHtml(T('admin.tenants.col.resources', 'Resources'))}</th>
              <th scope="col" class="admin-table-actions-col" data-no-sort="1">${escapeHtml(T('admin.tenants.col.actions', 'Actions'))}</th>
            </tr>
          </thead>
          <tbody>
            ${tenants.map(t => `
              <tr>
                <td>${escapeHtml(t.name)}</td>
                <td><code>${escapeHtml(t.id)}</code></td>
                <td><span class="status-badge ${t.config.active ? 'active' : 'inactive'}">${t.config.active ? escapeHtml(T('admin.status.active', 'Active')) : escapeHtml(T('admin.status.inactive', 'Inactive'))}</span></td>
                <td>${escapeHtml(T('admin.tenants.resWorkers', 'Workers:'))} ${t.usage.workers}/${t.config.max_workers || '∞'}, ${escapeHtml(T('admin.tenants.resMemory', 'Memory:'))} ${t.usage.memory_mb}MB/${t.config.max_memory_mb || '∞'}MB</td>
                <td>
                  <button type="button" class="btn" onclick='editTenant(${JSON.stringify(t.id)})'>${escapeHtml(T('admin.btn.edit', 'Edit'))}</button>
                  <button type="button" class="btn" onclick='refreshTenantUsage(${JSON.stringify(t.id)})'>${escapeHtml(T('admin.tenants.btn.usage', 'Usage'))}</button>
                  <button type="button" class="btn" onclick='probeTenantQuota(${JSON.stringify(t.id)})'>${escapeHtml(T('admin.tenants.btn.quota', 'Quota'))}</button>
                  <button type="button" class="btn btn-danger" onclick='deleteTenant(${JSON.stringify(t.id)})'>${escapeHtml(T('ui.delete', 'Delete'))}</button>
                </td>
              </tr>
            `).join('')}
          </tbody>
        </table></div>
      `;
      if (typeof adminInitTablesIn === 'function') adminInitTablesIn(el);
    }
    
    function showCreateTenantModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissionsAdminOp', 'Insufficient permissions. Admin or Operator role required.'), 'error');
        return;
      }
      showModal('createTenantModal');
    }
    
    async function handleCreateTenant(event) {
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
      btn.textContent = T('admin.tenants.creating', 'Creating…');
      
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
        
        Object.keys(payload.config).forEach(key => {
          if (payload.config[key] === null) {
            delete payload.config[key];
          }
        });
        
        await fetchJson('/api/enterprise/tenants', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification(T('admin.tenants.createdOk', 'Tenant created successfully'), 'success');
        hideModal('createTenantModal');
        form.reset();
        
        setTimeout(() => {
          loadTenants();
        }, 500);
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function editTenant(id) {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissionsAdminOp', 'Insufficient permissions. Admin or Operator role required.'), 'error');
        return;
      }
      
      try {
        const tenant = await fetchJson(`/api/enterprise/tenants/${id}`);
        
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
        showNotification(T('admin.tenants.loadErr', 'Error loading tenant: ') + e.message, 'error');
      }
    }
    
    async function handleEditTenant(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      const tenantId = document.getElementById('editTenantId').value;
      
      btn.disabled = true;
      btn.textContent = T('admin.tenants.updating', 'Updating…');
      
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
        
        Object.keys(payload.config).forEach(key => {
          if (payload.config[key] === null) {
            delete payload.config[key];
          }
        });
        
        await fetchJson(`/api/enterprise/tenants/${tenantId}`, {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification(T('admin.tenants.updatedOk', 'Tenant updated successfully'), 'success');
        hideModal('editTenantModal');
        form.reset();
        
        setTimeout(() => {
          loadTenants();
        }, 500);
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function deleteTenant(id) {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissionsAdminOp', 'Insufficient permissions. Admin or Operator role required.'), 'error');
        return;
      }
      
      if (!confirm(T('admin.tenants.confirmDel', 'Are you sure you want to delete this tenant? This action cannot be undone.'))) {
        return;
      }
      
      try {
        await fetchJson(`/api/enterprise/tenants/${id}`, { method: 'DELETE' });
        showNotification(T('admin.tenants.deletedOk', 'Tenant deleted successfully'), 'success');
        loadTenants();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      }
    }
    
    loadTenantStoreWire();
    loadTenants();
    "#;

    admin_layout_tenants(
        "admin.page.tenants",
        "Tenant Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2 data-i18n="admin.tenants.section">Tenants</h2>
            <span id="tenant-store-badge" class="tenant-store-badge muted" data-i18n="admin.tenants.storeLoading">Loading store…</span>
            <button class="btn btn-primary" onclick="showCreateTenantModal()" data-i18n="admin.tenants.createBtn" data-i18n-aria="admin.tenants.createBtn">Create Tenant</button>
          </div>
          <div id="tenants-list"></div>
        </div>
        
        <div id="createTenantModal" class="modal" role="dialog" aria-labelledby="createTenantModalTitle" aria-modal="false" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createTenantModalTitle" data-i18n="admin.tenants.createTitle">Create Tenant</h3>
              <button class="modal-close" data-i18n-aria="ui.closeDialogAria" onclick="hideModal('createTenantModal')">&times;</button>
            </div>
            <form id="createTenantForm" onsubmit="handleCreateTenant(event)">
              <div class="form-group">
                <label for="tenantName"><span data-i18n="admin.tenants.label.name">Tenant Name</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="text" id="tenantName" name="name" required aria-required="true" data-i18n-placeholder="admin.tenants.ph.name" placeholder="tenant-abc" />
              </div>
              <div class="form-group">
                <label for="tenantMaxWorkers" data-i18n="admin.tenants.label.maxWorkers">Max Workers</label>
                <input type="number" id="tenantMaxWorkers" name="max_workers" min="0" placeholder="10" />
                <small class="form-hint" data-i18n="admin.tenants.hint.unlimited">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="tenantMaxMemory" data-i18n="admin.tenants.label.maxMem">Max Memory (MB)</label>
                <input type="number" id="tenantMaxMemory" name="max_memory_mb" min="0" placeholder="1024" />
                <small class="form-hint" data-i18n="admin.tenants.hint.unlimited">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="tenantMaxCpuCores" data-i18n="admin.tenants.label.maxCpu">Max CPU Cores</label>
                <input type="number" id="tenantMaxCpuCores" name="max_cpu_cores" min="0" placeholder="4" />
                <small class="form-hint" data-i18n="admin.tenants.hint.unlimited">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="tenantMaxStorage" data-i18n="admin.tenants.label.maxStorage">Max Storage (MB)</label>
                <input type="number" id="tenantMaxStorage" name="max_storage_mb" min="0" placeholder="10000" />
                <small class="form-hint" data-i18n="admin.tenants.hint.unlimited">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="tenantMaxVmInstances" data-i18n="admin.tenants.label.maxVm">Max VM Instances</label>
                <input type="number" id="tenantMaxVmInstances" name="max_vm_instances" min="0" placeholder="5" />
                <small class="form-hint" data-i18n="admin.tenants.hint.unlimited">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="tenantActive">
                  <input type="checkbox" id="tenantActive" name="active" checked />
                  <span data-i18n="admin.tenants.active">Active</span>
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createTenantModal')" data-i18n="ui.cancel">Cancel</button>
                <button type="submit" class="btn btn-primary" data-i18n="ui.create">Create</button>
              </div>
            </form>
          </div>
        </div>
        
        <div id="editTenantModal" class="modal" role="dialog" aria-labelledby="editTenantModalTitle" aria-modal="false" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="editTenantModalTitle" data-i18n="admin.tenants.editTitle">Edit Tenant</h3>
              <button class="modal-close" data-i18n-aria="ui.closeDialogAria" onclick="hideModal('editTenantModal')">&times;</button>
            </div>
            <form id="editTenantForm" onsubmit="handleEditTenant(event)">
              <input type="hidden" id="editTenantId" />
              <div class="form-group">
                <label for="editTenantName"><span data-i18n="admin.tenants.label.name">Tenant Name</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="text" id="editTenantName" name="name" required aria-required="true" />
              </div>
              <div class="form-group">
                <label for="editTenantMaxWorkers" data-i18n="admin.tenants.label.maxWorkers">Max Workers</label>
                <input type="number" id="editTenantMaxWorkers" name="max_workers" min="0" />
                <small class="form-hint" data-i18n="admin.tenants.hint.unlimited">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="editTenantMaxMemory" data-i18n="admin.tenants.label.maxMem">Max Memory (MB)</label>
                <input type="number" id="editTenantMaxMemory" name="max_memory_mb" min="0" />
                <small class="form-hint" data-i18n="admin.tenants.hint.unlimited">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="editTenantMaxCpuCores" data-i18n="admin.tenants.label.maxCpu">Max CPU Cores</label>
                <input type="number" id="editTenantMaxCpuCores" name="max_cpu_cores" min="0" />
                <small class="form-hint" data-i18n="admin.tenants.hint.unlimited">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="editTenantMaxStorage" data-i18n="admin.tenants.label.maxStorage">Max Storage (MB)</label>
                <input type="number" id="editTenantMaxStorage" name="max_storage_mb" min="0" />
                <small class="form-hint" data-i18n="admin.tenants.hint.unlimited">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="editTenantMaxVmInstances" data-i18n="admin.tenants.label.maxVm">Max VM Instances</label>
                <input type="number" id="editTenantMaxVmInstances" name="max_vm_instances" min="0" />
                <small class="form-hint" data-i18n="admin.tenants.hint.unlimited">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="editTenantActive">
                  <input type="checkbox" id="editTenantActive" name="active" />
                  <span data-i18n="admin.tenants.active">Active</span>
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('editTenantModal')" data-i18n="ui.cancel">Cancel</button>
                <button type="submit" class="btn btn-primary" data-i18n="ui.update">Update</button>
              </div>
            </form>
          </div>
        </div>
        "#,
        script,
    )
}

#[tokio::test]
async fn admin_tenants_page_slim_tenants_i18n_patch_ph_s230() {
    let html = admin_tenants().await.0;
    assert!(html.contains("window.__poolaiAdminI18nRust="));
    assert!(html.contains(r#""admin.page.tenants""#));
    assert!(html.contains(r#""admin.tenants.section""#));
    assert!(!html.contains(r#""admin.jobs.leaseState.active""#));
    assert!(!html.contains(r#""admin.sec.tab.oauth""#));
}

#[tokio::test]
async fn admin_tenants_store_strip_and_quota_glue_ph_s1180() {
    let html = admin_tenants().await.0;
    assert!(html.contains("id=\"tenant-store-badge\""));
    assert!(html.contains("loadTenantStoreWire"));
    assert!(html.contains("/api/enterprise/tenants/store"));
    assert!(html.contains("refreshTenantUsage"));
    assert!(html.contains("probeTenantQuota"));
    assert!(html.contains("/usage"));
    assert!(html.contains("/quota"));
}
