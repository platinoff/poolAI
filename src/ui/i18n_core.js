/**
 * PoolAI UI i18n (FM-012): Ukrainian / English via localStorage `poolai_ui_lang` (`en` | `uk`).
 * Exposes: PoolAiI18n.{ getLang, setLang, t, apply, initAdminShell, initAuthPage, initDashboardShell }
 *
 * PH-S266: STRINGS core is empty — all keys injected from poolai-ui-core Rust patches.
 */
(function () {
  'use strict';

  var STORAGE_KEY = 'poolai_ui_lang';

  var STRINGS = {
    en: {
      // auth.* + dash.* + admin.nav.* + admin chrome → auth_dash_shell_patch (PH-S162, S242, S243)
      // home.* → admin_home_patch (PH-S258)
      // workers.* → workers_panel_patch (PH-S257)
      // form.* + residual err.* → admin_form_patch / admin_err_patch (PH-S259)
      // ui.save/search/retry → admin_ui_toolbar_patch (PH-S260)
      // common.* + residual ui.* → admin_ui_common_patch (PH-S263)
      // libs.* → libs_panel_patch (PH-S264)
      // raid.* → raid_panel_patch (PH-S265)
    },
    uk: {},
  };

  /** PH-S932: single merge path for Rust i18n patches (no duplicate Object.assign blocks). */
  function mergeRustI18nPatch(patch) {
    if (!patch) return;
    if (patch.en) Object.assign(STRINGS.en, patch.en);
    if (patch.uk) Object.assign(STRINGS.uk, patch.uk);
  }

  if (typeof window !== 'undefined') {
    mergeRustI18nPatch(window.__poolaiAdminI18nRust);
    mergeRustI18nPatch(window.__poolaiAuthDashI18nRust);
    mergeRustI18nPatch(window.__poolaiAdminTableI18nRust);
    mergeRustI18nPatch(window.__poolaiAdminStatusI18nRust);
    mergeRustI18nPatch(window.__poolaiAdminErrI18nRust);
    mergeRustI18nPatch(window.__poolaiVmModalI18nRust);
    mergeRustI18nPatch(window.__poolaiAdminUiConfirmI18nRust);
    mergeRustI18nPatch(window.__poolaiWorkersPanelI18nRust);
    mergeRustI18nPatch(window.__poolaiHomeI18nRust);
    mergeRustI18nPatch(window.__poolaiAdminFormI18nRust);
    mergeRustI18nPatch(window.__poolaiAdminUiToolbarI18nRust);
    mergeRustI18nPatch(window.__poolaiUiCommonI18nRust);
    mergeRustI18nPatch(window.__poolaiLibsPanelI18nRust);
    mergeRustI18nPatch(window.__poolaiRaidPanelI18nRust);
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

  window.poolaiT = function (key, enFallback) {
    var v = t(key);
    if (v === key && enFallback !== undefined) return enFallback;
    return v;
  };
})();
