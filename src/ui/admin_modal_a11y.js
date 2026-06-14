/** PH-S161: modal a11y from poolai-ui-core + wasm; DOM glue only (FM-019). */

function poolaiModalConfig() {
  return (
    window.__poolaiAdminModalRust || {
      focusable_selector:
        'a[href], button:not([disabled]), textarea:not([disabled]), input:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex="-1"])',
      dynamic_modal_id: 'adminDynamicModal',
    }
  );
}

function poolaiModalFocusableSelector() {
  const wasm = window.poolaiUiWasm;
  if (wasm && wasm.ready && typeof wasm.modalFocusableSelector === 'function') {
    return wasm.modalFocusableSelector();
  }
  return poolaiModalConfig().focusable_selector;
}

const ADMIN_DYNAMIC_MODAL_ID = poolaiModalConfig().dynamic_modal_id || 'adminDynamicModal';

let activeModal = null;
let activeOverlay = null;
let previousActiveElement = null;

function getModalFocusableElements(modal) {
  return Array.from(modal.querySelectorAll(poolaiModalFocusableSelector())).filter(
    (el) => el.offsetParent !== null || el === document.activeElement,
  );
}

function focusInitialModalElement(modal) {
  const focusable = getModalFocusableElements(modal);
  if (focusable.length > 0) {
    focusable[0].focus();
    return;
  }
  if (!modal.hasAttribute('tabindex')) {
    modal.setAttribute('tabindex', '-1');
  }
  modal.focus();
}

function poolaiTrapTabAction(e, focusable, active) {
  const wasm = window.poolaiUiWasm;
  const count = focusable.length;
  const inside = activeModal.contains(active);
  const first = focusable[0];
  const last = focusable[count - 1];
  if (wasm && wasm.ready && typeof wasm.trapTabAction === 'function') {
    return wasm.trapTabAction(e.shiftKey, count, inside, active === first, active === last);
  }
  if (count === 0) return 'root';
  if (e.shiftKey) {
    return !inside || active === first ? 'last' : 'none';
  }
  return !inside || active === last ? 'first' : 'none';
}

function attachModalA11y(modal) {
  modal.removeEventListener('keydown', trapModalFocus);
  modal.removeEventListener('focusin', keepFocusInModal);
  modal.addEventListener('keydown', trapModalFocus);
  modal.addEventListener('focusin', keepFocusInModal);
  document.removeEventListener('keydown', handleModalEscape);
  document.addEventListener('keydown', handleModalEscape);
}

function detachModalA11y(modal) {
  if (modal) {
    modal.removeEventListener('keydown', trapModalFocus);
    modal.removeEventListener('focusin', keepFocusInModal);
  }
  document.removeEventListener('keydown', handleModalEscape);
}

function showModal(modalIdOrTitle, optionalContent) {
  if (typeof optionalContent === 'string') {
    showModalContent(modalIdOrTitle, optionalContent);
    return;
  }
  const modal = document.getElementById(modalIdOrTitle);
  if (!modal) {
    console.warn('Modal not found:', modalIdOrTitle);
    return;
  }

  if (activeModal && activeModal !== modal) {
    detachModalA11y(activeModal);
    activeModal.setAttribute('aria-hidden', 'true');
    activeModal.setAttribute('aria-modal', 'false');
  }

  previousActiveElement = document.activeElement;
  const overlay = modal.closest('.modal-overlay') || createModalOverlay(modal);

  modal.setAttribute('aria-hidden', 'false');
  modal.setAttribute('aria-modal', 'true');
  overlay.classList.add('active');
  activeModal = modal;
  activeOverlay = overlay;
  document.body.style.overflow = 'hidden';

  attachModalA11y(modal);
  setTimeout(() => {
    focusInitialModalElement(modal);
    adminEnhanceFormA11y(modal);
  }, 100);
}

function hideModal(modalId) {
  const id = modalId || (activeModal && activeModal.id);
  if (!id) return;
  const modal = document.getElementById(id);
  if (!modal) {
    console.warn('Modal not found:', id);
    return;
  }

  modal.setAttribute('aria-hidden', 'true');
  modal.setAttribute('aria-modal', 'false');

  const overlay = modal.closest('.modal-overlay');
  if (overlay) overlay.classList.remove('active');

  detachModalA11y(modal);

  if (previousActiveElement && typeof previousActiveElement.focus === 'function') {
    previousActiveElement.focus();
  }
  previousActiveElement = null;
  activeModal = null;
  activeOverlay = null;
  document.body.style.overflow = '';
}

function ensureAdminDynamicModal() {
  let modal = document.getElementById(ADMIN_DYNAMIC_MODAL_ID);
  if (modal) return modal;
  modal = document.createElement('div');
  modal.id = ADMIN_DYNAMIC_MODAL_ID;
  modal.className = 'modal';
  modal.setAttribute('role', 'dialog');
  modal.setAttribute('aria-labelledby', 'adminDynamicModalTitle');
  modal.setAttribute('aria-modal', 'false');
  modal.setAttribute('aria-hidden', 'true');
  const cfg = poolaiModalConfig();
  const wasm = window.poolaiUiWasm;
  if (wasm && wasm.ready && typeof wasm.adminDynamicModalHtml === 'function') {
    modal.innerHTML = wasm.adminDynamicModalHtml();
  } else if (cfg.dynamic_modal_html) {
    modal.innerHTML = cfg.dynamic_modal_html;
  } else {
    modal.innerHTML =
      '<div class="modal-content"><div class="modal-header"><h3 id="adminDynamicModalTitle"></h3>' +
      '<button type="button" class="modal-close" data-i18n-aria="ui.closeDialogAria" onclick="hideModal(\'' +
      ADMIN_DYNAMIC_MODAL_ID +
      '\')">&times;</button></div><div id="adminDynamicModalBody" class="modal-body"></div></div>';
  }
  document.body.appendChild(modal);
  return modal;
}

function showModalContent(title, bodyHtml) {
  ensureAdminDynamicModal();
  const titleEl = document.getElementById('adminDynamicModalTitle');
  const bodyEl = document.getElementById('adminDynamicModalBody');
  if (titleEl) titleEl.textContent = title;
  if (bodyEl) bodyEl.innerHTML = bodyHtml;
  adminEnhanceFormA11y(document.getElementById(ADMIN_DYNAMIC_MODAL_ID));
  showModal(ADMIN_DYNAMIC_MODAL_ID);
}

function createModalOverlay(modal) {
  let overlay = document.getElementById('modal-overlay-global');
  if (!overlay) {
    overlay = document.createElement('div');
    overlay.id = 'modal-overlay-global';
    overlay.className = 'modal-overlay';
    document.body.appendChild(overlay);
    overlay.addEventListener('click', function (e) {
      if (e.target === overlay && activeModal) hideModal(activeModal.id);
    });
  }
  if (modal.parentElement !== overlay) overlay.appendChild(modal);
  return overlay;
}

function keepFocusInModal(e) {
  if (!activeModal || activeModal.contains(e.target)) return;
  const focusable = getModalFocusableElements(activeModal);
  if (focusable.length > 0) focusable[0].focus();
  else focusInitialModalElement(activeModal);
}

function trapModalFocus(e) {
  if (!activeModal || e.key !== 'Tab') return;
  const focusable = getModalFocusableElements(activeModal);
  const active = document.activeElement;
  const action = poolaiTrapTabAction(e, focusable, active);
  if (action === 'root') {
    e.preventDefault();
    focusInitialModalElement(activeModal);
    return;
  }
  if (action === 'first') {
    e.preventDefault();
    focusable[0].focus();
    return;
  }
  if (action === 'last') {
    e.preventDefault();
    focusable[focusable.length - 1].focus();
  }
}

function handleModalEscape(e) {
  if (e.key === 'Escape' && activeModal) {
    e.preventDefault();
    hideModal(activeModal.id);
  }
}

function adminSyncTabA11y(tablist) {
  if (!tablist) return;
  const panelId = tablist.querySelector('.tab[aria-controls]')?.getAttribute('aria-controls');
  const panel = panelId ? document.getElementById(panelId) : null;
  tablist.querySelectorAll('.tab').forEach((tab) => {
    const selected = tab.classList.contains('active');
    tab.setAttribute('aria-selected', selected ? 'true' : 'false');
    tab.setAttribute('tabindex', selected ? '0' : '-1');
    if (selected && panel && tab.id) panel.setAttribute('aria-labelledby', tab.id);
  });
}

function initTabs() {
  document.querySelectorAll('.admin-tabs').forEach((tablist) => {
    const section = tablist.closest('.admin-section');
    if (!section) return;
    const panel = section.querySelector('#security-content, #config-content');
    if (!panel || !panel.id) return;
    if (!tablist.getAttribute('role')) tablist.setAttribute('role', 'tablist');
    panel.setAttribute('role', 'tabpanel');
    tablist.querySelectorAll('.tab').forEach((tab) => {
      if (!tab.id && tab.dataset.tab) tab.id = panel.id + '-tab-' + tab.dataset.tab;
      if (!tab.getAttribute('role')) tab.setAttribute('role', 'tab');
      if (!tab.getAttribute('aria-controls')) tab.setAttribute('aria-controls', panel.id);
    });
    adminSyncTabA11y(tablist);
  });
}
