//! VM Management page
//!
//! Provides VM instance lifecycle management.

use super::admin_layout_vm;
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
    
    function renderVmInstances(instances) {
      const el = document.getElementById('vm-instances');
      if (!el) return;
      el.innerHTML = poolaiRenderVmPanel(instances, {
        name: T('admin.vmadm.col.name', 'Name'),
        status: T('admin.vmadm.col.status', 'Status'),
        resources: T('admin.vmadm.col.resources', 'Resources'),
        actions: T('admin.vmadm.col.actions', 'Actions'),
        tableAria: T('admin.vmadm.section', 'VM Instances'),
        resCpu: T('admin.vmadm.resCpu', 'CPU:'),
        resMem: T('admin.vmadm.resMem', 'Memory:'),
        start: T('vm.start', 'Start'),
        stop: T('vm.stop', 'Stop'),
        delete: T('ui.delete', 'Delete'),
        empty: T('admin.vmadm.empty', 'No VM instances found'),
      });
      el.querySelectorAll('[data-vm-id]').forEach(function(btn) {
        btn.addEventListener('click', function() {
          vmAction(btn.getAttribute('data-vm-id'), btn.getAttribute('data-vm-action'));
        });
      });
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
        await loadVmInstances();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    loadVmInstances();

    // admin_layout wraps page scripts in an IIFE; inline onclick/onsubmit need globals.
    const _g = typeof globalThis !== 'undefined' ? globalThis : window;
    _g.showCreateVmModal = showCreateVmModal;
    _g.handleCreateVm = handleCreateVm;
    _g.loadVmInstances = loadVmInstances;
    _g.vmAction = vmAction;
    "#;

    admin_layout_vm(
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

#[cfg(test)]
mod ph_s237_tests {
    use super::admin_vm;

    #[tokio::test]
    async fn admin_vm_page_slim_vm_i18n_patch_ph_s237() {
        let html = admin_vm().await.0;
        assert!(html.contains("window.__poolaiAdminI18nRust="));
        assert!(html.contains(r#""admin.page.vm""#));
        assert!(html.contains(r#""admin.vmadm.section""#));
        assert!(!html.contains(r#""admin.inst.title""#));
        assert!(!html.contains(r#""admin.jobs.leaseState.active""#));
    }
}
