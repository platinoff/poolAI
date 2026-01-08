// Admin Panel Common JavaScript

// API base URL
const API_BASE = '/api/v1';

// Enterprise API base URL
const ENTERPRISE_API_BASE = '/api/enterprise';

// Utility functions
function getUser() {
  const userStr = localStorage.getItem('user');
  if (!userStr) return null;
  try {
    return JSON.parse(userStr);
  } catch (e) {
    return null;
  }
}

function setUser(user) {
  localStorage.setItem('user', JSON.stringify(user));
}

function clearUser() {
  localStorage.removeItem('user');
}

function isAdmin() {
  const user = getUser();
  return user && user.role === 'Admin';
}

function requireAdmin() {
  if (!isAdmin()) {
    alert('Admin access required');
    window.location.href = '/ui/auth';
    return false;
  }
  return true;
}

// API helpers
async function fetchJson(url, options = {}) {
  const user = getUser();
  const headers = {
    'Content-Type': 'application/json',
    ...options.headers,
  };
  
  if (user && user.token) {
    headers['Authorization'] = `Bearer ${user.token}`;
  }
  
  const response = await fetch(url, {
    ...options,
    headers,
  });
  
  if (!response.ok) {
    const error = await response.json().catch(() => ({ error: response.statusText }));
    throw new Error(error.error || error.message || `HTTP ${response.status}`);
  }
  
  return response.json();
}

// Notification system
function showNotification(message, type = 'info') {
  const notification = document.createElement('div');
  notification.className = `notification notification-${type}`;
  notification.textContent = message;
  notification.style.cssText = `
    position: fixed;
    top: 20px;
    right: 20px;
    padding: 15px 20px;
    background: ${type === 'error' ? '#ff5555' : type === 'success' ? '#67e480' : '#8be9fd'};
    color: ${type === 'error' ? 'white' : '#0f1216'};
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0,0,0,0.3);
    z-index: 3000;
    animation: slideIn 0.3s ease-out;
  `;
  
  document.body.appendChild(notification);
  
  setTimeout(() => {
    notification.style.animation = 'slideOut 0.3s ease-out';
    setTimeout(() => notification.remove(), 300);
  }, 3000);
}

// Modal system
function showModal(modalId) {
  const modal = document.getElementById(modalId);
  if (modal) {
    const overlay = modal.closest('.modal-overlay') || createModalOverlay(modal);
    overlay.classList.add('active');
  }
}

function hideModal(modalId) {
  const modal = document.getElementById(modalId);
  if (modal) {
    const overlay = modal.closest('.modal-overlay');
    if (overlay) {
      overlay.classList.remove('active');
    }
  }
}

function createModalOverlay(modal) {
  const overlay = document.createElement('div');
  overlay.className = 'modal-overlay';
  overlay.onclick = (e) => {
    if (e.target === overlay) {
      overlay.classList.remove('active');
    }
  };
  modal.parentNode.insertBefore(overlay, modal);
  overlay.appendChild(modal);
  return overlay;
}

// Tab system
function initTabs() {
  document.querySelectorAll('.tab').forEach(tab => {
    tab.addEventListener('click', () => {
      const tabGroup = tab.closest('.admin-tabs');
      const contentId = tab.dataset.tab;
      
      // Update active tab
      tabGroup.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
      tab.classList.add('active');
      
      // Show corresponding content
      const contentArea = document.getElementById('security-content') || 
                         document.getElementById('config-content');
      if (contentArea) {
        contentArea.setAttribute('data-active-tab', contentId);
      }
    });
  });
}

// Logout
function logout() {
  clearUser();
  window.location.href = '/ui/auth';
}

// Initialize on page load
document.addEventListener('DOMContentLoaded', () => {
  // Check admin access
  if (!requireAdmin()) return;
  
  // Initialize tabs
  initTabs();
  
  // Set active nav item
  const currentPath = window.location.pathname;
  document.querySelectorAll('.admin-nav-item').forEach(item => {
    if (item.getAttribute('href') === currentPath) {
      item.classList.add('active');
    }
  });
  
  // Set user name
  const user = getUser();
  if (user) {
    const userNameEl = document.getElementById('admin-user-name');
    if (userNameEl) {
      userNameEl.textContent = user.username || 'Admin';
    }
  }
});

// Add CSS animations
const style = document.createElement('style');
style.textContent = `
  @keyframes slideIn {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }
  
  @keyframes slideOut {
    from {
      transform: translateX(0);
      opacity: 1;
    }
    to {
      transform: translateX(100%);
      opacity: 0;
    }
  }
`;
document.head.appendChild(style);
