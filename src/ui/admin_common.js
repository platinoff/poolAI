// Admin Panel Common JavaScript

// API base URL
const API_BASE = '/api/v1';

// Enterprise API base URL
const ENTERPRISE_API_BASE = '/api/enterprise';

/** Parse API error body: legacy flat `error` string or `{ error: { code, message } }`. */
function apiErrorMessageFromBody(payload) {
  if (!payload || typeof payload !== 'object') return null;
  const e = payload.error;
  if (typeof e === 'string') return e;
  if (e && typeof e === 'object' && typeof e.message === 'string') return e.message;
  if (typeof payload.message === 'string') return payload.message;
  return null;
}

// Utility functions
// Compatible with main UI module storage format
function getUser() {
  // Try new format first (poolai_user, poolai_role)
  const username = localStorage.getItem('poolai_user');
  const role = localStorage.getItem('poolai_role');
  const token = localStorage.getItem('poolai_token');
  
  if (username && role) {
    return { username, role, token };
  }
  
  // Fallback to old format (user JSON)
  const userStr = localStorage.getItem('user');
  if (!userStr) return null;
  try {
    return JSON.parse(userStr);
  } catch (e) {
    return null;
  }
}

function setUser(user) {
  if (typeof user === 'string') {
    // Legacy format: setUser(username, role) - called from login
    const role = arguments[1] || 'Viewer';
    localStorage.setItem('poolai_user', user);
    localStorage.setItem('poolai_role', role);
  } else if (user && typeof user === 'object') {
    // New format: setUser({username, role, token})
    if (user.username) localStorage.setItem('poolai_user', user.username);
    if (user.role) localStorage.setItem('poolai_role', user.role);
    if (user.token) localStorage.setItem('poolai_token', user.token);
    // Also store in old format for compatibility
    localStorage.setItem('user', JSON.stringify(user));
  }
}

function clearUser() {
  localStorage.removeItem('user');
  localStorage.removeItem('poolai_user');
  localStorage.removeItem('poolai_role');
  localStorage.removeItem('poolai_token');
  localStorage.removeItem('poolai_token_exp');
}

function isAdmin() {
  const user = getUser();
  return user && (user.role === 'Admin' || user.role === 'admin');
}

function requireAdmin() {
  const user = getUser();
  if (!user) {
    window.location.href = '/ui/auth';
    return false;
  }
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
  const token = user?.token || localStorage.getItem('poolai_token');
  const headers = {
    'Content-Type': 'application/json',
    ...options.headers,
  };
  
  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }
  
  const response = await fetch(url, {
    ...options,
    headers,
  });
  
  if (response.status === 401) {
    // Unauthorized - redirect to login
    clearUser();
    window.location.href = '/ui/auth';
    throw new Error('Unauthorized');
  }
  
  if (!response.ok) {
    const error = await response.json().catch(() => ({ error: response.statusText }));
    throw new Error(
      apiErrorMessageFromBody(error) || error.message || `HTTP ${response.status}`,
    );
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

// Modal system - Enhanced with overlay support and accessibility
let activeModal = null;
let activeOverlay = null;
let previousActiveElement = null;

function showModal(modalId) {
  const modal = document.getElementById(modalId);
  if (!modal) {
    console.warn('Modal not found:', modalId);
    return;
  }
  
  // Store previous active element for focus restoration
  previousActiveElement = document.activeElement;
  
  // Check if modal is already in an overlay
  let overlay = modal.closest('.modal-overlay');
  
  // Create overlay if it doesn't exist
  if (!overlay) {
    overlay = createModalOverlay(modal);
  }
  
  // Set ARIA attributes
  modal.setAttribute('aria-hidden', 'false');
  modal.setAttribute('aria-modal', 'true');
  overlay.classList.add('active');
  
  activeModal = modal;
  activeOverlay = overlay;
  
  // Prevent body scroll
  document.body.style.overflow = 'hidden';
  
  // Focus first focusable element after a short delay
  setTimeout(() => {
    const focusableElements = modal.querySelectorAll(
      'a[href], button:not([disabled]), textarea:not([disabled]), input:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex="-1"])'
    );
    if (focusableElements.length > 0) {
      focusableElements[0].focus();
    }
  }, 100);
  
  // Trap focus within modal
  modal.addEventListener('keydown', trapModalFocus);
  
  // Close on Escape key (global listener already handled in main UI)
  document.addEventListener('keydown', handleModalEscape);
}

function hideModal(modalId) {
  const modal = document.getElementById(modalId);
  if (!modal) {
    console.warn('Modal not found:', modalId);
    return;
  }
  
  // Remove ARIA attributes
  modal.setAttribute('aria-hidden', 'true');
  modal.setAttribute('aria-modal', 'false');
  
  // Hide overlay
  const overlay = modal.closest('.modal-overlay');
  if (overlay) {
    overlay.classList.remove('active');
  }
  
  // Remove focus trap
  if (activeModal) {
    activeModal.removeEventListener('keydown', trapModalFocus);
  }
  
  document.removeEventListener('keydown', handleModalEscape);
  
  // Restore previous focus
  if (previousActiveElement) {
    previousActiveElement.focus();
    previousActiveElement = null;
  }
  
  activeModal = null;
  activeOverlay = null;
  
  // Restore body scroll
  document.body.style.overflow = '';
}

function createModalOverlay(modal) {
  // Check if overlay already exists
  let overlay = document.getElementById('modal-overlay-global');
  if (!overlay) {
    overlay = document.createElement('div');
    overlay.id = 'modal-overlay-global';
    overlay.className = 'modal-overlay';
    document.body.appendChild(overlay);
    
    // Close modal on backdrop click
    overlay.addEventListener('click', function(e) {
      if (e.target === overlay && activeModal) {
        hideModal(activeModal.id);
      }
    });
  }
  
  // Move modal to overlay if not already there
  if (modal.parentElement !== overlay) {
    overlay.appendChild(modal);
  }
  
  return overlay;
}

function trapModalFocus(e) {
  if (!activeModal || e.key !== 'Tab') return;
  
  const focusableElements = Array.from(activeModal.querySelectorAll(
    'a[href], button:not([disabled]), textarea:not([disabled]), input:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex="-1"])'
  ));
  
  if (focusableElements.length === 0) return;
  
  const firstElement = focusableElements[0];
  const lastElement = focusableElements[focusableElements.length - 1];
  
  if (e.shiftKey) {
    // Shift + Tab
    if (document.activeElement === firstElement) {
      e.preventDefault();
      lastElement.focus();
    }
  } else {
    // Tab
    if (document.activeElement === lastElement) {
      e.preventDefault();
      firstElement.focus();
    }
  }
}

function handleModalEscape(e) {
  if (e.key === 'Escape' && activeModal) {
    hideModal(activeModal.id);
  }
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
