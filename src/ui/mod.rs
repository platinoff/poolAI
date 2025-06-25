pub mod dashboard;
pub mod components;

use crate::core::error::AppError;
use crate::monitoring::SystemStatus;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct UiConfig {
    pub host: String,
    pub port: u16,
    pub enable_websocket: bool,
    pub auto_refresh_interval_ms: u64,
    pub theme: UiTheme,
    pub enable_dark_mode: bool,
}

#[derive(Debug, Clone)]
pub enum UiTheme {
    Light,
    Dark,
    Auto,
}

#[derive(Debug, Clone)]
pub struct UiState {
    pub current_page: String,
    pub user_preferences: HashMap<String, String>,
    pub notifications: Vec<UiNotification>,
    pub system_status: Option<SystemStatus>,
}

#[derive(Debug, Clone)]
pub struct UiNotification {
    pub id: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub timestamp: std::time::Instant,
    pub read: bool,
}

#[derive(Debug, Clone)]
pub enum NotificationType {
    Info,
    Warning,
    Error,
    Success,
}

pub struct Ui {
    config: UiConfig,
    state: Arc<RwLock<UiState>>,
    dashboard: Option<dashboard::Dashboard>,
    components: components::ComponentManager,
}

impl Ui {
    pub fn new(config: UiConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(UiState {
                current_page: "dashboard".to_string(),
                user_preferences: HashMap::new(),
                notifications: Vec::new(),
                system_status: None,
            })),
            dashboard: None,
            components: components::ComponentManager::new(),
        }
    }

    pub async fn initialize(&mut self) -> Result<(), AppError> {
        // Инициализация UI компонентов
        self.components.initialize().await?;
        
        // Создание дашборда
        self.dashboard = Some(dashboard::Dashboard::new(self.state.clone()).await?);
        
        // Запуск фоновых задач
        self.start_background_tasks().await?;
        
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        // Выключение UI
        if let Some(dashboard) = &self.dashboard {
            dashboard.shutdown().await?;
        }
        
        self.components.shutdown().await?;
        
        Ok(())
    }

    pub async fn update_system_status(&self, status: SystemStatus) -> Result<(), AppError> {
        let mut state = self.state.write().await;
        state.system_status = Some(status);
        
        // Обновление дашборда
        if let Some(dashboard) = &self.dashboard {
            dashboard.update_status(status).await?;
        }
        
        Ok(())
    }

    pub async fn add_notification(&self, notification: UiNotification) -> Result<(), AppError> {
        let mut state = self.state.write().await;
        state.notifications.push(notification);
        
        // Ограничение количества уведомлений
        if state.notifications.len() > 100 {
            state.notifications.drain(0..10);
        }
        
        Ok(())
    }

    pub async fn mark_notification_read(&self, notification_id: &str) -> Result<(), AppError> {
        let mut state = self.state.write().await;
        
        if let Some(notification) = state.notifications.iter_mut().find(|n| n.id == notification_id) {
            notification.read = true;
        }
        
        Ok(())
    }

    pub async fn get_notifications(&self) -> Vec<UiNotification> {
        let state = self.state.read().await;
        state.notifications.clone()
    }

    pub async fn get_unread_notifications(&self) -> Vec<UiNotification> {
        let state = self.state.read().await;
        state.notifications.iter()
            .filter(|n| !n.read)
            .cloned()
            .collect()
    }

    pub async fn set_user_preference(&self, key: String, value: String) -> Result<(), AppError> {
        let mut state = self.state.write().await;
        state.user_preferences.insert(key, value);
        
        Ok(())
    }

    pub async fn get_user_preference(&self, key: &str) -> Option<String> {
        let state = self.state.read().await;
        state.user_preferences.get(key).cloned()
    }

    pub async fn navigate_to_page(&self, page: String) -> Result<(), AppError> {
        let mut state = self.state.write().await;
        state.current_page = page;
        
        Ok(())
    }

    pub async fn get_current_page(&self) -> String {
        let state = self.state.read().await;
        state.current_page.clone()
    }

    pub async fn get_dashboard_data(&self) -> Result<dashboard::DashboardData, AppError> {
        if let Some(dashboard) = &self.dashboard {
            dashboard.get_data().await
        } else {
            Err(AppError::ComponentNotInitialized)
        }
    }

    pub async fn render_component(&self, component_name: &str, data: serde_json::Value) -> Result<String, AppError> {
        self.components.render(component_name, data).await
    }

    async fn start_background_tasks(&self) -> Result<(), AppError> {
        let state = self.state.clone();
        let auto_refresh_interval = self.config.auto_refresh_interval_ms;
        
        // Задача автообновления UI
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(auto_refresh_interval));
            
            loop {
                interval.tick().await;
                
                // Обновление UI состояния
                let mut state_write = state.write().await;
                
                // Очистка старых уведомлений
                let now = std::time::Instant::now();
                state_write.notifications.retain(|n| {
                    now.duration_since(n.timestamp).as_secs() < 3600 // 1 час
                });
            }
        });
        
        Ok(())
    }
} 