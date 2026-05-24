//! Model instances management admin page

use super::admin_layout;

/// Admin instances page
pub async fn admin_instances() -> axum::response::Html<String> {
    let body = r#"
    <div class="admin-section">
      <h3 data-i18n="admin.inst.title">Model Instances</h3>
      <p data-i18n="admin.inst.intro">Manage AI model instances, placement, and lifecycle.</p>
      
      <div class="admin-section-header">
        <h4 data-i18n="admin.inst.sectionInst">Instances</h4>
        <button type="button" class="btn btn-primary" onclick="showCreateInstanceModal()" data-i18n="admin.inst.createBtn">Create Instance</button>
      </div>
      
      <div id="instances-list" class="admin-table-container">
        <table class="admin-table">
          <thead>
            <tr>
              <th data-i18n="admin.inst.col.instanceId">Instance ID</th>
              <th data-i18n="admin.inst.col.modelId">Model ID</th>
              <th data-i18n="admin.inst.col.status">Status</th>
              <th data-i18n="admin.inst.col.strategy">Strategy</th>
              <th data-i18n="admin.inst.col.nodes">Nodes</th>
              <th data-i18n="admin.inst.col.created">Created</th>
              <th data-i18n="admin.inst.col.actions">Actions</th>
            </tr>
          </thead>
          <tbody id="instances-tbody">
            <tr><td colspan="7" data-i18n="admin.inst.loadingRow">Loading…</td></tr>
          </tbody>
        </table>
      </div>

      <div class="admin-section-header">
        <h4 data-i18n="admin.inst.sectionPreview">Placement Preview</h4>
      </div>
      <div class="form-group">
        <label for="preview-model-id" data-i18n="admin.inst.modelIdLbl">Model ID:</label>
        <input type="text" id="preview-model-id" data-i18n-placeholder="admin.inst.ph.modelId" placeholder="Enter model ID" />
        <button type="button" class="btn btn-primary" onclick="previewPlacement()" data-i18n="admin.inst.previewBtn">Preview Placement</button>
      </div>
      <div id="placement-previews" class="admin-table-container">
        <table class="admin-table">
          <thead>
            <tr>
              <th data-i18n="admin.inst.col.strategy">Strategy</th>
              <th data-i18n="admin.inst.col.nodes">Nodes</th>
              <th data-i18n="admin.inst.col.memDelta">Memory Delta</th>
              <th data-i18n="admin.inst.col.placementErr">Error</th>
            </tr>
          </thead>
          <tbody id="placement-previews-tbody">
            <tr><td colspan="4" data-i18n="admin.inst.previewHint">Enter a model ID and click Preview</td></tr>
          </tbody>
        </table>
      </div>
    </div>
  "#;

    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }
    function Ep() { return typeof poolaiT === 'function' ? poolaiT('err.errorPrefix', 'Error: ') : 'Error: '; }

    async function loadInstances() {
      const tbody = document.getElementById('instances-tbody');
      if (!tbody) return;
      tbody.innerHTML = '<tr><td colspan="7" class="muted">' + escapeHtml(T('admin.inst.loadingRow', 'Loading…')) + '</td></tr>';
      try {
        const token = getAuthToken();
        const response = await fetch('/api/v1/instance', {
          headers: token ? { Authorization: 'Bearer ' + token } : {},
        });
        if (!response.ok) {
          const errBody = await response.json().catch(() => ({}));
          const msg =
            (errBody.error && errBody.error.message) ||
            errBody.message ||
            ('HTTP ' + response.status);
          throw new Error(msg);
        }
        const data = await response.json();
        
        tbody.innerHTML = '';
        
        if (!data.instances || data.instances.length === 0) {
          tbody.innerHTML = '<tr><td colspan="7">' + escapeHtml(T('admin.inst.empty', 'No instances found')) + '</td></tr>';
          return;
        }
        
        for (const instance of data.instances) {
          const row = document.createElement('tr');
          const iid = instance.instance_id;
          row.innerHTML = `
            <td>${escapeHtml(instance.instance_id)}</td>
            <td>${escapeHtml(instance.model_id)}</td>
            <td><span class="badge">${escapeHtml(instance.status)}</span></td>
            <td>${escapeHtml(instance.placement.strategy)}</td>
            <td>${instance.placement.node_ids.join(', ')}</td>
            <td>${escapeHtml(new Date(instance.created_at).toLocaleString())}</td>
            <td>
              <button type="button" class="btn btn-sm" onclick='viewInstance(${JSON.stringify(iid)})'>${escapeHtml(T('admin.inst.viewBtn', 'View'))}</button>
              <button type="button" class="btn btn-sm btn-danger" onclick='deleteInstance(${JSON.stringify(iid)})'>${escapeHtml(T('ui.delete', 'Delete'))}</button>
            </td>
          `;
          tbody.appendChild(row);
        }
      } catch (error) {
        console.error('Error loading instances:', error);
        const tb = document.getElementById('instances-tbody');
        if (tb) {
          const msg = error instanceof Error ? error.message : String(error);
          tb.innerHTML =
            '<tr><td colspan="7"><div class="admin-fetch-error" role="alert">' +
            escapeHtml(msg) +
            '</div></td></tr>';
        }
        showNotification(T('admin.inst.errLoad', 'Error loading instances: ') + (error && error.message ? error.message : error), 'error');
      }
    }

    async function previewPlacement() {
      const modelId = document.getElementById('preview-model-id').value.trim();
      if (!modelId) {
        showNotification(T('admin.inst.needModelId', 'Please enter a model ID'), 'error');
        return;
      }

      try {
        const response = await fetch(`/api/v1/instance/previews?model_id=${encodeURIComponent(modelId)}`);
        if (!response.ok) throw new Error('Failed to get placement previews');
        const data = await response.json();
        
        const tbody = document.getElementById('placement-previews-tbody');
        tbody.innerHTML = '';
        
        if (!data.previews || data.previews.length === 0) {
          tbody.innerHTML = '<tr><td colspan="4">' + escapeHtml(T('admin.inst.previewEmpty', 'No placement options available')) + '</td></tr>';
          return;
        }
        
        for (const preview of data.previews) {
          const row = document.createElement('tr');
          row.innerHTML = `
            <td>${escapeHtml(preview.sharding)}</td>
            <td>${Object.keys(preview.memory_delta_by_node || {}).join(', ') || escapeHtml(T('admin.na', 'N/A'))}</td>
            <td>${Object.values(preview.memory_delta_by_node || {}).reduce((a, b) => a + b, 0)} MB</td>
            <td>${preview.error ? escapeHtml(preview.error) : '-'}</td>
          `;
          tbody.appendChild(row);
        }
      } catch (error) {
        console.error('Error getting placement previews:', error);
        showNotification(T('admin.inst.previewErr', 'Error getting placement previews: ') + error.message, 'error');
      }
    }

    async function viewInstance(instanceId) {
      try {
        const response = await fetch(`/api/v1/instance/${encodeURIComponent(instanceId)}`);
        if (!response.ok) throw new Error('Failed to load instance');
        const instance = await response.json();
        
        const modalContent = `
          <h3>${escapeHtml(T('admin.inst.modalTitle', 'Instance Details'))}: ${escapeHtml(instanceId)}</h3>
          <div class="form-group">
            <label>${escapeHtml(T('admin.inst.lbl.modelId', 'Model ID:'))}</label>
            <div>${escapeHtml(instance.model_id)}</div>
          </div>
          <div class="form-group">
            <label>${escapeHtml(T('admin.inst.col.status', 'Status'))}</label>
            <div>${escapeHtml(instance.status)}</div>
          </div>
          <div class="form-group">
            <label>${escapeHtml(T('admin.inst.col.strategy', 'Strategy'))}</label>
            <div>${escapeHtml(instance.placement.strategy)}</div>
          </div>
          <div class="form-group">
            <label>${escapeHtml(T('admin.inst.col.nodes', 'Nodes'))}</label>
            <div>${instance.placement.node_ids.join(', ')}</div>
          </div>
          <div class="form-group">
            <label>${escapeHtml(T('admin.inst.col.created', 'Created'))}</label>
            <div>${escapeHtml(new Date(instance.created_at).toLocaleString())}</div>
          </div>
        `;
        showModal(T('admin.inst.modalTitle', 'Instance Details'), modalContent);
      } catch (error) {
        showNotification(T('admin.inst.errLoadOne', 'Error loading instance: ') + error.message, 'error');
      }
    }

    async function deleteInstance(instanceId) {
      if (!confirm(T('admin.inst.confirmDel', 'Are you sure you want to delete instance {id}?').replace(/\{id\}/g, instanceId))) {
        return;
      }

      try {
        const token = getAuthToken();
        const response = await fetch(`/api/v1/instance/${encodeURIComponent(instanceId)}`, {
          method: 'DELETE',
          headers: {
            'Authorization': `Bearer ${token}`
          }
        });
        
        if (!response.ok) throw new Error('Failed to delete instance');
        
        showNotification(T('admin.inst.deletedOk', 'Instance deleted successfully'), 'success');
        loadInstances();
      } catch (error) {
        showNotification(T('admin.inst.errDel', 'Error deleting instance: ') + error.message, 'error');
      }
    }

    function showCreateInstanceModal() {
      const modalContent = `
        <form id="create-instance-form" onsubmit="createInstance(event)">
          <div class="form-group">
            <label for="create-model-id">${escapeHtml(T('admin.inst.lbl.modelId', 'Model ID:'))} <span class="required" aria-hidden="true">*</span></label>
            <input type="text" id="create-model-id" required aria-required="true" />
          </div>
          <div class="form-group">
            <label for="create-placement">${escapeHtml(T('admin.inst.lbl.placementJson', 'Placement (JSON):'))} <span class="required" aria-hidden="true">*</span></label>
            <textarea id="create-placement" rows="5" required aria-required="true">{"model_id": "", "strategy": "Single", "node_ids": ["local"]}</textarea>
          </div>
          <div class="form-actions">
            <button type="submit" class="btn btn-primary" data-i18n="ui.create">Create</button>
            <button type="button" class="btn" onclick="hideModal()" data-i18n="ui.cancel">Cancel</button>
          </div>
        </form>
      `;
      showModal(T('admin.inst.modalCreateTitle', 'Create Instance'), modalContent);
    }

    async function createInstance(event) {
      event.preventDefault();
      const modelId = document.getElementById('create-model-id').value.trim();
      const placementJson = document.getElementById('create-placement').value;

      try {
        const placement = JSON.parse(placementJson);
        const token = getAuthToken();
        
        const response = await fetch('/api/v1/instance', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${token}`
          },
          body: JSON.stringify({
            instance: {
              model_id: modelId,
              ...placement
            }
          })
        });
        
        if (!response.ok) throw new Error('Failed to create instance');
        const data = await response.json();
        
        showNotification(T('admin.inst.createdOk', 'Instance created successfully: ') + data.instance_id, 'success');
        hideModal();
        loadInstances();
      } catch (error) {
        showNotification(T('admin.inst.errCreate', 'Error creating instance: ') + error.message, 'error');
      }
    }

    function getAuthToken() {
      return localStorage.getItem('poolai_token') || '';
    }

    loadInstances();
  "#;

    admin_layout("admin.page.instances", "Model Instances", body, script)
}
