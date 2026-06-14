//! Admin modal a11y helpers — focus trap + dynamic modal markup (PH-S161).
//!
//! Parity: `src/ui/admin_modal_a11y.js` (FM-019).

use serde::Serialize;

/// Focusable elements inside an admin modal (`getModalFocusableElements` query).
pub const MODAL_FOCUSABLE_SELECTOR: &str = "a[href], button:not([disabled]), textarea:not([disabled]), input:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex=\"-1\"])";

/// Dynamic modal root id (`ensureAdminDynamicModal`).
pub const ADMIN_DYNAMIC_MODAL_ID: &str = "adminDynamicModal";

/// Tab key focus-trap decision (`trapModalFocus`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FocusTrapAction {
    None,
    First,
    Last,
    Root,
}

impl FocusTrapAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::First => "first",
            Self::Last => "last",
            Self::Root => "root",
        }
    }
}

/// Pure focus-trap tab handler (PH-S161 / FM-019).
pub fn trap_tab_action(
    shift_key: bool,
    focusable_count: usize,
    active_inside: bool,
    active_is_first: bool,
    active_is_last: bool,
) -> FocusTrapAction {
    if focusable_count == 0 {
        return FocusTrapAction::Root;
    }
    if shift_key {
        if !active_inside || active_is_first {
            FocusTrapAction::Last
        } else {
            FocusTrapAction::None
        }
    } else if !active_inside || active_is_last {
        FocusTrapAction::First
    } else {
        FocusTrapAction::None
    }
}

/// Per-tab ARIA attrs after selection change (`adminSyncTabA11y`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TabA11yAttrs {
    pub aria_selected: bool,
    pub tabindex: i8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aria_labelledby: Option<String>,
}

pub fn tab_a11y_attrs(selected: bool, tab_id: Option<&str>) -> TabA11yAttrs {
    TabA11yAttrs {
        aria_selected: selected,
        tabindex: if selected { 0 } else { -1 },
        aria_labelledby: if selected {
            tab_id.map(|id| id.to_string())
        } else {
            None
        },
    }
}

/// Dynamic admin modal inner markup (`ensureAdminDynamicModal`).
pub fn admin_dynamic_modal_html() -> String {
    format!(
        r#"<div class="modal-content"><div class="modal-header"><h3 id="adminDynamicModalTitle"></h3><button type="button" class="modal-close" data-i18n-aria="ui.closeDialogAria" onclick="hideModal('{ADMIN_DYNAMIC_MODAL_ID}')">&times;</button></div><div id="adminDynamicModalBody" class="modal-body"></div></div>"#
    )
}

#[derive(Debug, Serialize)]
struct AdminModalPatch {
    focusable_selector: &'static str,
    dynamic_modal_id: &'static str,
    dynamic_modal_html: String,
}

/// JSON patch for `window.__poolaiAdminModalRust`.
pub fn admin_modal_patch_json() -> String {
    let patch = AdminModalPatch {
        focusable_selector: MODAL_FOCUSABLE_SELECTOR,
        dynamic_modal_id: ADMIN_DYNAMIC_MODAL_ID,
        dynamic_modal_html: admin_dynamic_modal_html(),
    };
    serde_json::to_string(&patch).expect("admin modal patch serializes")
}

/// Inline script body for admin layout injection.
pub fn admin_modal_patch_script() -> String {
    format!(
        "window.__poolaiAdminModalRust={};",
        admin_modal_patch_json()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trap_tab_forward_wraps_at_last() {
        assert_eq!(
            trap_tab_action(false, 3, true, false, true),
            FocusTrapAction::First
        );
    }

    #[test]
    fn trap_tab_shift_wraps_at_first() {
        assert_eq!(
            trap_tab_action(true, 3, true, true, false),
            FocusTrapAction::Last
        );
    }

    #[test]
    fn trap_tab_empty_focusable_targets_root() {
        assert_eq!(
            trap_tab_action(false, 0, false, false, false),
            FocusTrapAction::Root
        );
    }

    #[test]
    fn trap_tab_no_wrap_when_inside_middle() {
        assert_eq!(
            trap_tab_action(false, 3, true, false, false),
            FocusTrapAction::None
        );
    }

    #[test]
    fn dynamic_modal_html_has_close_handler() {
        let html = admin_dynamic_modal_html();
        assert!(html.contains("adminDynamicModalTitle"));
        assert!(html.contains("hideModal('adminDynamicModal')"));
    }

    #[test]
    fn patch_script_assigns_window() {
        let script = admin_modal_patch_script();
        assert!(script.starts_with("window.__poolaiAdminModalRust="));
        assert!(script.contains("focusable_selector"));
    }

    #[test]
    fn tab_a11y_selected_tab() {
        let attrs = tab_a11y_attrs(true, Some("sec-tab-oauth"));
        assert!(attrs.aria_selected);
        assert_eq!(attrs.tabindex, 0);
        assert_eq!(attrs.aria_labelledby.as_deref(), Some("sec-tab-oauth"));
    }
}
