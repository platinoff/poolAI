//! UI catalog (themes, components) and dashboard orchestration for HTTP handlers.

#[cfg(feature = "enterprise")]
use crate::core::state::ApiContext;
#[cfg(feature = "enterprise")]
use crate::enterprise::monitoring::Dashboard;
#[cfg(feature = "enterprise")]
use crate::services::enterprise_service::{
    DashboardCreateInput, DashboardUpdateInput, EnterpriseMonitoringError, EnterpriseService,
};
use crate::ui::components;
use crate::ui::{get_all_themes, get_theme};
use serde::Serialize;

/// Serializable theme entry for `/ui/themes`.
#[derive(Serialize)]
pub struct UiThemeResponse {
    pub name: String,
    pub css_variables: String,
    pub css: String,
}

/// Serializable component metadata for `/ui/components`.
#[derive(Serialize)]
pub struct UiComponentInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub component_type: String,
    pub styles: String,
    pub description: Option<String>,
}

pub struct UiService;

impl UiService {
    pub fn list_themes() -> Vec<UiThemeResponse> {
        get_all_themes()
            .into_iter()
            .map(|theme| UiThemeResponse {
                name: theme.name.to_string(),
                css_variables: theme.to_css_variables(),
                css: theme.to_css(),
            })
            .collect()
    }

    pub fn theme_by_name(name: &str) -> UiThemeResponse {
        let theme = get_theme(name);
        UiThemeResponse {
            name: theme.name.to_string(),
            css_variables: theme.to_css_variables(),
            css: theme.to_css(),
        }
    }

    pub fn list_components() -> Vec<UiComponentInfo> {
        vec![
            UiComponentInfo {
                name: "button".to_string(),
                component_type: "button".to_string(),
                styles: components::BUTTON_STYLES.to_string(),
                description: Some(
                    "Button component with primary, danger, secondary variants".to_string(),
                ),
            },
            UiComponentInfo {
                name: "card".to_string(),
                component_type: "card".to_string(),
                styles: components::CARD_STYLES.to_string(),
                description: Some("Card component for content containers".to_string()),
            },
            UiComponentInfo {
                name: "form".to_string(),
                component_type: "form".to_string(),
                styles: components::FORM_STYLES.to_string(),
                description: Some("Form component for input fields and validation".to_string()),
            },
        ]
    }

    pub fn get_component(name: &str) -> Option<UiComponentInfo> {
        match name {
            "button" => Some(UiComponentInfo {
                name: "button".to_string(),
                component_type: "button".to_string(),
                styles: components::BUTTON_STYLES.to_string(),
                description: Some(
                    "Button component with primary, danger, secondary variants".to_string(),
                ),
            }),
            "card" => Some(UiComponentInfo {
                name: "card".to_string(),
                component_type: "card".to_string(),
                styles: components::CARD_STYLES.to_string(),
                description: Some("Card component for content containers".to_string()),
            }),
            "form" => Some(UiComponentInfo {
                name: "form".to_string(),
                component_type: "form".to_string(),
                styles: components::FORM_STYLES.to_string(),
                description: Some("Form component for input fields and validation".to_string()),
            }),
            _ => None,
        }
    }

    #[cfg(feature = "enterprise")]
    pub async fn list_dashboards(
        ctx: &ApiContext,
    ) -> Result<Vec<Dashboard>, EnterpriseMonitoringError> {
        EnterpriseService::list_monitoring_dashboards(ctx, None).await
    }

    #[cfg(feature = "enterprise")]
    pub async fn get_dashboard(
        ctx: &ApiContext,
        id: uuid::Uuid,
    ) -> Result<Option<Dashboard>, EnterpriseMonitoringError> {
        EnterpriseService::get_monitoring_dashboard(ctx, id).await
    }

    #[cfg(feature = "enterprise")]
    pub async fn create_dashboard(
        ctx: &ApiContext,
        input: DashboardCreateInput,
    ) -> Result<Dashboard, EnterpriseMonitoringError> {
        EnterpriseService::create_monitoring_dashboard(ctx, input).await
    }

    #[cfg(feature = "enterprise")]
    pub async fn update_dashboard(
        ctx: &ApiContext,
        id: uuid::Uuid,
        input: DashboardUpdateInput,
    ) -> Result<Option<Dashboard>, EnterpriseMonitoringError> {
        EnterpriseService::update_monitoring_dashboard(ctx, id, input).await
    }

    #[cfg(feature = "enterprise")]
    pub async fn delete_dashboard(
        ctx: &ApiContext,
        id: uuid::Uuid,
    ) -> Result<bool, EnterpriseMonitoringError> {
        EnterpriseService::delete_monitoring_dashboard(ctx, id).await
    }
}
