//! Security Management page
//!
//! Provides OAuth2/SAML providers and security policies management.

use crate::ui::admin::admin_layout_security;
use axum::response::Html;

/// Security management page
pub async fn admin_security() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }
    function Ep() { return typeof poolaiT === 'function' ? poolaiT('err.errorPrefix', 'Error: ') : 'Error: '; }

    let currentTab = 'oauth2';
    
    function showTab(tabName) {
      currentTab = tabName;
      document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
      const active = document.querySelector(`[data-tab="${tabName}"]`);
      if (active) active.classList.add('active');
      if (typeof adminSyncTabA11y === 'function') {
        adminSyncTabA11y(document.querySelector('.admin-tabs'));
      }
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
        case 'rotation':
          await loadSecretRotation();
          break;
      }
    }
    
    // OAuth2 Providers Management
    async function loadOAuth2Providers() {
      const el = document.getElementById('security-content');
      if (!el) return;
      
      try {
        adminShowLoading('security-content', T('admin.sec.loadingOauth', 'Loading OAuth2 providers…'));
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
          <h3>${escapeHtml(T('admin.sec.oauthHeading', 'OAuth2 Providers'))}</h3>
          <button type="button" class="btn btn-primary" onclick="showCreateOAuth2Modal()" aria-label="${escapeHtml(T('ui.register', 'Register'))}">${escapeHtml(T('admin.sec.registerProv', 'Register Provider'))}</button>
        </div>
        <div id="oauth2-providers-list">
          ${providersList.length === 0 ? '<div class="muted">' + escapeHtml(T('admin.sec.noOAuth', 'No OAuth2 providers registered')) + '</div>' : `
            <table class="admin-table">
              <thead>
                <tr>
                  <th>${escapeHtml(T('admin.sec.col.name', 'Name'))}</th>
                  <th>${escapeHtml(T('admin.sec.col.clientId', 'Client ID'))}</th>
                  <th>${escapeHtml(T('admin.sec.col.authUrl', 'Authorization URL'))}</th>
                  <th>${escapeHtml(T('admin.mon.col.statusCol', 'Status'))}</th>
                  <th>${escapeHtml(T('admin.mon.col.actions', 'Actions'))}</th>
                </tr>
              </thead>
              <tbody>
                ${providersList.map(p => {
                  const pn = JSON.stringify(p.name || '');
                  return `
                  <tr>
                    <td><strong>${escapeHtml(p.name || 'unknown')}</strong></td>
                    <td><code>${escapeHtml(p.config?.client_id || T('admin.na', 'N/A'))}</code></td>
                    <td><code>${escapeHtml(p.config?.authorization_url || T('admin.na', 'N/A'))}</code></td>
                    <td><span class="status-badge ${p.enabled ? 'active' : 'inactive'}">${p.enabled ? escapeHtml(T('admin.mon.enabled', 'Enabled')) : escapeHtml(T('admin.mon.disabled', 'Disabled'))}</span></td>
                    <td>
                      <button type="button" class="btn" onclick='editOAuth2Provider(${pn})'>${escapeHtml(T('admin.btn.edit', 'Edit'))}</button>
                      <button type="button" class="btn btn-danger" onclick='deleteOAuth2Provider(${pn})'>${escapeHtml(T('ui.delete', 'Delete'))}</button>
                    </td>
                  </tr>
                `;
                }).join('')}
              </tbody>
            </table>
          `}
        </div>
      `;
    }
    
    function showCreateOAuth2Modal() {
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification(T('err.insufficientAdmin', 'Insufficient permissions. Admin role required.'), 'error');
        return;
      }
      showModal('createOAuth2Modal');
    }
    
    async function handleCreateOAuth2(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = T('admin.sec.registering', 'Registering…');
      
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
        
        showNotification(T('admin.sec.oauthRegOk', 'OAuth2 provider registered successfully'), 'success');
        hideModal('createOAuth2Modal');
        form.reset();
        loadOAuth2Providers();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function editOAuth2Provider(name) {
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
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
        showNotification(T('admin.sec.errLoadOauth', 'Error loading provider: ') + e.message, 'error');
      }
    }
    
    async function handleEditOAuth2(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      const name = document.getElementById('editOAuth2Name').value;
      
      btn.disabled = true;
      btn.textContent = T('admin.sec.updating', 'Updating…');
      
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
        
        showNotification(T('admin.sec.oauthUpdOk', 'OAuth2 provider updated successfully'), 'success');
        hideModal('editOAuth2Modal');
        loadOAuth2Providers();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function deleteOAuth2Provider(name) {
      if (!confirm(T('admin.sec.confirmDelOauth', 'Delete OAuth2 provider "{name}"? This action cannot be undone.').replace(/\{name\}/g, name))) {
        return;
      }
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
        return;
      }
      
      try {
        await fetchJson(`/api/enterprise/security/oauth2/providers/${encodeURIComponent(name)}`, {
          method: 'DELETE'
        });
        showNotification(T('admin.sec.oauthDelOk', 'OAuth2 provider deleted successfully'), 'success');
        loadOAuth2Providers();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      }
    }
    
    // SAML Providers Management
    async function loadSamlProviders() {
      const el = document.getElementById('security-content');
      if (!el) return;
      
      try {
        adminShowLoading('security-content', T('admin.sec.loadingSaml', 'Loading SAML providers…'));
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
          <h3>${escapeHtml(T('admin.sec.samlHeading', 'SAML Providers'))}</h3>
          <button type="button" class="btn btn-primary" onclick="showCreateSamlModal()" aria-label="${escapeHtml(T('ui.register', 'Register'))}">${escapeHtml(T('admin.sec.registerProv', 'Register Provider'))}</button>
        </div>
        <div id="saml-providers-list">
          ${providersList.length === 0 ? '<div class="muted">' + escapeHtml(T('admin.sec.noSaml', 'No SAML providers registered')) + '</div>' : `
            <table class="admin-table">
              <thead>
                <tr>
                  <th>${escapeHtml(T('admin.sec.col.name', 'Name'))}</th>
                  <th>${escapeHtml(T('admin.sec.col.entityId', 'Entity ID'))}</th>
                  <th>${escapeHtml(T('admin.sec.col.ssoUrl', 'SSO URL'))}</th>
                  <th>${escapeHtml(T('admin.mon.col.statusCol', 'Status'))}</th>
                  <th>${escapeHtml(T('admin.mon.col.actions', 'Actions'))}</th>
                </tr>
              </thead>
              <tbody>
                ${providersList.map(p => {
                  const pn = JSON.stringify(p.name || '');
                  return `
                  <tr>
                    <td><strong>${escapeHtml(p.name || 'unknown')}</strong></td>
                    <td><code>${escapeHtml(p.config?.entity_id || T('admin.na', 'N/A'))}</code></td>
                    <td><code>${escapeHtml(p.config?.sso_url || T('admin.na', 'N/A'))}</code></td>
                    <td><span class="status-badge ${p.enabled ? 'active' : 'inactive'}">${p.enabled ? escapeHtml(T('admin.mon.enabled', 'Enabled')) : escapeHtml(T('admin.mon.disabled', 'Disabled'))}</span></td>
                    <td>
                      <button type="button" class="btn" onclick='editSamlProvider(${pn})'>${escapeHtml(T('admin.btn.edit', 'Edit'))}</button>
                      <button type="button" class="btn btn-danger" onclick='deleteSamlProvider(${pn})'>${escapeHtml(T('ui.delete', 'Delete'))}</button>
                    </td>
                  </tr>
                `;
                }).join('')}
              </tbody>
            </table>
          `}
        </div>
      `;
    }
    
    function showCreateSamlModal() {
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification(T('err.insufficientAdmin', 'Insufficient permissions. Admin role required.'), 'error');
        return;
      }
      showModal('createSamlModal');
    }
    
    async function handleCreateSaml(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = T('admin.sec.registering', 'Registering…');
      
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
        
        showNotification(T('admin.sec.samlRegOk', 'SAML provider registered successfully'), 'success');
        hideModal('createSamlModal');
        form.reset();
        loadSamlProviders();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function editSamlProvider(name) {
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
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
        showNotification(T('admin.sec.errLoadOauth', 'Error loading provider: ') + e.message, 'error');
      }
    }
    
    async function handleEditSaml(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      const name = document.getElementById('editSamlName').value;
      
      btn.disabled = true;
      btn.textContent = T('admin.sec.updating', 'Updating…');
      
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
        
        showNotification(T('admin.sec.samlUpdOk', 'SAML provider updated successfully'), 'success');
        hideModal('editSamlModal');
        loadSamlProviders();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function deleteSamlProvider(name) {
      if (!confirm(T('admin.sec.confirmDelSaml', 'Delete SAML provider "{name}"? This action cannot be undone.').replace(/\{name\}/g, name))) {
        return;
      }
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
        return;
      }
      
      try {
        await fetchJson(`/api/enterprise/security/saml/providers/${encodeURIComponent(name)}`, {
          method: 'DELETE'
        });
        showNotification(T('admin.sec.samlDelOk', 'SAML provider deleted successfully'), 'success');
        loadSamlProviders();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      }
    }
    
    // Security Policies Management
    async function loadSecurityPolicies() {
      const el = document.getElementById('security-content');
      if (!el) return;
      
      try {
        adminShowLoading('security-content', T('admin.sec.loadingPolicies', 'Loading security policies…'));
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
          <h3>${escapeHtml(T('admin.sec.policiesHeading', 'Security Policies'))}</h3>
          <button type="button" class="btn btn-primary" onclick="showCreatePolicyModal()" aria-label="${escapeHtml(T('admin.sec.createPolicyBtn', 'Create Policy'))}">${escapeHtml(T('admin.sec.createPolicyBtn', 'Create Policy'))}</button>
        </div>
        <div id="security-policies-list">
          ${policiesList.length === 0 ? '<div class="muted">' + escapeHtml(T('admin.sec.noPolicies', 'No security policies defined')) + '</div>' : `
            <table class="admin-table">
              <thead>
                <tr>
                  <th>${escapeHtml(T('admin.sec.col.name', 'Name'))}</th>
                  <th>${escapeHtml(T('admin.sec.col.policyDesc', 'Description'))}</th>
                  <th>${escapeHtml(T('admin.sec.col.mfa', 'MFA Required'))}</th>
                  <th>${escapeHtml(T('admin.sec.col.sessionTo', 'Session Timeout'))}</th>
                  <th>${escapeHtml(T('admin.sec.col.maxFailed', 'Max Failed Attempts'))}</th>
                  <th>${escapeHtml(T('admin.mon.col.actions', 'Actions'))}</th>
                </tr>
              </thead>
              <tbody>
                ${policiesList.map(p => {
                  const pn = JSON.stringify(p.name || '');
                  return `
                  <tr>
                    <td><strong>${escapeHtml(p.name || 'unknown')}</strong></td>
                    <td>${escapeHtml(p.description || T('admin.sec.emDash', '—'))}</td>
                    <td><span class="status-badge ${p.require_mfa ? 'active' : 'inactive'}">${p.require_mfa ? escapeHtml(T('admin.status.yes', 'Yes')) : escapeHtml(T('admin.status.no', 'No'))}</span></td>
                    <td>${p.session_timeout || 0}s</td>
                    <td>${p.max_failed_attempts || 0}</td>
                    <td>
                      <button type="button" class="btn" onclick='editSecurityPolicy(${pn})'>${escapeHtml(T('admin.btn.edit', 'Edit'))}</button>
                      <button type="button" class="btn btn-danger" onclick='deleteSecurityPolicy(${pn})'>${escapeHtml(T('ui.delete', 'Delete'))}</button>
                    </td>
                  </tr>
                `;
                }).join('')}
              </tbody>
            </table>
          `}
        </div>
      `;
    }
    
    function showCreatePolicyModal() {
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification(T('err.insufficientAdmin', 'Insufficient permissions. Admin role required.'), 'error');
        return;
      }
      showModal('createPolicyModal');
    }
    
    async function handleCreatePolicy(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = T('admin.sec.creating', 'Creating…');
      
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
        
        showNotification(T('admin.sec.policyCreatedOk', 'Security policy created successfully'), 'success');
        hideModal('createPolicyModal');
        form.reset();
        loadSecurityPolicies();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function editSecurityPolicy(name) {
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
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
        showNotification(T('admin.sec.errLoadPolicy', 'Error loading policy: ') + e.message, 'error');
      }
    }
    
    async function handleEditPolicy(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      const name = document.getElementById('editPolicyName').value;
      
      btn.disabled = true;
      btn.textContent = T('admin.sec.updating', 'Updating…');
      
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
        
        showNotification(T('admin.sec.policyUpdOk', 'Security policy updated successfully'), 'success');
        hideModal('editPolicyModal');
        loadSecurityPolicies();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function deleteSecurityPolicy(name) {
      if (!confirm(T('admin.sec.confirmDelPolicy', 'Delete security policy "{name}"? This action cannot be undone.').replace(/\{name\}/g, name))) {
        return;
      }
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
        return;
      }
      
      try {
        await fetchJson(`/api/enterprise/security/policies/${encodeURIComponent(name)}`, {
          method: 'DELETE'
        });
        showNotification(T('admin.sec.policyDelOk', 'Security policy deleted successfully'), 'success');
        loadSecurityPolicies();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      }
    }

    function formatRotationKind(kind) {
      const labels = {
        jwt: T('admin.sec.rot.kind.jwt', 'JWT signing secret'),
        tls_certificate: T('admin.sec.rot.kind.tls', 'TLS certificate'),
        telegram_webhook: T('admin.sec.rot.kind.telegram', 'Telegram webhook secret'),
      };
      return labels[kind] || kind;
    }

    function formatUnixTime(ts) {
      if (!ts) return T('admin.sec.rot.never', 'Never');
      try {
        return new Date(ts * 1000).toLocaleString();
      } catch (_) {
        return String(ts);
      }
    }

    async function loadSecretRotation() {
      const el = document.getElementById('security-content');
      if (!el) return;
      try {
        adminShowLoading('security-content', T('admin.sec.rot.loading', 'Loading secret rotation status…'));
        const rows = await fetchJson('/api/v1/admin/secrets/rotation');
        renderSecretRotation(Array.isArray(rows) ? rows : []);
      } catch (e) {
        adminShowInlineError('security-content', e);
      }
    }

    function renderSecretRotation(rows) {
      const el = document.getElementById('security-content');
      if (!el) return;
      el.innerHTML = `
        <div class="admin-header">
          <h3>${escapeHtml(T('admin.sec.rot.heading', 'Secret rotation'))}</h3>
          <button type="button" class="btn" onclick="loadSecretRotation()">${escapeHtml(T('admin.topo.refresh', 'Refresh'))}</button>
        </div>
        <table class="admin-table" id="secret-rotation-table">
          <thead>
            <tr>
              <th>${escapeHtml(T('admin.sec.rot.col.kind', 'Secret'))}</th>
              <th>${escapeHtml(T('admin.mon.col.statusCol', 'Status'))}</th>
              <th>${escapeHtml(T('admin.sec.rot.col.hooks', 'Hooks'))}</th>
              <th>${escapeHtml(T('admin.sec.rot.col.last', 'Last rotated'))}</th>
              <th>${escapeHtml(T('admin.sec.rot.col.count', 'Count'))}</th>
              <th>${escapeHtml(T('admin.sec.rot.col.grace', 'JWT grace'))}</th>
              <th>${escapeHtml(T('admin.mon.col.actions', 'Actions'))}</th>
            </tr>
          </thead>
          <tbody>
            ${rows.map((r) => {
              const kind = r.kind || '';
              const kindJson = JSON.stringify(kind);
              const configured = !!r.configured;
              const statusBadge = configured
                ? '<span class="status-badge active">' + escapeHtml(T('admin.sec.rot.configured', 'Configured')) + '</span>'
                : '<span class="status-badge inactive">' + escapeHtml(T('admin.sec.rot.notConfigured', 'Not configured')) + '</span>';
              let actionBtn = '';
              if (kind === 'jwt') {
                actionBtn = `<button type="button" class="btn btn-primary" onclick='rotateSecret(${kindJson})'>${escapeHtml(T('admin.sec.rot.reloadJwt', 'Reload JWT from env'))}</button>`;
              } else if (configured && (r.hook_count || 0) > 0) {
                actionBtn = `<button type="button" class="btn" onclick='rotateSecret(${kindJson})'>${escapeHtml(T('admin.sec.rot.run', 'Run rotation'))}</button>`;
              } else {
                actionBtn = '<span class="muted">' + escapeHtml(T('admin.na', 'N/A')) + '</span>';
              }
              return `
              <tr>
                <td><strong>${escapeHtml(formatRotationKind(kind))}</strong><br><code>${escapeHtml(kind)}</code></td>
                <td>${statusBadge}</td>
                <td>${escapeHtml(String(r.hook_count ?? 0))}</td>
                <td>${escapeHtml(formatUnixTime(r.last_rotated_unix))}</td>
                <td>${escapeHtml(String(r.rotation_count ?? 0))}</td>
                <td>${r.grace_active ? escapeHtml(T('admin.mon.enabled', 'Enabled')) : escapeHtml(T('admin.mon.disabled', 'Disabled'))}</td>
                <td>${actionBtn}</td>
              </tr>`;
            }).join('')}
          </tbody>
        </table>
        <p class="muted">${escapeHtml(T('admin.sec.rot.hint', 'Rotation runs registered hooks only; env vars must be set on the coordinator host.'))}</p>
      `;
    }

    async function rotateSecret(kind) {
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
        return;
      }
      try {
        const report = await fetchJson('/api/v1/admin/secrets/rotate', {
          method: 'POST',
          body: JSON.stringify({ kind }),
        });
        if (report && report.success) {
          showNotification(T('admin.sec.rot.ok', 'Secret rotation completed'), 'success');
        } else {
          showNotification(T('admin.sec.rot.partial', 'Rotation finished with hook errors'), 'warning');
        }
        loadSecretRotation();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      }
    }
    
    document.querySelectorAll('.tab').forEach(tab => {
      tab.addEventListener('click', () => showTab(tab.dataset.tab));
    });
    
    loadTabContent('oauth2');
    "#;

    admin_layout_security(
        "admin.page.security",
        "Security Management",
        r#"
        <div class="admin-section">
          <div class="admin-tabs" role="tablist" aria-label="Security management">
            <button type="button" class="tab active" id="security-tab-oauth2" role="tab" aria-selected="true" aria-controls="security-content" data-tab="oauth2" data-i18n="admin.sec.tab.oauth">OAuth2 Providers</button>
            <button type="button" class="tab" id="security-tab-saml" role="tab" aria-selected="false" aria-controls="security-content" tabindex="-1" data-tab="saml" data-i18n="admin.sec.tab.saml">SAML Providers</button>
            <button type="button" class="tab" id="security-tab-policies" role="tab" aria-selected="false" aria-controls="security-content" tabindex="-1" data-tab="policies" data-i18n="admin.sec.tab.policies">Security Policies</button>
            <button type="button" class="tab" id="security-tab-rotation" role="tab" aria-selected="false" aria-controls="security-content" tabindex="-1" data-tab="rotation" data-i18n="admin.sec.tab.rotation">Secret rotation</button>
          </div>
          <div id="security-content" role="tabpanel" aria-labelledby="security-tab-oauth2" tabindex="0"></div>
        </div>
        
        <!-- Create OAuth2 Provider Modal -->
        <div id="createOAuth2Modal" class="modal" role="dialog" aria-labelledby="createOAuth2ModalTitle" aria-modal="false" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createOAuth2ModalTitle" data-i18n="admin.sec.oauthCreateTitle">Register OAuth2 Provider</h3>
              <button type="button" class="modal-close" data-i18n-aria="ui.closeDialogAria" onclick="hideModal('createOAuth2Modal')">&times;</button>
            </div>
            <form id="createOAuth2Form" onsubmit="handleCreateOAuth2(event)">
              <div class="form-group">
                <label for="oauth2Name"><span data-i18n="admin.sec.lbl.providerName">Provider Name</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="text" id="oauth2Name" name="name" required data-i18n-placeholder="admin.sec.ph.google" placeholder="google" />
              </div>
              <div class="form-group">
                <label for="oauth2ClientId"><span data-i18n="admin.sec.lbl.clientId">Client ID</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="text" id="oauth2ClientId" name="client_id" required />
              </div>
              <div class="form-group">
                <label for="oauth2ClientSecret"><span data-i18n="admin.sec.lbl.clientSecret">Client Secret</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="password" id="oauth2ClientSecret" name="client_secret" required />
              </div>
              <div class="form-group">
                <label for="oauth2AuthUrl"><span data-i18n="admin.sec.lbl.authUrl">Authorization URL</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="url" id="oauth2AuthUrl" name="authorization_url" required placeholder="https://accounts.google.com/o/oauth2/auth" />
              </div>
              <div class="form-group">
                <label for="oauth2TokenUrl"><span data-i18n="admin.sec.lbl.tokenUrl">Token URL</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="url" id="oauth2TokenUrl" name="token_url" required placeholder="https://oauth2.googleapis.com/token" />
              </div>
              <div class="form-group">
                <label for="oauth2RedirectUri"><span data-i18n="admin.sec.lbl.redirectUri">Redirect URI</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="url" id="oauth2RedirectUri" name="redirect_uri" required placeholder="https://poolai.example.com/callback" />
              </div>
              <div class="form-group">
                <label for="oauth2Scopes" data-i18n="admin.sec.lbl.scopesCsv">Scopes (comma-separated)</label>
                <input type="text" id="oauth2Scopes" name="scopes" placeholder="openid, profile, email" />
              </div>
              <div class="form-group">
                <label for="oauth2Enabled">
                  <input type="checkbox" id="oauth2Enabled" name="enabled" checked />
                  <span data-i18n="admin.mon.enabled">Enabled</span>
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createOAuth2Modal')" data-i18n="ui.cancel">Cancel</button>
                <button type="submit" class="btn btn-primary" data-i18n="ui.register">Register</button>
              </div>
            </form>
          </div>
        </div>
        
        <!-- Edit OAuth2 Provider Modal -->
        <div id="editOAuth2Modal" class="modal" role="dialog" aria-labelledby="editOAuth2ModalTitle" aria-modal="false" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="editOAuth2ModalTitle" data-i18n="admin.sec.oauthEditTitle">Edit OAuth2 Provider</h3>
              <button type="button" class="modal-close" data-i18n-aria="ui.closeDialogAria" onclick="hideModal('editOAuth2Modal')">&times;</button>
            </div>
            <form id="editOAuth2Form" onsubmit="handleEditOAuth2(event)">
              <input type="hidden" id="editOAuth2Name" />
              <div class="form-group">
                <label for="editOAuth2ClientId"><span data-i18n="admin.sec.lbl.clientId">Client ID</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="text" id="editOAuth2ClientId" name="client_id" required />
              </div>
              <div class="form-group">
                <label for="editOAuth2ClientSecret"><span data-i18n="admin.sec.lbl.clientSecret">Client Secret</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="password" id="editOAuth2ClientSecret" name="client_secret" required />
              </div>
              <div class="form-group">
                <label for="editOAuth2AuthUrl"><span data-i18n="admin.sec.lbl.authUrl">Authorization URL</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="url" id="editOAuth2AuthUrl" name="authorization_url" required />
              </div>
              <div class="form-group">
                <label for="editOAuth2TokenUrl"><span data-i18n="admin.sec.lbl.tokenUrl">Token URL</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="url" id="editOAuth2TokenUrl" name="token_url" required />
              </div>
              <div class="form-group">
                <label for="editOAuth2RedirectUri"><span data-i18n="admin.sec.lbl.redirectUri">Redirect URI</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="url" id="editOAuth2RedirectUri" name="redirect_uri" required />
              </div>
              <div class="form-group">
                <label for="editOAuth2Scopes" data-i18n="admin.sec.lbl.scopesCsv">Scopes (comma-separated)</label>
                <input type="text" id="editOAuth2Scopes" name="scopes" />
              </div>
              <div class="form-group">
                <label for="editOAuth2Enabled">
                  <input type="checkbox" id="editOAuth2Enabled" name="enabled" />
                  <span data-i18n="admin.mon.enabled">Enabled</span>
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('editOAuth2Modal')" data-i18n="ui.cancel">Cancel</button>
                <button type="submit" class="btn btn-primary" data-i18n="ui.update">Update</button>
              </div>
            </form>
          </div>
        </div>
        
        <!-- Create SAML Provider Modal -->
        <div id="createSamlModal" class="modal" role="dialog" aria-labelledby="createSamlModalTitle" aria-modal="false" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createSamlModalTitle" data-i18n="admin.sec.samlCreateTitle">Register SAML Provider</h3>
              <button type="button" class="modal-close" data-i18n-aria="ui.closeDialogAria" onclick="hideModal('createSamlModal')">&times;</button>
            </div>
            <form id="createSamlForm" onsubmit="handleCreateSaml(event)">
              <div class="form-group">
                <label for="samlName"><span data-i18n="admin.sec.lbl.providerName">Provider Name</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="text" id="samlName" name="name" required data-i18n-placeholder="admin.sec.ph.okta" placeholder="okta" />
              </div>
              <div class="form-group">
                <label for="samlEntityId"><span data-i18n="admin.sec.lbl.entityId">Entity ID</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="text" id="samlEntityId" name="entity_id" required />
              </div>
              <div class="form-group">
                <label for="samlSsoUrl"><span data-i18n="admin.sec.lbl.ssoUrl">SSO URL</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="url" id="samlSsoUrl" name="sso_url" required />
              </div>
              <div class="form-group">
                <label for="samlSloUrl" data-i18n="admin.sec.lbl.sloUrl">SLO URL (optional)</label>
                <input type="url" id="samlSloUrl" name="slo_url" />
              </div>
              <div class="form-group">
                <label for="samlCertificate"><span data-i18n="admin.sec.lbl.cert">X.509 Certificate</span> <span class="required" aria-hidden="true">*</span></label>
                <textarea id="samlCertificate" name="certificate" required rows="5" data-i18n-placeholder="admin.sec.ph.cert" placeholder="-----BEGIN CERTIFICATE-----..."></textarea>
              </div>
              <div class="form-group">
                <label for="samlAttributeMapping" data-i18n="admin.sec.lbl.attrMap">Attribute Mapping (one per line, format: saml_attribute: user_field)</label>
                <textarea id="samlAttributeMapping" name="attribute_mapping" rows="3" placeholder="email: email&#10;name: username"></textarea>
              </div>
              <div class="form-group">
                <label for="samlEnabled">
                  <input type="checkbox" id="samlEnabled" name="enabled" checked />
                  <span data-i18n="admin.mon.enabled">Enabled</span>
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createSamlModal')" data-i18n="ui.cancel">Cancel</button>
                <button type="submit" class="btn btn-primary" data-i18n="ui.register">Register</button>
              </div>
            </form>
          </div>
        </div>
        
        <!-- Edit SAML Provider Modal -->
        <div id="editSamlModal" class="modal" role="dialog" aria-labelledby="editSamlModalTitle" aria-modal="false" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="editSamlModalTitle" data-i18n="admin.sec.samlEditTitle">Edit SAML Provider</h3>
              <button type="button" class="modal-close" data-i18n-aria="ui.closeDialogAria" onclick="hideModal('editSamlModal')">&times;</button>
            </div>
            <form id="editSamlForm" onsubmit="handleEditSaml(event)">
              <input type="hidden" id="editSamlName" />
              <div class="form-group">
                <label for="editSamlEntityId"><span data-i18n="admin.sec.lbl.entityId">Entity ID</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="text" id="editSamlEntityId" name="entity_id" required />
              </div>
              <div class="form-group">
                <label for="editSamlSsoUrl"><span data-i18n="admin.sec.lbl.ssoUrl">SSO URL</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="url" id="editSamlSsoUrl" name="sso_url" required />
              </div>
              <div class="form-group">
                <label for="editSamlSloUrl" data-i18n="admin.sec.lbl.sloUrl">SLO URL (optional)</label>
                <input type="url" id="editSamlSloUrl" name="slo_url" />
              </div>
              <div class="form-group">
                <label for="editSamlCertificate"><span data-i18n="admin.sec.lbl.cert">X.509 Certificate</span> <span class="required" aria-hidden="true">*</span></label>
                <textarea id="editSamlCertificate" name="certificate" required rows="5"></textarea>
              </div>
              <div class="form-group">
                <label for="editSamlAttributeMapping" data-i18n="admin.sec.lbl.attrMap">Attribute Mapping (one per line, format: saml_attribute: user_field)</label>
                <textarea id="editSamlAttributeMapping" name="attribute_mapping" rows="3"></textarea>
              </div>
              <div class="form-group">
                <label for="editSamlEnabled">
                  <input type="checkbox" id="editSamlEnabled" name="enabled" />
                  <span data-i18n="admin.mon.enabled">Enabled</span>
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('editSamlModal')" data-i18n="ui.cancel">Cancel</button>
                <button type="submit" class="btn btn-primary" data-i18n="ui.update">Update</button>
              </div>
            </form>
          </div>
        </div>
        
        <!-- Create Security Policy Modal -->
        <div id="createPolicyModal" class="modal" role="dialog" aria-labelledby="createPolicyModalTitle" aria-modal="false" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createPolicyModalTitle" data-i18n="admin.sec.policyCreateTitle">Create Security Policy</h3>
              <button type="button" class="modal-close" data-i18n-aria="ui.closeDialogAria" onclick="hideModal('createPolicyModal')">&times;</button>
            </div>
            <form id="createPolicyForm" onsubmit="handleCreatePolicy(event)">
              <div class="form-group">
                <label for="policyName"><span data-i18n="admin.sec.lbl.policyName">Policy Name</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="text" id="policyName" name="name" required data-i18n-placeholder="admin.sec.ph.policy" placeholder="strict-policy" />
              </div>
              <div class="form-group">
                <label for="policyDescription" data-i18n="admin.sec.lbl.policyDesc">Description</label>
                <textarea id="policyDescription" name="description" rows="3" data-i18n-placeholder="admin.sec.ph.policyDesc" placeholder="Strict security policy for admin access"></textarea>
              </div>
              <div class="form-group">
                <label for="policyIpRanges" data-i18n="admin.sec.lbl.ipRanges">Allowed IP Ranges (CIDR, comma-separated)</label>
                <input type="text" id="policyIpRanges" name="ip_ranges" data-i18n-placeholder="admin.sec.ph.ipRanges" placeholder="192.168.1.0/24, 10.0.0.0/8" />
              </div>
              <div class="form-group">
                <label for="policyRequireMfa">
                  <input type="checkbox" id="policyRequireMfa" name="require_mfa" />
                  <span data-i18n="admin.sec.lbl.requireMfa">Require MFA</span>
                </label>
              </div>
              <div class="form-group">
                <label for="policySessionTimeout"><span data-i18n="admin.sec.lbl.sessionTimeout">Session Timeout (seconds)</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="number" id="policySessionTimeout" name="session_timeout" required min="60" value="3600" />
              </div>
              <div class="form-group">
                <label for="policyMaxFailedAttempts"><span data-i18n="admin.sec.lbl.maxFailed">Max Failed Login Attempts</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="number" id="policyMaxFailedAttempts" name="max_failed_attempts" required min="1" value="5" />
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createPolicyModal')" data-i18n="ui.cancel">Cancel</button>
                <button type="submit" class="btn btn-primary" data-i18n="ui.create">Create</button>
              </div>
            </form>
          </div>
        </div>
        
        <!-- Edit Security Policy Modal -->
        <div id="editPolicyModal" class="modal" role="dialog" aria-labelledby="editPolicyModalTitle" aria-modal="false" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="editPolicyModalTitle" data-i18n="admin.sec.policyEditTitle">Edit Security Policy</h3>
              <button type="button" class="modal-close" data-i18n-aria="ui.closeDialogAria" onclick="hideModal('editPolicyModal')">&times;</button>
            </div>
            <form id="editPolicyForm" onsubmit="handleEditPolicy(event)">
              <input type="hidden" id="editPolicyName" />
              <div class="form-group">
                <label for="editPolicyDescription" data-i18n="admin.sec.lbl.policyDesc">Description</label>
                <textarea id="editPolicyDescription" name="description" rows="3"></textarea>
              </div>
              <div class="form-group">
                <label for="editPolicyIpRanges" data-i18n="admin.sec.lbl.ipRanges">Allowed IP Ranges (CIDR, comma-separated)</label>
                <input type="text" id="editPolicyIpRanges" name="ip_ranges" />
              </div>
              <div class="form-group">
                <label for="editPolicyRequireMfa">
                  <input type="checkbox" id="editPolicyRequireMfa" name="require_mfa" />
                  <span data-i18n="admin.sec.lbl.requireMfa">Require MFA</span>
                </label>
              </div>
              <div class="form-group">
                <label for="editPolicySessionTimeout"><span data-i18n="admin.sec.lbl.sessionTimeout">Session Timeout (seconds)</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="number" id="editPolicySessionTimeout" name="session_timeout" required min="60" />
              </div>
              <div class="form-group">
                <label for="editPolicyMaxFailedAttempts"><span data-i18n="admin.sec.lbl.maxFailed">Max Failed Login Attempts</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="number" id="editPolicyMaxFailedAttempts" name="max_failed_attempts" required min="1" />
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('editPolicyModal')" data-i18n="ui.cancel">Cancel</button>
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
async fn admin_security_page_slim_security_i18n_patch_ph_s231() {
    let html = admin_security().await.0;
    assert!(html.contains("window.__poolaiAdminI18nRust="));
    assert!(html.contains(r#""admin.page.security""#));
    assert!(html.contains(r#""admin.sec.tab.oauth""#));
    assert!(!html.contains(r#""admin.jobs.leaseState.active""#));
    assert!(!html.contains(r#""admin.tenants.section""#));
}
