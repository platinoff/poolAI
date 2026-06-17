/**
 * PoolAI UI i18n (FM-012): Ukrainian / English via localStorage `poolai_ui_lang` (`en` | `uk`).
 * Exposes: PoolAiI18n.{ getLang, setLang, t, apply, initAdminShell, initAuthPage, initDashboardShell }
 */
(function () {
  'use strict';

  var STORAGE_KEY = 'poolai_ui_lang';

  var STRINGS = {
    en: {
      // auth.* + dash.* + admin.nav.* + admin chrome shell → poolai-ui-core i18n.rs (PH-S162, PH-S242, PH-S243)
      // home.* → admin_home_patch (PH-S258)
      // ui.save / ui.search* / ui.retry* → admin_ui_toolbar_patch (PH-S260)

      'ui.upload': 'Upload',
      'ui.register': 'Register',

      'common.loading': 'Loading…',
      'common.unauthorized': 'Unauthorized — session expired. Please sign in again.',

      'ui.create': 'Create',
      'ui.install': 'Install',
      'ui.delete': 'Delete',
      'ui.update': 'Update',
      'ui.uninstall': 'Uninstall',
      'ui.closeNotificationAria': 'Close notification',
      'ui.clearSearchAria': 'Clear search',
      'ui.stepOfTotal': 'Step {current} of {total}',
      'ui.tabsAria': 'Tabs',
      'ui.suggestion.checkInternet': 'Check your internet connection',
      'ui.suggestion.verifyServer': 'Verify the server is running',
      'ui.suggestion.refreshPage': 'Try refreshing the page',
      'ui.suggestion.contactSupport': 'Contact support if the problem persists',
      'ui.noItems': 'No items.',
      'ui.sourceLabel': 'Source:',
      'ui.artifactsLabel': 'Artifacts:',
      'ui.nodesLabel': 'Nodes:',
      'ui.creating': 'Creating…',
      'ui.installing': 'Installing…',
      'ui.deleting': 'Deleting…',

      // form.* + residual err.* → poolai-ui-core (PH-S259)

      'libs.empty': 'No libraries installed.',
      'libs.col.name': 'Name',
      'libs.col.version': 'Version',
      'libs.col.type': 'Type',
      'libs.col.status': 'Status',
      'libs.col.actions': 'Actions',
      'libs.updateAria': 'Update library {name}',
      'libs.uninstallAria': 'Uninstall library {name}',
      'libs.installBtn': 'Install Library',
      'libs.modalTitle': 'Install Library',
      'libs.label.name': 'Library name',
      'libs.label.version': 'Version (optional, defaults to latest)',
      'libs.ph.name': 'libtorch',
      'libs.ph.version': '1.13.0',
      'libs.installingSubmit': 'Installing…',
      'libs.installedOk': 'Library installed successfully',
      'libs.uninstalledOk': 'Library uninstalled successfully',
      'libs.uninstallingLoad': 'Uninstalling library…',
      'libs.updatingLoad': 'Updating library…',
      'libs.processingLoad': 'Processing library…',
      'libs.updatedOk': 'Library updated successfully',
      'libs.processingOk': 'Operation completed successfully',
      'libs.confirmUninstall':
        'Are you sure you want to uninstall library "{name}"? This action cannot be undone.',

      'raid.empty': 'No artifacts stored.',
      'raid.col.id': 'ID',
      'raid.col.name': 'Name',
      'raid.col.storedAt': 'Stored at',
      'raid.col.actions': 'Actions',
      'raid.createBtn': 'Create Artifact',
      'raid.createBtnAria': 'Create new artifact',
      'raid.modalTitle': 'Create Artifact',
      'raid.label.name': 'Artifact name',
      'raid.label.file': 'File',
      'raid.ph.name': 'my-artifact',
      'raid.sectionArtifacts': 'Artifacts:',
      'raid.sectionNodes': 'Nodes:',
      'raid.deleteAria': 'Delete artifact {name}',
      'raid.creatingSubmit': 'Creating…',
      'raid.createdOk': 'Artifact created successfully',
      'raid.deletedOk': 'Artifact deleted successfully',
      'raid.deletingLoad': 'Deleting artifact…',
      'raid.confirmDelete':
        'Are you sure you want to delete artifact "{name}" ({id})? This action cannot be undone.',
    },
    uk: {
      // auth.* + dash.* + admin.nav.* + admin chrome shell → poolai-ui-core i18n.rs (PH-S162, PH-S242, PH-S243)
      // home.* → admin_home_patch (PH-S258)
      // ui.save / ui.search* / ui.retry* → admin_ui_toolbar_patch (PH-S260)

      'ui.upload': 'Завантажити',
      'ui.register': 'Зареєструвати',

      'common.loading': 'Завантаження…',
      'common.unauthorized': 'Неавторизовано — сесію завершено. Увійдіть знову.',

      'ui.create': 'Створити',
      'ui.install': 'Встановити',
      'ui.delete': 'Видалити',
      'ui.update': 'Оновити',
      'ui.uninstall': 'Видалити (бібліотеку)',
      'ui.closeNotificationAria': 'Закрити сповіщення',
      'ui.clearSearchAria': 'Очистити пошук',
      'ui.stepOfTotal': 'Крок {current} з {total}',
      'ui.tabsAria': 'Вкладки',
      'ui.suggestion.checkInternet': 'Перевірте підключення до інтернету',
      'ui.suggestion.verifyServer': 'Переконайтеся, що сервер запущено',
      'ui.suggestion.refreshPage': 'Спробуйте оновити сторінку',
      'ui.suggestion.contactSupport': 'Зверніться до підтримки, якщо проблема не зникає',
      'ui.noItems': 'Немає записів.',
      'ui.sourceLabel': 'Джерело:',
      'ui.artifactsLabel': 'Артефакти:',
      'ui.nodesLabel': 'Вузли:',
      'ui.creating': 'Створення…',
      'ui.installing': 'Встановлення…',
      'ui.deleting': 'Видалення…',

      // form.* + residual err.* + workers.* → poolai-ui-core (PH-S257…S259)

      'libs.empty': 'Бібліотеки не встановлено.',
      'libs.col.name': 'Назва',
      'libs.col.version': 'Версія',
      'libs.col.type': 'Тип',
      'libs.col.status': 'Статус',
      'libs.col.actions': 'Дії',
      'libs.updateAria': 'Оновити бібліотеку {name}',
      'libs.uninstallAria': 'Видалити бібліотеку {name}',
      'libs.installBtn': 'Встановити бібліотеку',
      'libs.modalTitle': 'Встановити бібліотеку',
      'libs.label.name': 'Назва бібліотеки',
      'libs.label.version': 'Версія (необов’язково, за замовчуванням остання)',
      'libs.ph.name': 'libtorch',
      'libs.ph.version': '1.13.0',
      'libs.installingSubmit': 'Встановлення…',
      'libs.installedOk': 'Бібліотеку встановлено',
      'libs.uninstalledOk': 'Бібліотеку видалено',
      'libs.uninstallingLoad': 'Видалення бібліотеки…',
      'libs.updatingLoad': 'Оновлення бібліотеки…',
      'libs.processingLoad': 'Обробка бібліотеки…',
      'libs.updatedOk': 'Бібліотеку оновлено',
      'libs.processingOk': 'Операцію виконано',
      'libs.confirmUninstall':
        'Видалити бібліотеку «{name}»? Цю дію не можна скасувати.',

      'raid.empty': 'Артефактів не збережено.',
      'raid.col.id': 'ID',
      'raid.col.name': 'Назва',
      'raid.col.storedAt': 'Збережено',
      'raid.col.actions': 'Дії',
      'raid.createBtn': 'Створити артефакт',
      'raid.createBtnAria': 'Створити новий артефакт',
      'raid.modalTitle': 'Створити артефакт',
      'raid.label.name': 'Назва артефакту',
      'raid.label.file': 'Файл',
      'raid.ph.name': 'my-artifact',
      'raid.sectionArtifacts': 'Артефакти:',
      'raid.sectionNodes': 'Вузли:',
      'raid.deleteAria': 'Видалити артефакт {name}',
      'raid.creatingSubmit': 'Створення…',
      'raid.createdOk': 'Артефакт створено',
      'raid.deletedOk': 'Артефакт видалено',
      'raid.deletingLoad': 'Видалення артефакту…',
      'raid.confirmDelete':
        'Видалити артефакт «{name}» ({id})? Цю дію не можна скасувати.',
    },
  };

  // PH-S154: admin.jobs + admin.gridPricing EN/UK → poolai-ui-core i18n.rs (admin_layout inject).
  // PH-S197: admin.updatesCompat → poolai-ui-core i18n.rs.
  // PH-S207: admin.mon + admin.page.monitoring → poolai-ui-core i18n.rs.
  var rustAdmin = typeof window !== 'undefined' && window.__poolaiAdminI18nRust;
  if (rustAdmin) {
    if (rustAdmin.en) Object.assign(STRINGS.en, rustAdmin.en);
    if (rustAdmin.uk) Object.assign(STRINGS.uk, rustAdmin.uk);
  }

  // PH-S162: auth + dash shell EN/UK → poolai-ui-core i18n.rs (layout + login inject).
  // PH-S242: admin.nav.*; PH-S243: admin chrome keys — same auth_dash patch (not in STRINGS core).
  var rustAuthDash = typeof window !== 'undefined' && window.__poolaiAuthDashI18nRust;
  if (rustAuthDash) {
    if (rustAuthDash.en) Object.assign(STRINGS.en, rustAuthDash.en);
    if (rustAuthDash.uk) Object.assign(STRINGS.uk, rustAuthDash.uk);
  }

  // PH-S240: admin.table toolbar EN/UK → poolai-ui-core i18n.rs (all admin layouts inject).
  var rustAdminTable = typeof window !== 'undefined' && window.__poolaiAdminTableI18nRust;
  if (rustAdminTable) {
    if (rustAdminTable.en) Object.assign(STRINGS.en, rustAdminTable.en);
    if (rustAdminTable.uk) Object.assign(STRINGS.uk, rustAdminTable.uk);
  }

  // PH-S245: admin.status.* / admin.na / admin.btn.edit → poolai-ui-core i18n.rs (all admin layouts).
  var rustAdminStatus = typeof window !== 'undefined' && window.__poolaiAdminStatusI18nRust;
  if (rustAdminStatus) {
    if (rustAdminStatus.en) Object.assign(STRINGS.en, rustAdminStatus.en);
    if (rustAdminStatus.uk) Object.assign(STRINGS.uk, rustAdminStatus.uk);
  }

  // PH-S246: err.hint* / err.insufficientAdmin / admin.accessRequired → poolai-ui-core i18n.rs.
  var rustAdminErr = typeof window !== 'undefined' && window.__poolaiAdminErrI18nRust;
  if (rustAdminErr) {
    if (rustAdminErr.en) Object.assign(STRINGS.en, rustAdminErr.en);
    if (rustAdminErr.uk) Object.assign(STRINGS.uk, rustAdminErr.uk);
  }

  // PH-S248: vm.* modal EN/UK → poolai-ui-core i18n.rs (admin + dashboard VM pages).
  var rustVmModal = typeof window !== 'undefined' && window.__poolaiVmModalI18nRust;
  if (rustVmModal) {
    if (rustVmModal.en) Object.assign(STRINGS.en, rustVmModal.en);
    if (rustVmModal.uk) Object.assign(STRINGS.uk, rustVmModal.uk);
  }

  // PH-S252: ui.confirm* modal glue → poolai-ui-core i18n.rs (admin + dashboard shells).
  var rustUiConfirm = typeof window !== 'undefined' && window.__poolaiAdminUiConfirmI18nRust;
  if (rustUiConfirm) {
    if (rustUiConfirm.en) Object.assign(STRINGS.en, rustUiConfirm.en);
    if (rustUiConfirm.uk) Object.assign(STRINGS.uk, rustUiConfirm.uk);
  }

  // PH-S257: workers.* panel → poolai-ui-core i18n.rs (admin workers + dashboard /ui/workers).
  var rustWorkersPanel = typeof window !== 'undefined' && window.__poolaiWorkersPanelI18nRust;
  if (rustWorkersPanel) {
    if (rustWorkersPanel.en) Object.assign(STRINGS.en, rustWorkersPanel.en);
    if (rustWorkersPanel.uk) Object.assign(STRINGS.uk, rustWorkersPanel.uk);
  }

  // PH-S258: home.* shell → poolai-ui-core i18n.rs (dashboard home).
  var rustHome = typeof window !== 'undefined' && window.__poolaiHomeI18nRust;
  if (rustHome) {
    if (rustHome.en) Object.assign(STRINGS.en, rustHome.en);
    if (rustHome.uk) Object.assign(STRINGS.uk, rustHome.uk);
  }

  // PH-S259: form.* validation → poolai-ui-core i18n.rs (admin + dashboard).
  var rustForm = typeof window !== 'undefined' && window.__poolaiAdminFormI18nRust;
  if (rustForm) {
    if (rustForm.en) Object.assign(STRINGS.en, rustForm.en);
    if (rustForm.uk) Object.assign(STRINGS.uk, rustForm.uk);
  }

  // PH-S260: ui.save / ui.search* / ui.retry* toolbar glue → poolai-ui-core i18n.rs.
  var rustUiToolbar = typeof window !== 'undefined' && window.__poolaiAdminUiToolbarI18nRust;
  if (rustUiToolbar) {
    if (rustUiToolbar.en) Object.assign(STRINGS.en, rustUiToolbar.en);
    if (rustUiToolbar.uk) Object.assign(STRINGS.uk, rustUiToolbar.uk);
  }

  function normalizeLang(l) {
    if (!l) return 'en';
    var x = String(l).toLowerCase();
    if (x === 'uk' || x === 'ua') return 'uk';
    return 'en';
  }

  function getLang() {
    try {
      return normalizeLang(localStorage.getItem(STORAGE_KEY));
    } catch (e) {
      return 'en';
    }
  }

  function setLang(lang) {
    try {
      localStorage.setItem(STORAGE_KEY, normalizeLang(lang));
    } catch (e) {}
    document.documentElement.lang = getLang() === 'uk' ? 'uk' : 'en';
    apply(document.documentElement);
    document.dispatchEvent(new CustomEvent('poolai:langchange', { detail: { lang: getLang() } }));
  }

  function t(key) {
    var lang = getLang();
    var row = STRINGS[lang] || STRINGS.en;
    if (row[key]) return row[key];
    if (STRINGS.en[key]) return STRINGS.en[key];
    return key;
  }

  function apply(root) {
    if (!root || !root.querySelectorAll) return;
    root.querySelectorAll('[data-i18n]').forEach(function (el) {
      var key = el.getAttribute('data-i18n');
      if (!key) return;
      var val = t(key);
      if (el.tagName === 'TITLE') {
        document.title = val;
        return;
      }
      if (el.tagName === 'INPUT') {
        if (el.type === 'submit' || el.type === 'button') el.value = val;
        else el.setAttribute('placeholder', val);
        return;
      }
      if (el.tagName === 'BUTTON') {
        el.textContent = val;
        return;
      }
      el.textContent = val;
    });
    root.querySelectorAll('[data-i18n-html]').forEach(function (el) {
      var key = el.getAttribute('data-i18n-html');
      if (key) el.innerHTML = t(key);
    });
    root.querySelectorAll('[data-i18n-placeholder]').forEach(function (el) {
      var key = el.getAttribute('data-i18n-placeholder');
      if (key) el.setAttribute('placeholder', t(key));
    });
    root.querySelectorAll('[data-i18n-aria]').forEach(function (el) {
      var key = el.getAttribute('data-i18n-aria');
      if (key) el.setAttribute('aria-label', t(key));
    });
  }

  function bindLangSegment(el, langCode) {
    if (!el) return;
    el.addEventListener('click', function (e) {
      e.preventDefault();
      setLang(langCode);
      syncLangToggleActive();
    });
  }

  function syncLangToggleActive() {
    var cur = getLang();
    document.querySelectorAll('[data-lang-set]').forEach(function (btn) {
      var v = btn.getAttribute('data-lang-set');
      btn.classList.toggle('active', v === cur);
      btn.setAttribute('aria-pressed', v === cur ? 'true' : 'false');
    });
  }

  var LS_BOOTSTRAP_SHOW = 'poolai_bootstrap_admin_show';
  var LS_BOOTSTRAP_ACK = 'poolai_bootstrap_admin_ack';

  /** First-run reminder when default admin password is still in use (see AuthResponse.bootstrap_default_admin). */
  function initBootstrapBanner() {
    var host = document.getElementById('poolai-bootstrap-banner-host');
    if (!host) return;
    try {
      var role = localStorage.getItem('poolai_role');
      if (role !== 'Admin') return;
      if (localStorage.getItem(LS_BOOTSTRAP_ACK) === '1') return;
      if (localStorage.getItem(LS_BOOTSTRAP_SHOW) !== '1') return;
    } catch (e) {
      return;
    }
    host.removeAttribute('hidden');
    host.innerHTML =
      '<div class="poolai-bootstrap-inner">' +
      '<div class="poolai-bootstrap-text">' +
      '<div data-i18n="auth.bootstrapLine1"></div>' +
      '<div data-i18n="auth.bootstrapLine2" style="margin-top:6px;opacity:0.92;"></div>' +
      '</div>' +
      '<div class="poolai-bootstrap-actions">' +
      '<a class="btn btn-primary" href="/ui/admin/users" data-i18n="auth.bootstrapUsersLink"></a>' +
      '<button type="button" class="btn" data-i18n="auth.bootstrapDismiss"></button>' +
      '</div>' +
      '</div>';
    apply(host);
    var dismiss = host.querySelector('.poolai-bootstrap-actions button');
    if (dismiss) {
      dismiss.addEventListener('click', function () {
        try {
          localStorage.setItem(LS_BOOTSTRAP_ACK, '1');
        } catch (e2) {}
        host.setAttribute('hidden', '');
        host.innerHTML = '';
      });
    }
  }

  /** Admin header: expects #poolai-lang-toggle container */
  function initAdminShell() {
    var host = document.getElementById('poolai-lang-toggle');
    if (!host) return;
    host.innerHTML =
      '<span class="admin-lang-label" data-i18n="admin.lang.label"></span> ' +
      '<button type="button" class="btn-lang" data-lang-set="en" data-i18n="auth.lang.en"></button>' +
      '<button type="button" class="btn-lang" data-lang-set="uk" data-i18n="auth.lang.uk"></button>';
    apply(host);
    bindLangSegment(host.querySelector('[data-lang-set="en"]'), 'en');
    bindLangSegment(host.querySelector('[data-lang-set="uk"]'), 'uk');
    syncLangToggleActive();
    initBootstrapBanner();
  }

  function initAuthPage() {
    var host = document.getElementById('poolai-lang-toggle-auth');
    if (!host) return;
    host.innerHTML =
      '<button type="button" class="btn-lang" data-lang-set="en" data-i18n="auth.lang.en"></button>' +
      '<button type="button" class="btn-lang" data-lang-set="uk" data-i18n="auth.lang.uk"></button>';
    apply(host);
    bindLangSegment(host.querySelector('[data-lang-set="en"]'), 'en');
    bindLangSegment(host.querySelector('[data-lang-set="uk"]'), 'uk');
    syncLangToggleActive();
  }

  /** Main `/ui/*` layout: `#poolai-lang-toggle-dash` in topbar */
  function initDashboardShell() {
    var host = document.getElementById('poolai-lang-toggle-dash');
    if (!host) return;
    host.innerHTML =
      '<span class="admin-lang-label" data-i18n="admin.lang.label"></span> ' +
      '<button type="button" class="btn-lang" data-lang-set="en" data-i18n="auth.lang.en"></button>' +
      '<button type="button" class="btn-lang" data-lang-set="uk" data-i18n="auth.lang.uk"></button>';
    apply(host);
    bindLangSegment(host.querySelector('[data-lang-set="en"]'), 'en');
    bindLangSegment(host.querySelector('[data-lang-set="uk"]'), 'uk');
    syncLangToggleActive();
    initBootstrapBanner();
  }

  document.documentElement.lang = getLang() === 'uk' ? 'uk' : 'en';

  window.PoolAiI18n = {
    getLang: getLang,
    setLang: setLang,
    t: t,
    apply: apply,
    STRINGS: STRINGS,
    initAdminShell: initAdminShell,
    initAuthPage: initAuthPage,
    initDashboardShell: initDashboardShell,
  };

  /** Fallback for inline scripts: second arg is English default if key missing */
  window.poolaiT = function (key, enFallback) {
    var v = t(key);
    if (v === key && enFallback !== undefined) return enFallback;
    return v;
  };
})();
