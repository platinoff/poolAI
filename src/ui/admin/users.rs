//! User Management page
//!
//! Provides user CRUD operations with role-based access control.

use axum::response::Html;
use crate::ui::admin::admin_layout;

/// User management page
pub async fn admin_users() -> Html<String> {
    let script = r#"
    async function loadUsers() {
      try {
        const users = await fetchJson('/api/v1/users');
        renderUsers(users);
      } catch (e) {
        showNotification('Error loading users: ' + e.message, 'error');
      }
    }
    
    function renderUsers(users) {
      const el = document.getElementById('users-list');
      if (!el) return;
      if (!users || users.length === 0) {
        el.innerHTML = '<div class="muted">No users found</div>';
        return;
      }
      el.innerHTML = `
        <table class="admin-table">
          <thead>
            <tr>
              <th>Username</th>
              <th>Role</th>
              <th>Status</th>
              <th>Created</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            ${users.map(u => `
              <tr>
                <td>${u.username || u.id}</td>
                <td>${u.role || 'Viewer'}</td>
                <td><span class="status-badge ${u.active !== false ? 'active' : 'error'}">${u.active !== false ? 'Active' : 'Inactive'}</span></td>
                <td>${u.created_at ? new Date(u.created_at).toLocaleDateString() : 'N/A'}</td>
                <td>
                  <button class="btn" onclick="editUser('${u.id}')">Edit</button>
                  <button class="btn btn-danger" onclick="deleteUser('${u.id}')">Delete</button>
                </td>
              </tr>
            `).join('')}
          </tbody>
        </table>
      `;
    }
    
    function showCreateUserModal() {
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions. Admin role required.', 'error');
        return;
      }
      showModal('createUserModal');
    }
    
    async function handleCreateUser(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions. Admin role required.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = 'Creating...';
      
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
        
        showNotification('User created successfully', 'success');
        hideModal('createUserModal');
        form.reset();
        loadUsers();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function editUser(id) {
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions. Admin role required.', 'error');
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
        showNotification('Error loading user for edit: ' + e.message, 'error');
      }
    }
    
    async function handleEditUser(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions. Admin role required.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = 'Saving...';
      
      try {
        const id = document.getElementById('editUserId').value;
        const payload = {
          username: document.getElementById('editUserUsername').value,
          role: document.getElementById('editUserRole').value,
          active: document.getElementById('editUserActive').checked
        };
        
        // Only include password if it's provided
        const password = document.getElementById('editUserPassword').value;
        if (password) {
          payload.password = password;
        }
        
        await fetchJson(`/api/v1/users/${id}`, {
          method: 'PUT',
          body: JSON.stringify(payload)
        });
        
        showNotification('User updated successfully', 'success');
        hideModal('editUserModal');
        form.reset();
        loadUsers();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function deleteUser(id) {
      if (!confirm('Are you sure you want to delete this user? This action cannot be undone.')) {
        return;
      }
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions. Admin role required.', 'error');
        return;
      }
      
      try {
        await fetchJson(`/api/v1/users/${id}`, {
          method: 'DELETE'
        });
        showNotification('User deleted successfully', 'success');
        loadUsers();
      } catch (e) {
        showNotification('Error deleting user: ' + e.message, 'error');
      }
    }
    
    loadUsers();
    "#;

    admin_layout(
        "User Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2>Users</h2>
            <button class="btn btn-primary" onclick="showCreateUserModal()" aria-label="Create new user">Create User</button>
          </div>
          <div id="users-list"></div>
        </div>
        
        <!-- Create User Modal -->
        <div id="createUserModal" class="modal" role="dialog" aria-labelledby="createUserModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createUserModalTitle">Create New User</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('createUserModal')">&times;</button>
            </div>
            <form id="createUserForm" onsubmit="handleCreateUser(event)">
              <div class="form-group">
                <label for="userUsername">Username</label>
                <input type="text" id="userUsername" name="username" required placeholder="newuser" />
              </div>
              <div class="form-group">
                <label for="userPassword">Password</label>
                <input type="password" id="userPassword" name="password" required placeholder="Enter password" />
              </div>
              <div class="form-group">
                <label for="userRole">Role</label>
                <select id="userRole" name="role" required>
                  <option value="Admin">Admin</option>
                  <option value="Operator">Operator</option>
                  <option value="Viewer" selected>Viewer</option>
                </select>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createUserModal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Create User</button>
              </div>
            </form>
          </div>
        </div>
        
        <!-- Edit User Modal -->
        <div id="editUserModal" class="modal" role="dialog" aria-labelledby="editUserModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="editUserModalTitle">Edit User</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('editUserModal')">&times;</button>
            </div>
            <form id="editUserForm" onsubmit="handleEditUser(event)">
              <input type="hidden" id="editUserId" name="id" />
              <div class="form-group">
                <label for="editUserUsername">Username</label>
                <input type="text" id="editUserUsername" name="username" required />
              </div>
              <div class="form-group">
                <label for="editUserPassword">New Password (leave empty to keep current)</label>
                <input type="password" id="editUserPassword" name="password" placeholder="Enter new password" />
              </div>
              <div class="form-group">
                <label for="editUserRole">Role</label>
                <select id="editUserRole" name="role" required>
                  <option value="Admin">Admin</option>
                  <option value="Operator">Operator</option>
                  <option value="Viewer">Viewer</option>
                </select>
              </div>
              <div class="form-group">
                <label for="editUserActive">
                  <input type="checkbox" id="editUserActive" name="active" />
                  Active
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('editUserModal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Save Changes</button>
              </div>
            </form>
          </div>
        </div>
        "#,
        script,
    )
}
