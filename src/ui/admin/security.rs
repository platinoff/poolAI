//! Security Management page
//!
//! Provides OAuth2/SAML providers and security policies management.

use crate::ui::admin::admin_layout;
use axum::response::Html;

/// Security management page
pub async fn admin_security() -> Html<String> {
    let script = r#"
    let currentTab = 'oauth2';
    
    function showTab(tabName) {
      currentTab = tabName;
      document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
      document.querySelector(`[data-tab="${tabName}"]`).classList.add('active');
      loadTabContent(tabName);
    }
    
    async function loadTabContent(tabName) {
      const el = document.getElementById('security-content');
      if (!el) return;
      
      switch(tabName) {
        case 'oauth2':
          await loadOAuth2Providers();
          break;
        case 'saml':
          await loadSamlProviders();
          break;
        case 'policies':
          await loadSecurityPolicies();
          break;
      }
    }
    
    // OAuth2 Providers Management
    async function loadOAuth2Providers() {
      const el = document.getElementById('security-content');
      if (!el) return;
      
      try {
        adminShowLoading('security-content', 'Loading OAuth2 providers…');
        const providers = await fetchJson('/api/enterprise/security/oauth2/providers');
        renderOAuth2Providers(providers);
      } catch (e) {
        adminShowInlineError('security-content', e);
      }
    }
    
    function renderOAuth2Providers(providers) {
      const el = document.getElementById('security-content');
      if (!el) return;
      
      const providersList = Array.isArray(providers) ? providers : [];
      
      el.innerHTML = `
        <div class="admin-header">
          <h3>OAuth2 Providers</h3>
          <button class="btn btn-primary" onclick="showCreateOAuth2Modal()" aria-label="Register OAuth2 provider">Register Provider</button>
        </div>
        <div id="oauth2-providers-list">
          ${providersList.length === 0 ? '<div class="muted">No OAuth2 providers registered</div>' : `
            <table class="admin-table">
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Client ID</th>
                  <th>Authorization URL</th>
                  <th>Status</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                ${providersList.map(p => `
                  <tr>
                    <td><strong>${p.name || 'unknown'}</strong></td>
                    <td><code>${p.config?.client_id || 'N/A'}</code></td>
                    <td><code>${p.config?.authorization_url || 'N/A'}</code></td>
                    <td><span class="status-badge ${p.enabled ? 'active' : 'inactive'}">${p.enabled ? 'Enabled' : 'Disabled'}</span></td>
                    <td>
                      <button class="btn" onclick="editOAuth2Provider('${p.name}')">Edit</button>
                      <button class="btn btn-danger" onclick="deleteOAuth2Provider('${p.name}')">Delete</button>
                    </td>
                  </tr>
                `).join('')}
              </tbody>
            </table>
          `}
        </div>
      `;
    }
    
    function showCreateOAuth2Modal() {
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions. Admin role required.', 'error');
        return;
      }
      showModal('createOAuth2Modal');
    }
    
    async function handleCreateOAuth2(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = 'Registering...';
      
      try {
        const scopes = document.getElementById('oauth2Scopes').value.split(',').map(s => s.trim()).filter(s => s);
        
        const payload = {
          name: document.getElementById('oauth2Name').value,
          config: {
            client_id: document.getElementById('oauth2ClientId').value,
            client_secret: document.getElementById('oauth2ClientSecret').value,
            authorization_url: document.getElementById('oauth2AuthUrl').value,
            token_url: document.getElementById('oauth2TokenUrl').value,
            redirect_uri: document.getElementById('oauth2RedirectUri').value,
            scopes: scopes
          },
          enabled: document.getElementById('oauth2Enabled').checked
        };
        
        await fetchJson('/api/enterprise/security/oauth2/providers', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification('OAuth2 provider registered successfully', 'success');
        hideModal('createOAuth2Modal');
        form.reset();
        loadOAuth2Providers();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function editOAuth2Provider(name) {
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      try {
        const provider = await fetchJson(`/api/enterprise/security/oauth2/providers/${encodeURIComponent(name)}`);
        document.getElementById('editOAuth2Name').value = provider.name;
        document.getElementById('editOAuth2ClientId').value = provider.config.client_id;
        document.getElementById('editOAuth2ClientSecret').value = provider.config.client_secret;
        document.getElementById('editOAuth2AuthUrl').value = provider.config.authorization_url;
        document.getElementById('editOAuth2TokenUrl').value = provider.config.token_url;
        document.getElementById('editOAuth2RedirectUri').value = provider.config.redirect_uri;
        document.getElementById('editOAuth2Scopes').value = provider.config.scopes.join(', ');
        document.getElementById('editOAuth2Enabled').checked = provider.enabled;
        showModal('editOAuth2Modal');
      } catch (e) {
        showNotification('Error loading provider: ' + e.message, 'error');
      }
    }
    
    async function handleEditOAuth2(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      const name = document.getElementById('editOAuth2Name').value;
      
      btn.disabled = true;
      btn.textContent = 'Updating...';
      
      try {
        const scopes = document.getElementById('editOAuth2Scopes').value.split(',').map(s => s.trim()).filter(s => s);
        
        const payload = {
          config: {
            client_id: document.getElementById('editOAuth2ClientId').value,
            client_secret: document.getElementById('editOAuth2ClientSecret').value,
            authorization_url: document.getElementById('editOAuth2AuthUrl').value,
            token_url: document.getElementById('editOAuth2TokenUrl').value,
            redirect_uri: document.getElementById('editOAuth2RedirectUri').value,
            scopes: scopes
          },
          enabled: document.getElementById('editOAuth2Enabled').checked
        };
        
        await fetchJson(`/api/enterprise/security/oauth2/providers/${encodeURIComponent(name)}`, {
          method: 'PUT',
          body: JSON.stringify(payload)
        });
        
        showNotification('OAuth2 provider updated successfully', 'success');
        hideModal('editOAuth2Modal');
        loadOAuth2Providers();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function deleteOAuth2Provider(name) {
      if (!confirm('Delete OAuth2 provider "' + name + '"? This action cannot be undone.')) {
        return;
      }
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      try {
        await fetchJson(`/api/enterprise/security/oauth2/providers/${encodeURIComponent(name)}`, {
          method: 'DELETE'
        });
        showNotification('OAuth2 provider deleted successfully', 'success');
        loadOAuth2Providers();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      }
    }
    
    // SAML Providers Management
    async function loadSamlProviders() {
      const el = document.getElementById('security-content');
      if (!el) return;
      
      try {
        adminShowLoading('security-content', 'Loading SAML providers…');
        const providers = await fetchJson('/api/enterprise/security/saml/providers');
        renderSamlProviders(providers);
      } catch (e) {
        adminShowInlineError('security-content', e);
      }
    }
    
    function renderSamlProviders(providers) {
      const el = document.getElementById('security-content');
      if (!el) return;
      
      const providersList = Array.isArray(providers) ? providers : [];
      
      el.innerHTML = `
        <div class="admin-header">
          <h3>SAML Providers</h3>
          <button class="btn btn-primary" onclick="showCreateSamlModal()" aria-label="Register SAML provider">Register Provider</button>
        </div>
        <div id="saml-providers-list">
          ${providersList.length === 0 ? '<div class="muted">No SAML providers registered</div>' : `
            <table class="admin-table">
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Entity ID</th>
                  <th>SSO URL</th>
                  <th>Status</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                ${providersList.map(p => `
                  <tr>
                    <td><strong>${p.name || 'unknown'}</strong></td>
                    <td><code>${p.config?.entity_id || 'N/A'}</code></td>
                    <td><code>${p.config?.sso_url || 'N/A'}</code></td>
                    <td><span class="status-badge ${p.enabled ? 'active' : 'inactive'}">${p.enabled ? 'Enabled' : 'Disabled'}</span></td>
                    <td>
                      <button class="btn" onclick="editSamlProvider('${p.name}')">Edit</button>
                      <button class="btn btn-danger" onclick="deleteSamlProvider('${p.name}')">Delete</button>
                    </td>
                  </tr>
                `).join('')}
              </tbody>
            </table>
          `}
        </div>
      `;
    }
    
    function showCreateSamlModal() {
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions. Admin role required.', 'error');
        return;
      }
      showModal('createSamlModal');
    }
    
    async function handleCreateSaml(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = 'Registering...';
      
      try {
        const attributeMapping = {};
        const mappingText = document.getElementById('samlAttributeMapping').value;
        if (mappingText) {
          mappingText.split('\\n').forEach(line => {
            const parts = line.split(':').map(s => s.trim());
            if (parts.length === 2) {
              attributeMapping[parts[0]] = parts[1];
            }
          });
        }
        
        const payload = {
          name: document.getElementById('samlName').value,
          config: {
            entity_id: document.getElementById('samlEntityId').value,
            sso_url: document.getElementById('samlSsoUrl').value,
            slo_url: document.getElementById('samlSloUrl').value || null,
            certificate: document.getElementById('samlCertificate').value,
            attribute_mapping: attributeMapping
          },
          enabled: document.getElementById('samlEnabled').checked
        };
        
        await fetchJson('/api/enterprise/security/saml/providers', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification('SAML provider registered successfully', 'success');
        hideModal('createSamlModal');
        form.reset();
        loadSamlProviders();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function editSamlProvider(name) {
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      try {
        const provider = await fetchJson(`/api/enterprise/security/saml/providers/${encodeURIComponent(name)}`);
        document.getElementById('editSamlName').value = provider.name;
        document.getElementById('editSamlEntityId').value = provider.config.entity_id;
        document.getElementById('editSamlSsoUrl').value = provider.config.sso_url;
        document.getElementById('editSamlSloUrl').value = provider.config.slo_url || '';
        document.getElementById('editSamlCertificate').value = provider.config.certificate;
        const mappingText = Object.entries(provider.config.attribute_mapping || {})
          .map(([k, v]) => k + ': ' + v).join('\\n');
        document.getElementById('editSamlAttributeMapping').value = mappingText;
        document.getElementById('editSamlEnabled').checked = provider.enabled;
        showModal('editSamlModal');
      } catch (e) {
        showNotification('Error loading provider: ' + e.message, 'error');
      }
    }
    
    async function handleEditSaml(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      const name = document.getElementById('editSamlName').value;
      
      btn.disabled = true;
      btn.textContent = 'Updating...';
      
      try {
        const attributeMapping = {};
        const mappingText = document.getElementById('editSamlAttributeMapping').value;
        if (mappingText) {
          mappingText.split('\\n').forEach(line => {
            const parts = line.split(':').map(s => s.trim());
            if (parts.length === 2) {
              attributeMapping[parts[0]] = parts[1];
            }
          });
        }
        
        const payload = {
          config: {
            entity_id: document.getElementById('editSamlEntityId').value,
            sso_url: document.getElementById('editSamlSsoUrl').value,
            slo_url: document.getElementById('editSamlSloUrl').value || null,
            certificate: document.getElementById('editSamlCertificate').value,
            attribute_mapping: attributeMapping
          },
          enabled: document.getElementById('editSamlEnabled').checked
        };
        
        await fetchJson(`/api/enterprise/security/saml/providers/${encodeURIComponent(name)}`, {
          method: 'PUT',
          body: JSON.stringify(payload)
        });
        
        showNotification('SAML provider updated successfully', 'success');
        hideModal('editSamlModal');
        loadSamlProviders();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function deleteSamlProvider(name) {
      if (!confirm('Delete SAML provider "' + name + '"? This action cannot be undone.')) {
        return;
      }
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      try {
        await fetchJson(`/api/enterprise/security/saml/providers/${encodeURIComponent(name)}`, {
          method: 'DELETE'
        });
        showNotification('SAML provider deleted successfully', 'success');
        loadSamlProviders();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      }
    }
    
    // Security Policies Management
    async function loadSecurityPolicies() {
      const el = document.getElementById('security-content');
      if (!el) return;
      
      try {
        adminShowLoading('security-content', 'Loading security policies…');
        const policies = await fetchJson('/api/enterprise/security/policies');
        renderSecurityPolicies(policies);
      } catch (e) {
        adminShowInlineError('security-content', e);
      }
    }
    
    function renderSecurityPolicies(policies) {
      const el = document.getElementById('security-content');
      if (!el) return;
      
      const policiesList = Array.isArray(policies) ? policies : [];
      
      el.innerHTML = `
        <div class="admin-header">
          <h3>Security Policies</h3>
          <button class="btn btn-primary" onclick="showCreatePolicyModal()" aria-label="Create security policy">Create Policy</button>
        </div>
        <div id="security-policies-list">
          ${policiesList.length === 0 ? '<div class="muted">No security policies defined</div>' : `
            <table class="admin-table">
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Description</th>
                  <th>MFA Required</th>
                  <th>Session Timeout</th>
                  <th>Max Failed Attempts</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                ${policiesList.map(p => `
                  <tr>
                    <td><strong>${p.name || 'unknown'}</strong></td>
                    <td>${p.description || 'ΓÇö'}</td>
                    <td><span class="status-badge ${p.require_mfa ? 'active' : 'inactive'}">${p.require_mfa ? 'Yes' : 'No'}</span></td>
                    <td>${p.session_timeout || 0}s</td>
                    <td>${p.max_failed_attempts || 0}</td>
                    <td>
                      <button class="btn" onclick="editSecurityPolicy('${p.name}')">Edit</button>
                      <button class="btn btn-danger" onclick="deleteSecurityPolicy('${p.name}')">Delete</button>
                    </td>
                  </tr>
                `).join('')}
              </tbody>
            </table>
          `}
        </div>
      `;
    }
    
    function showCreatePolicyModal() {
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions. Admin role required.', 'error');
        return;
      }
      showModal('createPolicyModal');
    }
    
    async function handleCreatePolicy(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = 'Creating...';
      
      try {
        const ipRanges = document.getElementById('policyIpRanges').value.split(',').map(s => s.trim()).filter(s => s);
        
        const payload = {
          policy: {
            name: document.getElementById('policyName').value,
            description: document.getElementById('policyDescription').value,
            allowed_ip_ranges: ipRanges,
            require_mfa: document.getElementById('policyRequireMfa').checked,
            session_timeout: parseInt(document.getElementById('policySessionTimeout').value, 10),
            max_failed_attempts: parseInt(document.getElementById('policyMaxFailedAttempts').value, 10)
          }
        };
        
        await fetchJson('/api/enterprise/security/policies', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification('Security policy created successfully', 'success');
        hideModal('createPolicyModal');
        form.reset();
        loadSecurityPolicies();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function editSecurityPolicy(name) {
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      try {
        const policy = await fetchJson(`/api/enterprise/security/policies/${encodeURIComponent(name)}`);
        document.getElementById('editPolicyName').value = policy.name;
        document.getElementById('editPolicyDescription').value = policy.description;
        document.getElementById('editPolicyIpRanges').value = policy.allowed_ip_ranges.join(', ');
        document.getElementById('editPolicyRequireMfa').checked = policy.require_mfa;
        document.getElementById('editPolicySessionTimeout').value = policy.session_timeout;
        document.getElementById('editPolicyMaxFailedAttempts').value = policy.max_failed_attempts;
        showModal('editPolicyModal');
      } catch (e) {
        showNotification('Error loading policy: ' + e.message, 'error');
      }
    }
    
    async function handleEditPolicy(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      const name = document.getElementById('editPolicyName').value;
      
      btn.disabled = true;
      btn.textContent = 'Updating...';
      
      try {
        const ipRanges = document.getElementById('editPolicyIpRanges').value.split(',').map(s => s.trim()).filter(s => s);
        
        const payload = {
          policy: {
            name: name,
            description: document.getElementById('editPolicyDescription').value,
            allowed_ip_ranges: ipRanges,
            require_mfa: document.getElementById('editPolicyRequireMfa').checked,
            session_timeout: parseInt(document.getElementById('editPolicySessionTimeout').value, 10),
            max_failed_attempts: parseInt(document.getElementById('editPolicyMaxFailedAttempts').value, 10)
          }
        };
        
        await fetchJson(`/api/enterprise/security/policies/${encodeURIComponent(name)}`, {
          method: 'PUT',
          body: JSON.stringify(payload)
        });
        
        showNotification('Security policy updated successfully', 'success');
        hideModal('editPolicyModal');
        loadSecurityPolicies();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function deleteSecurityPolicy(name) {
      if (!confirm('Delete security policy "' + name + '"? This action cannot be undone.')) {
        return;
      }
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      try {
        await fetchJson(`/api/enterprise/security/policies/${encodeURIComponent(name)}`, {
          method: 'DELETE'
        });
        showNotification('Security policy deleted successfully', 'success');
        loadSecurityPolicies();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      }
    }
    
    document.querySelectorAll('.tab').forEach(tab => {
      tab.addEventListener('click', () => showTab(tab.dataset.tab));
    });
    
    loadTabContent('oauth2');
    "#;

    admin_layout(
        "Security Management",
        r#"
        <div class="admin-section">
          <div class="admin-tabs">
            <button class="tab active" data-tab="oauth2">OAuth2 Providers</button>
            <button class="tab" data-tab="saml">SAML Providers</button>
            <button class="tab" data-tab="policies">Security Policies</button>
          </div>
          <div id="security-content"></div>
        </div>
        
        <!-- Create OAuth2 Provider Modal -->
        <div id="createOAuth2Modal" class="modal" role="dialog" aria-labelledby="createOAuth2ModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createOAuth2ModalTitle">Register OAuth2 Provider</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('createOAuth2Modal')">&times;</button>
            </div>
            <form id="createOAuth2Form" onsubmit="handleCreateOAuth2(event)">
              <div class="form-group">
                <label for="oauth2Name">Provider Name <span class="required">*</span></label>
                <input type="text" id="oauth2Name" name="name" required placeholder="google" />
              </div>
              <div class="form-group">
                <label for="oauth2ClientId">Client ID <span class="required">*</span></label>
                <input type="text" id="oauth2ClientId" name="client_id" required />
              </div>
              <div class="form-group">
                <label for="oauth2ClientSecret">Client Secret <span class="required">*</span></label>
                <input type="password" id="oauth2ClientSecret" name="client_secret" required />
              </div>
              <div class="form-group">
                <label for="oauth2AuthUrl">Authorization URL <span class="required">*</span></label>
                <input type="url" id="oauth2AuthUrl" name="authorization_url" required placeholder="https://accounts.google.com/o/oauth2/auth" />
              </div>
              <div class="form-group">
                <label for="oauth2TokenUrl">Token URL <span class="required">*</span></label>
                <input type="url" id="oauth2TokenUrl" name="token_url" required placeholder="https://oauth2.googleapis.com/token" />
              </div>
              <div class="form-group">
                <label for="oauth2RedirectUri">Redirect URI <span class="required">*</span></label>
                <input type="url" id="oauth2RedirectUri" name="redirect_uri" required placeholder="https://poolai.example.com/callback" />
              </div>
              <div class="form-group">
                <label for="oauth2Scopes">Scopes (comma-separated)</label>
                <input type="text" id="oauth2Scopes" name="scopes" placeholder="openid, profile, email" />
              </div>
              <div class="form-group">
                <label for="oauth2Enabled">
                  <input type="checkbox" id="oauth2Enabled" name="enabled" checked />
                  Enabled
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createOAuth2Modal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Register</button>
              </div>
            </form>
          </div>
        </div>
        
        <!-- Edit OAuth2 Provider Modal -->
        <div id="editOAuth2Modal" class="modal" role="dialog" aria-labelledby="editOAuth2ModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="editOAuth2ModalTitle">Edit OAuth2 Provider</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('editOAuth2Modal')">&times;</button>
            </div>
            <form id="editOAuth2Form" onsubmit="handleEditOAuth2(event)">
              <input type="hidden" id="editOAuth2Name" />
              <div class="form-group">
                <label for="editOAuth2ClientId">Client ID <span class="required">*</span></label>
                <input type="text" id="editOAuth2ClientId" name="client_id" required />
              </div>
              <div class="form-group">
                <label for="editOAuth2ClientSecret">Client Secret <span class="required">*</span></label>
                <input type="password" id="editOAuth2ClientSecret" name="client_secret" required />
              </div>
              <div class="form-group">
                <label for="editOAuth2AuthUrl">Authorization URL <span class="required">*</span></label>
                <input type="url" id="editOAuth2AuthUrl" name="authorization_url" required />
              </div>
              <div class="form-group">
                <label for="editOAuth2TokenUrl">Token URL <span class="required">*</span></label>
                <input type="url" id="editOAuth2TokenUrl" name="token_url" required />
              </div>
              <div class="form-group">
                <label for="editOAuth2RedirectUri">Redirect URI <span class="required">*</span></label>
                <input type="url" id="editOAuth2RedirectUri" name="redirect_uri" required />
              </div>
              <div class="form-group">
                <label for="editOAuth2Scopes">Scopes (comma-separated)</label>
                <input type="text" id="editOAuth2Scopes" name="scopes" />
              </div>
              <div class="form-group">
                <label for="editOAuth2Enabled">
                  <input type="checkbox" id="editOAuth2Enabled" name="enabled" />
                  Enabled
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('editOAuth2Modal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Update</button>
              </div>
            </form>
          </div>
        </div>
        
        <!-- Create SAML Provider Modal -->
        <div id="createSamlModal" class="modal" role="dialog" aria-labelledby="createSamlModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createSamlModalTitle">Register SAML Provider</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('createSamlModal')">&times;</button>
            </div>
            <form id="createSamlForm" onsubmit="handleCreateSaml(event)">
              <div class="form-group">
                <label for="samlName">Provider Name <span class="required">*</span></label>
                <input type="text" id="samlName" name="name" required placeholder="okta" />
              </div>
              <div class="form-group">
                <label for="samlEntityId">Entity ID <span class="required">*</span></label>
                <input type="text" id="samlEntityId" name="entity_id" required />
              </div>
              <div class="form-group">
                <label for="samlSsoUrl">SSO URL <span class="required">*</span></label>
                <input type="url" id="samlSsoUrl" name="sso_url" required />
              </div>
              <div class="form-group">
                <label for="samlSloUrl">SLO URL (optional)</label>
                <input type="url" id="samlSloUrl" name="slo_url" />
              </div>
              <div class="form-group">
                <label for="samlCertificate">X.509 Certificate <span class="required">*</span></label>
                <textarea id="samlCertificate" name="certificate" required rows="5" placeholder="-----BEGIN CERTIFICATE-----..."></textarea>
              </div>
              <div class="form-group">
                <label for="samlAttributeMapping">Attribute Mapping (one per line, format: saml_attribute: user_field)</label>
                <textarea id="samlAttributeMapping" name="attribute_mapping" rows="3" placeholder="email: email&#10;name: username"></textarea>
              </div>
              <div class="form-group">
                <label for="samlEnabled">
                  <input type="checkbox" id="samlEnabled" name="enabled" checked />
                  Enabled
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createSamlModal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Register</button>
              </div>
            </form>
          </div>
        </div>
        
        <!-- Edit SAML Provider Modal -->
        <div id="editSamlModal" class="modal" role="dialog" aria-labelledby="editSamlModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="editSamlModalTitle">Edit SAML Provider</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('editSamlModal')">&times;</button>
            </div>
            <form id="editSamlForm" onsubmit="handleEditSaml(event)">
              <input type="hidden" id="editSamlName" />
              <div class="form-group">
                <label for="editSamlEntityId">Entity ID <span class="required">*</span></label>
                <input type="text" id="editSamlEntityId" name="entity_id" required />
              </div>
              <div class="form-group">
                <label for="editSamlSsoUrl">SSO URL <span class="required">*</span></label>
                <input type="url" id="editSamlSsoUrl" name="sso_url" required />
              </div>
              <div class="form-group">
                <label for="editSamlSloUrl">SLO URL (optional)</label>
                <input type="url" id="editSamlSloUrl" name="slo_url" />
              </div>
              <div class="form-group">
                <label for="editSamlCertificate">X.509 Certificate <span class="required">*</span></label>
                <textarea id="editSamlCertificate" name="certificate" required rows="5"></textarea>
              </div>
              <div class="form-group">
                <label for="editSamlAttributeMapping">Attribute Mapping (one per line, format: saml_attribute: user_field)</label>
                <textarea id="editSamlAttributeMapping" name="attribute_mapping" rows="3"></textarea>
              </div>
              <div class="form-group">
                <label for="editSamlEnabled">
                  <input type="checkbox" id="editSamlEnabled" name="enabled" />
                  Enabled
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('editSamlModal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Update</button>
              </div>
            </form>
          </div>
        </div>
        
        <!-- Create Security Policy Modal -->
        <div id="createPolicyModal" class="modal" role="dialog" aria-labelledby="createPolicyModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createPolicyModalTitle">Create Security Policy</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('createPolicyModal')">&times;</button>
            </div>
            <form id="createPolicyForm" onsubmit="handleCreatePolicy(event)">
              <div class="form-group">
                <label for="policyName">Policy Name <span class="required">*</span></label>
                <input type="text" id="policyName" name="name" required placeholder="strict-policy" />
              </div>
              <div class="form-group">
                <label for="policyDescription">Description</label>
                <textarea id="policyDescription" name="description" rows="3" placeholder="Strict security policy for admin access"></textarea>
              </div>
              <div class="form-group">
                <label for="policyIpRanges">Allowed IP Ranges (CIDR, comma-separated)</label>
                <input type="text" id="policyIpRanges" name="ip_ranges" placeholder="192.168.1.0/24, 10.0.0.0/8" />
              </div>
              <div class="form-group">
                <label for="policyRequireMfa">
                  <input type="checkbox" id="policyRequireMfa" name="require_mfa" />
                  Require MFA
                </label>
              </div>
              <div class="form-group">
                <label for="policySessionTimeout">Session Timeout (seconds) <span class="required">*</span></label>
                <input type="number" id="policySessionTimeout" name="session_timeout" required min="60" value="3600" />
              </div>
              <div class="form-group">
                <label for="policyMaxFailedAttempts">Max Failed Login Attempts <span class="required">*</span></label>
                <input type="number" id="policyMaxFailedAttempts" name="max_failed_attempts" required min="1" value="5" />
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createPolicyModal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Create</button>
              </div>
            </form>
          </div>
        </div>
        
        <!-- Edit Security Policy Modal -->
        <div id="editPolicyModal" class="modal" role="dialog" aria-labelledby="editPolicyModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="editPolicyModalTitle">Edit Security Policy</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('editPolicyModal')">&times;</button>
            </div>
            <form id="editPolicyForm" onsubmit="handleEditPolicy(event)">
              <input type="hidden" id="editPolicyName" />
              <div class="form-group">
                <label for="editPolicyDescription">Description</label>
                <textarea id="editPolicyDescription" name="description" rows="3"></textarea>
              </div>
              <div class="form-group">
                <label for="editPolicyIpRanges">Allowed IP Ranges (CIDR, comma-separated)</label>
                <input type="text" id="editPolicyIpRanges" name="ip_ranges" />
              </div>
              <div class="form-group">
                <label for="editPolicyRequireMfa">
                  <input type="checkbox" id="editPolicyRequireMfa" name="require_mfa" />
                  Require MFA
                </label>
              </div>
              <div class="form-group">
                <label for="editPolicySessionTimeout">Session Timeout (seconds) <span class="required">*</span></label>
                <input type="number" id="editPolicySessionTimeout" name="session_timeout" required min="60" />
              </div>
              <div class="form-group">
                <label for="editPolicyMaxFailedAttempts">Max Failed Login Attempts <span class="required">*</span></label>
                <input type="number" id="editPolicyMaxFailedAttempts" name="max_failed_attempts" required min="1" />
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('editPolicyModal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Update</button>
              </div>
            </form>
          </div>
        </div>
        "#,
        script,
    )
}
