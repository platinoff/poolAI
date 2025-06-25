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
        // Register standard components
        self.register_standard_components().await?;
        
        // Load templates
        self.load_templates().await?;
        
        // Load styles
        self.load_styles().await?;
        
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        // Clear components
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
        
        // Get component configuration
        let config = components.get(component_name)
            .ok_or_else(|| AppError::Model(format!("Component '{}' not found", component_name)))?;
        
        // Get template
        let template = templates.get(&config.template)
            .ok_or_else(|| AppError::Model(format!("Template '{}' not found", config.template)))?;
        
        // Get styles
        let component_styles = styles.get(&format!("{}.css", component_name))
            .unwrap_or(&String::new());
        
        // Render component
        let rendered = self.render_template(template, &data, &config.properties).await?;
        
        // Combine with styles
        let final_html = format!(
            "<style>{}</style>\n{}",
            component_styles,
            rendered
        );
        
        Ok(final_html)
    }

    async fn render_template(&self, template: &str, data: &serde_json::Value, properties: &HashMap<String, serde_json::Value>) -> Result<String, AppError> {
        let mut rendered = template.to_string();
        
        // Replace placeholders with data
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
        
        // Replace component properties
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
        // Register Chart component
        let chart_config = ComponentConfig {
            name: "chart".to_string(),
            component_type: ComponentType::Chart,
            template: "chart_template".to_string(),
            styles: HashMap::new(),
            properties: HashMap::new(),
        };
        self.register_component(chart_config).await?;
        
        // Register Table component
        let table_config = ComponentConfig {
            name: "table".to_string(),
            component_type: ComponentType::Table,
            template: "table_template".to_string(),
            styles: HashMap::new(),
            properties: HashMap::new(),
        };
        self.register_component(table_config).await?;
        
        // Register Metric component
        let metric_config = ComponentConfig {
            name: "metric".to_string(),
            component_type: ComponentType::Metric,
            template: "metric_template".to_string(),
            styles: HashMap::new(),
            properties: HashMap::new(),
        };
        self.register_component(metric_config).await?;
        
        // Register Alert component
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
        
        // Chart template
        templates.insert("chart_template".to_string(), r#"
            <div class="chart-container">
                <canvas id="{{chart_id}}" width="{{width}}" height="{{height}}"></canvas>
                <div class="chart-legend">
                    {{legend}}
                </div>
            </div>
        "#.to_string());
        
        // Table template
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
        
        // Metric template
        templates.insert("metric_template".to_string(), r#"
            <div class="metric-card">
                <div class="metric-title">{{title}}</div>
                <div class="metric-value">{{value}}</div>
                <div class="metric-unit">{{unit}}</div>
                <div class="metric-change {{change_class}}">{{change}}</div>
            </div>
        "#.to_string());
        
        // Alert template
        templates.insert("alert_template".to_string(), r#"
            <div class="alert alert-{{severity}}">
                <div class="alert-icon">{{icon}}</div>
                <div class="alert-content">
                    <div class="alert-title">{{title}}</div>
                    <div class="alert-message">{{message}}</div>
                </div>
                <div class="alert-close" onclick="closeAlert('{{id}}')">×</div>
            </div>
        "#.to_string());
        
        Ok(())
    }

    async fn load_styles(&self) -> Result<(), AppError> {
        let mut styles = self.styles.write().await;
        
        // Chart styles
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
            }
        "#.to_string());
        
        // Table styles
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
            }
        "#.to_string());
        
        // Metric styles
        styles.insert("metric.css".to_string(), r#"
            .metric-card {
                background: #fff;
                padding: 20px;
                border-radius: 8px;
                box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                text-align: center;
            }
            .metric-title {
                font-size: 14px;
                color: #666;
                margin-bottom: 8px;
            }
            .metric-value {
                font-size: 32px;
                font-weight: bold;
                color: #333;
                margin-bottom: 4px;
            }
            .metric-unit {
                font-size: 12px;
                color: #999;
                margin-bottom: 8px;
            }
            .metric-change {
                font-size: 12px;
                font-weight: 500;
            }
            .metric-change.positive {
                color: #28a745;
            }
            .metric-change.negative {
                color: #dc3545;
            }
        "#.to_string());
        
        // Alert styles
        styles.insert("alert.css".to_string(), r#"
            .alert {
                display: flex;
                align-items: center;
                padding: 12px 16px;
                margin: 8px 0;
                border-radius: 6px;
                border-left: 4px solid;
            }
            .alert-info {
                background: #e3f2fd;
                border-color: #2196f3;
                color: #1976d2;
            }
            .alert-warning {
                background: #fff3e0;
                border-color: #ff9800;
                color: #f57c00;
            }
            .alert-error {
                background: #ffebee;
                border-color: #f44336;
                color: #d32f2f;
            }
            .alert-success {
                background: #e8f5e8;
                border-color: #4caf50;
                color: #388e3c;
            }
            .alert-icon {
                margin-right: 12px;
                font-size: 18px;
            }
            .alert-content {
                flex: 1;
            }
            .alert-title {
                font-weight: 600;
                margin-bottom: 4px;
            }
            .alert-message {
                font-size: 14px;
            }
            .alert-close {
                cursor: pointer;
                font-size: 18px;
                font-weight: bold;
                margin-left: 12px;
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