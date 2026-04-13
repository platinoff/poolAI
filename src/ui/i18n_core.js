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
