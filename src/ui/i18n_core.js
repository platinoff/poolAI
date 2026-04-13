/**
 * PoolAI UI i18n (FM-012): Ukrainian / English via localStorage `poolai_ui_lang` (`en` | `uk`).
 * Exposes: PoolAiI18n.{ getLang, setLang, t, apply, initAdminShell, initAuthPage, initDashboardShell }
 */
(function () {
  'use strict';

  var STORAGE_KEY = 'poolai_ui_lang';

  var STRINGS = {
    en: {
      'auth.pageTitle': 'Login - PoolAI',
      'auth.cardTitle': 'Login',
      'auth.username': 'Username',
      'auth.password': 'Password',
      'auth.submit': 'Login',
      'auth.loggingIn': 'Logging in…',
      'auth.loginFailed': 'Login failed',
      'auth.oauthStartFail': 'Failed to start OAuth2 login: ',
      'auth.oauthFail': 'OAuth2 authentication failed: ',
      'auth.oauthTokenFail': 'Failed to process OAuth2 token: ',
      'auth.oauthOr': 'Or sign in with:',
      'auth.testAccounts': 'Test accounts:',
      'auth.testAdmin': 'Admin: admin / admin123',
      'auth.testOperator': 'Operator: operator / op123',
      'auth.testViewer': 'Viewer: viewer / view123',
      'auth.lang.en': 'EN',
      'auth.lang.uk': 'UA',

      'admin.brand': 'PoolAI Admin',
      'admin.nav.dashboard': 'Dashboard',
      'admin.nav.tenants': 'Tenants',
      'admin.nav.security': 'Security',
      'admin.nav.audit': 'Audit Logs',
      'admin.nav.monitoring': 'Monitoring',
      'admin.nav.vm': 'VM Instances',
      'admin.nav.workers': 'Workers',
      'admin.nav.libs': 'Libraries',
      'admin.nav.raid': 'RAID',
      'admin.nav.instances': 'Model Instances',
      'admin.nav.topology': 'Topology',
      'admin.nav.users': 'Users',
      'admin.nav.config': 'Configuration',
      'admin.lang.label': 'Language',
      'admin.logout': 'Log out',

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

      'dash.brand': 'PoolAI UI',
      'dash.subtitle': 'Dashboard with write operations (Stage 3)',
      'dash.skipMain': 'Skip to main content',
      'dash.skipNav': 'Skip to navigation',
      'dash.nav.home': 'Home',
      'dash.nav.status': 'Status',
      'dash.nav.health': 'Health',
      'dash.nav.metrics': 'Metrics',
      'dash.nav.workers': 'Workers',
      'dash.nav.libs': 'Libs',
      'dash.nav.vm': 'VM',
      'dash.nav.raid': 'RAID',
      'dash.aria.mainNav': 'Main navigation',
      'dash.aria.home': 'Home page',
      'dash.aria.status': 'System status',
      'dash.aria.health': 'Health check',
      'dash.aria.metrics': 'System metrics',
      'dash.aria.workers': 'Worker management',
      'dash.aria.libs': 'Library management',
      'dash.aria.vm': 'VM instance management',
      'dash.aria.raid': 'RAID artifact management',
      'dash.aria.mobileNav': 'Mobile navigation',
      'dash.aria.openMenu': 'Open navigation menu',
      'dash.aria.closeMenu': 'Close navigation menu',
      'dash.menuTitle': 'Menu',
      'dash.themeLabel': 'Theme:',
      'dash.aria.theme': 'Select theme',
      'dash.themeOptDark': '🌙 Dark',
      'dash.themeOptLight': '☀️ Light',
      'dash.themeOptHC': '🔆 High Contrast',
      'dash.login': 'Login',
      'dash.logout': 'Logout',
      'dash.pageAutoRefresh':
        'Auto-refresh is enabled (5s). Write operations are available for authenticated users with appropriate permissions.',
      'dash.title.home': 'Home',
      'dash.title.status': 'Status',
      'dash.title.health': 'Health',
      'dash.title.metrics': 'Metrics',
      'dash.title.workers': 'Workers',
      'dash.title.libraries': 'Libraries',
      'dash.title.vm': 'VM Instances',
      'dash.title.raid': 'RAID',
      'dash.updatedPrefix': 'Updated:',

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
      'ui.noItems': 'No items.',
      'ui.sourceLabel': 'Source:',
      'ui.artifactsLabel': 'Artifacts:',
      'ui.nodesLabel': 'Nodes:',
      'ui.creating': 'Creating…',
      'ui.installing': 'Installing…',
      'ui.deleting': 'Deleting…',

      'err.errorPrefix': 'Error: ',
      'err.insufficientPermissions': 'Insufficient permissions.',
      'err.insufficientPermissionsAdminOp':
        'Insufficient permissions. Admin or Operator role required.',
      'err.fillRequiredFields': 'Please fill in all required fields correctly.',
      'err.insufficientRole': 'Insufficient permissions. Required role: ',
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
      'auth.pageTitle': 'Вхід - PoolAI',
      'auth.cardTitle': 'Вхід',
      'auth.username': 'Користувач',
      'auth.password': 'Пароль',
      'auth.submit': 'Увійти',
      'auth.loggingIn': 'Вхід…',
      'auth.loginFailed': 'Не вдалося увійти',
      'auth.oauthStartFail': 'Не вдалося розпочати OAuth2: ',
      'auth.oauthFail': 'Помилка OAuth2: ',
      'auth.oauthTokenFail': 'Не вдалося обробити токен OAuth2: ',
      'auth.oauthOr': 'Або увійдіть через:',
      'auth.testAccounts': 'Тестові обліковки:',
      'auth.testAdmin': 'Адмін: admin / admin123',
      'auth.testOperator': 'Оператор: operator / op123',
      'auth.testViewer': 'Глядач: viewer / view123',
      'auth.lang.en': 'EN',
      'auth.lang.uk': 'UA',

      'admin.brand': 'PoolAI Адмін',
      'admin.nav.dashboard': 'Панель',
      'admin.nav.tenants': 'Орендарі',
      'admin.nav.security': 'Безпека',
      'admin.nav.audit': 'Журнал аудиту',
      'admin.nav.monitoring': 'Моніторинг',
      'admin.nav.vm': 'VM',
      'admin.nav.workers': 'Воркери',
      'admin.nav.libs': 'Бібліотеки',
      'admin.nav.raid': 'RAID',
      'admin.nav.instances': 'Інстанси моделей',
      'admin.nav.topology': 'Топологія',
      'admin.nav.users': 'Користувачі',
      'admin.nav.config': 'Конфігурація',
      'admin.lang.label': 'Мова',
      'admin.logout': 'Вийти',

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

      'dash.brand': 'PoolAI UI',
      'dash.subtitle': 'Панель з операціями запису (етап 3)',
      'dash.skipMain': 'Перейти до основного вмісту',
      'dash.skipNav': 'Перейти до навігації',
      'dash.nav.home': 'Головна',
      'dash.nav.status': 'Статус',
      'dash.nav.health': 'Здоров’я',
      'dash.nav.metrics': 'Метрики',
      'dash.nav.workers': 'Воркери',
      'dash.nav.libs': 'Бібліотеки',
      'dash.nav.vm': 'VM',
      'dash.nav.raid': 'RAID',
      'dash.aria.mainNav': 'Головна навігація',
      'dash.aria.home': 'Головна сторінка',
      'dash.aria.status': 'Статус системи',
      'dash.aria.health': 'Перевірка здоров’я',
      'dash.aria.metrics': 'Метрики системи',
      'dash.aria.workers': 'Керування воркерами',
      'dash.aria.libs': 'Керування бібліотеками',
      'dash.aria.vm': 'Керування інстансами VM',
      'dash.aria.raid': 'Керування артефактами RAID',
      'dash.aria.mobileNav': 'Мобільна навігація',
      'dash.aria.openMenu': 'Відкрити меню навігації',
      'dash.aria.closeMenu': 'Закрити меню навігації',
      'dash.menuTitle': 'Меню',
      'dash.themeLabel': 'Тема:',
      'dash.aria.theme': 'Обрати тему',
      'dash.themeOptDark': '🌙 Темна',
      'dash.themeOptLight': '☀️ Світла',
      'dash.themeOptHC': '🔆 Високий контраст',
      'dash.login': 'Увійти',
      'dash.logout': 'Вийти',
      'dash.pageAutoRefresh':
        'Автооновлення кожні 5 с. Операції запису доступні автентифікованим користувачам з відповідними правами.',
      'dash.title.home': 'Головна',
      'dash.title.status': 'Статус',
      'dash.title.health': 'Здоров’я',
      'dash.title.metrics': 'Метрики',
      'dash.title.workers': 'Воркери',
      'dash.title.libraries': 'Бібліотеки',
      'dash.title.vm': 'Інстанси VM',
      'dash.title.raid': 'RAID',
      'dash.updatedPrefix': 'Оновлено:',

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
      'ui.noItems': 'Немає записів.',
      'ui.sourceLabel': 'Джерело:',
      'ui.artifactsLabel': 'Артефакти:',
      'ui.nodesLabel': 'Вузли:',
      'ui.creating': 'Створення…',
      'ui.installing': 'Встановлення…',
      'ui.deleting': 'Видалення…',

      'err.errorPrefix': 'Помилка: ',
      'err.insufficientPermissions': 'Недостатньо прав.',
      'err.insufficientPermissionsAdminOp':
        'Недостатньо прав. Потрібна роль Admin або Operator.',
      'err.fillRequiredFields': 'Заповніть усі обов’язкові поля коректно.',
      'err.insufficientRole': 'Недостатньо прав. Потрібна роль: ',
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
