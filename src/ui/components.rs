use crate::core::error::AppError;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ComponentConfig {
    pub name: String,
    pub component_type: ComponentType,
    pub template: String,
    pub styles: HashMap<String, String>,
    pub properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub enum ComponentType {
    Chart,
    Table,
    Form,
    Modal,
    Button,
    Card,
    Alert,
    Progress,
    Metric,
}

#[derive(Debug, Clone)]
pub struct ComponentData {
    pub id: String,
    pub config: ComponentConfig,
    pub data: serde_json::Value,
    pub state: HashMap<String, serde_json::Value>,
}

pub struct ComponentManager {
    components: Arc<RwLock<HashMap<String, ComponentConfig>>>,
    templates: Arc<RwLock<HashMap<String, String>>>,
    styles: Arc<RwLock<HashMap<String, String>>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            components: Arc::new(RwLock::new(HashMap::new())),
            templates: Arc::new(RwLock::new(HashMap::new())),
            styles: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn initialize(&self) -> Result<(), AppError> {
        // Регистрация стандартных компонентов
        self.register_standard_components().await?;
        
        // Загрузка шаблонов
        self.load_templates().await?;
        
        // Загрузка стилей
        self.load_styles().await?;
        
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        // Очистка компонентов
        self.components.write().await.clear();
        self.templates.write().await.clear();
        self.styles.write().await.clear();
        
        Ok(())
    }

    pub async fn register_component(&self, config: ComponentConfig) -> Result<(), AppError> {
        let mut components = self.components.write().await;
        components.insert(config.name.clone(), config);
        
        Ok(())
    }

    pub async fn render(&self, component_name: &str, data: serde_json::Value) -> Result<String, AppError> {
        let components = self.components.read().await;
        let templates = self.templates.read().await;
        let styles = self.styles.read().await;
        
        // Получение конфигурации компонента
        let config = components.get(component_name)
            .ok_or(AppError::ComponentNotFound)?;
        
        // Получение шаблона
        let template = templates.get(&config.template)
            .ok_or(AppError::TemplateNotFound)?;
        
        // Получение стилей
        let component_styles = styles.get(&format!("{}.css", component_name))
            .unwrap_or(&String::new());
        
        // Рендеринг компонента
        let rendered = self.render_template(template, &data, &config.properties).await?;
        
        // Объединение с стилями
        let final_html = format!(
            "<style>{}</style>\n{}",
            component_styles,
            rendered
        );
        
        Ok(final_html)
    }

    async fn render_template(&self, template: &str, data: &serde_json::Value, properties: &HashMap<String, serde_json::Value>) -> Result<String, AppError> {
        let mut rendered = template.to_string();
        
        // Замена плейсхолдеров данными
        if let Some(obj) = data.as_object() {
            for (key, value) in obj {
                let placeholder = format!("{{{{{}}}}}", key);
                let value_str = match value {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    _ => value.to_string(),
                };
                rendered = rendered.replace(&placeholder, &value_str);
            }
        }
        
        // Замена свойств компонента
        for (key, value) in properties {
            let placeholder = format!("{{{{{}}}}}", key);
            let value_str = match value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                _ => value.to_string(),
            };
            rendered = rendered.replace(&placeholder, &value_str);
        }
        
        Ok(rendered)
    }

    async fn register_standard_components(&self) -> Result<(), AppError> {
        // Регистрация компонента Chart
        let chart_config = ComponentConfig {
            name: "chart".to_string(),
            component_type: ComponentType::Chart,
            template: "chart_template".to_string(),
            styles: HashMap::new(),
            properties: HashMap::new(),
        };
        self.register_component(chart_config).await?;
        
        // Регистрация компонента Table
        let table_config = ComponentConfig {
            name: "table".to_string(),
            component_type: ComponentType::Table,
            template: "table_template".to_string(),
            styles: HashMap::new(),
            properties: HashMap::new(),
        };
        self.register_component(table_config).await?;
        
        // Регистрация компонента Metric
        let metric_config = ComponentConfig {
            name: "metric".to_string(),
            component_type: ComponentType::Metric,
            template: "metric_template".to_string(),
            styles: HashMap::new(),
            properties: HashMap::new(),
        };
        self.register_component(metric_config).await?;
        
        // Регистрация компонента Alert
        let alert_config = ComponentConfig {
            name: "alert".to_string(),
            component_type: ComponentType::Alert,
            template: "alert_template".to_string(),
            styles: HashMap::new(),
            properties: HashMap::new(),
        };
        self.register_component(alert_config).await?;
        
        Ok(())
    }

    async fn load_templates(&self) -> Result<(), AppError> {
        let mut templates = self.templates.write().await;
        
        // Шаблон для графика
        templates.insert("chart_template".to_string(), r#"
            <div class="chart-container">
                <canvas id="{{chart_id}}" width="{{width}}" height="{{height}}"></canvas>
                <div class="chart-legend">
                    {{legend}}
                </div>
            </div>
        "#.to_string());
        
        // Шаблон для таблицы
        templates.insert("table_template".to_string(), r#"
            <div class="table-container">
                <table class="data-table">
                    <thead>
                        <tr>
                            {{#each headers}}
                            <th>{{this}}</th>
                            {{/each}}
                        </tr>
                    </thead>
                    <tbody>
                        {{#each rows}}
                        <tr>
                            {{#each this}}
                            <td>{{this}}</td>
                            {{/each}}
                        </tr>
                        {{/each}}
                    </tbody>
                </table>
            </div>
        "#.to_string());
        
        // Шаблон для метрики
        templates.insert("metric_template".to_string(), r#"
            <div class="metric-card">
                <div class="metric-icon">
                    <i class="{{icon}}"></i>
                </div>
                <div class="metric-content">
                    <div class="metric-value">{{value}}</div>
                    <div class="metric-label">{{label}}</div>
                    <div class="metric-change {{change_class}}">
                        {{change_value}}
                    </div>
                </div>
            </div>
        "#.to_string());
        
        // Шаблон для алерта
        templates.insert("alert_template".to_string(), r#"
            <div class="alert alert-{{type}}">
                <div class="alert-icon">
                    <i class="{{icon}}"></i>
                </div>
                <div class="alert-content">
                    <div class="alert-title">{{title}}</div>
                    <div class="alert-message">{{message}}</div>
                </div>
                <div class="alert-close">
                    <button onclick="closeAlert('{{id}}')">&times;</button>
                </div>
            </div>
        "#.to_string());
        
        Ok(())
    }

    async fn load_styles(&self) -> Result<(), AppError> {
        let mut styles = self.styles.write().await;
        
        // Стили для графика
        styles.insert("chart.css".to_string(), r#"
            .chart-container {
                position: relative;
                margin: 20px 0;
                padding: 20px;
                background: #fff;
                border-radius: 8px;
                box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            }
            
            .chart-legend {
                margin-top: 10px;
                text-align: center;
                font-size: 12px;
                color: #666;
            }
        "#.to_string());
        
        // Стили для таблицы
        styles.insert("table.css".to_string(), r#"
            .table-container {
                overflow-x: auto;
                margin: 20px 0;
            }
            
            .data-table {
                width: 100%;
                border-collapse: collapse;
                background: #fff;
                border-radius: 8px;
                overflow: hidden;
                box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            }
            
            .data-table th,
            .data-table td {
                padding: 12px;
                text-align: left;
                border-bottom: 1px solid #eee;
            }
            
            .data-table th {
                background: #f8f9fa;
                font-weight: 600;
                color: #333;
            }
            
            .data-table tr:hover {
                background: #f5f5f5;
            }
        "#.to_string());
        
        // Стили для метрики
        styles.insert("metric.css".to_string(), r#"
            .metric-card {
                display: flex;
                align-items: center;
                padding: 20px;
                background: #fff;
                border-radius: 8px;
                box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                margin: 10px 0;
            }
            
            .metric-icon {
                margin-right: 15px;
                font-size: 24px;
                color: #007bff;
            }
            
            .metric-content {
                flex: 1;
            }
            
            .metric-value {
                font-size: 24px;
                font-weight: bold;
                color: #333;
            }
            
            .metric-label {
                font-size: 14px;
                color: #666;
                margin-top: 5px;
            }
            
            .metric-change {
                font-size: 12px;
                margin-top: 5px;
            }
            
            .metric-change.positive {
                color: #28a745;
            }
            
            .metric-change.negative {
                color: #dc3545;
            }
        "#.to_string());
        
        // Стили для алерта
        styles.insert("alert.css".to_string(), r#"
            .alert {
                display: flex;
                align-items: center;
                padding: 15px;
                margin: 10px 0;
                border-radius: 6px;
                border-left: 4px solid;
            }
            
            .alert-info {
                background: #d1ecf1;
                border-color: #17a2b8;
                color: #0c5460;
            }
            
            .alert-warning {
                background: #fff3cd;
                border-color: #ffc107;
                color: #856404;
            }
            
            .alert-error {
                background: #f8d7da;
                border-color: #dc3545;
                color: #721c24;
            }
            
            .alert-success {
                background: #d4edda;
                border-color: #28a745;
                color: #155724;
            }
            
            .alert-icon {
                margin-right: 10px;
                font-size: 18px;
            }
            
            .alert-content {
                flex: 1;
            }
            
            .alert-title {
                font-weight: bold;
                margin-bottom: 5px;
            }
            
            .alert-message {
                font-size: 14px;
            }
            
            .alert-close button {
                background: none;
                border: none;
                font-size: 18px;
                cursor: pointer;
                color: inherit;
                opacity: 0.7;
            }
            
            .alert-close button:hover {
                opacity: 1;
            }
        "#.to_string());
        
        Ok(())
    }

    pub async fn get_component_list(&self) -> Vec<String> {
        let components = self.components.read().await;
        components.keys().cloned().collect()
    }

    pub async fn get_component_config(&self, name: &str) -> Option<ComponentConfig> {
        let components = self.components.read().await;
        components.get(name).cloned()
    }
} 