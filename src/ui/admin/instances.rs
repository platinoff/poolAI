//! Model instances management admin page

use super::admin_layout;

/// Admin instances page
pub async fn admin_instances() -> axum::response::Html<String> {
    let body = r#"
    <div class="admin-section">
      <h3>Model Instances</h3>
      <p>Manage AI model instances, placement, and lifecycle.</p>
      
      <div class="admin-section-header">
        <h4>Instances</h4>
        <button class="btn btn-primary" onclick="showCreateInstanceModal()">Create Instance</button>
      </div>
      
      <div id="instances-list" class="admin-table-container">
        <table class="admin-table">
          <thead>
            <tr>
              <th>Instance ID</th>
              <th>Model ID</th>
              <th>Status</th>
              <th>Strategy</th>
              <th>Nodes</th>
              <th>Created</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody id="instances-tbody">
            <tr><td colspan="7">Loading...</td></tr>
          </tbody>
        </table>
      </div>

      <div class="admin-section-header">
        <h4>Placement Preview</h4>
      </div>
      <div class="form-group">
        <label for="preview-model-id">Model ID:</label>
        <input type="text" id="preview-model-id" placeholder="Enter model ID" />
        <button class="btn btn-primary" onclick="previewPlacement()">Preview Placement</button>
      </div>
      <div id="placement-previews" class="admin-table-container">
        <table class="admin-table">
          <thead>
            <tr>
              <th>Strategy</th>
              <th>Nodes</th>
              <th>Memory Delta</th>
              <th>Error</th>
            </tr>
          </thead>
          <tbody id="placement-previews-tbody">
            <tr><td colspan="4">Enter a model ID and click Preview</td></tr>
          </tbody>
        </table>
      </div>
    </div>
  "#;

    let script = r#"
    async function loadInstances() {
      try {
        const response = await fetch('/api/v1/instance');
        if (!response.ok) throw new Error('Failed to load instances');
        const data = await response.json();
        
        const tbody = document.getElementById('instances-tbody');
        tbody.innerHTML = '';
        
        if (!data.instances || data.instances.length === 0) {
          tbody.innerHTML = '<tr><td colspan="7">No instances found</td></tr>';
          return;
        }
        
        for (const instance of data.instances) {
          const row = document.createElement('tr');
          row.innerHTML = `
            <td>${escapeHtml(instance.instance_id)}</td>
            <td>${escapeHtml(instance.model_id)}</td>
            <td><span class="badge">${escapeHtml(instance.status)}</span></td>
            <td>${escapeHtml(instance.placement.strategy)}</td>
            <td>${instance.placement.node_ids.join(', ')}</td>
            <td>${new Date(instance.created_at).toLocaleString()}</td>
            <td>
              <button class="btn btn-sm" onclick="viewInstance('${escapeHtml(instance.instance_id)}')">View</button>
              <button class="btn btn-sm btn-danger" onclick="deleteInstance('${escapeHtml(instance.instance_id)}')">Delete</button>
            </td>
          `;
          tbody.appendChild(row);
        }
      } catch (error) {
        console.error('Error loading instances:', error);
        document.getElementById('instances-tbody').innerHTML = '<tr><td colspan="7">Error loading instances</td></tr>';
      }
    }

    async function previewPlacement() {
      const modelId = document.getElementById('preview-model-id').value.trim();
      if (!modelId) {
        showNotification('Please enter a model ID', 'error');
        return;
      }

      try {
        const response = await fetch(`/api/v1/instance/previews?model_id=${encodeURIComponent(modelId)}`);
        if (!response.ok) throw new Error('Failed to get placement previews');
        const data = await response.json();
        
        const tbody = document.getElementById('placement-previews-tbody');
        tbody.innerHTML = '';
        
        if (!data.previews || data.previews.length === 0) {
          tbody.innerHTML = '<tr><td colspan="4">No placement options available</td></tr>';
          return;
        }
        
        for (const preview of data.previews) {
          const row = document.createElement('tr');
          row.innerHTML = `
            <td>${escapeHtml(preview.sharding)}</td>
            <td>${Object.keys(preview.memory_delta_by_node || {}).join(', ') || 'N/A'}</td>
            <td>${Object.values(preview.memory_delta_by_node || {}).reduce((a, b) => a + b, 0)} MB</td>
            <td>${preview.error ? escapeHtml(preview.error) : '-'}</td>
          `;
          tbody.appendChild(row);
        }
      } catch (error) {
        console.error('Error getting placement previews:', error);
        showNotification('Error getting placement previews: ' + error.message, 'error');
      }
    }

    async function viewInstance(instanceId) {
      try {
        const response = await fetch(`/api/v1/instance/${encodeURIComponent(instanceId)}`);
        if (!response.ok) throw new Error('Failed to load instance');
        const instance = await response.json();
        
        const modalContent = `
          <h3>Instance: ${escapeHtml(instanceId)}</h3>
          <div class="form-group">
            <label>Model ID:</label>
            <div>${escapeHtml(instance.model_id)}</div>
          </div>
          <div class="form-group">
            <label>Status:</label>
            <div>${escapeHtml(instance.status)}</div>
          </div>
          <div class="form-group">
            <label>Strategy:</label>
            <div>${escapeHtml(instance.placement.strategy)}</div>
          </div>
          <div class="form-group">
            <label>Nodes:</label>
            <div>${instance.placement.node_ids.join(', ')}</div>
          </div>
          <div class="form-group">
            <label>Created:</label>
            <div>${new Date(instance.created_at).toLocaleString()}</div>
          </div>
        `;
        showModal('Instance Details', modalContent);
      } catch (error) {
        showNotification('Error loading instance: ' + error.message, 'error');
      }
    }

    async function deleteInstance(instanceId) {
      if (!confirm(`Are you sure you want to delete instance ${instanceId}?`)) {
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
        
        showNotification('Instance deleted successfully', 'success');
        loadInstances();
      } catch (error) {
        showNotification('Error deleting instance: ' + error.message, 'error');
      }
    }

    function showCreateInstanceModal() {
      const modalContent = `
        <form id="create-instance-form" onsubmit="createInstance(event)">
          <div class="form-group">
            <label for="create-model-id">Model ID:</label>
            <input type="text" id="create-model-id" required />
          </div>
          <div class="form-group">
            <label>Placement (JSON):</label>
            <textarea id="create-placement" rows="5" required>{"model_id": "", "strategy": "Single", "node_ids": ["local"]}</textarea>
          </div>
          <div class="form-actions">
            <button type="submit" class="btn btn-primary">Create</button>
            <button type="button" class="btn" onclick="hideModal()">Cancel</button>
          </div>
        </form>
      `;
      showModal('Create Instance', modalContent);
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
        
        showNotification('Instance created successfully: ' + data.instance_id, 'success');
        hideModal();
        loadInstances();
      } catch (error) {
        showNotification('Error creating instance: ' + error.message, 'error');
      }
    }

    function escapeHtml(text) {
      const div = document.createElement('div');
      div.textContent = text;
      return div.innerHTML;
    }

    function getAuthToken() {
      return localStorage.getItem('jwt_token') || '';
    }

    // Load instances on page load
    loadInstances();
  "#;

    admin_layout("Model Instances", body, script)
}
