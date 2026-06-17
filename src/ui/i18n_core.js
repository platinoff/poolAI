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

      'ui.save': 'Save Changes',
      'ui.upload': 'Upload',
      'ui.register': 'Register',

      'common.loading': 'Loading…',
      'common.unauthorized': 'Unauthorized — session expired. Please sign in again.',

      'home.apiTitle': 'API',
      'home.apiBase': 'Base:',
      'home.uiTitle': 'UI',
      'home.uiHint': 'Pages under',
      'home.openDashboard': 'Open read-only dashboard',
      'home.quickLinks': 'Quick links',
      'home.notesTitle': 'Notes',
      'home.notesBody':
        'Write operations are available for authenticated users with appropriate permissions.',

      'ui.create': 'Create',
      'ui.install': 'Install',
      'ui.delete': 'Delete',
      'ui.update': 'Update',
      'ui.uninstall': 'Uninstall',
      'ui.closeNotificationAria': 'Close notification',
      'ui.clearSearchAria': 'Clear search',
      'ui.searchTableAria': 'Search table',
      'ui.searchStatusFound': '{visible} of {total} results found',
      'ui.searchStatusSimple': '{visible} of {total} results',
      'ui.searchStatusAll': 'All results shown',
      'ui.searchNoResultsFor': 'No results found for "{query}"',
      'ui.stepOfTotal': 'Step {current} of {total}',
      'ui.tabsAria': 'Tabs',
      'ui.retry': 'Retry',
      'ui.retryFailedOpAria': 'Retry the operation that failed',
      'ui.requestSucceededAfter': 'Request succeeded after {count} {unit}',
      'ui.retryUnit.one': 'retry',
      'ui.retryUnit.many': 'retries',
      'ui.requestRetrying': 'Request failed. Retrying in {seconds}s... ({attempt}/{max})',
      'ui.requestFailedAllRetries': 'Request failed after all retries',
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

      'form.fieldRequired': 'This field is required',
      'form.validNumber': 'Please enter a valid number',
      'form.valueMin': 'Value must be at least {min}',
      'form.valueMax': 'Value must be at most {max}',
      'form.validEmail': 'Please enter a valid email address',
      'form.invalidFormat': 'Invalid format',

      'err.errorPrefix': 'Error: ',
      'err.title': 'Error',
      'err.showDetails': 'Show details',
      'err.suggestions': 'Suggestions:',
      'err.insufficientPermissions': 'Insufficient permissions.',
      'err.insufficientPermissionsAdminOp':
        'Insufficient permissions. Admin or Operator role required.',
      'err.fillRequiredFields': 'Please fill in all required fields correctly.',
      'err.insufficientRole': 'Insufficient permissions. Required role: ',
      'role.admin': 'Admin',
      'role.operator': 'Operator',
      'role.viewer': 'Viewer',
      'err.selectFileUpload': 'Please select a file to upload.',
      'err.readFileFailed': 'Error reading file',
      'err.unauthorized': 'Unauthorized',

      'workers.empty': 'No workers available.',
      'workers.listAria': 'Workers list',
      'workers.tableDesc':
        'Table showing workers: id, health, state, task, metrics, actions',
      'workers.col.id': 'ID',
      'workers.col.health': 'Health',
      'workers.col.state': 'State',
      'workers.col.task': 'Current task',
      'workers.col.requests': 'Requests',
      'workers.col.queue': 'Queue',
      'workers.col.actions': 'Actions',
      'workers.healthy': 'Healthy',
      'workers.unhealthy': 'Unhealthy',
      'workers.rowAriaPrefix': 'Worker',
      'workers.deleteAria': 'Delete worker {id}',
      'workers.permDeleteDesc': 'Permanently delete worker {id}',
      'workers.noActionsRole': 'No actions available for your role',
      'workers.createBtn': 'Create Worker',
      'workers.createBtnAria': 'Create new worker',
      'workers.modalTitle': 'Create Worker',
      'workers.label.id': 'Worker ID',
      'workers.label.maxConcurrent': 'Max concurrent requests',
      'workers.label.timeout': 'Request timeout (ms)',
      'workers.label.healthInterval': 'Health check interval (ms)',
      'workers.label.maxMemory': 'Max memory (MB)',
      'workers.label.cpuPriority': 'CPU priority (1–10)',
      'workers.label.gpuDevice': 'GPU device ID (optional)',
      'workers.label.cacheSize': 'Cache size',
      'workers.ph.id': 'worker-1',
      'workers.ph.gpu': 'Leave empty for no GPU',
      'workers.enableCache': 'Enable caching',
      'workers.autoRestart': 'Auto restart on failure',
      'workers.resourceMonitoring': 'Resource monitoring',
      'workers.creatingSubmit': 'Creating…',
      'workers.createdOk': 'Worker created successfully',
      'workers.deletedOk': 'Worker deleted successfully',
      'workers.deletingLoad': 'Deleting worker…',
      'workers.confirmDelete':
        'Are you sure you want to delete worker "{id}"? This action cannot be undone.',

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

      'ui.save': 'Зберегти зміни',
      'ui.upload': 'Завантажити',
      'ui.register': 'Зареєструвати',

      'common.loading': 'Завантаження…',
      'common.unauthorized': 'Неавторизовано — сесію завершено. Увійдіть знову.',

      'home.apiTitle': 'API',
      'home.apiBase': 'База:',
      'home.uiTitle': 'UI',
      'home.uiHint': 'Сторінки під',
      'home.openDashboard': 'Відкрити панель лише для читання',
      'home.quickLinks': 'Швидкі посилання',
      'home.notesTitle': 'Нотатки',
      'home.notesBody':
        'Операції запису доступні автентифікованим користувачам з відповідними правами.',

      'ui.create': 'Створити',
      'ui.install': 'Встановити',
      'ui.delete': 'Видалити',
      'ui.update': 'Оновити',
      'ui.uninstall': 'Видалити (бібліотеку)',
      'ui.closeNotificationAria': 'Закрити сповіщення',
      'ui.clearSearchAria': 'Очистити пошук',
      'ui.searchTableAria': 'Пошук у таблиці',
      'ui.searchStatusFound': 'Знайдено {visible} з {total} результатів',
      'ui.searchStatusSimple': 'Показано {visible} з {total} результатів',
      'ui.searchStatusAll': 'Показано всі результати',
      'ui.searchNoResultsFor': 'За запитом "{query}" нічого не знайдено',
      'ui.stepOfTotal': 'Крок {current} з {total}',
      'ui.tabsAria': 'Вкладки',
      'ui.retry': 'Повторити',
      'ui.retryFailedOpAria': 'Повторити операцію, що завершилась помилкою',
      'ui.requestSucceededAfter': 'Запит успішний після {count} {unit}',
      'ui.retryUnit.one': 'повтору',
      'ui.retryUnit.many': 'повторів',
      'ui.requestRetrying': 'Запит не вдався. Повтор через {seconds} с... ({attempt}/{max})',
      'ui.requestFailedAllRetries': 'Запит не вдався після всіх повторів',
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

      'form.fieldRequired': 'Це поле обов’язкове',
      'form.validNumber': 'Введіть коректне число',
      'form.valueMin': 'Значення має бути не менше {min}',
      'form.valueMax': 'Значення має бути не більше {max}',
      'form.validEmail': 'Введіть коректну адресу електронної пошти',
      'form.invalidFormat': 'Некоректний формат',

      'err.errorPrefix': 'Помилка: ',
      'err.title': 'Помилка',
      'err.showDetails': 'Показати деталі',
      'err.suggestions': 'Підказки:',
      'err.insufficientPermissions': 'Недостатньо прав.',
      'err.insufficientPermissionsAdminOp':
        'Недостатньо прав. Потрібна роль Admin або Operator.',
      'err.fillRequiredFields': 'Заповніть усі обов’язкові поля коректно.',
      'err.insufficientRole': 'Недостатньо прав. Потрібна роль: ',
      'role.admin': 'Адміністратор',
      'role.operator': 'Оператор',
      'role.viewer': 'Спостерігач',
      'err.selectFileUpload': 'Оберіть файл для завантаження.',
      'err.readFileFailed': 'Помилка читання файлу',
      'err.unauthorized': 'Неавторизовано',

      'workers.empty': 'Немає доступних воркерів.',
      'workers.listAria': 'Список воркерів',
      'workers.tableDesc':
        'Таблиця воркерів: id, стан здоров’я, статус, поточне завдання, метрики, дії',
      'workers.col.id': 'ID',
      'workers.col.health': 'Здоров’я',
      'workers.col.state': 'Статус',
      'workers.col.task': 'Поточне завдання',
      'workers.col.requests': 'Запити',
      'workers.col.queue': 'Черга',
      'workers.col.actions': 'Дії',
      'workers.healthy': 'OK',
      'workers.unhealthy': 'Проблема',
      'workers.rowAriaPrefix': 'Воркер',
      'workers.deleteAria': 'Видалити воркер {id}',
      'workers.permDeleteDesc': 'Остаточно видалити воркер {id}',
      'workers.noActionsRole': 'Для вашої ролі дії недоступні',
      'workers.createBtn': 'Створити воркер',
      'workers.createBtnAria': 'Створити нового воркера',
      'workers.modalTitle': 'Створити воркер',
      'workers.label.id': 'ID воркера',
      'workers.label.maxConcurrent': 'Макс. одночасних запитів',
      'workers.label.timeout': 'Таймаут запиту (мс)',
      'workers.label.healthInterval': 'Інтервал health-check (мс)',
      'workers.label.maxMemory': 'Макс. пам’ять (МБ)',
      'workers.label.cpuPriority': 'Пріоритет CPU (1–10)',
      'workers.label.gpuDevice': 'ID GPU-пристрою (необов’язково)',
      'workers.label.cacheSize': 'Розмір кешу',
      'workers.ph.id': 'worker-1',
      'workers.ph.gpu': 'Порожньо, якщо без GPU',
      'workers.enableCache': 'Увімкнути кешування',
      'workers.autoRestart': 'Автоперезапуск при збої',
      'workers.resourceMonitoring': 'Моніторинг ресурсів',
      'workers.creatingSubmit': 'Створення…',
      'workers.createdOk': 'Воркер успішно створено',
      'workers.deletedOk': 'Воркер успішно видалено',
      'workers.deletingLoad': 'Видалення воркера…',
      'workers.confirmDelete':
        'Видалити воркера «{id}»? Цю дію не можна скасувати.',

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
