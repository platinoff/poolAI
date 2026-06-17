/**
 * PoolAI UI i18n (FM-012): Ukrainian / English via localStorage `poolai_ui_lang` (`en` | `uk`).
 * Exposes: PoolAiI18n.{ getLang, setLang, t, apply, initAdminShell, initAuthPage, initDashboardShell }
 */
(function () {
  'use strict';

  var STORAGE_KEY = 'poolai_ui_lang';

  var STRINGS = {
    en: {
      // auth.* + dash.* shell → poolai-ui-core i18n.rs (PH-S162); merged from __poolaiAuthDashI18nRust

      'admin.brand': 'PoolAI Admin',
      'admin.skipMain': 'Skip to main content',
      'admin.skipNav': 'Skip to navigation',
      'admin.nav.dashboard': 'Dashboard',
      'admin.nav.tenants': 'Tenants',
      'admin.nav.security': 'Security',
      'admin.nav.audit': 'Audit Logs',
      'admin.nav.monitoring': 'Monitoring',
      'admin.nav.vm': 'VM Instances',
      'admin.nav.workers': 'Workers',
      'admin.nav.jobs': 'Jobs',
      'admin.nav.gridPricing': 'Grid pricing',
      'admin.nav.updatesCompat': 'Updates',
      'admin.nav.libs': 'Libraries',
      'admin.nav.raid': 'RAID',
      'admin.nav.instances': 'Model Instances',
      'admin.nav.topology': 'Topology',
      'admin.nav.users': 'Users',
      'admin.nav.config': 'Configuration',
      'admin.lang.label': 'Language',
      'admin.logout': 'Log out',
      'admin.browserSuffix': ' - PoolAI Admin',

      'admin.page.vm': 'VM Management',
      'admin.page.users': 'User Management',
      'admin.page.config': 'System Configuration',

      'admin.table.empty': 'No data to display',
      'admin.table.searchPh': 'Filter table…',
      'admin.table.exportCsv': 'Export CSV',
      'admin.table.exportJson': 'Export JSON',
      'admin.table.exportCsvAria': 'Export visible rows as CSV',
      'admin.table.exportJsonAria': 'Export visible rows as JSON',
      'admin.table.exportedCsv': 'Table exported as CSV',
      'admin.table.exportedJson': 'Table exported as JSON',
      'admin.table.sortedBy': 'Sorted by {column} {direction}',

      'err.insufficientAdmin': 'Insufficient permissions. Admin role required.',
      'admin.status.active': 'Active',
      'admin.status.inactive': 'Inactive',
      'admin.status.yes': 'Yes',
      'admin.status.no': 'No',
      'admin.btn.edit': 'Edit',
      'admin.na': 'N/A',
      'ui.save': 'Save Changes',
      'ui.upload': 'Upload',
      'ui.register': 'Register',

      'admin.vmadm.loading': 'Loading VM instances…',
      'admin.vmadm.errLoad': 'Error loading VM instances: ',
      'admin.vmadm.empty': 'No VM instances found',
      'admin.vmadm.col.name': 'Name',
      'admin.vmadm.col.status': 'Status',
      'admin.vmadm.col.resources': 'Resources',
      'admin.vmadm.col.actions': 'Actions',
      'admin.vmadm.resCpu': 'CPU:',
      'admin.vmadm.resMem': 'Memory:',
      'admin.vmadm.section': 'VM Instances',
      'admin.vmadm.createBtn': 'Create VM Instance',
      'admin.vmadm.actionOk': 'VM {action} successful',
      'admin.vmadm.creating': 'Creating…',

      'admin.usr.loading': 'Loading users…',
      'admin.usr.errLoad': 'Error loading users: ',
      'admin.usr.empty': 'No users found',
      'admin.usr.col.user': 'Username',
      'admin.usr.col.role': 'Role',
      'admin.usr.col.status': 'Status',
      'admin.usr.col.created': 'Created',
      'admin.usr.col.actions': 'Actions',
      'admin.usr.section': 'Users',
      'admin.usr.createBtn': 'Create User',
      'admin.usr.createTitle': 'Create New User',
      'admin.usr.editTitle': 'Edit User',
      'admin.usr.label.user': 'Username',
      'admin.usr.ph.user': 'newuser',
      'admin.usr.label.pw': 'Password',
      'admin.usr.label.pwNew': 'New Password (leave empty to keep current)',
      'admin.usr.label.role': 'Role',
      'admin.usr.ph.pw': 'Enter password',
      'admin.usr.ph.pwNew': 'Enter new password',
      'admin.usr.creating': 'Creating…',
      'admin.usr.saving': 'Saving…',
      'admin.usr.createSubmit': 'Create User',
      'admin.usr.createdOk': 'User created successfully',
      'admin.usr.updatedOk': 'User updated successfully',
      'admin.usr.loadEditErr': 'Error loading user for edit: ',
      'admin.usr.confirmDel':
        'Are you sure you want to delete this user? This action cannot be undone.',
      'admin.usr.deletedOk': 'User deleted successfully',
      'admin.usr.errDel': 'Error deleting user: ',

      'admin.cfg.loading': 'Loading configuration…',
      'admin.cfg.unknownTab': 'Unknown tab: ',

      'admin.cfg.tab.general': 'General',
      'admin.cfg.tab.performance': 'Performance',
      'admin.cfg.tab.gpu': 'GPU',
      'admin.cfg.tab.security': 'Security',
      'admin.cfg.tab.monitoring': 'Monitoring',
      'admin.cfg.tab.health': 'Health',
      'admin.cfg.saveBtn': 'Save Configuration',
      'admin.cfg.savedOk': 'Configuration saved successfully',
      'admin.cfg.saveErr': 'Error saving configuration: ',
      'admin.cfg.saving': 'Saving…',
      'admin.cfg.gen.systemName': 'System Name',
      'admin.cfg.gen.logLevel': 'Log Level',
      'admin.cfg.gen.maxWorkers': 'Max Workers',
      'admin.cfg.gen.queueSize': 'Queue Size',
      'admin.cfg.gen.metricsInterval': 'Metrics Interval (seconds)',
      'admin.cfg.log.trace': 'Trace',
      'admin.cfg.log.debug': 'Debug',
      'admin.cfg.log.info': 'Info',
      'admin.cfg.log.warn': 'Warn',
      'admin.cfg.log.error': 'Error',
      'admin.cfg.perf.poolMaxWorkers': 'Pool Max Workers',
      'admin.cfg.perf.poolQueue': 'Pool Queue Size',
      'admin.cfg.perf.autoScaling': 'Auto Scaling',
      'admin.cfg.perf.scalingThreshold': 'Scaling Threshold (0.0-1.0)',
      'admin.cfg.perf.requestTimeout': 'Request Timeout (seconds)',
      'admin.cfg.https.enable': 'Enable HTTPS',
      'admin.cfg.https.certPath': 'Certificate Path',
      'admin.cfg.https.keyPath': 'Key Path',
      'admin.cfg.mon.metricsInterval': 'Metrics Interval (seconds)',
      'admin.cfg.mon.alertThreshold': 'Alert Threshold (0.0-1.0)',
      'admin.cfg.mon.retentionDays': 'Retention Days',
      'admin.cfg.mon.detailedLogging': 'Detailed Logging',
      'admin.cfg.gpu.enable': 'Enable GPU',
      'admin.cfg.gpu.memLimit': 'GPU Memory Limit (MB)',
      'admin.cfg.gpu.tempLimit': 'Temperature Limit (°C)',
      'admin.cfg.gpu.powerLimit': 'Power Limit (Watts)',
      'admin.cfg.gpu.count': 'GPU Count',
      'admin.cfg.health.expectedWorkers': 'Expected Workers',
      'admin.cfg.health.hint': 'Number of workers expected for health checks',

      'common.loading': 'Loading…',
      'common.unauthorized': 'Unauthorized — session expired. Please sign in again.',
      'admin.accessRequired': 'Admin access required',

      'err.hint403': 'You may need Admin or Operator role, or sign in again.',
      'err.hint503.generic': 'A subsystem may still be starting or unavailable.',
      'err.hint503.raid': 'RAID manager is not initialized on this server.',
      'err.hint503.library': 'Library subsystem may not be initialized.',
      'err.hint503.vm': 'VM manager may not be attached.',
      'err.hint404.enterprise':
        'Build and run the server with the enterprise feature for this API.',

      'home.apiTitle': 'API',
      'home.apiBase': 'Base:',
      'home.uiTitle': 'UI',
      'home.uiHint': 'Pages under',
      'home.openDashboard': 'Open read-only dashboard',
      'home.quickLinks': 'Quick links',
      'home.notesTitle': 'Notes',
      'home.notesBody':
        'Write operations are available for authenticated users with appropriate permissions.',

      'ui.confirmTitle': 'Confirm action',
      'ui.confirmBtn': 'Confirm',
      'ui.cancel': 'Cancel',
      'ui.create': 'Create',
      'ui.install': 'Install',
      'ui.delete': 'Delete',
      'ui.update': 'Update',
      'ui.uninstall': 'Uninstall',
      'ui.closeDialogAria': 'Close dialog',
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

      'vm.createBtn': 'Create VM Instance',
      'vm.createBtnAria': 'Create new VM instance',
      'vm.modalTitle': 'Create VM Instance',
      'vm.label.name': 'Instance name',
      'vm.label.cpu': 'CPU cores',
      'vm.label.memory': 'Memory (MB)',
      'vm.label.gpu': 'GPU required',
      'vm.label.isolation': 'Isolation type',
      'vm.iso.process': 'Process sandbox',
      'vm.iso.hardware': 'Hardware VM',
      'vm.ph.name': 'my-vm-instance',
      'vm.start': 'Start',
      'vm.stop': 'Stop',
      'vm.restart': 'Restart',
      'vm.startAria': 'Start VM instance {id}',
      'vm.stopAria': 'Stop VM instance {id}',
      'vm.restartAria': 'Restart VM instance {id}',
      'vm.deleteAria': 'Delete VM instance {id}',
      'vm.loadingStart': 'Starting VM instance…',
      'vm.loadingStop': 'Stopping VM instance…',
      'vm.loadingRestart': 'Restarting VM instance…',
      'vm.loadingGeneric': 'Processing VM instance…',
      'vm.deletingLoad': 'Deleting VM instance…',
      'vm.createdOk': 'VM instance created successfully',
      'vm.deletedOk': 'VM instance deleted successfully',
      'vm.successStart': 'VM instance started successfully',
      'vm.successStop': 'VM instance stopped successfully',
      'vm.successRestart': 'VM instance restarted successfully',
      'vm.successGeneric': 'Operation completed successfully',
      'vm.confirmDelete':
        'Are you sure you want to delete VM instance "{name}" ({id})? This action cannot be undone.',

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
      // auth.* + dash.* shell → poolai-ui-core i18n.rs (PH-S162); merged from __poolaiAuthDashI18nRust

      'admin.brand': 'PoolAI Адмін',
      'admin.skipMain': 'Перейти до основного вмісту',
      'admin.skipNav': 'Перейти до навігації',
      'admin.nav.dashboard': 'Панель',
      'admin.nav.tenants': 'Орендарі',
      'admin.nav.security': 'Безпека',
      'admin.nav.audit': 'Журнал аудиту',
      'admin.nav.monitoring': 'Моніторинг',
      'admin.nav.vm': 'VM',
      'admin.nav.workers': 'Воркери',
      'admin.nav.jobs': 'Задачі',
      'admin.nav.gridPricing': 'Ціни Grid',
      'admin.nav.updatesCompat': 'Оновлення',
      'admin.nav.libs': 'Бібліотеки',
      'admin.nav.raid': 'RAID',
      'admin.nav.instances': 'Інстанси моделей',
      'admin.nav.topology': 'Топологія',
      'admin.nav.users': 'Користувачі',
      'admin.nav.config': 'Конфігурація',
      'admin.lang.label': 'Мова',
      'admin.logout': 'Вийти',
      'admin.browserSuffix': ' — PoolAI Адмін',

      'admin.page.vm': 'Керування VM',
      'admin.page.users': 'Керування користувачами',
      'admin.page.config': 'Конфігурація системи',

      'admin.table.empty': 'Немає даних для відображення',
      'admin.table.searchPh': 'Фільтр таблиці…',
      'admin.table.exportCsv': 'Експорт CSV',
      'admin.table.exportJson': 'Експорт JSON',
      'admin.table.exportCsvAria': 'Експортувати видимі рядки як CSV',
      'admin.table.exportJsonAria': 'Експортувати видимі рядки як JSON',
      'admin.table.exportedCsv': 'Таблицю експортовано у CSV',
      'admin.table.exportedJson': 'Таблицю експортовано у JSON',
      'admin.table.sortedBy': 'Сортування: {column} {direction}',

      'err.insufficientAdmin': 'Недостатньо прав. Потрібна роль Admin.',
      'admin.status.active': 'Активний',
      'admin.status.inactive': 'Неактивний',
      'admin.status.yes': 'Так',
      'admin.status.no': 'Ні',
      'admin.btn.edit': 'Змінити',
      'admin.na': 'Н/Д',
      'ui.save': 'Зберегти зміни',
      'ui.upload': 'Завантажити',
      'ui.register': 'Зареєструвати',

      'admin.vmadm.loading': 'Завантаження інстансів VM…',
      'admin.vmadm.errLoad': 'Помилка завантаження VM: ',
      'admin.vmadm.empty': 'Інстансів VM не знайдено',
      'admin.vmadm.col.name': 'Назва',
      'admin.vmadm.col.status': 'Статус',
      'admin.vmadm.col.resources': 'Ресурси',
      'admin.vmadm.col.actions': 'Дії',
      'admin.vmadm.resCpu': 'CPU:',
      'admin.vmadm.resMem': 'Пам’ять:',
      'admin.vmadm.section': 'Інстанси VM',
      'admin.vmadm.createBtn': 'Створити інстанс VM',
      'admin.vmadm.actionOk': 'VM: дія «{action}» виконана',
      'admin.vmadm.creating': 'Створення…',

      'admin.usr.loading': 'Завантаження користувачів…',
      'admin.usr.errLoad': 'Помилка завантаження користувачів: ',
      'admin.usr.empty': 'Користувачів не знайдено',
      'admin.usr.col.user': 'Користувач',
      'admin.usr.col.role': 'Роль',
      'admin.usr.col.status': 'Статус',
      'admin.usr.col.created': 'Створено',
      'admin.usr.col.actions': 'Дії',
      'admin.usr.section': 'Користувачі',
      'admin.usr.createBtn': 'Створити користувача',
      'admin.usr.createTitle': 'Новий користувач',
      'admin.usr.editTitle': 'Редагувати користувача',
      'admin.usr.label.user': 'Ім’я користувача',
      'admin.usr.ph.user': 'newuser',
      'admin.usr.label.pw': 'Пароль',
      'admin.usr.label.pwNew': 'Новий пароль (порожньо — без змін)',
      'admin.usr.label.role': 'Роль',
      'admin.usr.ph.pw': 'Введіть пароль',
      'admin.usr.ph.pwNew': 'Новий пароль',
      'admin.usr.creating': 'Створення…',
      'admin.usr.saving': 'Збереження…',
      'admin.usr.createSubmit': 'Створити користувача',
      'admin.usr.createdOk': 'Користувача створено',
      'admin.usr.updatedOk': 'Користувача оновлено',
      'admin.usr.loadEditErr': 'Помилка завантаження для редагування: ',
      'admin.usr.confirmDel': 'Видалити цього користувача? Дію не скасувати.',
      'admin.usr.deletedOk': 'Користувача видалено',
      'admin.usr.errDel': 'Помилка видалення: ',

      'admin.cfg.loading': 'Завантаження конфігурації…',
      'admin.cfg.unknownTab': 'Невідома вкладка: ',

      'admin.cfg.tab.general': 'Загальне',
      'admin.cfg.tab.performance': 'Продуктивність',
      'admin.cfg.tab.gpu': 'GPU',
      'admin.cfg.tab.security': 'Безпека',
      'admin.cfg.tab.monitoring': 'Моніторинг',
      'admin.cfg.tab.health': 'Здоров’я',
      'admin.cfg.saveBtn': 'Зберегти конфігурацію',
      'admin.cfg.savedOk': 'Конфігурацію збережено',
      'admin.cfg.saveErr': 'Помилка збереження: ',
      'admin.cfg.saving': 'Збереження…',
      'admin.cfg.gen.systemName': 'Назва системи',
      'admin.cfg.gen.logLevel': 'Рівень логування',
      'admin.cfg.gen.maxWorkers': 'Макс. воркерів',
      'admin.cfg.gen.queueSize': 'Розмір черги',
      'admin.cfg.gen.metricsInterval': 'Інтервал метрик (с)',
      'admin.cfg.log.trace': 'Trace',
      'admin.cfg.log.debug': 'Debug',
      'admin.cfg.log.info': 'Info',
      'admin.cfg.log.warn': 'Warn',
      'admin.cfg.log.error': 'Error',
      'admin.cfg.perf.poolMaxWorkers': 'Макс. воркерів пулу',
      'admin.cfg.perf.poolQueue': 'Черга пулу',
      'admin.cfg.perf.autoScaling': 'Автомасштабування',
      'admin.cfg.perf.scalingThreshold': 'Поріг масштабування (0.0–1.0)',
      'admin.cfg.perf.requestTimeout': 'Таймаут запиту (с)',
      'admin.cfg.https.enable': 'Увімкнути HTTPS',
      'admin.cfg.https.certPath': 'Шлях до сертифіката',
      'admin.cfg.https.keyPath': 'Шлях до ключа',
      'admin.cfg.mon.metricsInterval': 'Інтервал метрик (с)',
      'admin.cfg.mon.alertThreshold': 'Поріг сповіщень (0.0–1.0)',
      'admin.cfg.mon.retentionDays': 'Днів зберігання',
      'admin.cfg.mon.detailedLogging': 'Детальне логування',
      'admin.cfg.gpu.enable': 'Увімкнути GPU',
      'admin.cfg.gpu.memLimit': 'Ліміт пам’яті GPU (МБ)',
      'admin.cfg.gpu.tempLimit': 'Ліміт температури (°C)',
      'admin.cfg.gpu.powerLimit': 'Ліміт потужності (Вт)',
      'admin.cfg.gpu.count': 'Кількість GPU',
      'admin.cfg.health.expectedWorkers': 'Очікувані воркери',
      'admin.cfg.health.hint': 'Кількість воркерів для health-check',

      'common.loading': 'Завантаження…',
      'common.unauthorized': 'Неавторизовано — сесію завершено. Увійдіть знову.',
      'admin.accessRequired': 'Потрібні права адміністратора',

      'err.hint403': 'Можливо, потрібна роль Admin або Operator, або увійдіть знову.',
      'err.hint503.generic': 'Підсистема ще стартує або тимчасово недоступна.',
      'err.hint503.raid': 'RAID-менеджер на цьому сервері не ініціалізовано.',
      'err.hint503.library': 'Підсистему бібліотек може бути не ініціалізовано.',
      'err.hint503.vm': 'VM-менеджер може бути не підключено.',
      'err.hint404.enterprise':
        'Зберіть і запустіть сервер з функцією enterprise для цього API.',

      'home.apiTitle': 'API',
      'home.apiBase': 'База:',
      'home.uiTitle': 'UI',
      'home.uiHint': 'Сторінки під',
      'home.openDashboard': 'Відкрити панель лише для читання',
      'home.quickLinks': 'Швидкі посилання',
      'home.notesTitle': 'Нотатки',
      'home.notesBody':
        'Операції запису доступні автентифікованим користувачам з відповідними правами.',

      'ui.confirmTitle': 'Підтвердження',
      'ui.confirmBtn': 'Підтвердити',
      'ui.cancel': 'Скасувати',
      'ui.create': 'Створити',
      'ui.install': 'Встановити',
      'ui.delete': 'Видалити',
      'ui.update': 'Оновити',
      'ui.uninstall': 'Видалити (бібліотеку)',
      'ui.closeDialogAria': 'Закрити діалог',
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

      'vm.createBtn': 'Створити інстанс VM',
      'vm.createBtnAria': 'Створити новий інстанс VM',
      'vm.modalTitle': 'Створити інстанс VM',
      'vm.label.name': 'Назва інстансу',
      'vm.label.cpu': 'Ядра CPU',
      'vm.label.memory': 'Пам’ять (МБ)',
      'vm.label.gpu': 'Потрібен GPU',
      'vm.label.isolation': 'Тип ізоляції',
      'vm.iso.process': 'Пісочниця процесу',
      'vm.iso.hardware': 'Апаратна VM',
      'vm.ph.name': 'my-vm-instance',
      'vm.start': 'Запустити',
      'vm.stop': 'Зупинити',
      'vm.restart': 'Перезапустити',
      'vm.startAria': 'Запустити інстанс VM {id}',
      'vm.stopAria': 'Зупинити інстанс VM {id}',
      'vm.restartAria': 'Перезапустити інстанс VM {id}',
      'vm.deleteAria': 'Видалити інстанс VM {id}',
      'vm.loadingStart': 'Запуск інстансу VM…',
      'vm.loadingStop': 'Зупинка інстансу VM…',
      'vm.loadingRestart': 'Перезапуск інстансу VM…',
      'vm.loadingGeneric': 'Обробка інстансу VM…',
      'vm.deletingLoad': 'Видалення інстансу VM…',
      'vm.createdOk': 'Інстанс VM створено',
      'vm.deletedOk': 'Інстанс VM видалено',
      'vm.successStart': 'Інстанс VM запущено',
      'vm.successStop': 'Інстанс VM зупинено',
      'vm.successRestart': 'Інстанс VM перезапущено',
      'vm.successGeneric': 'Операцію виконано',
      'vm.confirmDelete':
        'Видалити інстанс VM «{name}» ({id})? Цю дію не можна скасувати.',

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
  var rustAuthDash = typeof window !== 'undefined' && window.__poolaiAuthDashI18nRust;
  if (rustAuthDash) {
    if (rustAuthDash.en) Object.assign(STRINGS.en, rustAuthDash.en);
    if (rustAuthDash.uk) Object.assign(STRINGS.uk, rustAuthDash.uk);
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
