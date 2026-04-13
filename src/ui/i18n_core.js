/**
 * PoolAI UI i18n (FM-012): Ukrainian / English via localStorage `poolai_ui_lang` (`en` | `uk`).
 * Exposes: PoolAiI18n.{ getLang, setLang, t, apply, initAdminShell, initAuthPage }
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

  document.documentElement.lang = getLang() === 'uk' ? 'uk' : 'en';

  window.PoolAiI18n = {
    getLang: getLang,
    setLang: setLang,
    t: t,
    apply: apply,
    STRINGS: STRINGS,
    initAdminShell: initAdminShell,
    initAuthPage: initAuthPage,
  };

  /** Fallback for inline scripts: second arg is English default if key missing */
  window.poolaiT = function (key, enFallback) {
    var v = t(key);
    if (v === key && enFallback !== undefined) return enFallback;
    return v;
  };
})();
