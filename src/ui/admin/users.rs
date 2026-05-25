//! User Management page
//!
//! Provides user CRUD operations with role-based access control.

use crate::ui::admin::admin_layout;
use axum::response::Html;

/// User management page
pub async fn admin_users() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }
    function Ep() { return typeof poolaiT === 'function' ? poolaiT('err.errorPrefix', 'Error: ') : 'Error: '; }

    async function loadUsers() {
      adminShowLoading('users-list', T('admin.usr.loading', 'Loading users…'));
      try {
        const users = await fetchJson('/api/v1/users');
        renderUsers(users);
      } catch (e) {
        adminShowInlineError('users-list', e);
        showNotification(T('admin.usr.errLoad', 'Error loading users: ') + e.message, 'error');
      }
    }
    
    function renderUsers(users) {
      const el = document.getElementById('users-list');
      if (!el) return;
      if (!users || users.length === 0) {
        el.innerHTML = '<div class="muted">' + escapeHtml(T('admin.usr.empty', 'No users found')) + '</div>';
        return;
      }
      el.innerHTML = `
        <table class="admin-table">
          <thead>
            <tr>
              <th>${escapeHtml(T('admin.usr.col.user', 'Username'))}</th>
              <th>${escapeHtml(T('admin.usr.col.role', 'Role'))}</th>
              <th>${escapeHtml(T('admin.usr.col.status', 'Status'))}</th>
              <th>${escapeHtml(T('admin.usr.col.created', 'Created'))}</th>
              <th>${escapeHtml(T('admin.usr.col.actions', 'Actions'))}</th>
            </tr>
          </thead>
          <tbody>
            ${users.map(u => `
              <tr>
                <td>${escapeHtml(u.username || u.id)}</td>
                <td>${escapeHtml(u.role || 'Viewer')}</td>
                <td><span class="status-badge ${u.active !== false ? 'active' : 'error'}">${u.active !== false ? escapeHtml(T('admin.status.active', 'Active')) : escapeHtml(T('admin.status.inactive', 'Inactive'))}</span></td>
                <td>${u.created_at ? escapeHtml(new Date(u.created_at).toLocaleDateString()) : escapeHtml(T('admin.na', 'N/A'))}</td>
                <td>
                  <button type="button" class="btn" onclick='editUser(${JSON.stringify(u.id)})'>${escapeHtml(T('admin.btn.edit', 'Edit'))}</button>
                  <button type="button" class="btn btn-danger" onclick='deleteUser(${JSON.stringify(u.id)})'>${escapeHtml(T('ui.delete', 'Delete'))}</button>
                </td>
              </tr>
            `).join('')}
          </tbody>
        </table>
      `;
    }
    
    function showCreateUserModal() {
      const user = getUser();
      if (!isAdmin()) {
        showNotification(T('err.insufficientAdmin', 'Insufficient permissions. Admin role required.'), 'error');
        return;
      }
      showModal('createUserModal');
    }
    
    async function handleCreateUser(event) {
      event.preventDefault();
      const user = getUser();
      if (!isAdmin()) {
        showNotification(T('err.insufficientAdmin', 'Insufficient permissions. Admin role required.'), 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = T('admin.usr.creating', 'Creating…');
      
      try {
        const payload = {
          username: document.getElementById('userUsername').value,
          password: document.getElementById('userPassword').value,
          role: document.getElementById('userRole').value
        };
        
        await fetchJson('/api/v1/users', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification(T('admin.usr.createdOk', 'User created successfully'), 'success');
        hideModal('createUserModal');
        form.reset();
        loadUsers();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function editUser(id) {
      const user = getUser();
      if (!isAdmin()) {
        showNotification(T('err.insufficientAdmin', 'Insufficient permissions. Admin role required.'), 'error');
        return;
      }
      
      try {
        const userData = await fetchJson(`/api/v1/users/${id}`);
        document.getElementById('editUserId').value = userData.id;
        document.getElementById('editUserUsername').value = userData.username;
        document.getElementById('editUserRole').value = userData.role;
        document.getElementById('editUserActive').checked = userData.active !== false;
        showModal('editUserModal');
      } catch (e) {
        showNotification(T('admin.usr.loadEditErr', 'Error loading user for edit: ') + e.message, 'error');
      }
    }
    
    async function handleEditUser(event) {
      event.preventDefault();
      const user = getUser();
      if (!isAdmin()) {
        showNotification(T('err.insufficientAdmin', 'Insufficient permissions. Admin role required.'), 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = T('admin.usr.saving', 'Saving…');
      
      try {
        const id = document.getElementById('editUserId').value;
        const payload = {
          username: document.getElementById('editUserUsername').value,
          role: document.getElementById('editUserRole').value,
          active: document.getElementById('editUserActive').checked
        };
        
        const password = document.getElementById('editUserPassword').value;
        if (password) {
          payload.password = password;
        }
        
        await fetchJson(`/api/v1/users/${id}`, {
          method: 'PUT',
          body: JSON.stringify(payload)
        });
        
        showNotification(T('admin.usr.updatedOk', 'User updated successfully'), 'success');
        const editedName = document.getElementById('editUserUsername').value;
        const newPwd = document.getElementById('editUserPassword').value;
        if (editedName === 'admin' && newPwd) {
          try {
            localStorage.removeItem('poolai_bootstrap_admin_show');
            localStorage.setItem('poolai_bootstrap_admin_ack', '1');
          } catch (e) {}
          var bh = document.getElementById('poolai-bootstrap-banner-host');
          if (bh) {
            bh.setAttribute('hidden', '');
            bh.innerHTML = '';
          }
        }
        hideModal('editUserModal');
        form.reset();
        loadUsers();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function deleteUser(id) {
      if (!confirm(T('admin.usr.confirmDel', 'Are you sure you want to delete this user? This action cannot be undone.'))) {
        return;
      }
      const user = getUser();
      if (!isAdmin()) {
        showNotification(T('err.insufficientAdmin', 'Insufficient permissions. Admin role required.'), 'error');
        return;
      }
      
      try {
        await fetchJson(`/api/v1/users/${id}`, {
          method: 'DELETE'
        });
        showNotification(T('admin.usr.deletedOk', 'User deleted successfully'), 'success');
        loadUsers();
      } catch (e) {
        showNotification(T('admin.usr.errDel', 'Error deleting user: ') + e.message, 'error');
      }
    }
    
    loadUsers();
    "#;

    admin_layout(
        "admin.page.users",
        "User Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2 data-i18n="admin.usr.section">Users</h2>
            <button type="button" class="btn btn-primary" onclick="showCreateUserModal()" data-i18n="admin.usr.createBtn" data-i18n-aria="admin.usr.createBtn">Create User</button>
          </div>
          <div id="users-list"></div>
        </div>
        
        <div id="createUserModal" class="modal" role="dialog" aria-labelledby="createUserModalTitle" aria-modal="false" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createUserModalTitle" data-i18n="admin.usr.createTitle">Create New User</h3>
              <button type="button" class="modal-close" data-i18n-aria="ui.closeDialogAria" onclick="hideModal('createUserModal')">&times;</button>
            </div>
            <form id="createUserForm" onsubmit="handleCreateUser(event)">
              <div class="form-group">
                <label for="userUsername"><span data-i18n="admin.usr.label.user">Username</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="text" id="userUsername" name="username" required aria-required="true" autocomplete="username" data-i18n-placeholder="admin.usr.ph.user" placeholder="newuser" />
              </div>
              <div class="form-group">
                <label for="userPassword"><span data-i18n="admin.usr.label.pw">Password</span> <span class="required" aria-hidden="true">*</span></label>
                <input type="password" id="userPassword" name="password" required aria-required="true" autocomplete="new-password" data-i18n-placeholder="admin.usr.ph.pw" placeholder="Enter password" />
              </div>
              <div class="form-group">
                <label for="userRole"><span data-i18n="admin.usr.label.role">Role</span> <span class="required" aria-hidden="true">*</span></label>
                <select id="userRole" name="role" required aria-required="true">
                  <option value="Admin">Admin</option>
                  <option value="Operator">Operator</option>
                  <option value="Viewer" selected>Viewer</option>
                </select>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createUserModal')" data-i18n="ui.cancel">Cancel</button>
                <button type="submit" class="btn btn-primary" data-i18n="admin.usr.createSubmit">Create User</button>
              </div>
            </form>
          </div>
        </div>
        
        <div id="editUserModal" class="modal" role="dialog" aria-labelledby="editUserModalTitle" aria-modal="false" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="editUserModalTitle" data-i18n="admin.usr.editTitle">Edit User</h3>
              <button type="button" class="modal-close" data-i18n-aria="ui.closeDialogAria" onclick="hideModal('editUserModal')">&times;</button>
            </div>
            <form id="editUserForm" onsubmit="handleEditUser(event)">
              <input type="hidden" id="editUserId" name="id" />
              <div class="form-group">
                <label for="editUserUsername" data-i18n="admin.usr.label.user">Username</label>
                <input type="text" id="editUserUsername" name="username" required aria-required="true" autocomplete="username" />
              </div>
              <div class="form-group">
                <label for="editUserPassword" data-i18n="admin.usr.label.pwNew">New Password (leave empty to keep current)</label>
                <input type="password" id="editUserPassword" name="password" autocomplete="new-password" data-i18n-placeholder="admin.usr.ph.pwNew" placeholder="Enter new password" />
              </div>
              <div class="form-group">
                <label for="editUserRole" data-i18n="admin.usr.label.role">Role</label>
                <select id="editUserRole" name="role" required aria-required="true">
                  <option value="Admin">Admin</option>
                  <option value="Operator">Operator</option>
                  <option value="Viewer">Viewer</option>
                </select>
              </div>
              <div class="form-group">
                <label for="editUserActive">
                  <input type="checkbox" id="editUserActive" name="active" />
                  <span data-i18n="admin.status.active">Active</span>
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('editUserModal')" data-i18n="ui.cancel">Cancel</button>
                <button type="submit" class="btn btn-primary" data-i18n="ui.save">Save Changes</button>
              </div>
            </form>
          </div>
        </div>
        "#,
        script,
    )
}
