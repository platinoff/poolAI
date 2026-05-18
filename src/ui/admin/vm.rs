//! VM Management page
//!
//! Provides VM instance lifecycle management.

use crate::ui::admin::admin_layout;
use axum::response::Html;

/// VM management page
pub async fn admin_vm() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }
    function Ep() { return typeof poolaiT === 'function' ? poolaiT('err.errorPrefix', 'Error: ') : 'Error: '; }

    async function loadVmInstances() {
      adminShowLoading('vm-instances', T('admin.vmadm.loading', 'Loading VM instances…'));
      try {
        const instances = await fetchJson('/api/v1/vm/instances');
        renderVmInstances(instances);
      } catch (e) {
        adminShowInlineError('vm-instances', e);
        showNotification(T('admin.vmadm.errLoad', 'Error loading VM instances: ') + e.message, 'error');
      }
    }
    
    function vmStatusBadge(status) {
      const s = (typeof status === 'string') ? status : JSON.stringify(status ?? '');
      const low = s.toLowerCase();
      const cls = low.startsWith('failed') ? 'error' : (low === 'stopped' ? 'inactive' : 'active');
      return `<span class="status-badge ${cls}">${s}</span>`;
    }
    
    function renderVmInstances(instances) {
      const el = document.getElementById('vm-instances');
      if (!el) return;
      if (!instances || instances.length === 0) {
        el.innerHTML = '<div class="muted">' + escapeHtml(T('admin.vmadm.empty', 'No VM instances found')) + '</div>';
        return;
      }
      el.innerHTML = `
        <table class="admin-table">
          <thead>
            <tr>
              <th>${escapeHtml(T('admin.vmadm.col.name', 'Name'))}</th>
              <th>${escapeHtml(T('admin.vmadm.col.status', 'Status'))}</th>
              <th>${escapeHtml(T('admin.vmadm.col.resources', 'Resources'))}</th>
              <th>${escapeHtml(T('admin.vmadm.col.actions', 'Actions'))}</th>
            </tr>
          </thead>
          <tbody>
            ${instances.map(i => `
              <tr>
                <td>${i.name}</td>
                <td>${vmStatusBadge(i.status)}</td>
                <td>${escapeHtml(T('admin.vmadm.resCpu', 'CPU:'))} ${i.resources ? i.resources.cpu_cores : '—'}, ${escapeHtml(T('admin.vmadm.resMem', 'Memory:'))} ${i.resources ? i.resources.memory_mb : '—'}MB</td>
                <td>
                  <button type="button" class="btn" onclick="vmAction('${i.id}', 'start')">${escapeHtml(T('vm.start', 'Start'))}</button>
                  <button type="button" class="btn" onclick="vmAction('${i.id}', 'stop')">${escapeHtml(T('vm.stop', 'Stop'))}</button>
                  <button type="button" class="btn btn-danger" onclick="vmAction('${i.id}', 'delete')">${escapeHtml(T('ui.delete', 'Delete'))}</button>
                </td>
              </tr>
            `).join('')}
          </tbody>
        </table>
      `;
    }
    
    async function vmAction(id, action) {
      try {
        if (action === 'delete') {
          await fetchJson(`/api/v1/vm/instances/${id}`, { method: 'DELETE' });
        } else {
          await fetchJson(`/api/v1/vm/instances/${id}/${action}`, { method: 'POST' });
        }
        const msg = T('admin.vmadm.actionOk', 'VM {action} successful').replace(/\{action\}/g, action);
        showNotification(msg, 'success');
        loadVmInstances();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      }
    }
    
    function showCreateVmModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissionsAdminOp', 'Insufficient permissions. Admin or Operator role required.'), 'error');
        return;
      }
      showModal('createVmModal');
    }
    
    async function handleCreateVm(event) {
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
      btn.textContent = T('admin.vmadm.creating', 'Creating…');
      
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
        
        await fetchJson('/api/v1/vm/instances', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification(T('vm.createdOk', 'VM instance created successfully'), 'success');
        hideModal('createVmModal');
        form.reset();
        
        setTimeout(() => {
          loadVmInstances();
        }, 500);
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    loadVmInstances();
    "#;

    admin_layout(
        "admin.page.vm",
        "VM Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2 data-i18n="admin.vmadm.section">VM Instances</h2>
            <button type="button" class="btn btn-primary" onclick="showCreateVmModal()" data-i18n="admin.vmadm.createBtn" data-i18n-aria="vm.createBtnAria">Create VM Instance</button>
          </div>
          <div id="vm-instances"></div>
        </div>
        
        <div id="createVmModal" class="modal" role="dialog" aria-labelledby="createVmModalTitle" aria-modal="false" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createVmModalTitle" data-i18n="vm.modalTitle">Create VM Instance</h3>
              <button type="button" class="modal-close" data-i18n-aria="ui.closeDialogAria" onclick="hideModal('createVmModal')">&times;</button>
            </div>
            <form id="createVmForm" onsubmit="handleCreateVm(event)">
              <div class="form-group">
                <label for="vmName"><span data-i18n="vm.label.name">Instance Name</span> <span class="required">*</span></label>
                <input type="text" id="vmName" name="name" required data-i18n-placeholder="vm.ph.name" placeholder="my-vm-instance" />
              </div>
              <div class="form-group">
                <label for="vmCpuCores"><span data-i18n="vm.label.cpu">CPU Cores</span> <span class="required">*</span></label>
                <input type="number" id="vmCpuCores" name="cpu_cores" required min="1" max="64" value="2" />
              </div>
              <div class="form-group">
                <label for="vmMemoryMb"><span data-i18n="vm.label.memory">Memory (MB)</span> <span class="required">*</span></label>
                <input type="number" id="vmMemoryMb" name="memory_mb" required min="256" max="131072" value="2048" />
              </div>
              <div class="form-group">
                <label for="vmGpuRequired">
                  <input type="checkbox" id="vmGpuRequired" name="gpu_required" />
                  <span data-i18n="vm.label.gpu">GPU Required</span>
                </label>
              </div>
              <div class="form-group">
                <label for="vmIsolation"><span data-i18n="vm.label.isolation">Isolation Type</span> <span class="required">*</span></label>
                <select id="vmIsolation" name="isolation" required>
                  <option value="ProcessSandbox" data-i18n="vm.iso.process">Process sandbox</option>
                  <option value="HardwareVm" data-i18n="vm.iso.hardware">Hardware VM</option>
                </select>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createVmModal')" data-i18n="ui.cancel">Cancel</button>
                <button type="submit" class="btn btn-primary" data-i18n="ui.create">Create</button>
              </div>
            </form>
          </div>
        </div>
        "#,
        script,
    )
}
